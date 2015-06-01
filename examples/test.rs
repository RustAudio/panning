
extern crate panning;

fn main() {

    // Print the output of 20 equally spaced stereo pans from hard left to hard right.
    for pan in (0i32..21).map(|i| (i - 10) as f32 / 10.0) {
        println!("pan: {:?} -> {:?}", pan, panning::stereo::pan(pan));
    }

}

