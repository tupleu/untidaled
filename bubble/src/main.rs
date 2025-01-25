use bevy::{prelude::*, window::{WindowMode, WindowResolution}};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bubble".to_string(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        mode: WindowMode ::Fullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_gravity, check_for_collisions)
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .run();
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
struct Player {
    //coyote_time: Timer,
    is_grabbing: bool,
    is_grounded: bool,
    h_speed: f32,
    jump_force: f32,
    gravity: f32,
}

#[derive(Component)]
struct Bubble;

#[derive(Component)]
struct Spikes;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
struct Fan
{
    direction: Direction,
    strength: f32,
    distance: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Name::new("Player"),
        Sprite::from_image(asset_server.load("test.png")),
        Transform::from_scale(Vec3::splat(2.)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
        Player{is_grabbing:false, is_grounded:false, jump_force:1., h_speed:500., gravity:20.,},
        Collider,
    ));
}

fn apply_gravity(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    mut player_obj: Single<&mut Player>,
    mut velocity: Single<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {

    if player_obj.is_grounded && velocity.y < 0.
    {
        velocity.y = 0.;
    }

    velocity.y -= player_obj.gravity * time.delta_secs();

    if player_obj.is_grounded && keyboard_input.pressed(KeyCode::Space)
    {
        velocity.y = player_obj.jump_force;
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity), With<Player>>,
) {
    for (mut input, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        // if keyboard_input.pressed(KeyCode::KeyS) {
        //     input.y -= 1.0;
        // }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }

        velocity.0 += input.extend(0.0).normalize_or_zero() * 5.;
    }
}

fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.0 = Vec2::ZERO;
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<
        (
            &mut Transform,
            &PhysicalTranslation,
            &PreviousPhysicalTranslation,
        ),
        With<Player>,
    >,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}

fn check_for_collisions() {}
