use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use chrono::{Local, Timelike};
use serde::Deserialize;
use std::collections::HashMap;

/// NewApi API response structure
#[derive(Debug, Deserialize)]
struct NewApiStatResponse {
    success: bool,
    #[allow(dead_code)]
    message: String,
    data: NewApiStatData,
}

/// NewApi stat data structure
#[derive(Debug, Deserialize)]
struct NewApiStatData {
    quota: i64,
    #[allow(dead_code)]
    rpm: Option<i64>,
    #[allow(dead_code)]
    tpm: Option<i64>,
}

/// NewApi Cost segment for displaying today's consumption
#[derive(Debug, Clone)]
pub struct NewApiCostSegment {
    pub base_url: Option<String>,
    pub user_token: Option<String>,
    pub user_id: Option<String>,
    pub token_name: Option<String>,
    pub provider: Option<String>,
}

impl Default for NewApiCostSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl NewApiCostSegment {
    pub fn new() -> Self {
        Self {
            base_url: None,
            user_token: None,
            user_id: None,
            token_name: None,
            provider: None,
        }
    }

    /// Load configuration from segment options HashMap
    pub fn with_config_from_options(mut self, options: &HashMap<String, serde_json::Value>) -> Self {
        if let Some(value) = options.get("base_url") {
            self.base_url = value.as_str().map(|s| s.to_string());
        }
        if let Some(value) = options.get("user_token") {
            self.user_token = value.as_str().map(|s| s.to_string());
        }
        if let Some(value) = options.get("user_id") {
            self.user_id = value.as_str().map(|s| s.to_string());
        }
        if let Some(value) = options.get("token_name") {
            self.token_name = value.as_str().map(|s| s.to_string());
        }
        if let Some(value) = options.get("provider") {
            self.provider = value.as_str().map(|s| s.to_string());
        }
        self
    }

    /// Builder method for base_url (used for CLI override)
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Builder method for user_token (used for CLI override)
    pub fn with_user_token(mut self, user_token: String) -> Self {
        self.user_token = Some(user_token);
        self
    }

    /// Builder method for user_id (used for CLI override)
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Builder method for token_name (used for CLI override)
    pub fn with_token_name(mut self, token_name: String) -> Self {
        self.token_name = Some(token_name);
        self
    }

    /// Builder method for provider (used for CLI override)
    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Get today's start and end timestamps (seconds since epoch)
    /// Returns (start_of_today, current_time)
    fn get_today_timestamps() -> (i64, i64) {
        let now = Local::now();

        // Set time to 00:00:00 for start of today
        let start_of_day = now
            .with_hour(0)
            .and_then(|dt| dt.with_minute(0))
            .and_then(|dt| dt.with_second(0))
            .and_then(|dt| dt.with_nanosecond(0))
            .unwrap_or(now);

        (start_of_day.timestamp(), now.timestamp())
    }

    /// Fetch today's cost data from NewApi
    fn fetch_today_cost(&self) -> Option<f64> {
        // Validate required fields
        let base_url = self.base_url.as_ref()?;
        let user_token = self.user_token.as_ref()?;
        let user_id = self.user_id.as_ref()?;

        // Get today's timestamps
        let (start_timestamp, end_timestamp) = Self::get_today_timestamps();

        // Build query parameters
        let mut url = format!(
            "{}/api/log/self/stat?start_timestamp={}&end_timestamp={}&type=2",
            base_url, start_timestamp, end_timestamp
        );

        // Add token_name if provided
        if let Some(token_name) = &self.token_name {
            if !token_name.is_empty() {
                url.push_str(&format!("&token_name={}", token_name));
            }
        }

        // Get timeout from config (default 5 seconds)
        let timeout_secs = self.get_timeout_from_config().unwrap_or(5);

        // Build HTTP client
        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .build();

        // Send GET request
        let response = agent
            .get(&url)
            .set("Content-Type", "application/json")
            .set("Authorization", &format!("Bearer {}", user_token))
            .set("New-Api-User", user_id)
            .call()
            .ok()?;

        // Check status code
        if response.status() != 200 {
            return None;
        }

        // Parse response
        let api_response: NewApiStatResponse = response.into_json().ok()?;

        // Check success flag
        if !api_response.success {
            return None;
        }

        // Calculate cost: quota / 500000
        let cost = api_response.data.quota as f64 / 500000.0;

        Some(cost)
    }

    /// Get timeout configuration from segment options
    fn get_timeout_from_config(&self) -> Option<u64> {
        let config = crate::config::Config::load().ok()?;
        let segment_config = config
            .segments
            .iter()
            .find(|s| s.id == SegmentId::NewApiCost)?;

        segment_config
            .options
            .get("timeout")
            .and_then(|v| v.as_u64())
    }
}

impl Segment for NewApiCostSegment {
    fn collect(&self, _input: &InputData) -> Option<SegmentData> {
        // Fetch today's cost from API
        let cost = self.fetch_today_cost()?;

        // Primary display: today's cost
        let primary = if cost == 0.0 || cost < 0.01 {
            "¥0".to_string()
        } else {
            format!("¥{:.2}", cost)
        };

        // Secondary display: could be used for additional info (e.g., provider name)
        let secondary = self.provider.clone().unwrap_or_default();

        // Store metadata
        let mut metadata = HashMap::new();
        metadata.insert("cost".to_string(), cost.to_string());
        if let Some(provider) = &self.provider {
            metadata.insert("provider".to_string(), provider.clone());
        }

        Some(SegmentData {
            primary,
            secondary,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::NewApiCost
    }
}
