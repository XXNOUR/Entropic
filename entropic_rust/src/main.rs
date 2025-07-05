use bevy::{color::palettes::css::RED, prelude::*};
use rand::Rng;

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

struct random_color {
    r: f32,
    g: f32,
    b: f32,
}
impl random_color {
    fn new() -> Self {
        Self {
            r: rand::rng().random_range(-800.0..800.0),

            g: rand::rng().random_range(-800.0..800.0),
            b: rand::rng().random_range(-800.0..800.0),
        }
    }
}
#[derive(Component)]
struct Particle {
    velocity: Vec2,
}
/*#[derive(Resource)]
struct Damping(f32); */

fn move_particle(mut query: Query<(&mut Transform, &mut Particle)>, time: Res<Time>) {
    const GRAVITY: f32 = 10.0;
    const Collision_Dumpign: f32 = 0.7;
    let down = Vec2::new(0.0, -1.0);

    for (mut trans, mut part) in &mut query {
        part.velocity += down * GRAVITY * time.delta_secs();

        trans.translation.x += part.velocity.x * time.delta_secs();

        trans.translation.y += part.velocity.y * time.delta_secs();

        if trans.translation.x > 43.0 {
            trans.translation.x = 43.0;
            part.velocity.x *= -1.0 * Collision_Dumpign;
        }

        if trans.translation.x < -43.0 {
            trans.translation.x = -43.0;
            part.velocity.x *= -1.0 * Collision_Dumpign;
        }
        if trans.translation.y < -406.0 {
            info!("{}", trans.translation.y);
            trans.translation.y = -406.0;
            part.velocity.y *= -1.0 * Collision_Dumpign;
        }
    }
}
// function to spawn sprites using the left mouse ;
fn spawn_at_mouse_click(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                // Convert cursor position to world coordinates
                let world_x = cursor_pos.x - (window.width() / 2.0);
                let world_y = (window.height() / 2.0) - cursor_pos.y;

                let shape = meshes.add(Circle::new(30.0));
                let random_c = random_color::new();
                let color = Color::srgb(random_c.r, random_c.g, random_c.b);

                commands.spawn((
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(world_x, world_y, 0.0),
                    Particle {
                        velocity: Vec2::new(55.0, 20.0),
                    },
                ));
            }
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, startup);
    app.add_systems(Update, move_particle);
    app.add_systems(Update, spawn_at_mouse_click);
    app.run();
}
