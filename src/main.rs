use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_die)
        .run();
}

/// Composant attaché à l'entité dé.
/// `value` représente la face visible (1-6).
#[derive(Component)]
struct Die {
    value: u8,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // --- Caméra 3D ---
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // --- Lumière ambiante ---
    commands.spawn(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
        ..default()
    });

    // --- Lumière directionnelle ---
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // --- Le dé ---
    // Conventions dé standard : face opposées = 7 (1+6, 2+5, 3+4)
    // (face, direction_normale, rotation_du_quad)
    let faces: &[(u8, Vec3, Quat)] = &[
        (1, Vec3::Z,     Quat::IDENTITY),
        (6, Vec3::NEG_Z, Quat::from_rotation_y(std::f32::consts::PI)),
        (2, Vec3::Y,     Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        (5, Vec3::NEG_Y, Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        (3, Vec3::X,     Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        (4, Vec3::NEG_X, Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
    ];

    let offset = 0.501_f32;
    let quad_mesh = meshes.add(Rectangle::new(1.0, 1.0));

    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.95, 0.95, 0.95),
                perceptual_roughness: 0.4,
                ..default()
            })),
            Transform::default(),
            Die { value: 1 },
        ))
        .with_children(|parent| {
            for &(number, direction, rotation) in faces {
                let texture: Handle<Image> =
                    asset_server.load(format!("textures/die_{number}.png"));

                parent.spawn((
                    Mesh3d(quad_mesh.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(texture),
                        base_color: Color::WHITE,
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    })),
                    Transform {
                        translation: direction * offset,
                        rotation,
                        ..default()
                    },
                ));
            }
        });
}

/// Fait tourner le dé sur Y et légèrement sur X pour montrer le volume.
fn rotate_die(time: Res<Time>, mut query: Query<&mut Transform, With<Die>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs());
        transform.rotate_x(time.delta_secs() * 0.3);
    }
}
