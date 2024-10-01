use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_paddles));
    app.add_systems(Update, move_paddle);
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
            custom_size: Some(Vec2::new(700.0, 500.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10.0, 150.0)),
            ..default()
        },
        ..default()
    }, Paddle {
        move_up: KeyCode::KeyT,
        move_down: KeyCode::KeyH
    }));

    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10.0, 150.0)),
            ..default()
        },
        ..default()
    }, Paddle {
        move_up: KeyCode::ArrowUp,
        move_down: KeyCode::ArrowDown
    }));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) && pos.translation.y < 170.0 {
            pos.translation.y += 100.0 * time.delta_seconds();
        }
        if input.pressed(settings.move_down) && pos.translation.y > -170.0 {
            pos.translation.y -= 100.0 * time.delta_seconds();
        }
    }
}
