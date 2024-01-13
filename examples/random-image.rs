use image::{ImageBuffer, Luma};
use quantum_rand::QRNG;

fn main() {
    let mut rng = QRNG::new();
    ImageBuffer::from_fn(512, 512, |_, _| Luma([rng.u8()]))
        .save("random.png")
        .unwrap();
}
