mod amount_map;
mod empty_pos;
mod entity_map;
mod error;
mod insert;
mod pos;
mod remove;

pub use amount_map::*;
pub use empty_pos::*;
pub use entity_map::*;
pub use error::*;
pub use insert::*;
pub use pos::*;
pub use remove::*;

use crate::{item::*, Owner, TryApplyCommand};
use bevy::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(empty_pos::plugin);
    app.add_plugins(entity_map::plugin);
    app.add_plugins(pos::plugin);

    app.world_mut()
        .register_component_hooks::<InventoryItem>()
        .on_insert(|world, entity, component_id| {
            on_insert_inventory_item(world, entity, component_id);
        });
    app.world_mut()
        .register_component_hooks::<InventoryItem>()
        .on_remove(|world, entity, component_id| {
            on_remove_inventory_item(world, entity, component_id);
        });
}

// 库存物品
#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct InventoryItem;

// 添加库存物品的时候，需要维护一些结构，以方便查询
fn on_insert_inventory_item(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let owner = world.get::<Owner>(entity).clone()?.0;
    let id = world.get::<ItemId>(entity).cloned()?;
    let amount = world.get::<ItemAmount>(entity).cloned()?;

    // 如果没有指定库存位置，则获取一个空位置
    if world.get::<InventoryPos>(entity).is_none() {
        let pos = world.get_mut::<InventoryEmptyPos>(owner)?.pop_first()?;
        world.commands().entity(entity).insert(pos);
    }

    // 维护InventoryEntityMap
    world
        .get_mut::<InventoryEntityMap>(owner)?
        .insert_entity(id, entity);

    // 维护InventoryAmountMap
    if let Some(mut amount_map) = world.get_mut::<InventoryAmountMap>(owner) {
        let total_amount = amount_map.entry(id).or_default();
        *total_amount += amount;
    }

    Some(())
}

fn on_remove_inventory_item(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let owner = world.get::<Owner>(entity).clone()?.0;
    let id = world.get::<ItemId>(entity).cloned()?;
    let amount = world.get::<ItemAmount>(entity).cloned()?;

    // 维护InventoryEntityMap
    world
        .get_mut::<InventoryEntityMap>(owner)?
        .remove_entity(&id, &entity);

    // 维护InventoryAmountMap
    if let Some(mut amount_map) = world.get_mut::<InventoryAmountMap>(owner) {
        let total_amount = amount_map.entry(id).or_default();
        *total_amount -= amount;
    }

    Some(())
}

// TODO 交换位置，排序，锁定位置
