use ndarray::{Array1, Array2};
use rand::RngExt;

/// Generates a spiral classification dataset.
///
/// Returns:
///  - x: shape `(samples * classes, 2)`
///  - y: shape `(samples * classes,)`
pub fn spiral_data(samples: usize, classes: usize) -> (Array2<f64>, Array1<usize>) {
    //
    let total_samples = samples * classes;
    let mut x = Array2::zeros((total_samples, 2));
    let mut y = Array1::zeros(total_samples);
    let mut rng = rand::rng();

    for class_number in 0..classes {
        for sample_number in 0..samples {
            let index = class_number * samples + sample_number;

            let radius = sample_number as f64 / (samples - 1) as f64;

            let angle_start = class_number as f64 * 4.0;
            let angle_end = (class_number + 1) as f64 * 4.0;
            let progress = sample_number as f64 / (samples - 1) as f64;

            let noise: f64 = rng.random_range(-0.2..0.2);
            let angle = angle_start + (angle_end - angle_start) * progress + noise;

            x[[index, 0]] = radius * (angle * 2.5).sin();
            x[[index, 1]] = radius * (angle * 2.5).cos();
            y[index] = class_number;
        }
    }

    (x, y)
}
