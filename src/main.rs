#[allow(clippy::float_cmp)]
mod vec3;
use image::{ImageBuffer, RgbImage};
use imageproc::drawing;
use indicatif::ProgressBar;
use rand::Rng;

pub use vec3::Vec3;

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(1024, 1024);
    let bar = ProgressBar::new(1024);

    for x in 0..1024 {
        for y in 0..1024 {
            let pixel = img.get_pixel_mut(x, y);
            let r = 255 as u8;
            let g = 255 as u8;
            let b = 241 as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        bar.inc(1);
    }
    let mut a: i32;
    let mut b: i32;
    for _x in 0..100 {
        let mut rng1 = rand::thread_rng();
        a = rng1.gen();
        a %= 200;
        if a % 2 == 0 {
            a = -a;
        }
        let mut rng2 = rand::thread_rng();
        b = rng2.gen();
        b %= 200;
        if b % 2 == 0 {
            b = -b;
        }
        drawing::draw_line_segment_mut(
            &mut img,
            (512_f32, 512_f32),
            ((512 + a) as f32, (512 + b) as f32),
            image::Rgb([255, 0, 0]),
        )
    }
    //drawing::draw_line_segment_mut(&mut img, (0_f32, 0_f32), (1000_f32, 1000_f32), image::Rgb([255, 0, 0]));
    img.save("output/test.png").unwrap();
    bar.finish();
}
