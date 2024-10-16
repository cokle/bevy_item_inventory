use super::*;

/*
    删除物品操作
        根据id删除，大多用于合成物品什么的，需要通过id索引到物品
        TODO 根据entity删除，直接从背包什么的视图删除，可以直接获取其entity，直接删除
*/
#[derive(Debug)]
pub struct RemoveItem {
    pub owner: Owner,
    pub items: HashMap<ItemId, ItemAmount>,
}

// 这里写得很丑陋...
impl TryApplyCommand<InventoryError> for RemoveItem {
    // 尝试是否能删除
    fn try_can(&self, world: &World) -> Result<(), InventoryError> {
        let Some(amount_map) = world.get::<InventoryAmountMap>(self.owner.0) else {
            return Err(InventoryError::NotInventory);
        };

        let insufficient_quantity = self.items.iter().any(|(id, &amount)| {
            !amount_map.contains_key(id) || amount_map.get(id).cloned().unwrap() < amount
        });
        if insufficient_quantity {
            return Err(InventoryError::InsufficientQuantity);
        }

        Ok(())
    }

    // 强制删除，必须保证能够删除
    fn apply(&mut self, world: &mut World) -> Option<()> {
        let mut remove = Vec::new();

        for (id, amount) in &mut self.items {
            let entities = world
                .get::<InventoryEntityMap>(self.owner.0)?
                .get(&id)
                .cloned()
                .unwrap_or_default();

            for entity in entities {
                let mut item_amount = world.get_mut::<ItemAmount>(entity)?;

                // TODO 这里不这样写好像有bug
                if item_amount.0 <= amount.0 {
                    amount.0 = amount.0 - item_amount.0;
                    remove.push(entity);
                } else {
                    item_amount.0 = item_amount.0 - amount.0;
                    amount.0 = amount.0 - amount.0;
                    break;
                }
            }
        }

        for entity in remove {
            world.commands().entity(entity).despawn_recursive();
        }

        None
    }
}

impl RemoveItem {
    pub fn new(owner: Owner, items: HashMap<ItemId, ItemAmount>) -> Self {
        Self { owner, items }
    }
}
