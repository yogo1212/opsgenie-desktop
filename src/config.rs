use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// OpsGenie API Key
    #[arg(short = 'k', long = "api-key", value_name = "API_KEY")]
    pub api_key: String,

    /// Oldest closed alert to retrieve
    #[arg(long = "history", value_name = "DAYS", default_value = "7")]
    pub history_days: u8,

    /// How long to wait between queries in seconds
    #[arg(short = 'i', long = "interval", value_name = "SECONDS", default_value = "60")]
    pub update_interval: u8,

    /// Number of alerts per request
    #[arg(long = "request-limit", value_name = "COUNT", default_value = "100")]
    pub request_limit: u8,

    /// OpsGenie API base path to use
    #[arg(long = "base-path", value_name = "URL", default_value = "https://api.opsgenie.com")]
    pub base_path: String,
}

impl Config {
    pub(crate) fn from_clap() -> Self {
        Config::parse()
    }
}
