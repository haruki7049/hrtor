use std::sync::Arc;

use rlua::{Function, Lua, Table};

use crate::{actions::command_status_ok, file_loader::FileInfo, CommandStatus, HrtorProcessor};

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

            let new_func: Function = ctx
                .create_function(move |ctx, (table,): (Table,)| {
                    let result: Table = ctx.create_table().unwrap();

                    result
                        .set("action", table.get::<&str, Function>("action").unwrap())
                        .unwrap();
                    result
                        .set("trigger", table.get::<&str, Table>("trigger").unwrap())
                        .unwrap();

                    Ok(result)
                })
                .unwrap();

            let register_func: Function = ctx
                .create_function(move |ctx, (table,): (Table,)| {
                    let result: Table = ctx.create_table().unwrap();

                    // result
                    //     .set("", table.get::<&str, Function>("").unwrap())
                    //     .unwrap();

                    Ok(())
                })
                .unwrap();

            let api = ctx
                .create_table_from(vec![("quit", quit_func), ("echo", echo_func)])
                .unwrap();

            let command: Table = ctx.create_table_from(vec![("new", new_func)]).unwrap();

            let hrtor: Table = ctx.create_table().unwrap();
            hrtor.set("api", api).unwrap();
            hrtor.set("command", command).unwrap();
            hrtor.set("register_command", register_func).unwrap();

            globals.set("hrtor", hrtor).unwrap();

            ctx.load(&self.entrypoint.context).exec().unwrap();
        });
    }

    fn request_handle(&self, request: &str) -> Option<CommandStatus> {
        if request == "papa" {
            println!("mama!");
            return Some(command_status_ok());
        }
        None
    }
}
