mod internal;

use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use rlua::{Function, Lua, Table};

use crate::{
    actions::command_status_ok, user_script::lua::internal::HrtorInternal, CommandStatus,
    HrtorProcessor,
};

use hrtor_utils::FileInfo;

use self::internal::HrtorInternalFunction;

use super::UserScript;

pub struct LuaScript {
    pub(crate) hrtor: Arc<HrtorProcessor>,
    pub(crate) entrypoint: FileInfo,
    registered_sinatures: Arc<Mutex<Vec<LuaCommandSignature>>>,
    tx: mpsc::Sender<LuaScriptSignal>,
    rx: Arc<Mutex<mpsc::Receiver<LuaScriptSignal>>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct LuaCommandSignature {
    trigger: Vec<String>,
}

#[derive(Clone)]
struct LuaScriptTriggerRequest {
    signature: LuaCommandSignature,
    finish: mpsc::Sender<()>,
}

enum LuaScriptSignal {
    Trigger(LuaScriptTriggerRequest),
    Finish,
}

#[derive(PartialEq, Eq)]
struct LuaCommandExecutor {
    action: HrtorInternalFunction,
}

impl LuaScript {
    pub fn new(hrtor: Arc<HrtorProcessor>, entrypoint: FileInfo) -> Self {
        let (tx, rx) = mpsc::channel::<LuaScriptSignal>();
        Self {
            tx,
            rx: Arc::new(Mutex::new(rx)),
            hrtor,
            entrypoint,
            registered_sinatures: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserScript for LuaScript {
    fn init(&self) {
        println!("Loading {}...", self.entrypoint.path);
        let rx = Arc::clone(&self.rx);
        let hrtor = Arc::clone(&self.hrtor);
        let context = self.entrypoint.context.clone();
        let registered_sinatures = Arc::clone(&self.registered_sinatures);
        thread::spawn(move || {
            let lua = Lua::new();
            lua.context(move |ctx| {
                let globals = ctx.globals();
                let internal = HrtorInternal::new();
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
                let register_func: Function = ctx
                    .create_function(move |ctx, (table,): (Table,)| {
                        let action = table.get::<&str, Function>("action").unwrap();
                        let trigger: Vec<String> = table
                            .get::<&str, Table>("trigger")
                            .unwrap()
                            .sequence_values::<String>()
                            .collect::<Result<Vec<String>, _>>()
                            .unwrap();

                        let internal = &internal;
                        let internal_action = internal.put_function(&ctx, action);
                        let signature = LuaCommandSignature { trigger };
                        let executor = LuaCommandExecutor {
                            action: internal_action,
                        };

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

                HrtorInternal::ready(&ctx);
                ctx.load(&context).exec().unwrap();

                while let LuaScriptSignal::Trigger(sig) = { rx.lock().unwrap().recv().unwrap() } {
                    registered_commands
                        .lock()
                        .unwrap()
                        .get(&sig.signature)
                        .unwrap()
                        .action
                        .get(&ctx)
                        .call::<(), ()>(())
                        .unwrap();
                    sig.finish.send(()).unwrap();
                }
            });
        });
    }

    fn request_handle(&self, request: &str) -> Option<CommandStatus> {
        self.registered_sinatures
            .lock()
            .unwrap()
            .iter()
            .find(|v| v.trigger.iter().any(|s| s.as_str() == request))
            .map(|v| {
                let (finish_tx, finish_rx) = mpsc::channel::<()>();
                self.tx
                    .send(LuaScriptSignal::Trigger(LuaScriptTriggerRequest {
                        signature: v.clone(),
                        finish: finish_tx,
                    }))
                    .unwrap();
                finish_rx.recv().unwrap();
                command_status_ok()
            })
    }
}

impl Drop for LuaScript {
    fn drop(&mut self) {
        self.tx.send(LuaScriptSignal::Finish).unwrap();
    }
}
