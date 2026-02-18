use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_click, animate_roll))
        .run();
}

/// Composant attaché au dé pour gérer son état d'animation
#[derive(Component)]
struct Dice {
    is_rolling: bool,
    timer: f32,
    duration: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Caméra 3D
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Lumière ambiante
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    // Lumière directionnelle
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.5, 0.0)),
    ));

    // Le dé (cube 1.5 unités)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.95, 0.95),
            perceptual_roughness: 0.4,
            ..default()
        })),
        Transform::default(),
        Dice {
            is_rolling: false,
            timer: 0.0,
            duration: 1.5,
        },
    ));
}

/// Détecte le clic de souris sur le dé via un test d'intersection rayon-sphère
fn handle_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut dice_query: Query<(&GlobalTransform, &mut Dice)>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else {
        return;
    };

    for (dice_transform, mut dice) in dice_query.iter_mut() {
        if dice.is_rolling {
            continue;
        }

        // Test d'intersection rayon / sphère englobante du dé (rayon ≈ 1.3 pour un cube de 1.5)
        let dice_pos = dice_transform.translation();
        let oc = ray.origin - dice_pos;
        let b = oc.dot(*ray.direction);
        let c = oc.dot(oc) - 1.3_f32 * 1.3_f32;
        let discriminant = b * b - c;

        if discriminant >= 0.0 {
            dice.is_rolling = true;
            dice.timer = 0.0;
        }
    }
}

/// Anime la rotation du dé quand il est en train de rouler
fn animate_roll(
    time: Res<Time>,
    mut dice_query: Query<(&mut Transform, &mut Dice)>,
) {
    for (mut transform, mut dice) in dice_query.iter_mut() {
        if !dice.is_rolling {
            continue;
        }

        dice.timer += time.delta_secs();

        if dice.timer >= dice.duration {
            // Animation terminée : arrêt
            dice.is_rolling = false;
            dice.timer = 0.0;
        } else {
            // Ralentissement progressif vers la fin
            let progress = dice.timer / dice.duration;
            let speed = 15.0 * (1.0 - progress * 0.85);
            transform.rotate_x(speed * time.delta_secs());
            transform.rotate_y(speed * 0.7 * time.delta_secs());
            transform.rotate_z(speed * 0.3 * time.delta_secs());
        }
    }
}
