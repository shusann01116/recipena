use anyhow::Context;
use tracing_subscriber::layer::SubscriberExt;

use crate::config::AppConfig;
use crate::prelude::*;

pub fn init_logger(config: &AppConfig) -> Result<()> {
    let stack_driver = if config.debug {
        None
    } else {
        Some(tracing_stackdriver::layer())
    };

    let fmt_layer = if config.debug {
        Some(tracing_subscriber::fmt::layer())
    } else {
        None
    };

    let subscriber = tracing_subscriber::Registry::default()
        .with(stack_driver)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .with_context(|| format!("Failed to set global default logger"))?;
    Ok(())
}
