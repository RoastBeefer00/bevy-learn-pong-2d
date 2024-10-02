use bevy::{ecs::component, prelude::*, window::WindowResolution};
use rand::Rng;

const WINDOW_HEIGHT: f32 = 1280.0;
const WINDOW_WIDTH: f32 = 720.0;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            resizable: false,
            ..default()
        }),
        ..default()
    }));
    app.add_systems(Startup, (spawn_camera, spawn_paddles, spawn_ball));
    app.add_systems(Update, (move_paddle, move_ball, ball_collide));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_paddles(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WINDOW_HEIGHT, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0 - 20.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 150.0)),
                ..default()
            },
            ..default()
        },
        Paddle {
            move_up: KeyCode::KeyT,
            move_down: KeyCode::KeyH,
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0 + 20.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 150.0)),
                ..default()
            },
            ..default()
        },
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 200.0 * time.delta_seconds();
            pos.translation.y = pos
                .translation
                .y
                .clamp(-WINDOW_HEIGHT / 2.0 - 75.0, WINDOW_HEIGHT / 2.0 - 75.0);
        }
        if input.pressed(settings.move_down) {
            pos.translation.y -= 200.0 * time.delta_seconds();
            pos.translation.y = pos
                .translation
                .y
                .clamp(-WINDOW_HEIGHT / 2.0 - 75.0, WINDOW_HEIGHT / 2.0 - 75.0);
        }
    }
}

#[derive(Component)]
struct Ball(Vec2);

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            ..default()
        },
        Ball(Vec2::new(-100.0, 0.0)),
    ));
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut pos, ball) in &mut balls {
        pos.translation += ball.0.extend(0.0) * time.delta_seconds();
    }
}

const BWIDTH: f32 = 25.0;
const PWIDTH: f32 = 25.0;
const PHEIGHT: f32 = 25.0;

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball, mut velocity) in &mut balls {
        if ball.translation.y.abs() + BWIDTH / 2.0 > 250.0 {
            velocity.0.y *= -1.0;
        }
        for paddle in &paddles {
            if ball.translation.x - BWIDTH / 2.0 < paddle.translation.x + PWIDTH / 2.0
                && ball.translation.y - BWIDTH / 2.0 < paddle.translation.y + PHEIGHT / 2.0
                && ball.translation.x + BWIDTH / 2.0 > paddle.translation.x - PWIDTH / 2.0
                && ball.translation.y + BWIDTH / 2.0 > paddle.translation.y - PHEIGHT / 2.0
            {
                velocity.0 *= -1.0;
                velocity.0.y = rand::thread_rng().gen_range(-1.0..1.0) * 100.0;
            }
        }
    }
}
