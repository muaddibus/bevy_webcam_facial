//use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::{Event, EventWriter},
        system::{Res, ResMut, Resource},
    },
    log::{debug, error, info},
};

use crossbeam_channel::{bounded, Receiver, SendError, Sender};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// rscam, v4l wrapper
use rscam::Camera;
use rscam::Config;
// rustface detector
use rustface::ImageData;
// image utils
use image::{DynamicImage, ImageBuffer};

// Plugin that reads webcamera, detects face calculates frame box
// and sends coordinates to Bevy as Event.
// (Coordinates 0,0 are in the center of camera frame)
pub struct WebcamFacialPlugin {
    pub config_webcam_device: String,
    pub config_webcam_width: u32,
    pub config_webcam_height: u32,
    pub config_webcam_framerate: u32,
    pub config_webcam_autostart: bool,
}
// Plugin configuration for webcam to be accesible from plugin system
#[derive(Resource)]
struct WebcamFacialPluginConfig {
    webcam_device: String,
    webcam_width: u32,
    webcam_height: u32,
    webcam_framerate: u32,
}
// Inner flag to control task starting/stopping
#[derive(Resource)]
struct WebcamFacialTaskRunning(pub Arc<AtomicBool>);

// External control for plugin
#[derive(Resource)]
pub struct WebcamFacialControl(pub bool);

// Channels for data exchange between task and plugin
#[derive(Resource)]
struct WebcamFacialStreamReceiver(Receiver<WebcamFacialData>);
#[derive(Resource)]
struct WebcamFacialStreamSender(Sender<WebcamFacialData>);

// WebcamFacialEvent event for sending WebcamFacialData to main Bevy app
#[derive(Event)]
pub struct WebcamFacialDataEvent(pub WebcamFacialData);

// Data structure to be exchanged with Bevy
#[derive(Default, Debug)]
pub struct WebcamFacialData {
    pub center_x: i32,
    pub center_y: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub score: f32,
}

impl Plugin for WebcamFacialPlugin {
    fn build(&self, app: &mut App) {
        // Store plugins settings in resource
        let plugin = WebcamFacialPluginConfig {
            webcam_device: self.config_webcam_device.clone(),
            webcam_width: self.config_webcam_width.clone(),
            webcam_height: self.config_webcam_height.clone(),
            webcam_framerate: self.config_webcam_framerate.clone(),
        };
        // Add thread channels
        let (sender, receiver) = bounded(1);

        // Insert nesecary resources, events and systems
        app.insert_resource(plugin)
            .insert_resource(WebcamFacialTaskRunning(Arc::new(AtomicBool::new(false))))
            .insert_resource(WebcamFacialControl(self.config_webcam_autostart))
            .insert_resource(WebcamFacialStreamReceiver(receiver))
            .insert_resource(WebcamFacialStreamSender(sender))
            .add_event::<WebcamFacialDataEvent>()
            .add_systems(Update, webcam_facial_task_runner)
            .add_systems(Update, webcam_facial_proxy_system);
    }
}

fn webcam_facial_task_runner(
    running: ResMut<WebcamFacialTaskRunning>,
    control: Res<WebcamFacialControl>,
    sender: Res<WebcamFacialStreamSender>,
    config: Res<WebcamFacialPluginConfig>,
) {
    // If enabled and not running - start task
    if control.0 & !running.0.load(Ordering::SeqCst) {
        // Get Arc clones
        let task_running = running.0.clone();
        let sender_clone = sender.0.clone();

        let device_path = config.webcam_device.to_string();
        let width = config.webcam_width;
        let height = config.webcam_height;
        let framerate = config.webcam_framerate;
        info!("Starting webcam capture.");
        std::thread::spawn(move || {
            // Initialize webcam
            let mut camera = Camera::new(&device_path).unwrap();
            camera
                .start(&Config {
                    interval: (1, framerate),
                    resolution: (width, height),
                    format: b"YUYV",
                    ..Default::default()
                })
                .unwrap_or_else(|_error| error!("Failed to start camera device!"));

            let mut detector =
                match rustface::create_detector(&"assets/NN_Models/seeta.bin".to_string()) {
                    Ok(detector) => detector,
                    Err(error) => {
                        println!("Failed to create detector: {}", error.to_string());
                        std::process::exit(1)
                    }
                };

            detector.set_min_face_size(20);
            detector.set_score_thresh(2.0);
            detector.set_pyramid_scale_factor(0.8);
            detector.set_slide_window_step(4, 4);

            while task_running.load(Ordering::SeqCst) {
                // Get frame from buffer
                let buf = camera.capture().expect("Failed to get frame!");
                let rgb_frame = yuyv_to_rgb(&buf, width as usize, height as usize);
                // Create a new ImageBuffer from converting Vec<u8>
                let image_buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                    ImageBuffer::from_vec(width, height, rgb_frame)
                        .expect("Failed to create ImageBuffer");
                // Convert ImageBuffer to DynamicImage
                let image: DynamicImage = DynamicImage::ImageRgb8(image_buffer);
                // Convert to grayscale image buffer
                let gray = image.to_luma8();
                // Get Image data from buffer data
                let mut grayscale_image_data = ImageData::new(&gray, width, height);
                // Detect face data
                let faces = detector.detect(&mut grayscale_image_data);

                // Initialize zero values if face not found
                let mut facial_data = WebcamFacialData::default();

                // Get face with maximum human face probability (best candidate)
                let max_face = faces.iter().max_by_key(|p| p.score() as i32);
                match max_face {
                    Some(max_face) => {
                        debug!("Max score face: {:?}", max_face);
                        // Take face rectangle coords
                        // Calculate "nose" coords relative from center of image ( image center is 0,0)
                        facial_data.x = faces[0].bbox().x() as i32;
                        facial_data.y = faces[0].bbox().y() as i32;
                        facial_data.width = faces[0].bbox().width() as i32;
                        facial_data.height = faces[0].bbox().height() as i32;
                        facial_data.score = faces[0].score() as f32;
                        // center x = (rect_w/2 + x) - (image_w/2)
                        facial_data.center_x =
                            (facial_data.width / 2 + facial_data.x) - (width / 2) as i32;
                        facial_data.center_y =
                            (facial_data.height / 2 + facial_data.y) - (height / 2) as i32;
                    }
                    None => {
                        debug!("No faces found. Using default zero values.");
                    }
                }
                // Send processed data
                match sender_clone.send(facial_data) {
                    Ok(()) => {
                        debug!("Data from thread send.")
                    }
                    Err(SendError(data)) => {
                        error!("Failed to send data: {:?}", data);
                    }
                }
            }
            info!("Camera stopped");
        });
        // Set flag that we started thread
        running.0.store(true, Ordering::SeqCst);
    }
    // If not enabled and task is running set flag to stop
    if !control.0 & running.0.load(Ordering::SeqCst) {
        running.0.store(false, Ordering::SeqCst);
    }
}

fn webcam_facial_proxy_system(
    receiver: Res<WebcamFacialStreamReceiver>,
    mut events: EventWriter<WebcamFacialDataEvent>,
) {
    while let Ok(data) = receiver.0.try_recv() {
        debug!("Send Bevy event {:?}", data);
        events.send(WebcamFacialDataEvent(data));
    }
}

// Converter from YUYV to RBG
fn yuyv_to_rgb(yuyv_frame: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut rgb_frame = vec![0u8; width * height * 3];
    for i in (0..width * height).step_by(2) {
        let y0 = yuyv_frame[i * 2] as f32;
        let u = yuyv_frame[i * 2 + 1] as f32;
        let y1 = yuyv_frame[i * 2 + 2] as f32;
        let v = yuyv_frame[i * 2 + 3] as f32;
        // Convert YUV to RGB
        let r0 = (y0 + 1.4075 * (v - 128.0)) as u8;
        let g0 = (y0 - 0.3455 * (u - 128.0) - (0.7169 * (v - 128.0))) as u8;
        let b0 = (y0 + 1.7790 * (u - 128.0)) as u8;
        let r1 = (y1 + 1.4075 * (v - 128.0)) as u8;
        let g1 = (y1 - 0.3455 * (u - 128.0) - (0.7169 * (v - 128.0))) as u8;
        let b1 = (y1 + 1.7790 * (u - 128.0)) as u8;
        // Fill the RGB frame with the converted pixel values
        let index = i * 3;
        rgb_frame[index] = r0;
        rgb_frame[index + 1] = g0;
        rgb_frame[index + 2] = b0;
        rgb_frame[index + 3] = r1;
        rgb_frame[index + 4] = g1;
        rgb_frame[index + 5] = b1;
    }
    rgb_frame
}
