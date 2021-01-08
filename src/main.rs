use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(64., 64.)),
            ..Default::default()
        });
}
