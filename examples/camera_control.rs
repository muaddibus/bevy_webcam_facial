use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

// bevy_mod_interp modified/adapted for compatability with bevy 0.11
use bevy_mod_interp::*;

use bevy_webcam_facial::*;

#[derive(Component)]
struct CameraControl;

#[derive(Component)]
struct CameraEye;

#[derive(Resource, Default)]
struct Average {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .insert_resource(Average::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(WebcamFacialPlugin {
            config_webcam_device: "/dev/video0".to_string(),
            config_webcam_width: 640,
            config_webcam_height: 480,
            config_webcam_framerate: 33,
            config_webcam_autostart: true,
        })
        .add_plugins(HookPlugin)
        .add_plugins(InterpPlugin)
        .add_systems(Startup, load_scene)
        .add_systems(
            Update,
            (
                set_camera_position_from_plugin,
                user_input_to_plugin_control_system,
            ),
        )
        .run();
}

/// set up 3D scene
fn load_scene(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("rooster.gltf#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str()) {
                Some("Camera") => {
                    cmds.insert(CameraEye);
                    // .insert(InterpTransform::new(
                    //     Vec3::new(0.0, 2.0, 7.0),            // Target translation
                    //     Quat::from_rotation_y(0.0),     // Target rotation (90 degrees around Y-axis)
                    //     Vec3::new(1.0, 1.0, 1.0),            // Target scale
                    //     0.1,                                 // Easing duration (2 seconds)
                    //     1.0,                                 // Easing exponent
                    // ));
                }
                Some("CameraTarget") => {
                    cmds.insert(CameraControl).insert(InterpTransform::new(
                        Vec3::new(0.0, 2.0, 7.0),   // Target translation
                        Quat::from_rotation_y(0.0), // Target rotation (90 degrees around Y-axis)
                        Vec3::new(1.0, 1.0, 1.0),   // Target scale
                        0.1,                        // Easing duration (2 seconds)
                        1.0,                        // Easing exponent
                    ));
                }
                _ => {}
            };
        }),
    });
}

fn set_camera_position_from_plugin(
    mut camera_target: Query<&mut InterpTransform, With<CameraControl>>,
    mut camera: Query<&mut Transform, With<CameraEye>>,

    mut average: ResMut<Average>,
    mut webcam_data: EventReader<WebcamFacialDataEvent>,
    _time: Res<Time>,
) {
    // Simply dirty average all coord sum, and average them with previous values to smooth a bit
    for event in webcam_data.iter() {
        let x = event.0.center_x as f32;
        let y = event.0.center_y as f32;
        average.x = (x + average.x) / 20.0;
        average.y = (y + average.y) / 20.0;

        for mut interp_transform in camera_target.iter_mut() {
            // Default bevy cam movement
            //interp_transform.target_translation = Transform::from_xyz(0.0, 2.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y).translation;

            // Update the translation target of InterpTransform

            interp_transform.update_target(Transform::from_translation(Vec3::new(
                -average.x, 1.0, average.y,
            )));

            //interp_transform.target_translation = Vec3::new(average.x, 2.0, average.y-7.0);

            //interp_transform.target_rotation = Quat::from_rotation_y(-average.x);

            //interp_transform.target_rotation *= Quat::from_rotation_x(average.x) + Quat::from_rotation_z(average.y);

            // interp_transform.target_scale *= 1;

            // Set camera to loot at target
            for mut transform in camera.iter_mut() {
                // get target
                // Set look at
                transform.rotation = Transform::from_xyz(0.0, 2.0, 5.0)
                    .looking_at(interp_transform.target_translation, Vec3::Y)
                    .rotation;
                info!("Target{:?}", interp_transform.target_translation);
                info!("Camera{:?}", transform.rotation);
            }
        }
    }
}

// Keyboard control for webcam and app
fn user_input_to_plugin_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut webcam_facial_control: ResMut<WebcamFacialControl>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        webcam_facial_control.0 = true;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        webcam_facial_control.0 = false;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
