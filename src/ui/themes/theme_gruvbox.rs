use crate::config::{
    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn model_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Model,
        enabled: true,
        icon: IconConfig {
            plain: "🤖".to_string(),
            nerd_font: "\u{e26d}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 208 }), // Gruvbox orange
            text: Some(AnsiColor::Color256 { c256: 208 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn directory_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Directory,
        enabled: true,
        icon: IconConfig {
            plain: "📁".to_string(),
            nerd_font: "\u{f024b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 142 }), // Gruvbox green
            text: Some(AnsiColor::Color256 { c256: 142 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn git_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Git,
        enabled: true,
        icon: IconConfig {
            plain: "🌿".to_string(),
            nerd_font: "\u{f02a2}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 109 }), // Gruvbox cyan
            text: Some(AnsiColor::Color256 { c256: 109 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: {
            let mut opts = HashMap::new();
            opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
            opts
        },
    }
}

pub fn context_window_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::ContextWindow,
        enabled: true,
        icon: IconConfig {
            plain: "⚡️".to_string(),
            nerd_font: "\u{f49b}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 5 }),
            text: Some(AnsiColor::Color16 { c16: 5 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cost,
        enabled: false,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{eec1}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 214 }), // Gruvbox yellow
            text: Some(AnsiColor::Color256 { c256: 214 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn newapi_cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::NewApiCost,
        enabled: false,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{f0d6f}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 214 }), // Gruvbox yellow
            text: Some(AnsiColor::Color256 { c256: 214 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: {
            let mut opts = HashMap::new();
            opts.insert("base_url".to_string(), serde_json::Value::String("".to_string()));
            opts.insert("user_token".to_string(), serde_json::Value::String("".to_string()));
            opts.insert("user_id".to_string(), serde_json::Value::String("".to_string()));
            opts.insert("token_name".to_string(), serde_json::Value::String("".to_string()));
            opts.insert("provider".to_string(), serde_json::Value::String("".to_string()));
            opts.insert("timeout".to_string(), serde_json::Value::Number(5.into()));
            opts
        },
    }
}

pub fn session_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Session,
        enabled: false,
        icon: IconConfig {
            plain: "⏱️".to_string(),
            nerd_font: "\u{f19bb}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 142 }), // Gruvbox green
            text: Some(AnsiColor::Color256 { c256: 142 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn output_style_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OutputStyle,
        enabled: false,
        icon: IconConfig {
            plain: "🎯".to_string(),
            nerd_font: "\u{f12f5}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color256 { c256: 109 }), // Gruvbox cyan
            text: Some(AnsiColor::Color256 { c256: 109 }),
            background: None,
        },
        styles: TextStyleConfig { text_bold: true },
        options: HashMap::new(),
    }
}

pub fn usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage,
        enabled: false,
        icon: IconConfig {
            plain: "📊".to_string(),
            nerd_font: "\u{f0a9e}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Color16 { c16: 14 }),
            text: Some(AnsiColor::Color16 { c16: 14 }),
            background: None,
        },
        styles: TextStyleConfig::default(),
        options: {
            let mut opts = HashMap::new();
            opts.insert(
                "api_base_url".to_string(),
                serde_json::Value::String("https://api.anthropic.com".to_string()),
            );
            opts.insert(
                "cache_duration".to_string(),
                serde_json::Value::Number(180.into()),
            );
            opts.insert("timeout".to_string(), serde_json::Value::Number(2.into()));
            opts
        },
    }
}
