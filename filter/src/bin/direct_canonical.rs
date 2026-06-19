use filter::transfer_function::TransferFunction;

fn main() {
    let numerator = filter::numerator::Numerator::new([0.1, 0.2]);
    let denominator = filter::denominator::Denominator::new([1.0, -0.5, 2.0]);

    let tf = TransferFunction::new(numerator, denominator);
    let tf_string = tf.to_beauty_string();
    println!("Transfer Function:\n{}", tf_string);

    let mut direct_canonical = tf.build_direct_canonical();

    let x_samples = [
        0.0, 0.12, 0.31, 0.48, 0.59, 0.63, 0.55, 0.39, 0.18, -0.05, -0.28, -0.46, -0.57, -0.61,
        -0.52, -0.34, -0.10, 0.14, 0.36, 0.51,
    ];

    for x in x_samples.iter() {
        let y = direct_canonical.process(*x);
        println!("Input: {}, Output: {}", x, y);
    }
}
