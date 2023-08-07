// Plugin that reads webcamera, detects face calculates frame box
// and sends coordinates to Bevy as Event.
// (Coordinates 0,0 are in the center of camera frame)

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventWriter},
        system::{Commands, Query, Res, Resource},
    },
    log::{debug, error, info},
    tasks::{AsyncComputeTaskPool, Task},
};

use crossbeam_channel::{bounded, Receiver, SendError, Sender};
use futures_lite::future;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

// camera capture
use camera_capture;
// rustface detector
use rustface::ImageData;
// image utils
use image::{ImageBuffer, Luma};
// Data filter/smoothing
mod filter;
pub use filter::SmoothingFilterType;
use filter::WebcamFacialDataFiltered;

pub struct WebcamFacialPlugin {
    pub config_webcam_device: u32,
    pub config_webcam_width: u32,
    pub config_webcam_height: u32,
    pub config_webcam_framerate: u32,
    pub config_webcam_autostart: bool,
    pub config_filter_type: SmoothingFilterType,
    pub config_filter_length: u32,
}
// Plugin configuration for webcam to be accesible from plugin system
#[derive(Resource)]
pub struct WebcamFacialController {
    pub sender: Sender<WebcamFacialData>,
    pub receiver: Receiver<WebcamFacialData>,
    pub control: bool,
    pub status: Arc<AtomicBool>,
    config_device: u32,
    config_width: u32,
    config_height: u32,
    config_framerate: u32,
    config_filter_type: SmoothingFilterType,
    config_filter_length: u32,
}

#[derive(Component)]
struct WebcamFacialTask(Task<bool>);

// WebcamFacialEvent event for sending WebcamFacialData to main Bevy app
#[derive(Event)]
pub struct WebcamFacialDataEvent(pub WebcamFacialData);

// Data structure to be exchanged with Bevy
#[derive(Default, Clone, Debug)]
pub struct WebcamFacialData {
    pub center_x: f32,
    pub center_y: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub score: f32,
}

impl Plugin for WebcamFacialPlugin {
    fn build(&self, app: &mut App) {
        // Add thread channels for data exchange
        let (task_channel_sender, task_channel_receiver) = bounded(1);
        let task_status = Arc::new(AtomicBool::new(false));
        // Store plugin control,data channels and settings in a resource
        let plugin = WebcamFacialController {
            sender: task_channel_sender,
            receiver: task_channel_receiver,
            control: self.config_webcam_autostart.clone(),
            status: task_status,

            config_device: self.config_webcam_device.clone(),
            config_width: self.config_webcam_width.clone(),
            config_height: self.config_webcam_height.clone(),
            config_framerate: self.config_webcam_framerate.clone(),
            config_filter_type: self.config_filter_type.clone(),
            config_filter_length: self.config_filter_length.clone(),
        };
        // Insert nesecary resources, events and systems
        app.insert_resource(plugin)
            .add_event::<WebcamFacialDataEvent>()
            .add_systems(Update, webcam_facial_task_runner);
    }
}

impl Default for WebcamFacialPlugin {
    fn default() -> Self {
        Self {
            config_webcam_device: 0,
            config_webcam_width: 640,
            config_webcam_height: 480,
            config_webcam_framerate: 30,
            config_webcam_autostart: true,
            config_filter_type: SmoothingFilterType::LowPass(0.1),
            config_filter_length: 15,
        }
    }
}

fn webcam_facial_task_runner(
    webcam_facial: Res<WebcamFacialController>,
    mut commands: Commands,
    mut task: Query<(Entity, &mut WebcamFacialTask)>,
    mut events: EventWriter<WebcamFacialDataEvent>,
) {
    // If enabled and not running - start task
    if webcam_facial.control & !webcam_facial.status.load(Ordering::SeqCst) {
        // Get Arc clones
        let task_running = webcam_facial.status.clone();
        let sender_clone = webcam_facial.sender.clone();

        let camera_device = webcam_facial.config_device;
        let camera_width = webcam_facial.config_width;
        let camera_height = webcam_facial.config_height;
        let camera_framerate = webcam_facial.config_framerate;
        let filter_type = webcam_facial.config_filter_type;
        let filter_length = webcam_facial.config_filter_length;

        info!("Starting webcam capture. Launching capture and recognition task.");
        let thread_pool = AsyncComputeTaskPool::get();
        // Main task loop
        let task = thread_pool.spawn(async move {
            // Initialize webcam
            let mut cam_iter = match get_camera_frame_iterator(
                camera_device,
                camera_width,
                camera_height,
                camera_framerate,
            ) {
                Some(iter) => iter,
                None => {
                    error!("Camera setup failed. Continuing without camera.");
                    return false;
                }
            };
            // Initialize face detector
            //TODO Model selection
            let mut detector =
                match rustface::create_detector(&"assets/NN_Models/seeta.bin".to_string()) {
                    Ok(detector) => {
                        info!("Using assets/NN_Models/seeta.bin recognition model.");
                        detector
                    }
                    Err(error) => {
                        error!("Failed to create detector: {}", error.to_string());
                        std::process::exit(1)
                    }
                };

            detector.set_min_face_size(20);
            detector.set_score_thresh(2.0);
            detector.set_pyramid_scale_factor(0.8);
            detector.set_slide_window_step(4, 4);

            let mut filtered_data = WebcamFacialDataFiltered::new(filter_length, filter_type);

            while task_running.load(Ordering::SeqCst) {
                // Get frame from buffer
                let rgb_frame = cam_iter.next().unwrap();
                // Convert RGB frame to grayscale
                let grayscale_image = ImageBuffer::from_fn(camera_width, camera_height, |x, y| {
                    let rgb_pixel = *rgb_frame.get_pixel(x, y);
                    let gray_value = rgb_pixel[0] as u32 * 77
                        + rgb_pixel[1] as u32 * 150
                        + rgb_pixel[2] as u32 * 29;
                    Luma([((gray_value >> 8) & 0xFF) as u8])
                });
                // Get Image data from buffer data
                let grayscale_image_data =
                    ImageData::new(&grayscale_image, camera_width, camera_height);

                // Detect face data in privided image data
                let faces = detector.detect(&grayscale_image_data);

                // Initialize zero values if face not found
                let mut facial_data = WebcamFacialData::default();

                // Get face with maximum human face probability (best candidate)
                let max_face = faces.iter().max_by_key(|p| p.score() as i32);
                match max_face {
                    Some(max_face) => {
                        debug!("Max score face: {:?}", max_face);
                        // Take face rectangle coords and score
                        facial_data.x = faces[0].bbox().x() as f32;
                        facial_data.y = faces[0].bbox().y() as f32;
                        facial_data.width = faces[0].bbox().width() as f32;
                        facial_data.height = faces[0].bbox().height() as f32;
                        facial_data.score = faces[0].score() as f32;

                        // Calculate the scale factor to map the camera resolution
                        let w_scale_factor = 100.0 / camera_width as f32;
                        let h_scale_factor = 100.0 / camera_width as f32;

                        // Calculate the coordinates and dimensions in the desired range (-50.0) to (50.0)
                        facial_data.x = facial_data.x * w_scale_factor - 50.0;
                        facial_data.y = facial_data.y * h_scale_factor - 50.0;
                        facial_data.width = facial_data.width * w_scale_factor;
                        facial_data.height = facial_data.height as f32 * h_scale_factor;
                        facial_data.center_x = (2.0 * facial_data.x + facial_data.width) / -2.0; // minus flips values so negative is left
                        facial_data.center_y = (2.0 * facial_data.y + facial_data.height) / 2.0;
                    }
                    None => {
                        debug!("No faces found. Using default zero values.");
                    }
                }
                filtered_data.push(facial_data);

                // Send processed data
                match sender_clone.send(filtered_data.get()) {
                    Ok(()) => {
                        debug!("Data from task sent.")
                    }
                    Err(SendError(data)) => {
                        error!("Failed to send task data: {:?}", data);
                    }
                }
            }
            info!("Camera stopped. Task off.");
            true
        });
        commands.spawn(WebcamFacialTask(task));
        // Set flag that we started thread
        webcam_facial.status.store(true, Ordering::SeqCst);
    }
    // If not enabled and task is running set flag to stop
    if !webcam_facial.control & webcam_facial.status.load(Ordering::SeqCst) {
        webcam_facial.status.store(false, Ordering::SeqCst);
    }
    for (entity, mut task) in &mut task {
        if let Some(_status) = future::block_on(future::poll_once(&mut task.0)) {
            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<WebcamFacialTask>();
        }
    }
    while let Ok(data) = webcam_facial.receiver.try_recv() {
        debug!("Send Bevy event {:?}", data);
        events.send(WebcamFacialDataEvent(data));
    }
}

fn get_camera_frame_iterator(
    camera_device: u32,
    camera_width: u32,
    camera_height: u32,
    camera_framerate: u32,
) -> Option<camera_capture::ImageIterator> {
    // Create the camera device
    let camera_device = match camera_capture::create(camera_device) {
        Ok(device) => {
            #[cfg(unix)]
            info!("Using '/dev/video{}' camera.", camera_device);
            #[cfg(windows)]
            info!("Using camera ID:{}.", camera_device);
            device
        }
        Err(err) => {
            error!(
                "Error creating camera device [{}]: {:?}",
                camera_device, err
            );
            return None;
        }
    };
    // Set the resolution
    let resolution_device = match camera_device.resolution(camera_width, camera_height) {
        Ok(resolution) => {
            info!(
                "Camera resolution set to {}x{}.",
                camera_width, camera_height
            );
            resolution
        }
        Err(err) => {
            error!("Error setting camera resolution: {:?}", err);
            return None;
        }
    };
    // Set the frame rate and start the camera capture
    let cam_iter = match resolution_device.fps(camera_framerate as f64) {
        Ok(fps) => {
            info!("Camera fps set to {}.", camera_framerate);
            fps.start()
        }
        Err(err) => {
            error!("Error setting camera frame rate: {:?}", err);
            return None;
        }
    };
    match cam_iter {
        Ok(iter) => Some(iter),
        Err(err) => {
            error!("Error starting camera: {:?}", err);
            None
        }
    }
}
