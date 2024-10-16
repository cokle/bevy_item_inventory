use super::*;
use std::cmp::min;

/*
    添加物品操作
        根据id添加，大多用于合成物品什么的，需要通过id索引到物品

    没想到有什么需求是需要通过其他方式添加的
*/
#[derive(Debug)]
pub struct InsertItem {
    pub owner: Owner,
    pub items: HashMap<ItemId, ItemAmount>,
}

// 这里写得很丑陋...
impl TryApplyCommand<InventoryError> for InsertItem {
    /*
        尝试是否能添加物品
            优先堆叠到未堆叠到上限的物品上（这里没考虑顺序，后续可以考虑pos非降序）；
            如果都堆叠到上限后还有剩余，则尝试spawn新的物品格子
    */
    fn try_can(&self, world: &World) -> Result<(), InventoryError> {
        let Some(entity_map) = world.get::<InventoryEntityMap>(self.owner.0) else {
            return Err(InventoryError::NotInventory);
        };

        for (id, mut amount) in self.items.clone() {
            let entities = entity_map.get(&id).cloned().unwrap_or_default();

            for entity in entities {
                let Some(item_amount) = world.get::<ItemAmount>(entity) else {
                    return Err(InventoryError::Unknown);
                };

                // 处理是否有堆叠上限组件
                let add_amount = if let Some(item_overlap) = world.get::<ItemOverlap>(entity) {
                    min(amount, item_overlap.0 - *item_amount)
                } else {
                    amount
                };

                amount -= add_amount;

                if amount.0 == 0 {
                    break;
                }
            }

            // TODO 这里应该是amount <= empty_pos.len() * item_overlap
            if amount.0 > 0 {
                let Some(empty_pos) = world.get::<InventoryEmptyPos>(self.owner.0) else {
                    return Err(InventoryError::InventoryFull);
                };

                if empty_pos.is_empty() {
                    return Err(InventoryError::InventoryFull);
                }
            }
        }

        Ok(())
    }

    /*
        强制添加物品
            优先堆叠到未堆叠到上限的物品上（这里没考虑顺序，后续可以考虑pos非降序）；
            如果都堆叠到上限后还有剩余，则spawn新的物品格子
    */
    fn apply(&mut self, world: &mut World) -> Option<()> {
        let mut inserts = Vec::new();
        let mut adds = Vec::new();

        for (id, mut amount) in self.items.clone() {
            let entities = world
                .get::<InventoryEntityMap>(self.owner.0)?
                .get(&id)
                .cloned()
                .unwrap_or_default();

            for entity in entities {
                let item_amount = world.get::<ItemAmount>(entity).cloned()?;

                // 处理是否有堆叠上限组件
                let add_amount = if let Some(item_overlap) = world.get::<ItemOverlap>(entity) {
                    min(amount, item_overlap.0 - item_amount)
                } else {
                    amount
                };

                amount -= add_amount;
                inserts.push((entity, item_amount + add_amount));

                if amount.0 == 0 {
                    break;
                }
            }

            // TODO同上
            if amount.0 > 0 {
                adds.push((id, amount));
            }
        }

        // 仅需更新amount
        for (entity, amount) in inserts {
            world.commands().entity(entity).insert(amount);
        }

        // 需要新开格子
        for (id, amount) in adds {
            // TODO 应该从配置表生成
            world.commands().spawn((
                InventoryItem,
                id,
                ItemName(id.to_string()),
                amount,
                self.owner,
                ItemOverlap(ItemAmount(25)),
            ));
        }

        Some(())
    }
}

impl InsertItem {
    pub fn new(owner: Owner, items: HashMap<ItemId, ItemAmount>) -> Self {
        Self { owner, items }
    }
}
