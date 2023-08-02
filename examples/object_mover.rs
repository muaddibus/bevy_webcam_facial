use bevy::prelude::*;

use bevy_webcam_facial::*;

#[derive(Component)]
struct WebcamControlledObject;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add plugin with a *MUST* camera parameters
        .add_plugins(WebcamFacialPlugin {
            config_webcam_device: "/dev/video0".to_string(),
            config_webcam_width: 320,
            config_webcam_height: 240,
            config_webcam_framerate: 33,
            config_webcam_autostart: true,
        })
        .add_systems(Startup, setup)
        // Add system to read data events and do something with data
        .add_systems(Update, move_object)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Cube object with WebcamControlledObject
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        // Mark object
        WebcamControlledObject,
    ));
    // Some light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera for scene
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_object(
    // Get object with WebcamControlledObject
    mut query: Query<&mut Transform, With<WebcamControlledObject>>,
    // Read data event with camera data
    mut reader: EventReader<WebcamFacialDataEvent>,
) {
    for event in reader.iter() {
        // Get raw data from event. Data is pretty noisy, needs manual smoothing/averaging or something else
        // (watch other examples for some ideas and better usage)
        let x = event.0.center_x;
        let y = event.0.center_y;
        let width = event.0.width;
        let height = event.0.height;
        // Do basic transforms
        for mut transform in query.iter_mut() {
            // Move object with x100 less influence
            transform.translation.x = -x as f32 / 100.0;
            transform.translation.z = y as f32 / 100.0;
            // Scale object relative to face size (face area / 10000)
            transform.scale = Vec3::splat((width * height) as f32 / 10000.0);
        }
    }
}
