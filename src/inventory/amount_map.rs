use super::*;

// 物品id对应的库存数量，可以快速查询id对应的库存数量（目前主要优化删除时的尝试判断，可能还多余了）
#[derive(Component, Debug, Default, Deref, DerefMut, Clone, Reflect)]
#[reflect(Component)]
pub struct InventoryAmountMap(pub BTreeMap<ItemId, ItemAmount>);
