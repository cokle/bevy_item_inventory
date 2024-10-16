use super::*;

pub(super) fn plugin(app: &mut App) {
    app.world_mut()
        .register_component_hooks::<InventoryPos>()
        .on_insert(|world, entity, component_id| {
            on_insert_inventory_pos(world, entity, component_id);
        });
    app.world_mut()
        .register_component_hooks::<InventoryPos>()
        .on_remove(|world, entity, component_id| {
            on_remove_inventory_pos(world, entity, component_id);
        });
}

// 库存位置
#[derive(
    Component, Debug, Default, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Reflect,
)]
#[reflect(Component)]
pub struct InventoryPos(pub u32);

// 当插入了库存位置时，从InventoryEmptyPos中删除该位置（可能不需要
fn on_insert_inventory_pos(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let pos = world.get::<InventoryPos>(entity).cloned()?;
    let owner = world.get::<Owner>(entity)?.0;

    world.get_mut::<InventoryEmptyPos>(owner)?.remove(&pos);

    Some(())
}

// 当删除了库存位置时，归还到InventoryEmptyPos
fn on_remove_inventory_pos(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let pos = world.get::<InventoryPos>(entity).cloned()?;
    let owner = world.get::<Owner>(entity)?.0;

    world.get_mut::<InventoryEmptyPos>(owner)?.insert(pos);

    Some(())
}
