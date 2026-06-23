use ndarray::{Array1, Array2};
use ndarray_rand::rand::{Rng, rng};
use ndarray_rand::rand_distr::StandardNormal;

pub fn spiral_data(samples: usize, classes: usize) -> (Array2<f64>, Array1<usize>) {
    let total_samples = samples * classes;
    let mut x = Array2::<f64>::zeros((total_samples, 2));
    let mut y = Array1::<usize>::zeros(total_samples);
    let mut rng = rng();

    for class_number in 0..classes {
        for sample_number in 0..samples {
            let index = class_number * samples + sample_number;

            let progress = sample_number as f64 / (samples - 1) as f64;
            let radius = progress;

            // Sample from N(0, 1), then scale to N(0, 0.2²).
            let noise: f64 = rng.sample(StandardNormal);
            let angle = class_number as f64 * 4.0 + progress * 4.0 + noise * 0.2;

            x[[index, 0]] = radius * (angle * 2.5).sin();
            x[[index, 1]] = radius * (angle * 2.5).cos();
            y[index] = class_number;
        }
    }

    (x, y)
}
