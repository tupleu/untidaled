use std::time::Duration;

use bevy::{color::palettes::css::*, math::bounding::*, prelude::*, window::WindowResolution};

const WIDTH: f32 = 1920.;
const HEIGHT: f32 = 1080.;
const BSIZE: u32 = 32;

const LEVEL_WIDTH: usize = 20;
const LEVEL_HEIGHT: usize = 10;

const LOW_WIND: f32 = 1.;
const MID_WIND: f32 = 2.;
const HI_WIND: f32 = 3.;

const TEST_LEVEL: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        00, 00, 00, 00, 00, 00, 00, 32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        00, 32, 00, 00, 00, 00, 00, 32, 32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        32, 00, 00, 00, 19, 00, 00, 11, 32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        32, 00, 00, 32, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 32,
    ],
    [
        32, 00, 32, 00, 32, 32, 32, 32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 32,
    ],
    [
        00, 32, 32, 00, 32, 32, 00, 32, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ],
    [
        00, 00, 00, 00, 00, 00, 32, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 99, 00,
    ],
    [
        00, 00, 00, 00, 00, 00, 00, 00, 00, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 00,
    ],
];

const LEVEL_1: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 31, 33, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 11, 00, 12, 00, 31, 33, 00, 42, 00, 00, 34, 36, 00, 00, 00, 00, 00, 99, 34,
    ],
    [
        35, 32, 32, 32, 32, 35, 35, 32, 32, 32, 32, 35, 35, 32, 32, 32, 32, 32, 32, 35,
    ],
];

const LEVEL_2: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 99, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        35, 32, 33, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        35, 35, 36, 15, 11, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 12, 00, 34,
    ],
    [
        35, 35, 36, 40, 31, 33, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 42, 00, 34,
    ],
    [
        35, 35, 35, 35, 35, 35, 32, 32, 33, 14, 14, 31, 33, 14, 14, 31, 32, 32, 32, 35,
    ],
];

const LEVEL_3: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 35,
    ],
    [
        36, 00, 00, 00, 99, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 12, 31, 32, 33, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 15, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        35, 33, 40, 00, 00, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 00, 00, 00, 00, 34,
    ],
    [
        35, 35, 32, 32, 32, 32, 32, 33, 00, 00, 00, 00, 00, 00, 00, 31, 33, 00, 00, 34,
    ],
    [
        36, 11, 00, 00, 12, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 00, 00, 00, 31, 35,
    ],
    [
        35, 32, 32, 32, 32, 32, 32, 32, 33, 14, 14, 14, 14, 14, 14, 14, 31, 32, 35, 35,
    ],
];

const LEVEL_4: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 99, 34,
    ],
    [
        35, 33, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 31, 32, 32, 35,
    ],
    [
        35, 36, 00, 00, 00, 00, 42, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34, 35, 35, 35,
    ],
    [
        35, 36, 00, 00, 00, 31, 32, 33, 00, 00, 00, 00, 00, 00, 00, 00, 34, 35, 35, 35,
    ],
    [
        35, 36, 11, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 34, 35, 35, 35,
    ],
    [
        35, 35, 32, 32, 32, 32, 32, 32, 32, 33, 14, 14, 14, 14, 14, 14, 34, 35, 35, 35,
    ],
];

const LEVEL_5: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 31, 32, 33, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 41, 34, 35, 36, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 31, 33, 00, 00, 31, 33, 37, 38, 39, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 12, 00, 32, 12, 00, 00, 00, 00, 34, 36, 00, 00, 12, 00, 00, 34,
    ],
    [
        36, 12, 11, 00, 00, 00, 35, 00, 31, 32, 32, 32, 35, 36, 00, 31, 32, 32, 32, 35,
    ],
    [
        36, 42, 42, 42, 42, 00, 35, 00, 34, 35, 35, 35, 35, 36, 00, 12, 00, 00, 99, 34,
    ],
    [
        35, 32, 32, 32, 32, 32, 35, 32, 35, 35, 35, 35, 35, 35, 32, 32, 32, 32, 32, 35,
    ],
];

const LEVEL_6: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 14, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 00, 00, 00, 00, 99, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 31, 35,
    ],
    [
        36, 14, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 14, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 14, 00, 15, 14, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 15, 14, 00, 00, 14, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 31, 00, 15, 00, 00, 00, 14, 14, 00, 00, 15, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 31, 35, 00, 15, 00, 12, 00, 15, 12, 00, 00, 15, 00, 14, 34,
    ],
    [
        36, 11, 00, 00, 00, 12, 12, 00, 40, 00, 42, 00, 40, 42, 00, 00, 40, 00, 00, 34,
    ],
    [
        35, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 35,
    ],
];

const LEVEL_7: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 99, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 12, 34,
    ],
    [
        36, 11, 00, 00, 00, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        35, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 35,
    ],
];

const LEVEL_8: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 99, 00, 00, 34,
    ],
    [
        36, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 11, 00, 00, 00, 00, 12, 00, 00, 00, 14, 00, 00, 00, 14, 00, 00, 15, 12, 34,
    ],
    [
        36, 00, 00, 00, 00, 42, 42, 42, 42, 12, 15, 00, 14, 12, 42, 15, 14, 15, 00, 34,
    ],
    [
        36, 00, 00, 42, 42, 42, 42, 42, 42, 14, 14, 00, 42, 14, 14, 40, 00, 40, 00, 34,
    ],
    [
        35, 32, 32, 32, 32, 32, 35, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 35,
    ],
];
const LEVEL_0: [[i32; LEVEL_WIDTH]; LEVEL_HEIGHT] = [
    [
        35, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 35,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 34,
    ],
    [
        36, 11, 00, 12, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 99, 34,
    ],
    [
        35, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 35,
    ],
];

const LEVEL_LIST: [[[i32; LEVEL_WIDTH]; LEVEL_HEIGHT]; 9] = [
    TEST_LEVEL, LEVEL_1, LEVEL_2, LEVEL_3, LEVEL_4, LEVEL_5, LEVEL_6, LEVEL_7, LEVEL_8,
];

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
                        title: "Untidaled".to_string(),
                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(LevelIndex(8))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, (setup, scale_screen).chain())
        .add_systems(OnEnter(GameState::Playing), spawn_level)
        .add_systems(
            FixedUpdate,
            (
                apply_gravity,
                advance_physics,
                check_for_collisions,
                ground_check,
                check_for_exit,
                oob_check,
                coyote_time,
                scale_screen,
                wind_collision,
                death_respawn,
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

#[derive(Component, Default)]
struct NoBubble;

#[derive(Component)]
struct Player {
    coyote_timer: Timer,
    is_grounded: bool,
    is_left: bool,
    is_moving: bool,
    is_bubbling: bool,
    bubbled: bool,
    jumped: bool,
    can_jump: bool,
    h_speed: f32,
    jump_force: f32,
    gravity: f32,
}

#[derive(Component)]
struct Bubble;

#[derive(Component)]
struct Exit;

#[derive(Component)]
struct Spike;

#[derive(Component)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// #[derive(Component)]
// struct Fan {
//     direction: Direction
// }

#[derive(Component)]
struct Wind {
    direction: Direction,
    force: f32,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let image = asset_server.load("oceanbg.png");
    commands.spawn((
        Sprite::from_image(image),
        Transform::from_scale(Vec3::splat(2.5)),
    ));
}

fn wind_collision(
    mut bubble_query: Query<&mut Transform, With<Bubble>>,
    mut wind_query: Query<(&mut Transform, &Wind), Without<Bubble>>,
) {
    for mut bubble in bubble_query.iter_mut() {
        let bubble_center = bubble.translation.truncate();
        let bubble_aabb = Aabb2d::new(bubble_center, Vec2::splat(16.));

        for (wind, windprops) in wind_query.iter_mut() {
            let wind_center = wind.translation.truncate();
            let wind_aabb = Aabb2d::new(wind_center, Vec2::splat(16.));

            let x_overlaps =
                bubble_aabb.min.x < wind_aabb.max.x && bubble_aabb.max.x > wind_aabb.min.x;
            let y_overlaps =
                bubble_aabb.min.y < wind_aabb.max.y && bubble_aabb.max.y > wind_aabb.min.y;

            if x_overlaps && y_overlaps {
                match windprops.direction {
                    Direction::Down => bubble.translation.y -= windprops.force,

                    Direction::Up => bubble.translation.y += windprops.force,

                    Direction::Right => bubble.translation.x += windprops.force,

                    Direction::Left => bubble.translation.x -= windprops.force,
                }
            }
        }
    }

    //if colliding with wind
}

fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_index: Res<LevelIndex>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let level = LEVEL_LIST[level_index.0];
    match level_index.0 {
        1 => {
            commands.spawn((
                StateScoped(GameState::Playing),
                Text2d::new("A/D to move\nspace to jump\nshift to interact"),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_translation(Vec3::new(
                    BSIZE as f32 * 4. - 16. * LEVEL_WIDTH as f32,
                    -(BSIZE as f32 * 2. - 16. * LEVEL_HEIGHT as f32),
                    10.,
                )),
            ));
        }
        5 => {
            commands.spawn((
                StateScoped(GameState::Playing),
                Text2d::new("esc to respawn"),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_translation(Vec3::new(
                    BSIZE as f32 * 4. - 16. * LEVEL_WIDTH as f32,
                    -(BSIZE as f32 * 2. - 16. * LEVEL_HEIGHT as f32),
                    10.,
                )),
            ));
        }
        _ => (),
    }
    for (i, row) in level.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            match elem {
                //0 is empty
                //Player
                11 => {
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
                            10.,
                        ),
                        Name::new("Player"),
                        AccumulatedInput::default(),
                        Velocity::default(),
                        PhysicalTranslation(Vec3::new(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            10.,
                        )),
                        PreviousPhysicalTranslation::default(),
                        Player {
                            coyote_timer: Timer::new(
                                Duration::from_secs_f32(0.2),
                                TimerMode::Repeating,
                            ),
                            is_grounded: false,
                            is_left: false,
                            is_moving: false,
                            is_bubbling: false,
                            bubbled: false,
                            jumped: false,
                            can_jump: false,
                            jump_force: 210., //jump force? peak peak
                            h_speed: 100.,
                            gravity: 600.,
                        },
                        Collider,
                    ));
                }
                //Bubble Objects
                12 => {
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
                            5.,
                        ),
                        Collider,
                        Bubble,
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Spike Objects
                14 => {
                    let texture = asset_server.load("spikes.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 1, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Spike,
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Low-Up-Wind
                15 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Up,
                            force: LOW_WIND,
                        },
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
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Mid-Up-Wind
                16 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Up,
                            force: MID_WIND,
                        },
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
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Hi-Up-Wind
                17 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Up,
                            force: HI_WIND,
                        },
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
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Low-Down-Wind
                18 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Down,
                            force: LOW_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(180f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Mid-Down-Wind
                19 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Down,
                            force: MID_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(180f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Hi-Down-Wind
                20 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Down,
                            force: HI_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(180f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Low-Right-Wind
                21 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Right,
                            force: LOW_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Mid-Right-Wind
                22 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Right,
                            force: MID_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Hi-Right-Wind
                23 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Right,
                            force: HI_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Low-Left-Wind
                24 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Left,
                            force: LOW_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(-90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Mid-Left-Wind
                25 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Left,
                            force: MID_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(-90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                //Hi-Left-Wind
                26 => {
                    let texture = asset_server.load("windSquare-Sheet-32x32.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices { first: 0, last: 2 };
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Wind {
                            direction: Direction::Left,
                            force: HI_WIND,
                        },
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: animation_indices.first,
                            },
                        ),
                        Transform {
                            translation: Vec3::new(
                                BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                                -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                                2.,
                            ),
                            rotation: Quat::from_rotation_z(-90f32.to_radians()),
                            scale: Vec3::ONE,
                        },
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                    ));
                }
                // platform
                31 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                32 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 1,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                33 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 2,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                34 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 3,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                35 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 4,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                36 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 5,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                37 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 6,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                38 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 7,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                39 => {
                    let texture = asset_server.load("platform.aesprite.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 3, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 8,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        Collider,
                    ));
                }
                // volcano
                40 => {
                    let texture = asset_server.load("volcanobuttoncoral.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 5, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                    ));
                }
                // coral
                41 => {
                    let texture = asset_server.load("volcanobuttoncoral.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 5, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 3,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        NoBubble,
                    ));
                }
                // tree
                42 => {
                    let texture = asset_server.load("volcanobuttoncoral.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 5, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        StateScoped(GameState::Playing),
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 4,
                            },
                        ),
                        Transform::from_xyz(
                            BSIZE as f32 * j as f32 - 16. * LEVEL_WIDTH as f32,
                            -(BSIZE as f32 * i as f32 - 16. * LEVEL_HEIGHT as f32),
                            2.,
                        ),
                        NoBubble,
                    ));
                }
                99 => {
                    let texture = asset_server.load("rustacean.png");
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
                        animation_indices,
                        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
                        Exit,
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

fn death_respawn(
    player_query: Single<&PhysicalTranslation, With<Player>>,
    mut spikes_query: Query<&mut Transform, With<Spike>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let phys_translation = player_query.into_inner();

    let player_center = phys_translation.truncate();
    let player_aabb = Aabb2d::new(player_center, Vec2::new(10., 16.));
    for spikes in spikes_query.iter_mut() {
        let spikes_center = spikes.translation.truncate();
        let spikes_aabb = Aabb2d::new(spikes_center, Vec2::splat(10.));

        let x_overlaps =
            player_aabb.min.x < spikes_aabb.max.x && player_aabb.max.x > spikes_aabb.min.x;
        let y_overlaps =
            player_aabb.min.y < spikes_aabb.max.y && player_aabb.max.y > spikes_aabb.min.y;

        if x_overlaps && y_overlaps {
            next_state.set(GameState::GameOver);
        }
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
    collider_query: Query<&Transform, (Or<(With<NoBubble>, With<Collider>)>, Without<Player>)>,
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
        player.jumped = true;
        // player.is_grounded = false;
        // player.can_jump = false;
    } else {
        player.jumped = false;
    }
    if keyboard_input.just_pressed(KeyCode::ShiftLeft)
        | keyboard_input.just_pressed(KeyCode::ShiftRight)
    {
        let center = Vec2::new(
            position.x + if player.is_left { -32. } else { 32. },
            position.y,
        );
        let aabb = Aabb2d::new(center, Vec2::splat(16.));
        // gizmos.rect_2d(center, aabb.half_size() * 2., RED);
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
        let center = Vec2::new(
            f32::round(center.x / 32.) * 32.,
            f32::round(center.y / 32.) * 32.,
        );
        let aabb = Aabb2d::new(center, Vec2::splat(16.));
        // check if bubble would collide
        for collider in collider_query.iter() {
            let collider_center = collider.translation.truncate();
            let collider_aabb = Aabb2d::new(collider_center, Vec2::splat(16.));

            let x_overlaps = aabb.min.x < collider_aabb.max.x && aabb.max.x > collider_aabb.min.x;
            let y_overlaps = aabb.min.y < collider_aabb.max.y && aabb.max.y > collider_aabb.min.y;

            // if intersects, move back by larger axis
            if x_overlaps && y_overlaps {
                return;
            }
        }

        player.bubbled = true;
        player.is_bubbling = false;
        let texture = asset_server.load("bubble-idle-32x32.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(BSIZE), 3, 1, None, None);
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
                f32::round(center.x / 32.) * 32.,
                f32::round(center.y / 32.) * 32.,
                5.,
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

fn ground_check(
    player_query: Single<(&PhysicalTranslation, &mut Player)>,
    collider_query: Query<&Transform, (With<Collider>, Without<Player>)>,
) {
    let (physical_translation, mut player) = player_query.into_inner();
    player.is_grounded = false;
    for collider in collider_query.iter() {
        let center = physical_translation.truncate();
        let aabb = Aabb2d::new(Vec2::new(center.x, center.y - 0.001), Vec2::new(12., 16.));

        let collider_center = collider.translation.truncate();
        let collider_aabb = Aabb2d::new(collider_center, Vec2::splat(16.));

        let x_overlaps = aabb.min.x < collider_aabb.max.x && aabb.max.x > collider_aabb.min.x;
        let y_overlaps = aabb.min.y < collider_aabb.max.y && aabb.max.y > collider_aabb.min.y;

        if x_overlaps
            && y_overlaps
            && f32::abs(physical_translation.y - collider_center.y)
                > f32::abs(physical_translation.x - collider_center.x)
        {
            player.is_grounded = true;
            player.can_jump = true;
        }
    }
    if player.jumped {
        player.can_jump = false;
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
    let aabb = Aabb2d::new(center, Vec2::new(12., 16.));

    // gizmos.rect_2d(center, Vec2::new(12., 16.) * 2., YELLOW);

    let mut check_once = true;
    'outer: for collider in collider_query.iter() {
        let collider_center = collider.translation.truncate();
        let collider_aabb = Aabb2d::new(collider_center, Vec2::splat(16.));

        let x_overlaps = aabb.min.x < collider_aabb.max.x && aabb.max.x > collider_aabb.min.x;
        let y_overlaps = aabb.min.y < collider_aabb.max.y && aabb.max.y > collider_aabb.min.y;

        // if intersects, move back by larger axis
        if x_overlaps && y_overlaps {
            let displacement;
            // let mut to_ground = player.is_grounded;
            let aabb2;
            // check which axis is larger first
            if f32::abs(previous_physical_translation.y - collider_center.y)
                > f32::abs(previous_physical_translation.x - collider_center.x)
            {
                displacement = if previous_physical_translation.y > collider_center.y {
                    // to_ground = true;
                    aabb.min.y - collider_aabb.max.y
                } else {
                    aabb.max.y - collider_aabb.min.y
                };
                let center2 = Vec2::new(center.x, center.y - displacement);
                aabb2 = Aabb2d::new(center2, Vec2::new(12., 16.));

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
                // player.is_grounded = to_ground;
                // player.can_jump = to_ground;
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
                aabb2 = Aabb2d::new(center2, Vec2::new(12., 16.));

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
                            // to_ground = true;
                            aabb2.min.y - collider_aabb2.max.y
                        } else {
                            aabb2.max.y - collider_aabb2.min.y
                        };
                        physical_translation.y -= displacement2;
                        // player.is_grounded = to_ground;
                        // player.can_jump = to_ground;
                        velocity.y = 0.;
                        break;
                    }
                }
                velocity.x = 0.;
                physical_translation.x -= displacement;
            }
            // gizmos.rect_2d(
            //     collider_center,
            //     collider_aabb.half_size() * 2.,
            //     SPRING_GREEN,
            // );
            // skip the rest of collision checking once one collision is found
            break;
        }
    }
}

fn check_for_exit(
    // mut gizmos: Gizmos,
    player_query: Single<&PhysicalTranslation, With<Player>>,
    exit_query: Query<&Transform, With<Exit>>,
    mut ev_nextlevel: EventWriter<NextLevelEvent>,
) {
    let physical_translation = player_query.into_inner();

    let center = physical_translation.truncate();
    let aabb = Aabb2d::new(center, Vec2::new(12., 16.));
    // gizmos.rect_2d(center, aabb.half_size() * 2., YELLOW);

    // player.is_grounded = false;
    for exit in exit_query.iter() {
        let exit_center = exit.translation.truncate();
        let exit_aabb = Aabb2d::new(exit_center, Vec2::splat(12.));

        let x_overlaps = aabb.min.x < exit_aabb.max.x && aabb.max.x > exit_aabb.min.x;
        let y_overlaps = aabb.min.y < exit_aabb.max.y && aabb.max.y > exit_aabb.min.y;

        // if intersects, move back by larger axis
        if x_overlaps && y_overlaps {
            ev_nextlevel.send(NextLevelEvent(1));
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
    if index == LEVEL_LIST.len() as i32 {
        index = 1;
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
