use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ccline")]
#[command(version, about = "High-performance Claude Code StatusLine")]
pub struct Cli {
    /// Enter TUI configuration mode
    #[arg(short = 'c', long = "config")]
    pub config: bool,

    /// Set theme
    #[arg(short = 't', long = "theme")]
    pub theme: Option<String>,

    /// Print current configuration
    #[arg(long = "print")]
    pub print: bool,

    /// Initialize config file
    #[arg(long = "init")]
    pub init: bool,

    /// Check configuration
    #[arg(long = "check")]
    pub check: bool,

    /// Check for updates
    #[arg(short = 'u', long = "update")]
    pub update: bool,

    /// Patch Claude Code cli.js to disable context warnings
    #[arg(long = "patch")]
    pub patch: Option<String>,

    /// NewApi Cost: Base URL for API
    #[arg(long = "newapi-base-url")]
    pub newapi_base_url: Option<String>,

    /// NewApi Cost: User token for authentication
    #[arg(long = "newapi-user-token")]
    pub newapi_user_token: Option<String>,

    /// NewApi Cost: User ID
    #[arg(long = "newapi-user-id")]
    pub newapi_user_id: Option<String>,

    /// NewApi Cost: Token name
    #[arg(long = "newapi-token-name")]
    pub newapi_token_name: Option<String>,

    /// NewApi Cost: Provider name
    #[arg(long = "newapi-provider")]
    pub newapi_provider: Option<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
