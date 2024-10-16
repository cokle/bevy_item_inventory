use super::*;

pub(super) fn plugin(_app: &mut App) {}

// 物品栏位置
#[derive(Component, Debug, Default, Deref, DerefMut, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct ShortcutSlotPos(pub u32);

// 当前选择物品栏中的物品
#[derive(Component, Debug, Deref, DerefMut, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct SelectedShortcutSlot(pub Entity);
