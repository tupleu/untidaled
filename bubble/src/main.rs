use bevy::{prelude::*, window::WindowResolution};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;
const PLAYER_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bubble".to_string(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_gravity, move_player, check_for_collisions)
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .run();
}

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("test.png")),
        Transform::from_xyz(8., 8., 2.),
        Player,
        Collider,
    ));
}

fn apply_gravity() {}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position =
        player_transform.translation.x + direction * PLAYER_SPEED * time.delta_secs();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = -WIDTH / 2. + 200.;
    let right_bound = WIDTH / 2. - 200.;

    player_transform.translation.x = new_player_position.clamp(left_bound, right_bound);
    // println!("{:?}", player_transform.translation.x);
}

fn check_for_collisions() {}
