use c3_dense_layer::{dense_layer::DenseLayer, spiral_data::spiral_data};
use ndarray::s;

fn main() {
    // Generate input data.
    let (x, _y) = spiral_data(100, 3);

    // Create a dense layer with 2 input features and 3 output values.
    let mut dense1 = DenseLayer::new(2, 3);

    // Forward pass.
    dense1.forward(&x);

    // Print the first five rows of the output.
    if let Some(output) = &dense1.output {
        println!("{}", output.slice(s![..5, ..]));
    }
}
