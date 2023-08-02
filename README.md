# About bevy_webcam_facial

Plugin for rust Bevy game engine. Captures webcam image, finds face and provides all available data (face rectangle coordinates, its width and height, face probability) to Bevy game engine via events for further use in Bevy game engine.

## Features

* Webcam capture using [rscam] (https://github.com/loyd/rscam/) via Linux V4L backend
* Face position recognition using [rustface](https://github.com/atomashpolskiy/) with realtime and lightweight [SeetaFace Detection model] (https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection/)
* Runs in separate task and doesnt block

# Available for use in Bevy

## Plugin config
```rust
WebcamFacialPlugin {
    config_webcam_device: String      ( Path to webcamera device ex."/dev/video0" )
    config_webcam_width: u32          ( Width of frame: 640 )
    config_webcam_height: u32         ( Width of frame: 480 )
    config_webcam_framerate: u32      ( Frames per second: 33 )
    config_webcam_autostart: bool     ( Start capturing instantly after plugin activation: true/false ) ( If false can be enabled anytime at runtime via <Res>WebcamFacialControl )
  }
```

## Resources:

<Res>WebcamFacialControl Boolean - Enable/disable webcam capture and recognition

## Event on data arival

WebcamFacialDataEvent

## Data struct returned in Event

WebcamFacialData {
    * center_x: i32                     ( Face center point x coordinate )
    * center_y: i32                     ( Face center point y coordinate )
    * x: i32                            ( Face rectangle frame x coordinate )
    * y: i32                            ( Face rectangle frame y coordinate )
    * width: i32                        ( Face rectangle frame width )
    * height: i32                       ( Face rectangle frame height )
    * score: f32                        ( Probability of a detected object being a true face 0-30..)
  }

# Usage in Examples:

Three examples are provided in [examples] folder:

* object_mover - move object using raw unfiltered/noisy data
* camera_control - control bevy camera view using averaged data from camera + interpolation
* neck_trainer - train you neck :) most complex example with averaged data + interpolation + bone animation

# Versions

+------+---------------------+
| bevy | bevy_webcam_facial  |
|------+---------------------|
| 0.11 | 0.1.0               |
+------+---------------------+

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

# Reference Material

The following were used for coding of plugin:

* [Bevy The Book] https://bevyengine.org/learn/book/
* [Unofficial Bevy Cheat Book] https://bevy-cheatbook.github.io/
* [Bevy Third Party Plugin Guidelines] https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md
* [rustface] https://github.com/atomashpolskiy/rustface

Additional interesting sources about [tensorflow] engine, which was succesfully tested earlier. Bloats dependencies, but have lots of potential for full face data recognition with different models, animated face mash creation:

* [Tensorflow] https://github.com/tensorflow/rust/
* https://cetra3.github.io/blog/face-detection-with-tensorflow-rust/ [Face Detection with Tensorflow Rust]
* https://github.com/blaueck/tf-mtcnn/
* https://github.com/cetra3/mtcnn/
* https://storage.googleapis.com/tfjs-models/demos/face-landmarks-detection/index.html?model=mediapipe_face_mesh
* https://vaaaaanquish.github.io/Awesome-Rust-MachineLearning/#image-processing

# License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
