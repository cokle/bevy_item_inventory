use super::*;

pub(super) fn plugin(_app: &mut App) {}

// 物品id对应的库存物品，可以方便的从id查找到对应的库存物品
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Reflect)]
#[reflect(Component)]
pub struct InventoryEntityMap(pub BTreeMap<ItemId, BTreeSet<Entity>>);

impl InventoryEntityMap {
    pub fn insert_entity(&mut self, id: ItemId, entity: Entity) {
        self.entry(id).or_default().insert(entity);
    }

    pub fn remove_entity(&mut self, id: &ItemId, entity: &Entity) -> Option<()> {
        self.get_mut(id)?.remove(entity);
        if self.get(id)?.is_empty() {
            self.remove(id);
        }
        Some(())
    }
}
