use bevy::{app::AppExit, pbr::DirectionalLightShadowMap, prelude::*};
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use bevy_webcam_facial::*;

#[derive(Component)]
struct CameraControl;

#[derive(Component)]
struct CameraEye;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins)
        .add_plugins(WebcamFacialPlugin {
            config_webcam_device: 0,
            config_webcam_width: 640,
            config_webcam_height: 480,
            config_webcam_framerate: 33,
            config_webcam_autostart: true,
            // Using LowPass filter, with value of 'alpha' at 0.01 for last 20 frames to get more smoothing
            config_filter_length: 20,
            config_filter_type: SmoothingFilterType::LowPass(0.01),
        })
        // Using HookPlugin to get named object from loaded gltf scene
        .add_plugins(HookPlugin)
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
fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("rooster.gltf#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            if entity.get::<Name>().map(Name::as_str) == Some("Camera") {
                cmds.insert(CameraEye);
            }
        }),
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    });
    commands.spawn(
        TextBundle::from_section(
            "A: Start capture\nS: Stop capture\nESC: Exit app",
            TextStyle {
                font_size: 20.0,
                color: Color::BLUE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
    );
}

fn set_camera_position_from_plugin(
    mut camera: Query<&mut Transform, With<CameraEye>>,
    mut webcam_data: EventReader<WebcamFacialDataEvent>,
    _time: Res<Time>,
) {
    for event in webcam_data.read() {
        let x = event.0.center_x;
        let y = -event.0.center_y + 3.0;

        for mut transform in &mut camera {
            // For camera moving while looking at target:
            transform.translation = Transform::from_xyz(x, y, 10.0)
                .looking_at(
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3::Y,
                )
                .translation;
            transform.rotation = Transform::from_xyz(x, y, 10.0)
                .looking_at(
                    Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    Vec3::Y,
                )
                .rotation;

            /* OR */

            // For FPS Mode (there is a better way to do this using bevy_fly_camera or bevy_orbit_controls or something else, plus easing/interpolation...):
            // let pitch = y - 3.0;
            // let yaw = -x;
            // transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw.to_radians() * 10.0)
            //     * Quat::from_axis_angle(-Vec3::X, pitch.to_radians() * 10.0);
        }
    }
}

// Keyboard control for plugin stop/start and app
fn user_input_to_plugin_control_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut webcam_facial_control: ResMut<WebcamFacialController>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::A) {
        webcam_facial_control.control = true;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        webcam_facial_control.control = false;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
