use bevy::{color::palettes::css::RED, prelude::*};

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
    time: Res<Time>,
) {
    let sin_t_scaled = ops::sin(time.elapsed_secs()) * 50.;
    gizmos.line_2d(Vec2::Y * -sin_t_scaled, Vec2::splat(-80.), RED);
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(30.0));

    let black = Color::srgb(0.0, 0.0, 0.0);
    let white = Color::srgb(255.0, 255.0, 255.0);

    // draw Floor
    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(900.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(-43.0, -436.6257, 0.0),
    ));

    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(10.0, 883.0)),
            ..default()
        },
        Transform::from_xyz(-73.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(10.0, 883.0)),
            ..default()
        },
        Transform::from_xyz(73.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(black)),
        Transform::from_xyz(50.0, 50.0, 0.0),
        Particle {
            velocity: Vec2::new(55.0, 20.0),
        },
    ));
}

#[derive(Component)]
struct Particle {
    velocity: Vec2,
}
fn move_particle(mut query: Query<(&mut Transform, &mut Particle)>, time: Res<Time>) {
    const GRAVITY: f32 = 10.0;

    let down = Vec2::new(0.0, -1.0);

    for (mut trans, mut part) in &mut query {
        part.velocity += down * GRAVITY * time.delta_secs();

        trans.translation.x += part.velocity.x * time.delta_secs();

        trans.translation.y += part.velocity.y * time.delta_secs();

        if trans.translation.x > 43.0 {
            trans.translation.x = 43.0;
            part.velocity.x *= -1.0;
        }

        if trans.translation.x < -43.0 {
            trans.translation.x = -43.0;
            part.velocity.x *= -1.0;
        }
        if trans.translation.y < -406.0 {
            info!("{}", trans.translation.y);
            trans.translation.y = -406.0;
            part.velocity.y *= -1.0;
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, startup);
    app.add_systems(Update, move_particle);
    app.run();
}
