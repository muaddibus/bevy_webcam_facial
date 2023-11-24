use bevy::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use bevy_webcam_facial::*;

fn main() {
    App::new()
        .init_resource::<Average>()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(WebcamFacialPlugin {
            config_webcam_device: 0,
            config_webcam_width: 640,
            config_webcam_height: 480,
            config_webcam_framerate: 33,
            config_webcam_autostart: true,
            config_filter_length: 15,
            config_filter_type: SmoothingFilterType::LowPass(0.1),
        })
        .add_plugins(HookPlugin)
        .add_systems(Startup, load_scene)
        .add_systems(Update, (keyboard_animation_control, bone_move))
        .run();
}

#[derive(Resource, Default)]
struct Average {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct HeadBone;

/// set up 3D scene
fn load_scene(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load("rooster.gltf#Scene0"),
            ..default()
        },
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(Name::as_str) {
                Some("Headas") => cmds.insert(HeadBone),
                _ => cmds,
            };
        }),
    });
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    let Ok(mut player) = animation_player.get_single_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::Space) {
        if player.is_paused() {
            player.resume();
        } else {
            player.pause();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Up) {}
}
fn bone_move(
    mut head: Query<&mut Transform, With<HeadBone>>,
    mut average: ResMut<Average>,
    mut reader: EventReader<WebcamFacialDataEvent>,
) {
    for event in reader.read() {
        let x = event.0.center_x;
        let y = event.0.center_y;
        average.x = (x + average.x) / 2.0;
        average.y = (y + average.y) / 2.0;
        for mut head in &mut head {
            head.translation += 0.005;
        }
    }
}
