use std::sync::Arc;

use rlua::Lua;

use crate::{file_loader::FileInfo, HrtorProcessor};

use super::UserScript;

pub struct LuaScript {
    pub(crate) hrtor: Arc<HrtorProcessor>,
    pub(crate) entrypoint: FileInfo,
}

impl UserScript for LuaScript {
    fn init(&self) {
        println!("Loading {}...", self.entrypoint.path);
        let lua = Lua::new();
        let hrtor = Arc::clone(&self.hrtor);
        lua.context(move |ctx| {
            let globals = ctx.globals();

            let quit_hrtor = Arc::clone(&hrtor);
            let quit_func = ctx
                .create_function(move |_, (): ()| {
                    quit_hrtor.interpret_command_status(quit_hrtor.quit());
                    Ok(())
                })
                .unwrap();

            let echo_func = ctx
                .create_function(move |_, (s,): (String,)| {
                    println!("{}", s);
                    Ok(())
                })
                .unwrap();

            let api = ctx
                .create_table_from(vec![("quit", quit_func), ("echo", echo_func)])
                .unwrap();

            let hrtor = ctx.create_table_from(vec![("api", api)]).unwrap();

            globals.set("hrtor", hrtor).unwrap();

            ctx.load(&self.entrypoint.context).exec().unwrap();
        });
    }
}
