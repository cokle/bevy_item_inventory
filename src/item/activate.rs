use super::*;
use bevy::ecs::world::Command;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_activate_item);
}

/*
    使用物品
    commands.add(DoActivate(owner));
*/
pub struct DoActivate(pub Owner);

impl Command for DoActivate {
    fn apply(self, world: &mut World) {
        apply_do_activate(self, world);
    }
}

#[derive(Event)]
pub struct OnActivateItem;

fn apply_do_activate(do_activate: DoActivate, world: &mut World) -> Option<()> {
    let owner = do_activate.0;
    // 默认使用当前选中物品栏中的物品，也可以选择传entity过来使用
    let item_entity = world.get::<SelectedShortcutSlot>(owner.0).cloned()?.0;

    // 触发观察者，有多种方法可以实现使用物品的逻辑（例如给物品添加对应效果的观察者，这样只需要触发使用事件就好了）
    // 这里暂时使用触发对应物品的观察者
    world
        .commands()
        .trigger_targets(OnActivateItem, item_entity);

    Some(())
}

fn on_activate_item(_trigger: Trigger<OnActivateItem>) {}
