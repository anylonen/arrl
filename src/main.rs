use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Update, hello_world_system)
    .run();
}

fn hello_world_system() {
    println!("Hello World!");
}