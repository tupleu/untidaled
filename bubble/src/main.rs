use std::time::Duration;

use bevy::{color::palettes::css::*, math::bounding::*, prelude::*, window::WindowResolution};

const TEST_LEVEL: [[i32; 10]; 10] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 2, 0, 0],
    [1, 0, 0, 0, 0, 0, 1, 1, 1, 0],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 1, 0, 1, 1, 0, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;
const BSIZE: u32 = 32;

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
        .add_systems(Startup, (setup, spawn_level))
        .add_systems(
            FixedUpdate,
            (
                apply_gravity,
                advance_physics,
                check_for_collisions,
                death_respawn,
                coyote_time,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        )
        .add_systems(Update, animate_sprite)
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
    coyote_timer: Timer,
    spawn_x: f32,
    spawn_y: f32,
    is_grabbing: bool,
    is_grounded: bool,
    can_jump: bool,
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
struct Fan {
    direction: Direction,
    strength: f32,
    distance: f32,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // commands.spawn(
    //     (
    //         Fan,
    //         Transform::from_scale(Vec3::splat(2.)),
    //     )
    // );

    commands.spawn((
        Name::new("Player"),
        Sprite::from_image(asset_server.load("test.png")),
        Transform::from_xyz(0., 0., 2.),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation(Vec3::new(0., 64., 2.)),
        PreviousPhysicalTranslation::default(),
        Player {
            coyote_timer: Timer::new(Duration::from_secs_f32(2.), TimerMode::Repeating),
            spawn_x: 0.,
            spawn_y: 64.,
            is_grabbing: false,
            is_grounded: false,
            can_jump: false,
            jump_force: 210., //jump force? peak peak
            h_speed: 100.,
            gravity: 600.,
        },
        Collider,
    ));
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level = TEST_LEVEL;
    for (i, row) in level.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            let texture = asset_server.load("bubble-idle-32x32.png");
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 2 };
            match elem {
                1 => {
                    commands.spawn((
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 160.,
                            -(BSIZE as f32 * i as f32 - 160.),
                            2.,
                        ),
                        Collider,
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                2 => {}
                _ => (),
            }
        }
    }
}

fn apply_gravity(player_query: Single<(&mut Velocity, &Player)>, time: Res<Time>) {
    let (mut velocity, player) = player_query.into_inner();
    // if player.is_grounded {
    //     return;
    // }
    velocity.y -= player.gravity * time.delta_secs();
}

fn death_respawn(player_query: Single<(&mut PhysicalTranslation, &Player)>) {
    let (mut phys_translation, player) = player_query.into_inner();

    if phys_translation.x > WIDTH * 2.
        || phys_translation.x < (WIDTH / 2.) * -1.
        || phys_translation.y < (HEIGHT / 2.) * -1.
        || phys_translation.y > HEIGHT * 2.
    {
        phys_translation.x = player.spawn_x;
        phys_translation.y = player.spawn_y;
    }
}

fn coyote_time(_time: Res<Time>, player_query: Single<&mut Player>) {
    let mut player = player_query.into_inner();

    if !player.is_grounded {
        player.coyote_timer.tick(_time.delta());

        if player.coyote_timer.finished() {
            player.can_jump = false;
        }
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Single<(&mut AccumulatedInput, &mut Velocity, &mut Player)>,
) {
    let (mut input, mut velocity, mut player) = player_query.into_inner();
    // if keyboard_input.pressed(KeyCode::KeyW) {
    //     input.y += 1.0;
    // }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }
    if player.can_jump && keyboard_input.pressed(KeyCode::Space) {
        velocity.y = player.jump_force;
        player.is_grounded = false;
        player.can_jump = false;
    } else {
        input.y = 0.;
    }

    // velocity.0 += input.extend(0.0).normalize_or_zero() * 5.;
    // velocity.0 += input.extend(0.);
    // println!("1 {:?} {:?}", player.is_grounded, velocity);
}

fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
        &Player,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
        player,
    ) in query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 +=
            (velocity.0 + input.extend(0.) * player.h_speed) * fixed_time.delta_secs();

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

fn check_for_collisions(
    mut gizmos: Gizmos,
    player_query: Single<(
        &mut Velocity,
        &mut PhysicalTranslation,
        &PreviousPhysicalTranslation,
        &mut Player,
    )>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
) {
    let (mut velocity, mut physical_translation, previous_physical_translation, mut player) =
        player_query.into_inner();

    let center = physical_translation.truncate();
    let aabb = Aabb2d::new(center, Vec2::splat(16.));
    gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);

    // player.is_grounded = false;
    let mut check_once = true;
    'outer: for collider in collider_query.iter() {
        let collider_center = collider.translation.truncate();
        let collider_aabb = Aabb2d::new(collider_center, Vec2::splat(16.));

        let x_overlaps = aabb.min.x < collider_aabb.max.x && aabb.max.x > collider_aabb.min.x;
        let y_overlaps = aabb.min.y < collider_aabb.max.y && aabb.max.y > collider_aabb.min.y;

        // if intersects, move back by larger axis
        if x_overlaps && y_overlaps {
            let displacement;
            let mut to_ground = player.is_grounded;
            let aabb2;
            // check which axis is larger first
            if f32::abs(previous_physical_translation.y - collider_center.y)
                > f32::abs(previous_physical_translation.x - collider_center.x)
            {
                displacement = if previous_physical_translation.y > collider_center.y {
                    // player.is_grounded = true;
                    // player.can_jump = true;
                    to_ground = true;
                    aabb.min.y - collider_aabb.max.y
                } else {
                    aabb.max.y - collider_aabb.min.y
                };
                let center2 = Vec2::new(center.x, center.y - displacement);
                aabb2 = Aabb2d::new(center2, Vec2::splat(16.));

                // check if displacement collides with other colliders
                'inner: for collider2 in collider_query.iter() {
                    if collider == collider2 {
                        continue 'inner;
                    }
                    let collider_center2 = collider2.translation.truncate();
                    let collider_aabb2 = Aabb2d::new(collider_center2, Vec2::splat(16.));
                    let x_overlaps2 =
                        aabb2.min.x < collider_aabb2.max.x && aabb2.max.x > collider_aabb2.min.x;
                    let y_overlaps2 =
                        aabb2.min.y < collider_aabb2.max.y && aabb2.max.y > collider_aabb2.min.y;
                    if x_overlaps2 && y_overlaps2 {
                        if check_once {
                            check_once = false;
                            continue 'outer;
                        }
                        let displacement2 = if previous_physical_translation.x > collider_center2.x
                        {
                            aabb2.min.x - collider_aabb2.max.x
                        } else {
                            aabb2.max.x - collider_aabb2.min.x
                        };
                        physical_translation.x -= displacement2;
                        velocity.x = 0.;
                        break;
                    }
                }
                player.is_grounded = to_ground;
                player.can_jump = to_ground;
                velocity.y = 0.;
                // player.is_grounded = false;
                physical_translation.y -= displacement;
            } else {
                displacement = if previous_physical_translation.x > collider_center.x {
                    aabb.min.x - collider_aabb.max.x
                } else {
                    aabb.max.x - collider_aabb.min.x
                };
                let center2 = Vec2::new(center.x - displacement, center.y);
                aabb2 = Aabb2d::new(center2, Vec2::splat(16.));

                // check if displacement collides with other colliders
                'inner: for collider2 in collider_query.iter() {
                    if collider == collider2 {
                        continue 'inner;
                    }
                    let collider_center2 = collider2.translation.truncate();
                    let collider_aabb2 = Aabb2d::new(collider_center2, Vec2::splat(16.));
                    let x_overlaps2 =
                        aabb2.min.x < collider_aabb2.max.x && aabb2.max.x > collider_aabb2.min.x;
                    let y_overlaps2 =
                        aabb2.min.y < collider_aabb2.max.y && aabb2.max.y > collider_aabb2.min.y;
                    if x_overlaps2 && y_overlaps2 {
                        if check_once {
                            check_once = false;
                            continue 'outer;
                        }
                        let displacement2 = if previous_physical_translation.y > collider_center2.y
                        {
                            // player.is_grounded = true;
                            to_ground = true;
                            aabb2.min.y - collider_aabb2.max.y
                        } else {
                            aabb2.max.y - collider_aabb2.min.y
                        };
                        physical_translation.y -= displacement2;
                        player.is_grounded = to_ground;
                        player.can_jump = to_ground;
                        velocity.y = 0.;
                        break;
                    }
                }
                velocity.x = 0.;
                physical_translation.x -= displacement;
            }
            gizmos.rect_2d(
                collider_center,
                collider_aabb.half_size() * 2.,
                SPRING_GREEN,
            );
            break;
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
