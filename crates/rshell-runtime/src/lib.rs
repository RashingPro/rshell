mod component;
pub mod error;
mod lua;
mod render;

use crate::error::Result;
use crate::render::runtime::RenderRuntime;
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

    pub async fn run(self) -> Result<()> {
        let (lua, collector) = self.config.run().await?;
        let components = collector.read();
        info!(
            "Config executed successfully. Collected {} component(s).",
            components.len()
        );

        let render_runtime = RenderRuntime::new(lua, components);
        render_runtime.run().await?;

        Ok(())
    }
}
