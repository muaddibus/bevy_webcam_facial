[![Rust](https://github.com/muaddibus/bevy_webcam_facial/actions/workflows/rust.yml/badge.svg)](https://github.com/muaddibus/bevy_webcam_facial/actions/workflows/rust.yml)

# About bevy_webcam_facial

Plugin for [Bevy](https://bevyengine.org/) game engine. Captures webcam image, finds face and provides all available data (face rectangle coordinates, face probability) to Bevy game engine via events for further use in Bevy game engine.

## Features

* Webcam capture using [rscam](https://github.com/loyd/rscam/) via Linux V4L backend
* Face position recognition using [rustface](https://github.com/atomashpolskiy/rustface)
* Realtime and lightweight [SeetaFace Detection model](https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection/)
* Runs in separate Bevy AsyncTaskpool task without blocking

## Plans
- [ ] Windows / MacOSX webcam support
- [ ] Several AI face recognition models to choose by default (simple frame, with face features like eyes/nose/mouth, full face mesh recognition, emotion detection...)

## Supported Platforms

- [x] Linux via v4l2 [rscam]
- [ ] MacOSX
- [ ] Windows

## Available for use in Bevy:

### Plugin config

Needs several parameters when including in `.add_plugins`:
```rust
.add_plugins(WebcamFacialPlugin {
    config_webcam_device: "/dev/video0".to_string(),
    config_webcam_width: 640,
    config_webcam_height: 480,
    config_webcam_framerate: 33,
    config_webcam_autostart: true,
})
```
Parameters: 
* Path to webcamera device ex."/dev/video0"
* Width of frame: 640
* Width of frame: 480
* Frames per second: 33
* Start capturing and sending events instantly after plugin activation: true/false (can be enabled/disabled anytime at runtime via `ResMut<WebcamFacialController>`)

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
    pub center_x: i32,
    pub center_y: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub score: f32,
}
```
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
* Background movement in 2D games or camera movement in 3D games for better depth perception or 'looking around'
* Camera FPS like movement (bit sceptic about that, maybe after implementing other tensorflow model)
* Rotation around scenes, player or other objects
* Zooming in scenes (map zoom, scene zoom, sniper zoom...)
* Scaring horror games to pop beasts on detected face closeup
* Your imagination...

## Examples
Three examples are provided in [examples] folder:
(under construction)
- [x] [object_mover](examples/object_mover.rs) - simplest example to move object using raw unfiltered/noisy data
- [ ] [camera_control](examples/camera_control.rs) - control bevy camera view using filtered data
- [ ] [neck_trainer](examples/neck_trainer.rs) - train you neck :) most complex example with filtered data + bone animation and skin

Unchecked - not finished

## Versions

| bevy | bevy_webcam_facial  |
|  ---:|                 ---:|
| 0.11 | 0.1.2               |


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
