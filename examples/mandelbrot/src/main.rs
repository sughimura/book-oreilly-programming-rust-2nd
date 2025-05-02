fn square_loop(mut c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

fn main() {
    println!("Hello, world!");
}
