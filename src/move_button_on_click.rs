use amethyst::ecs::{Entity, WorldExt};
use amethyst::ui::UiTransform;
use amethyst::prelude::{StateData,GameData};

pub fn move_button (data: StateData<GameData>,entity: Option<Entity>, y_trasnaltion: f32){
    let StateData { world, .. } = data;
    let mut ui_transform = world.write_storage::<UiTransform>();
    if let Some(entity) = entity.and_then(|entity| ui_transform.get_mut(entity)) {
        entity.local_y = entity.local_y - y_trasnaltion;
    }
}