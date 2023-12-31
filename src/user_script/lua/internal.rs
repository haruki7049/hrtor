use std::sync::atomic::AtomicU64;

use rlua::{Context, Function, Table};

pub(super) struct HrtorInternal {
    counter: AtomicU64,
}

impl HrtorInternal {
    pub(super) fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }

    pub(super) fn ready(ctx: &Context<'_>) {
        ctx.globals()
            .get::<_, Table>("hrtor")
            .unwrap()
            .set("__internal", ctx.create_table().unwrap())
            .unwrap();
    }

    pub(super) fn put_function<'lua>(
        &self,
        ctx: &Context<'lua>,
        func: Function<'lua>,
    ) -> HrtorInternalFunction {
        let id = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        ctx.globals()
            .get::<_, Table>("hrtor")
            .unwrap()
            .get::<_, Table>("__internal")
            .unwrap()
            .set(format!("function_{}", id), func)
            .unwrap();
        HrtorInternalFunction::new(id)
    }
}

#[derive(PartialEq, Eq)]
pub(super) struct HrtorInternalFunction {
    id: u64,
}

impl HrtorInternalFunction {
    fn new(id: u64) -> Self {
        Self { id }
    }

    pub(super) fn get<'lua>(&self, ctx: &Context<'lua>) -> Function<'lua> {
        ctx.globals()
            .get::<_, Table>("hrtor")
            .unwrap()
            .get::<_, Table>("__internal")
            .unwrap()
            .get::<_, Function>(format!("function_{}", self.id))
            .unwrap()
    }
}
