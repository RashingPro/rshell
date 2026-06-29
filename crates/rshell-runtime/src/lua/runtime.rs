use crate::component::Component;
use crate::lua::globals::init_globals;
use mlua::{Lua, LuaOptions, StdLib};
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::spawn;
use tokio::task::JoinHandle;

pub struct ConfigRuntime {
    task_handle: JoinHandle<()>,
    collector: Arc<RwLock<ConfigRuntimeCollector>>
}

impl ConfigRuntime {
    pub fn new(config: PathBuf) -> Self {
        let lua = Lua::new_with(StdLib::NONE, LuaOptions::default())
            .expect("Failed to create Lua runtime");

        let collector = Arc::new(RwLock::new(ConfigRuntimeCollector::new()));

        init_globals(&lua, collector.clone());

        Self {
            task_handle: spawn(Self::run(lua, config)),
            collector
        }
    }

    async fn run(lua: Lua, config: PathBuf) {
        if let Err(error) = lua.load(config).exec_async().await {
            println!("Config execution error:\n{}", error);
        }
    }

    pub async fn join(self) -> ConfigRuntimeCollector {
        self.task_handle.await.unwrap();
        Arc::into_inner(self.collector).unwrap().into_inner()
    }
}

pub struct ConfigRuntimeCollector {
    render_components: Vec<Component>
}

impl ConfigRuntimeCollector {
    pub fn new() -> Self {
        Self {
            render_components: Vec::new()
        }
    }

    pub fn collect_render(&mut self, root: Component) {
        self.render_components.push(root);
    }

    pub fn read(self) -> Vec<Component> {
        self.render_components
    }
}
