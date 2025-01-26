use std::time::Duration;

use bevy::{color::palettes::css::*, math::bounding::*, prelude::*, window::WindowResolution};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;
const BSIZE: u32 = 32;

const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 10;

const TEST_LEVEL: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 9, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
];

const LEVEL_1: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 9, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const LEVEL_2: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

const LEVEL_LIST: [[[i32; LEVEL_WIDTH]; LEVEL_HEIGHT]; 3] = [TEST_LEVEL, LEVEL_1, LEVEL_2];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}
#[derive(Resource)]
struct LevelIndex(usize);

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
        .insert_resource(LevelIndex(1))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), spawn_level)
        .add_systems(
            FixedUpdate,
            (
                apply_gravity,
                advance_physics,
                check_for_collisions,
                oob_check,
                coyote_time,
                scale_screen,
            )
                // `chain`ing systems together runs them in order
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                flip_sprite.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), reset)
        .add_systems(Update, (animate_sprite, next_level))
        .add_event::<NextLevelEvent>()
        .run();
}

#[derive(Event)]
struct NextLevelEvent(i32);

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
    is_grounded: bool,
    is_left: bool,
    is_moving: bool,
    is_bubbling: bool,
    bubbled: bool,
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

fn scale_screen(query: Single<&mut OrthographicProjection, With<Camera>>) {
    let mut ortho = query.into_inner();
    ortho.scale = 0.5;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_index: Res<LevelIndex>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level = LEVEL_LIST[level_index.0];
    for (i, row) in level.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            match elem {
                1 => {
                    // bubble
                    let texture = asset_server.load("bubble-idle-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                        Bubble,
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                2 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
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
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                9 => {
                    //Player
                    let texture = asset_server.load("playerRemake-Sheet.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 4, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices: AnimationIndices =
                        AnimationIndices { first: 9, last: 11 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Name::new("Player"),
                        AccumulatedInput::default(),
                        Velocity::default(),
                        PhysicalTranslation(Vec3::new(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        )),
                        PreviousPhysicalTranslation::default(),
                        Player {
                            coyote_timer: Timer::new(
                                Duration::from_secs_f32(2.),
                                TimerMode::Repeating,
                            ),
                            spawn_x: 0.,
                            spawn_y: 64.,
                            is_grounded: false,
                            is_left: false,
                            is_moving: false,
                            is_bubbling: false,
                            bubbled: false,
                            can_jump: false,
                            jump_force: 210., //jump force? peak peak
                            h_speed: 100.,
                            gravity: 600.,
                        },
                        Collider,
                    ));
                }
                _ => (),
            }
        }
    }
}

fn reset(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

fn oob_check(
    player_query: Single<&PhysicalTranslation, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let phys_translation = player_query.into_inner();

    if phys_translation.x > WIDTH * 2.
        || phys_translation.x < (WIDTH / 2.) * -1.
        || phys_translation.y < (HEIGHT / 2.) * -1.
        || phys_translation.y > HEIGHT * 2.
    {
        next_state.set(GameState::GameOver);
    }
}

fn apply_gravity(player_query: Single<(&mut Velocity, &Player)>, time: Res<Time>) {
    let (mut velocity, player) = player_query.into_inner();
    // if player.is_grounded {
    //     return;
    // }
    velocity.y -= player.gravity * time.delta_secs();
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
    mut gizmos: Gizmos,
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut ev_nextlevel: EventWriter<NextLevelEvent>,
    player_query: Single<(
        &mut AccumulatedInput,
        &PhysicalTranslation,
        &mut Velocity,
        &mut Player,
    )>,
    bubble_query: Query<(&Transform, Entity), With<Bubble>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        ev_nextlevel.send(NextLevelEvent(0));
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Minus) {
        ev_nextlevel.send(NextLevelEvent(-1));
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Equal) {
        ev_nextlevel.send(NextLevelEvent(1));
        return;
    }

    let (mut input, position, mut velocity, mut player) = player_query.into_inner();
    if keyboard_input.pressed(KeyCode::KeyA) {
        player.is_left = true;
        input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        player.is_left = false;
        input.x += 1.0;
    }

    if player.can_jump && keyboard_input.pressed(KeyCode::Space) {
        velocity.y = player.jump_force;
        player.is_grounded = false;
        player.can_jump = false;
    } else {
        input.y = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
        let center = Vec2::new(
            position.x + if player.is_left { -32. } else { 32. },
            position.y,
        );
        let aabb = Aabb2d::new(center, Vec2::splat(16.));
        gizmos.rect_2d(center, aabb.half_size() * 2., RED);
        for (bubble_transformation, bubble) in bubble_query.iter() {
            let bubble_center = bubble_transformation.translation.truncate();
            let bubble_aabb = Aabb2d::new(bubble_center, Vec2::splat(16.));

            let x_overlaps = aabb.min.x < bubble_aabb.max.x && aabb.max.x > bubble_aabb.min.x;
            let y_overlaps = aabb.min.y < bubble_aabb.max.y && aabb.max.y > bubble_aabb.min.y;

            // if intersects, move back by larger axis
            if x_overlaps && y_overlaps && center.distance(bubble_center) < 16. {
                if player.is_bubbling {
                    return;
                }
                player.bubbled = true;
                player.is_bubbling = true;
                commands.entity(bubble).despawn();
                return;
            }
        }
        if !player.is_bubbling {
            return;
        }
        player.bubbled = true;
        player.is_bubbling = false;
        let texture = asset_server.load("bubble-idle-32x32.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 2 };
        commands.spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
            ),
            Transform::from_xyz(
                f32::round(center.x / 32.) * 32.,
                f32::round(center.y / 32.) * 32.,
                2.,
            ),
            Collider,
            Bubble,
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
        ));
    }
}

fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    player_query: Single<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &mut AnimationIndices,
        &mut Sprite,
        &Velocity,
        &mut Player,
    )>,
) {
    let (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        mut indices,
        mut sprite,
        velocity,
        mut player,
    ) = player_query.into_inner();

    previous_physical_translation.0 = current_physical_translation.0;
    current_physical_translation.0 +=
        (velocity.0 + input.extend(0.) * player.h_speed) * fixed_time.delta_secs();

    if player.is_bubbling {
        if input.x != 0. {
            if player.bubbled || !player.is_moving {
                player.is_moving = true;
                player.bubbled = false;
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 3;
                }
            }
            indices.first = 3;
            indices.last = 5;
        } else {
            if player.bubbled || player.is_moving {
                player.is_moving = false;
                player.bubbled = false;
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = 0;
                }
            }
            indices.first = 0;
            indices.last = 2;
        }
    } else if input.x != 0. {
        if player.bubbled || !player.is_moving {
            player.is_moving = true;
            player.bubbled = false;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 6;
            }
        }
        indices.first = 6;
        indices.last = 8;
    } else {
        if player.bubbled || player.is_moving {
            player.is_moving = false;
            player.bubbled = false;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 9;
            }
        }
        indices.first = 9;
        indices.last = 11;
    }

    // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
    input.0 = Vec2::ZERO;
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

fn next_level(
    mut commands: Commands,
    mut ev_levelup: EventReader<NextLevelEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    level_index: Res<LevelIndex>,
) {
    if ev_levelup.is_empty() {
        return;
    }
    let mut index = level_index.0 as i32;
    for ev in ev_levelup.read() {
        index += ev.0
    }
    index = index.clamp(0, LEVEL_LIST.len() as i32 - 1);
    commands.insert_resource(LevelIndex(index as usize));
    next_state.set(GameState::GameOver);
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

fn flip_sprite(player_query: Single<(&mut Sprite, &Player)>) {
    let (mut sprite, player) = player_query.into_inner();
    sprite.flip_x = !player.is_left;
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
