mod activate;
mod amount;
mod building;
mod crafta;
mod drop;
mod id;
mod pickup;
mod place;
mod slot;

pub use activate::*;
pub use amount::*;
pub use building::*;
pub use crafta::*;
pub use drop::*;
pub use id::*;
pub use pickup::*;
pub use place::*;
pub use slot::*;

use crate::{inventory::*, Owner, TryApplyCommand};
use bevy::prelude::*;
use std::hash::Hash;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(activate::plugin);
    app.add_plugins(amount::plugin);
    app.add_plugins(crafta::plugin);
    app.add_plugins(drop::plugin);
    app.add_plugins(id::plugin);
    app.add_plugins(pickup::plugin);
    app.add_plugins(place::plugin);
    app.add_plugins(slot::plugin);

    app.register_type::<Item>();
    app.register_type::<ItemName>();
    app.register_type::<ItemOverlap>();
}

// 物品
#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Item;

// 物品名称
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Reflect)]
#[reflect(Component)]
pub struct ItemName(pub String);

// 物品堆叠上限
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ItemOverlap(pub ItemAmount);
