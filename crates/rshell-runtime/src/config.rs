use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;
use std::time::Duration;
use tokio::spawn;
use tokio::task::JoinHandle;

pub struct ConfigRuntime {
    task_handle: JoinHandle<()>
}

impl ConfigRuntime {
    pub fn new(config: PathBuf) -> Self {
        let lua = Lua::new_with(StdLib::NONE, LuaOptions::default())
            .expect("Failed to create Lua runtime");

        lua.globals()
            .set(
                "sleep",
                lua.create_async_function(async move |_, amount: u64| {
                    tokio::time::sleep(Duration::from_millis(amount)).await;
                    Ok(())
                })
                .expect("Failed to create global function")
            )
            .expect("Failed to set global function");

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
