use crate::lua::globals::init_globals;
use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;
use tokio::spawn;
use tokio::task::JoinHandle;

pub struct ConfigRuntime {
    task_handle: JoinHandle<()>
}

impl ConfigRuntime {
    pub fn new(config: PathBuf) -> Self {
        let lua = Lua::new_with(StdLib::VECTOR, LuaOptions::default())
            .expect("Failed to create Lua runtime");

        init_globals(&lua);

        Self {
            task_handle: spawn(Self::run(lua, config))
        }
    }

    async fn run(lua: Lua, config: PathBuf) {
        if let Err(error) = lua.load(config).exec_async().await {
            println!("Config execution error:\n{}", error);
        }
    }

    pub async fn join(self) {
        self.task_handle.await.unwrap();
    }
}
