use colored::Colorize;
use mcp_passport::config::Config;
use tracing::info;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    // Log files can contain session metadata: keep the directory private.
    let log_dir = config.resolved_log_dir();
    if let Err(e) = mcp_passport::logging::prepare_log_dir(&log_dir) {
        eprintln!(
            "{} Failed to prepare log directory at {}: {:#}",
            "error:".red().bold(),
            log_dir.display(),
            e
        );
        std::process::exit(1);
    }

    let file_appender = tracing_appender::rolling::never(&log_dir, "mcp-passport.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    // Initialize tracing with dual outputs: stderr (for human/client) and file (for debugging)
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(fmt::layer().with_ansi(false).with_writer(non_blocking))
        .init();

    info!(
        "mcp-passport starting up (logs in {})...",
        log_dir.display()
    );
    info!("Configuration: {:?}", config);

    if let Err(e) = mcp_passport::run(config, tokio::io::stdin(), tokio::io::stdout()).await {
        eprintln!("{} {:#}", "error:".red().bold(), e);

        let err_str = format!("{:#}", e);
        if err_str.contains("must use HTTPS") {
            eprintln!(
                "{} Use an {} URL, or pass {} for local development.",
                "hint:".blue().bold(),
                "https://".cyan(),
                "--allow-insecure-http".cyan()
            );
        } else if err_str.contains("already in use") {
            eprintln!(
                "{} The local callback port is busy. Try closing other instances of mcp-passport.",
                "hint:".blue().bold()
            );
        }

        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_main_help() {
        // We can't easily test main() because it calls Config::parse() which might exit.
        // But we can test that it compiles and has basic structure.
    }
}
