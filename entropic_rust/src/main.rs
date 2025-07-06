use bevy::{color::palettes::css::RED, prelude::*};
use rand::Rng;

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn Camera
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(20.0));
    let black = Color::srgb(0.0, 0.0, 0.0);
    let white = Color::srgb(1.0, 1.0, 1.0);

    // Box dimensions - much bigger and better proportioned
    let box_width = 800.0;
    let box_height = 600.0;
    let wall_thickness = 20.0;

    // Calculate positions for centered box
    let half_width = box_width / 2.0;
    let half_height = box_height / 2.0;

    // Draw Floor
    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(box_width + wall_thickness, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -half_height, 0.0),
    ));

    // Draw Ceiling
    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(box_width + wall_thickness, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, half_height, 0.0),
    ));

    // Left Wall
    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(wall_thickness, box_height)),
            ..default()
        },
        Transform::from_xyz(-half_width, 0.0, 0.0),
    ));

    // Right Wall
    commands.spawn((
        Sprite {
            color: white,
            custom_size: Some(Vec2::new(wall_thickness, box_height)),
            ..default()
        },
        Transform::from_xyz(half_width, 0.0, 0.0),
    ));

    // Spawn initial particle
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(black)),
        Transform::from_xyz(0.0, 200.0, 0.0),
        Particle {
            velocity: Vec2::new(240.0, 80.0),
        },
    ));
}

#[derive(Component)]
struct Particle {
    velocity: Vec2,
}

struct RandomColor {
    r: f32,
    g: f32,
    b: f32,
}

impl RandomColor {
    fn new() -> Self {
        Self {
            r: rand::rng().random_range(0.0..1.0),
            g: rand::rng().random_range(0.0..1.0),
            b: rand::rng().random_range(0.0..1.0),
        }
    }
}

fn move_particle(mut query: Query<(&mut Transform, &mut Particle)>, time: Res<Time>) {
    const GRAVITY: f32 = 400.0; // Increased gravity for better physics feel
    const COLLISION_DAMPING: f32 = 0.80;
    const MIN_VELOCITY: f32 = 5.0; // Stop very small movements

    // Box collision boundaries (accounting for ball radius of 20)
    const LEFT_BOUNDARY: f32 = -380.0; // -400 + 20 (radius)
    const RIGHT_BOUNDARY: f32 = 380.0; // 400 - 20 (radius)
    const BOTTOM_BOUNDARY: f32 = -280.0; // -300 + 20 (radius)
    const TOP_BOUNDARY: f32 = 280.0; // 300 - 20 (radius)

    let down = Vec2::new(0.0, -1.0);

    for (mut trans, mut part) in &mut query {
        // Apply gravity
        part.velocity += down * GRAVITY * time.delta_secs();

        // Update position
        trans.translation.x += part.velocity.x * time.delta_secs();
        trans.translation.y += part.velocity.y * time.delta_secs();

        // Right wall collision
        if trans.translation.x > RIGHT_BOUNDARY {
            trans.translation.x = RIGHT_BOUNDARY;
            part.velocity.x *= -COLLISION_DAMPING;
        }

        // Left wall collision
        if trans.translation.x < LEFT_BOUNDARY {
            trans.translation.x = LEFT_BOUNDARY;
            part.velocity.x *= -COLLISION_DAMPING;
        }

        // Floor collision
        if trans.translation.y < BOTTOM_BOUNDARY {
            trans.translation.y = BOTTOM_BOUNDARY;
            part.velocity.y *= -COLLISION_DAMPING;
        }

        // Ceiling collision
        if trans.translation.y > TOP_BOUNDARY {
            trans.translation.y = TOP_BOUNDARY;
            part.velocity.y *= -COLLISION_DAMPING;
        }

        // Stop very small movements to prevent eternal tiny bounces
        if part.velocity.length() < MIN_VELOCITY {
            part.velocity = Vec2::ZERO;
        }
    }
}

// Function to spawn sprites using the left mouse
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

                let shape = meshes.add(Circle::new(20.0));
                let random_c = RandomColor::new();
                let color = Color::srgb(random_c.r, random_c.g, random_c.b);

                commands.spawn((
                    Mesh2d(shape),
                    MeshMaterial2d(materials.add(color)),
                    Transform::from_xyz(world_x, world_y, 0.0),
                    Particle {
                        velocity: Vec2::new(
                            rand::rng().random_range(-300.0..300.0), // Allow negative velocities
                            rand::rng().random_range(-100.0..300.0),
                        ),
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
