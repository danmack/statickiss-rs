//! generate bitmap of a stream of random numbers from an
//! implementation of the KISS() pseudo random number generator.
//!
//! demonstrations how to use static/global variables in rust, bitwise
//! operations, and the image crate.

extern crate image;

use std::fs::File;

/// Multiply with carry (MWC) component
pub static mut X: usize = 1234567890987654321;

/// XOR Shift (XSH) - left 13, right 17, left 43
pub static mut Y: usize = 362436362436362436;

/// Congruential number generator (CNG) seed
pub static mut Z: usize = 1066149217761810;
pub static mut C: usize = 123456123456123456;

/// width of the png image
pub const WIDTH:  u32 = 700;

/// height of the png image
pub const HEIGHT: u32 = 700;

fn main() {
    // fill the imagebuffer with a blank slate, except 0,0
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let pixel = img[(0,0)];
    for x in 0 .. WIDTH {
        for y in 0 .. HEIGHT {
            if kiss() % 2 == 0 {
                img.put_pixel(x, y, pixel);
            }
        }
    }

    let ref mut fout = File::create("static.png").unwrap();
    let _ = image::ImageLuma8(img).save(fout, image::PNG);
}

/// my attempt at implementing a 64-bit version of the KISS pseudo
/// random number generator from Marsaglia and Zaman, Florida State U
/// "Even a bad RNG is pretty good"
/// For some strange reason, there isn't a good article for this on
/// wikipedia in english - but I used this reference from the German
/// version of wikipedia:
/// [KISS Algorithm](https://de.wikipedia.org/wiki/KISS_(Zufallszahlengenerator)
pub fn kiss() -> usize {
    unsafe {
        Z = 6906969069 * Z + 1234567;
        Y ^= Y<<13;
        Y ^= Y>>17;
        Y ^= Y<<43;

        let t: usize = (X<<58)+C;
        C =  X>>6;
        X += t;
        C += X<<t;
        X + Y + Z
    }
}
