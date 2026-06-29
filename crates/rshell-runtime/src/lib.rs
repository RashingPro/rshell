mod component;
mod lua;

use log::info;
use lua::runtime::ConfigRuntime;
use std::path::PathBuf;

pub struct Runtime {
    config: ConfigRuntime
}

impl Runtime {
    pub fn new(config: PathBuf) -> Self {
        Self {
            config: ConfigRuntime::new(config)
        }
    }

    pub async fn join(self) {
        let collector = self.config.join().await;
        let render_components = collector.read();
        info!(
            "Config executed successfully. Collected {} component(s) to render.",
            render_components.len()
        );
    }
}
