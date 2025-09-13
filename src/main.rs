use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource)]
struct InspectorVisible(bool);

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(inspector_should_run),
        ))
        .insert_resource(InspectorVisible(true))
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_inspector, handle_exit, player_movement))
        .run();
}

fn inspector_should_run(visible: Res<InspectorVisible>) -> bool {
    visible.0
}

fn toggle_inspector(mut visible: ResMut<InspectorVisible>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::F1) {
        visible.0 = !visible.0;
        println!("Inspector toggled: {}", visible.0);
    }
}

fn handle_exit(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        println!("ESC pressed - Exiting game");
        exit.write(AppExit::Success);
    }
}

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let speed = 300.0; // pixels per second
        let movement = speed * time.delta_secs();

        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= movement;
        }
        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += movement;
        }
        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement;
        }
    }
}

fn setup(mut commands: Commands) {
    // Add a 2D camera
    commands.spawn((Camera2d, Name::new("2D Camera")));

    // Add player sprite
    commands.spawn((
        Sprite::from_color(Color::srgb_u8(124, 144, 255), Vec2::new(100.0, 100.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Player"),
        Player,
    ));
}
