use super::*;
use crate::TryApplyCommand;
use bevy::ecs::world::Command;

pub(super) fn plugin(_app: &mut App) {}

// 放置时消耗的物品数量，可能不需要？
#[derive(Component, Debug, Default, Deref, DerefMut, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct PlaceConsAmount(pub ItemAmount);

/*
    放置物品
    commands.add(DoPlace{owner, pos});
*/
pub struct DoPlace {
    pub owner: Owner,
    pub pos: Vec2,
}

impl Command for DoPlace {
    fn apply(self, world: &mut World) {
        apply_do_place(self, world);
    }
}

fn apply_do_place(do_place: DoPlace, world: &mut World) -> Option<()> {
    let owner = do_place.owner;
    // 这里默认操作的是选中物品栏的物品
    let item_entity = world.get::<SelectedShortcutSlot>(owner.0).cloned()?.0;
    let id = world.get::<ItemId>(item_entity).cloned()?;
    let cons_amount = world.get::<PlaceConsAmount>(item_entity).cloned()?.0;

    let mut remove_item = RemoveItem::new(owner, vec![(id, cons_amount)].into_iter().collect());

    remove_item.try_can(world).ok()?;

    remove_item.apply(world).unwrap();

    // 生成建筑，应该从配置生成，这里简单示意一下
    let drop_info = DropInfo::new_one(id, cons_amount);
    let transform = Transform::from_translation(do_place.pos.extend(0.0));
    world
        .commands()
        .spawn((Building, owner, transform, drop_info));

    Some(())
}
