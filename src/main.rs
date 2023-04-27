use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let sprite_bundle = SpriteBundle {
        transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
        texture: asset_server.load("sprites/ball_blue_large.png"),
        ..default()
    };
    let player = Player { name: String::from("Gelo") };

    commands.spawn((
        sprite_bundle,
        player
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
            ..default()
        }
    );
}

pub const PLAYER_SPEED: f32 = 250.0;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let is_moving_left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let is_moving_right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);
        let is_moving_down = keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);
        let is_moving_up = keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);

        let mut direction = Vec3::ZERO;

        if is_moving_left {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if is_moving_right {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if is_moving_down {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if is_moving_up {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct Player {
    pub name: String
}
