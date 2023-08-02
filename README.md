# About bevy_webcam_facial

Plugin for rust [Bevy](https://bevyengine.org/) game engine. Captures webcam image, finds face and provides all available data (face rectangle coordinates, its width and height, face probability) to Bevy game engine via events for further use in Bevy game engine.

## Features

* Webcam capture using [rscam] (https://github.com/loyd/rscam/) via Linux V4L backend
* Face position recognition using [rustface](https://github.com/atomashpolskiy/rustface)
* Realtime and lightweight [SeetaFace Detection model](https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection/)
* Runs in separate task and doesnt block

## Supported Platforms

- [x] Linux via v4l2 [rscam]
- [ ] MacOSX
- [ ] Windows


## Available for use in Bevy:

### Plugin config

Needs several parameters when including in `.add_plugins`:
```rust
app.add_plugins(WebcamFacialPlugin {
    config_webcam_device: String,
    config_webcam_width: u32,
    config_webcam_height: u32,
    config_webcam_framerate: u32,
    config_webcam_autostart: bool,
});
```
Parameters: 
* Path to webcamera device ex."/dev/video0"
* Width of frame: 640
* Width of frame: 480
* Frames per second: 33
* Start capturing instantly after plugin activation: true/false # can be enabled anytime at runtime via <Res>WebcamFacialControl

### Resources:
Enable/disable webcam capture and recognition from Bevy via mutable `Resource`
```rust
<ResMut>WebcamFacialControl bool
```
### Event on data arival
```rust
<Event>WebcamFacialDataEvent
```
### Data struct returned in Event
```rust
WebcamFacialData {
    center_x: i32,
    center_y: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    score: f32,
}
```
* [center_x) Face center point x coordinate
* (center_y) Face center point y coordinate
* (x) Face rectangle frame x coordinate
* (y) Face rectangle frame y coordinate
* (width) Face rectangle frame width
* (height) Face rectangle frame height
* (score) Probability of a detected object being a true face 0-30..
  
## Usage in Examples:

Three examples are provided in [examples] folder:

* [object_mover](examples/object_mover.rs) - move object using raw unfiltered/noisy data
* [camera_control](examples/camera_control.rs) - control bevy camera view using averaged data from camera
* [neck_trainer](examples/neck_trainer.rs) - train you neck :) most complex example with averaged data + bone animation

## Versions


| bevy | bevy_webcam_facial  |
|  ---:|                 ---:|
| 0.11 | 0.1.0               |


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