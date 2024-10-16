use super::*;

pub(super) fn plugin(app: &mut App) {
    app.world_mut()
        .register_component_hooks::<InventoryCapacity>()
        .on_insert(|world, entity, component_id| {
            on_insert_inventory_capacity(world, entity, component_id);
        });
}

// 库存空位置集合，可以快速查询非降序的库存空位置
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Reflect)]
#[reflect(Component)]
pub struct InventoryEmptyPos(pub BTreeSet<InventoryPos>);

// 库存容量
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Reflect)]
#[reflect(Component)]
pub struct InventoryCapacity(pub u32);

// TODO 当库存容量变化时，需要更新库存空位置集合，这里简单实现一下，意思一下
fn on_insert_inventory_capacity(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let capacity = world.get::<InventoryCapacity>(entity).cloned()?;

    let mut empty_pos = InventoryEmptyPos::default();

    for i in 0..capacity.0 {
        empty_pos.insert(InventoryPos(i));
    }

    world.commands().entity(entity).insert(empty_pos);

    Some(())
}
