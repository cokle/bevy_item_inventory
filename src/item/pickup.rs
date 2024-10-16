use super::*;
use crate::TryApplyCommand;
use bevy::ecs::world::Command;

pub(super) fn plugin(_app: &mut App) {}

/*
    捡起物品
    commands.add(DoPickup(owner, entity));
*/
pub struct DoPickup(pub Owner, pub Entity);

impl Command for DoPickup {
    fn apply(self, world: &mut World) {
        apply_do_pickup(self, world);
    }
}

// TODO 简单实现一下
fn apply_do_pickup(do_pickup: DoPickup, world: &mut World) -> Option<()> {
    world.get::<DropItem>(do_pickup.1)?;
    let owner = do_pickup.0;
    let id = world.get::<ItemId>(do_pickup.1).cloned()?;
    let amount = world.get::<ItemAmount>(do_pickup.1).cloned()?;

    let mut insert_item = InsertItem::new(owner, vec![(id, amount)].into_iter().collect());

    insert_item.try_can(world).ok()?;

    insert_item.apply(world).unwrap();
    world.commands().entity(do_pickup.1).despawn_recursive();

    Some(())
}
