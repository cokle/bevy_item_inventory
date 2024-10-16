mod inventory;
mod item;

use bevy::prelude::*;
use inventory::*;
use item::*;
use log::info;
use std::fmt::Debug;

fn main() {
    let mut app = App::new();

    app.add_plugins(inventory::plugin);
    app.add_plugins(item::plugin);

    app.run();
}

// 所有者
#[derive(Component, Debug, Deref, DerefMut, Clone, Copy)]
pub struct Owner(pub Entity);

// 可能会失败的命令，可能使用“状态结构”来实现更合理
pub trait TryApplyCommand<E: Debug> {
    fn try_can(&self, world: &World) -> Result<(), E>;
    fn apply(&mut self, world: &mut World) -> Option<()>;
    fn try_apply(&mut self, world: &mut World) -> Result<(), E> {
        self.try_can(world)?;
        self.apply(world);
        Ok(())
    }
}
