use ndarray::{Array2, Axis};
use ndarray_rand::{RandomExt, rand_distr::StandardNormal};

pub struct DenseLayer {
    weights: Array2<f64>,
    biases: Array2<f64>,
    pub output: Option<Array2<f64>>,
}

impl DenseLayer {
    /// Creates a dense layer with normally distributed weights
    /// and zero-initialized biases.
    pub fn new(input_size: usize, neurons_size: usize) -> Self {
        let weights = Array2::random((input_size, neurons_size), StandardNormal) * 0.01;
        let biases = Array2::zeros((1, neurons_size));
        Self {
            weights,
            biases,
            output: None,
        }
    }

    /// Calculates: output = inputs x weights + biases
    pub fn forward(&mut self, inputs: &Array2<f64>) {
        let mut output = inputs.dot(&self.weights);
        // Add the bias row to every row of the output.
        for mut row in output.axis_iter_mut(Axis(0)) {
            row += &self.biases.row(0);
        }
        self.output = Some(output);
    }
}
