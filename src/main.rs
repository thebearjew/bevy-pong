mod components;
mod constants;
mod keymaps;
mod systems;

use crate::components::*;
use crate::constants::*;
use crate::systems::{
    collision::collision_system, input::keyboard_input_system, particles::particle_emission_system,
    particles::particle_update_time_system, velocity::velocity_system,
    round::{randomize_ball_direction, round_system},
};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::{prelude::*, render::pass::ClearColor};


fn main() {
    App::build()
        // Clear Color is the background color
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Game::default())
        .insert_resource(WindowDescriptor {
            title: "Bevy Pong".to_string(),
            width: 1000.0,
            height: 800.0,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(startup_system.system())
        .add_system(round_system.system())
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_input_system.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label("physics")
                .with_system(collision_system.system().label("collision"))
                .with_system(
                    velocity_system
                        .system()
                        .label("velocity")
                        .after("collision"),
                ),
        )
        .add_system(particle_emission_system.system())
        .add_system(particle_update_time_system.system())
        // .add_system(render_system.system().after("physics"))
        .run();
}

// The origin (0,0) of bevy's coordinate system is in the center of the screen
fn startup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowDescriptor>,
    game: Res<Game>,
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Left Player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            transform: Transform::from_xyz(-PADDLE_X_OFFSET * window.width, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Player {
            player_type: PlayerType::Left,
        })
        .insert(Size::new(PADDLE_SIZE[0], PADDLE_SIZE[1]))
        .insert(Collidable::Reflect);

    // Right Player
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(PADDLE_SIZE)),
            transform: Transform::from_xyz(PADDLE_X_OFFSET * window.width, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Player {
            player_type: PlayerType::Right,
        })
        .insert(Size::new(PADDLE_SIZE[0], PADDLE_SIZE[1]))
        .insert(Collidable::Reflect);

    // Ball
    let initial_ball_velocity = randomize_ball_direction(&window, &game).1;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            sprite: Sprite::new(Vec2::from(BALL_SIZE)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(ParticleEmitter)
        .insert(Ball)
        .insert(Size::new(BALL_SIZE[0], BALL_SIZE[1]))
        .insert(initial_ball_velocity);

    // Invisible walls for collision detection
    let wall_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let wall_thickness = BALL_SIZE[0];
    commands // Left wall
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(wall_thickness, window.height)),
            transform: Transform::from_xyz((-window.width / 2.0) + 1.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(WallSide::Left)
        .insert(Collidable::End);

    commands // Right wall
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(wall_thickness, window.height)),
            transform: Transform::from_xyz((window.width / 2.0) - 1.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(WallSide::Right)
        .insert(Collidable::End);

    commands // Top wall
        .spawn_bundle(SpriteBundle {
            material: wall_material.clone(),
            sprite: Sprite::new(Vec2::new(window.width, wall_thickness)),
            transform: Transform::from_xyz(0.0, (window.height / 2.0) - 1.0, 0.0),
            ..Default::default()
        })
        .insert(Collidable::Reflect);
    commands // Bottom wall
        .spawn_bundle(SpriteBundle {
            material: wall_material,
            sprite: Sprite::new(Vec2::new(window.width, wall_thickness)),
            transform: Transform::from_xyz(0.0, -(window.height / 2.0) + 1.0, 0.0),
            ..Default::default()
        })
        .insert(Collidable::Reflect);

    // Dashes
    let window_top = (window.height / 2.0).abs();
    let dash_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let dash_sprite = Sprite::new(Vec2::from(DASH_SIZE));
    let dash_count = ((window.height) / (DASH_HEIGHT + DASH_PADDING)).floor() as u16;
    let mut dashes = vec![];
    for i in 0..=dash_count {
        let y = window_top - (i as f32 * (DASH_PADDING + DASH_HEIGHT));
        dashes.push(SpriteBundle {
            material: dash_material.clone(),
            sprite: dash_sprite.clone(),
            transform: Transform::from_xyz(0.0, y, 0.0),
            ..Default::default()
        });
    }

    commands.spawn_batch(dashes);
}
