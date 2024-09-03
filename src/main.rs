use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Update, hello_world_system)
    .run();
}

fn hello_world_system() {
    println!("Hello World!");
}