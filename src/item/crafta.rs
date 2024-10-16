use super::*;
use bevy::ecs::world::Command;
use std::collections::HashMap;

pub(super) fn plugin(_app: &mut App) {}

// 合成表，如果需要动态解锁的合成表，也可以是个组件，跟随玩家
#[derive(Resource, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct CraftaTable(pub HashMap<ItemId, HashMap<ItemId, ItemAmount>>);

impl Default for CraftaTable {
    fn default() -> Self {
        let mut crafta_table = HashMap::new();

        let mut materials = HashMap::new();
        materials.insert(ItemId(2), ItemAmount(3));
        crafta_table.insert(ItemId(1), materials);

        Self(Default::default())
    }
}

impl CraftaTable {
    pub fn get_materials(
        &self,
        id: &ItemId,
        amount: &ItemAmount,
    ) -> Option<HashMap<ItemId, ItemAmount>> {
        let mut materials = self.get(id).cloned()?;

        for unit_amount in materials.values_mut() {
            *unit_amount *= *amount;
        }

        Some(materials)
    }
}

// 合成
pub struct DoCrafta {
    pub owner: Owner,
    pub id: ItemId,
    pub amount: ItemAmount,
}

impl Command for DoCrafta {
    fn apply(self, world: &mut World) {
        apply_do_crafta(self, world);
    }
}

fn apply_do_crafta(do_crafta: DoCrafta, world: &mut World) -> Option<()> {
    let DoCrafta { owner, id, amount } = do_crafta;

    let materials = world
        .get_resource::<CraftaTable>()?
        .get_materials(&id, &amount)?;

    let mut remove_item = RemoveItem::new(owner, materials);
    let mut insert_item = InsertItem::new(owner, vec![(id, amount)].into_iter().collect());

    // 需要两个操作都能执行（事务操作）
    remove_item.try_can(world).ok()?;
    insert_item.try_can(world).ok()?;

    remove_item.apply(world).unwrap();
    insert_item.apply(world).unwrap();

    Some(())
}
