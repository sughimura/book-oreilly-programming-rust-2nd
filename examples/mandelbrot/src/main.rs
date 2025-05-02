use num::Complex;

fn complex_square_add_loop(mut c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn main() {
    println!("Hello, world!");
}
