## 0.1.4 - 2023-08-07
### Added
Default settings for plugin:
```rust
            config_webcam_device: 0,
            config_webcam_width: 640,
            config_webcam_height: 480,
            config_webcam_framerate: 15,
            config_webcam_autostart: true,
            config_filter_type: SmoothingFilterType::LowPass(0.1),
            config_filter_length: 10,
```
Added more informative exception handling
### Changed
Plugin now uses [camera_capture] instead of [rscam]. Should bring Windows support.
config_webcam_device is now u32 type. 0 - default camera, or device number (Linux: number is added to "/dev/video{}", Windows: device number)
### Removed
Converter from yuyv not needed anymore

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
