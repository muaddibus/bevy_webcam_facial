# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## 0.1.3 - 2023-08-05
### Added
Filters to smooth noise recognition data. 2 new fields in plugin configuration:
```rust
.add_plugins(WebcamFacialPlugin {
    config_webcam_device: "/dev/video0".to_string(),
    config_webcam_width: 640,
    config_webcam_height: 480,
    config_webcam_framerate: 33,
    config_webcam_autostart: true,
    config_filter_type: SmoothingFilterType::LowPass(0.1),
    config_filter_length: 10,
})
```
Added default-features = false to Cargo.toml

### Changed
<Event>WebcamFacialDataEvent now returns float coordinate values in range -50.0 .. 50.0, counted from center of frame
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

Updated second example (Still needs nice scene)

## 0.1.0 - 2023-07-24
### Added
