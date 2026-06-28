use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;
use tokio::spawn;
use tokio::task::JoinHandle;

pub struct ConfigRuntime {
    task_handle: JoinHandle<()>
}

impl ConfigRuntime {
    pub fn new(config: PathBuf) -> Self {
        let lua = Lua::new_with(StdLib::NONE, LuaOptions::default())
            .expect("Failed to create Lua runtime");
        // TODO: global API initialization

        Self {
            task_handle: spawn(Self::run(lua, config))
        }
    }

    async fn run(lua: Lua, config: PathBuf) {
        lua.load(config).exec_async().await.unwrap();
    }

    pub async fn join(self) {
        self.task_handle.await.unwrap();
    }
}
