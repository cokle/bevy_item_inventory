use super::*;

pub(super) fn plugin(app: &mut App) {
    app.world_mut()
        .register_component_hooks::<ItemId>()
        .on_insert(|world, entity, component_id| {
            on_insert_item_id(world, entity, component_id);
        });
    app.world_mut()
        .register_component_hooks::<ItemId>()
        .on_remove(|world, entity, component_id| {
            on_remove_item_id(world, entity, component_id);
        });
}

// 物品id
#[derive(
    Component,
    Debug,
    Default,
    Deref,
    DerefMut,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    Reflect,
)]
#[reflect(Component)]
pub struct ItemId(pub u32);

impl From<u32> for ItemId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

// 插入物品id时，如果是库存的物品，则添加到InventoryEntityMap
fn on_insert_item_id(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    world.get::<InventoryItem>(entity)?;
    let id = world.get::<ItemId>(entity).cloned()?;
    let owner = world.get::<Owner>(entity).cloned()?.0;
    world
        .get_mut::<InventoryEntityMap>(owner)?
        .insert_entity(id, entity);

    Some(())
}

// 删除物品id时，如果是库存的物品，则从InventoryEntityMap删除
fn on_remove_item_id(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    world.get::<InventoryItem>(entity)?;
    let id = world.get::<ItemId>(entity).cloned()?;
    let owner = world.get::<Owner>(entity).cloned()?.0;
    world
        .get_mut::<InventoryEntityMap>(owner)?
        .remove_entity(&id, &entity);

    Some(())
}
