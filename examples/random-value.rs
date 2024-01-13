use quantum_rand::QRNG;

fn main() {
    let mut rng = QRNG::new();
    println!("{}", rng.u32());
}
