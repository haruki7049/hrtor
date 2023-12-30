use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
};

use rlua::{Function, Lua, Table};

use crate::{actions::command_status_ok, file_loader::FileInfo, CommandStatus, HrtorProcessor};

use super::UserScript;

pub struct LuaScript {
    pub(crate) hrtor: Arc<HrtorProcessor>,
    pub(crate) entrypoint: FileInfo,
    registered_sinatures: Arc<Mutex<Vec<LuaCommandSignature>>>,
    tx: mpsc::Sender<LuaCommandSignature>,
    rx: mpsc::Receiver<LuaCommandSignature>,
}

struct LuaCommandExecutor<'lua> {
    action: Function<'lua>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct LuaCommandSignature {
    trigger: Vec<String>,
}

impl LuaScript {
    pub fn new(hrtor: Arc<HrtorProcessor>, entrypoint: FileInfo) -> Self {
        let (tx, rx) = mpsc::channel::<LuaCommandSignature>();
        Self {
            tx,
            rx,
            hrtor,
            entrypoint,
            registered_sinatures: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserScript for LuaScript {
    fn init(&self) {
        println!("Loading {}...", self.entrypoint.path);
        let lua = Lua::new();
        let hrtor = Arc::clone(&self.hrtor);
        lua.context(move |ctx| {
            let globals = ctx.globals();
            let registered_commands = Arc::new(Mutex::new(HashMap::<
                LuaCommandSignature,
                LuaCommandExecutor,
            >::new()));

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

            let registerer_commands = Arc::clone(&registered_commands);
            let registered_sinatures = Arc::clone(&self.registered_sinatures);
            let register_func: Function = ctx
                .create_function(move |_, (table,): (Table,)| {
                    let action = table.get::<&str, Function>("action").unwrap();
                    let trigger: Vec<String> = table
                        .get::<&str, Table>("trigger")
                        .unwrap()
                        .sequence_values::<String>()
                        .collect::<Result<Vec<String>, _>>()
                        .unwrap();

                    let signature = LuaCommandSignature { trigger };
                    let executor = LuaCommandExecutor { action };

                    registered_sinatures.lock().unwrap().push(signature.clone());
                    registerer_commands
                        .lock()
                        .unwrap()
                        .insert(signature, executor);

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

            loop {
                let sig = self.rx.recv().unwrap();
                registered_commands
                    .lock()
                    .unwrap()
                    .get(&sig)
                    .unwrap()
                    .action
                    .call::<(), ()>(())
                    .unwrap();
            }
        });
    }

    fn request_handle(&self, request: &String) -> Option<CommandStatus> {
        self.registered_sinatures
            .lock()
            .unwrap()
            .iter()
            .find(|v| v.trigger.contains(request))
            .map(|v| {
                self.tx.send(v.clone()).unwrap();
                command_status_ok()
            })
    }
}
