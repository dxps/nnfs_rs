# Dense Layer

A dense layer is a fully connected layer.

<br/>

## Run

Using the standard `cargo run`, the output looks like this:

```shell
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/c3_dense_layer`
[[0, 0, 0],
 [-0.000058161553820588654, 0.00011050753358180105, 0.00008534778086559951],
 [-0.0000035020212034089315, 0.00002865059528695317, 0.00007419656008871333],
 [-0.00011971950285662241, 0.00023947538277420985, 0.0002133753074272142],
 [-0.0003353074932216885, 0.0006098463378387753, 0.0004065164152829383]]
❯
```

The first row is exactly `[0, 0, 0]` because the first spiral sample has radius `0`,<br/>
so its input is `[0, 0]` and thus `[0, 0] * weights + [0, 0, 0] = [0, 0, 0]`.

The remaining values are small because the weights are initialized from a normal distribution and scaled by `0.01`.
