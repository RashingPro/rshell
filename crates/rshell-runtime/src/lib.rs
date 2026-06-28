mod lua;

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
        self.config.join().await;
    }
}
