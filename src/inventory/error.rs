// 库存相关的错误
#[derive(Debug)]
pub enum InventoryError {
    // 没有库存
    NotInventory,
    // 数量不足
    InsufficientQuantity,
    // 库存满了
    InventoryFull,
    // 未知错误
    Unknown,
}
