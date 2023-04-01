use anyhow::Result;
use tracing_log::LogTracer;

use crate::config::LogConfig;

pub struct Logger;

impl Logger {
    pub fn new(LogConfig { level, file: _ }: LogConfig) -> Result<Self> {
        tracing::subscriber::set_global_default(
            tracing_subscriber::FmtSubscriber::builder()
                .with_env_filter(level)
                .with_line_number(true)
                .with_file(true)
                //.json()
                .finish(),
        )?;

        LogTracer::init()?;

        Ok(Self)
    }
}
