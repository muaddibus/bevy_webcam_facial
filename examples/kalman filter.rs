use kalman_rs::{KalmanFilter, MeasurementModel, TransitionModel};
use nalgebra::{Matrix2, Vector2};

fn main() {
    let transition_model = TransitionModel::new(Matrix2::new(1.0, 0.0, 0.0, 1.0)); // Assuming a constant velocity model in 2D (x and y are independent)
    let measurement_model = MeasurementModel::new(Matrix2::identity()); // Assuming we can directly measure position

    // Adjust the process noise covariance matrix to increase the uncertainty in the transition model
    let process_noise_covariance = Matrix2::new(0.01, 0.0, 0.0, 0.01);
    let mut kalman_filter = KalmanFilter::new_with_process_noise(
        &transition_model,
        &measurement_model,
        &process_noise_covariance,
    );

    // Simulate receiving noisy data every 10ms
    loop {
        let noisy_coordinate = get_noisy_coordinate(); // Replace this with your real data input

        // Perform Kalman filter prediction step (assuming constant velocity model)
        kalman_filter.predict();

        // Adjust the measurement noise covariance matrix to increase the influence of noisy measurements
        let measurement_noise_covariance = Matrix2::new(0.01, 0.0, 0.0, 0.01);
        kalman_filter.update_with_measurement_noise(
            &Vector2::new(noisy_coordinate.x, noisy_coordinate.y),
            &measurement_noise_covariance,
        );

        // Get the smoothed estimate from the Kalman filter
        let smoothed_state = kalman_filter.state_estimate();

        // Use the smoothed_state as needed (e.g., visualization or further processing)
        let smoothed_coordinate = Coordinate::new(smoothed_state[0], smoothed_state[1]);
    }
}
