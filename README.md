[![Rust](https://github.com/muaddibus/bevy_webcam_facial/actions/workflows/rust.yml/badge.svg)](https://github.com/muaddibus/bevy_webcam_facial/actions/workflows/rust.yml)

# About bevy_webcam_facial

Plugin for [Bevy](https://bevyengine.org/) game engine. Captures webcam image, finds face and provides all available data (face rectangle coordinates, face probability) to Bevy game engine via events for further use in Bevy game engine.

## Features

* Webcam capture using [camera_capture](https://github.com/oli-obk/camera_capture)
* Face position recognition using [rustface](https://github.com/atomashpolskiy/rustface)
* Realtime and lightweight [SeetaFace Detection model](https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection/)
* Runs in separate Bevy AsyncTaskpool task without blocking
* 2 data smoothing/denoising filters

## Plans
- [ ] MacOSX webcam support
- [ ] Several AI face recognition models to choose by default (simple frame, with face features like eyes/nose/mouth, full face mesh recognition, emotion detection...)

## Supported Platforms

- [x] Linux
- [ ] MacOSX
- [x] Windows

## Available for use in Bevy:

### Plugin config

Several parameters when including in `.add_plugins` or use default camera and settings `.add_plugins(WebcamFacialPlugin::default())`:
```rust
.add_plugins(WebcamFacialPlugin {
    config_webcam_device: 0,
    config_webcam_width: 640,
    config_webcam_height: 480,
    config_webcam_framerate: 15,
    config_webcam_autostart: true,
    config_filter_type: SmoothingFilterType::LowPass(0.1),
    config_filter_length: 10,
})
```
Parameters: 
* Webcamera device number (0-first default) ex.0,1,2...
    * Linux: Number get appended to `/dev/video{number}`
    * Windows: Device number
* Width of frame: 640
* Width of frame: 480
* Frames per second: 15
* Start capturing and sending events instantly after plugin activation: true/false (can be enabled/disabled anytime at runtime via `ResMut<WebcamFacialController>`)
* Smoothing filter for coordinates (currently: MeanMedian, LowPass(f32), NoFilter)
* From how many frames take data for smoothing 5-10 optimal (more frames - less noisy data, but slower response)

### Resources:
Enable/disable webcam capture and recognition from Bevy via mutable resource `ResMut<WebcamFacialController>`
```rust
pub struct WebcamFacialController {
...
    pub control: bool,
...
}
```
### Event with captured data
```rust
<Event>WebcamFacialDataEvent
```
### Data struct returned via Event
```rust
pub struct WebcamFacialData {
    pub center_x: f32,
    pub center_y: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub score: f32,
}
```
Coordinates are mapped as floating point number in range of -50.0 .. 50.0, camera resolution doesn't matter
* [center_x) Face center point x coordinate
* (center_y) Face center point y coordinate
* (x) Face rectangle frame x coordinate
* (y) Face rectangle frame y coordinate
* (width) Face rectangle frame width
* (height) Face rectangle frame height
* (score) Probability of a detected object being a true face 0-30..


## Some ideas and use cases of data comming from plugin:
* Controlling game object transformations (transform, rotate, scale)
* Object control (car driving, player movement...)
* Background scene movement in 2D games or background scene movement in 3D top/side view games for better depth perception or 'looking around'
* Camera FPS like movement
* Rotation around scenes, player or other objects
* Zooming in scenes (map zoom, scene zoom, sniper zoom...)
* Scaring horror games to pop beasts on detected face closeup
* Your imagination...

*Note: Use some interpolation for transforms for smoother transforms like "bevy_easings" or "bevy_mod_interp"


## Examples
Three examples are provided in [examples] folder:
(under construction)
- [x] [object_mover](examples/object_mover.rs) - simplest example to move object using raw unfiltered/noisy data
- [x] [camera_control](examples/camera_control.rs) - control bevy camera view using filtered data
- [ ] [neck_trainer](examples/neck_trainer.rs) - train you neck :) most complex example with filtered data + bone animation and skin

Unchecked - not finished

## Versions

| bevy | bevy_webcam_facial  |
|  ---:|                 ---:|
| 0.11 | 0.1.4               |
| 0.11 | 0.1.3               |
| 0.11 | 0.1.2               |
| 0.11 | 0.1.1               |

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

## Reference Material

The following were used for coding of plugin:

* [Bevy The Book](https://bevyengine.org/learn/book/)
* [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/0)
* [Bevy Third Party Plugin Guidelines](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md)
* [rustface](https://github.com/atomashpolskiy/rustface)

Additional interesting sources for future research:

* [Tensorflow rust](https://github.com/tensorflow/rust/) 
* [Face Detection with Tensorflow Rust](https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/) (Was succesfully tested earlier)
* [Tensorflow model implementing the mtcnn face detector](https://github.com/blaueck/tf-mtcnn/)
* https://github.com/cetra3/mtcnn/
* [tfjs-model web demo of face mesh](https://storage.googleapis.com/tfjs-models/demos/face-landmarks-detection/index.html?model=mediapipe_face_mesh)
* [Awesome-Rust-MachineLearning Docs/Projects](https://github.com/vaaaaanquish/Awesome-Rust-MachineLearning/)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
