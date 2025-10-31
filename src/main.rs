use ccometixline::cli::Cli;
use ccometixline::config::{Config, InputData};
use ccometixline::core::{collect_all_segments, StatusLineGenerator};
use std::io::{self, IsTerminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_args();

    // Handle configuration commands
    if cli.init {
        Config::init()?;
        return Ok(());
    }

    if cli.print {
        let mut config = Config::load().unwrap_or_else(|_| Config::default());

        // Apply theme override if provided
        if let Some(theme) = cli.theme {
            config = ccometixline::ui::themes::ThemePresets::get_theme(&theme);
        }

        config.print()?;
        return Ok(());
    }

    if cli.check {
        let config = Config::load()?;
        config.check()?;
        println!("✓ Configuration valid");
        return Ok(());
    }

    if cli.config {
        #[cfg(feature = "tui")]
        {
            ccometixline::ui::run_configurator()?;
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("TUI feature is not enabled. Please install with --features tui");
            std::process::exit(1);
        }
        return Ok(());
    }

    if cli.update {
        #[cfg(feature = "self-update")]
        {
            println!("Update feature not implemented in new architecture yet");
        }
        #[cfg(not(feature = "self-update"))]
        {
            println!("Update check not available (self-update feature disabled)");
        }
        return Ok(());
    }

    // Handle Claude Code patcher
    if let Some(claude_path) = cli.patch {
        use ccometixline::utils::ClaudeCodePatcher;

        println!("🔧 Claude Code Context Warning Disabler");
        println!("Target file: {}", claude_path);

        // Create backup in same directory
        let backup_path = format!("{}.backup", claude_path);
        std::fs::copy(&claude_path, &backup_path)?;
        println!("📦 Created backup: {}", backup_path);

        // Load and patch
        let mut patcher = ClaudeCodePatcher::new(&claude_path)?;

        // Apply all modifications
        println!("\n🔄 Applying patches...");

        // 1. Set verbose property to true
        if let Err(e) = patcher.write_verbose_property(true) {
            println!("⚠️ Could not modify verbose property: {}", e);
        }

        // 2. Disable context low warnings
        patcher.disable_context_low_warnings()?;

        // 3. Disable ESC interrupt display
        if let Err(e) = patcher.disable_esc_interrupt_display() {
            println!("⚠️ Could not disable esc/interrupt display: {}", e);
        }

        patcher.save()?;

        println!("✅ All patches applied successfully!");
        println!("💡 To restore warnings, replace your cli.js with the backup file:");
        println!("   cp {} {}", backup_path, claude_path);

        return Ok(());
    }

    // Load configuration
    let mut config = Config::load().unwrap_or_else(|_| Config::default());

    // Apply theme override if provided
    if let Some(theme) = cli.theme {
        config = ccometixline::ui::themes::ThemePresets::get_theme(&theme);
    }

    // Apply NewApi Cost CLI parameter overrides
    if cli.newapi_base_url.is_some()
        || cli.newapi_user_token.is_some()
        || cli.newapi_user_id.is_some()
        || cli.newapi_token_name.is_some()
        || cli.newapi_provider.is_some()
        || cli.newapi_quota_per_unit.is_some()
    {
        if let Some(segment_config) = config
            .segments
            .iter_mut()
            .find(|s| s.id == ccometixline::config::SegmentId::NewApiCost)
        {
            if let Some(base_url) = &cli.newapi_base_url {
                segment_config
                    .options
                    .insert("base_url".to_string(), serde_json::json!(base_url));
            }
            if let Some(user_token) = &cli.newapi_user_token {
                segment_config
                    .options
                    .insert("user_token".to_string(), serde_json::json!(user_token));
            }
            if let Some(user_id) = &cli.newapi_user_id {
                segment_config
                    .options
                    .insert("user_id".to_string(), serde_json::json!(user_id));
            }
            if let Some(token_name) = &cli.newapi_token_name {
                segment_config
                    .options
                    .insert("token_name".to_string(), serde_json::json!(token_name));
            }
            if let Some(provider) = &cli.newapi_provider {
                segment_config
                    .options
                    .insert("provider".to_string(), serde_json::json!(provider));
            }
            if let Some(quota_per_unit) = cli.newapi_quota_per_unit {
                segment_config
                    .options
                    .insert("quota_per_unit".to_string(), serde_json::json!(quota_per_unit));
            }
        }
    }

    // Check if stdin has data
    if io::stdin().is_terminal() {
        // No input data available, show main menu
        #[cfg(feature = "tui")]
        {
            use ccometixline::ui::{MainMenu, MenuResult};

            if let Some(result) = MainMenu::run()? {
                match result {
                    MenuResult::LaunchConfigurator => {
                        ccometixline::ui::run_configurator()?;
                    }
                    MenuResult::InitConfig => {
                        ccometixline::config::Config::init()?;
                        println!("Configuration initialized successfully!");
                    }
                    MenuResult::CheckConfig => {
                        let config = ccometixline::config::Config::load()?;
                        config.check()?;
                        println!("Configuration is valid!");
                    }
                    MenuResult::Exit => {
                        // Exit gracefully
                    }
                }
            }
        }
        #[cfg(not(feature = "tui"))]
        {
            eprintln!("No input data provided and TUI feature is not enabled.");
            eprintln!("Usage: echo '{{...}}' | ccline");
            eprintln!("   or: ccline --help");
        }
        return Ok(());
    }

    // Read Claude Code data from stdin
    let stdin = io::stdin();
    let input: InputData = serde_json::from_reader(stdin.lock())?;

    // Collect segment data
    let segments_data = collect_all_segments(&config, &input);

    // Render statusline
    let generator = StatusLineGenerator::new(config);
    let statusline = generator.generate(segments_data);

    println!("{}", statusline);

    Ok(())
}
