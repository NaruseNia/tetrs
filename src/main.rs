mod mino;
mod meta;
mod mino_types;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::meta::*;
use crate::mino::MinoPlugin;

fn main() {
    println!("Tetrs v0.1");
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Tet.rs".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MinoPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
