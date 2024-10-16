use super::*;
use rand::{thread_rng, Rng};

pub(super) fn plugin(app: &mut App) {
    app.world_mut()
        .register_component_hooks::<DropInfo>()
        .on_remove(|world, entity, component_id| {
            on_remove_drop_info(world, entity, component_id);
        });
}

#[derive(Component)]
pub struct DropItem;

// 简单的掉落
#[derive(Component, Debug, Clone)]
pub struct DropInfo {
    pub id_range: (u32, u32),
    pub amount_range: (u32, u32),
}

impl DropInfo {
    pub fn new_one(id: ItemId, amount: ItemAmount) -> Self {
        Self {
            id_range: (id.0, id.0),
            amount_range: (amount.0, amount.0),
        }
    }

    // 简单的生成掉落
    pub fn drop_item(&mut self) -> (ItemId, ItemAmount) {
        let mut rng = thread_rng();
        let id = rng.gen_range(self.id_range.0..=self.id_range.1);
        let amount = rng.gen_range(self.amount_range.0..=self.amount_range.1);
        (ItemId(id), ItemAmount(amount))
    }
}

// 拥有DropInfo的Entity，在despawn时生成掉落物品（可能需要添加判断是否在游戏中
fn on_remove_drop_info(
    mut world: bevy::ecs::world::DeferredWorld,
    entity: Entity,
    _component_id: bevy::ecs::component::ComponentId,
) -> Option<()> {
    let (id, amount) = world.get_mut::<DropInfo>(entity)?.drop_item();
    let owner = world.get::<Owner>(entity).cloned()?;

    world.commands().spawn((DropItem, id, amount, owner));

    Some(())
}
