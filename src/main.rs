use cooler_couleur::{image::color, image::DataRange, inferno, magma, plasma, viridis};
use image;

fn color_greyscale(img: &mut image::RgbImage, map: &[[u8; 3]]) {
    let w = img.width();
    let h = img.height();
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel_mut(x, y);
            let v = p[0] as usize;
            let i = (v * (map.len() - 1)) / 255;
            assert!(i < map.len());
            *p = image::Rgb(map[i]);
        }
    }
}

fn main() {
    let mut img = image::io::Reader::open("./example.png")
        .expect("loading image")
        .decode()
        .expect("decoding image")
        .into_rgb8();

    // Grayscale to get a heat map.
    let w = img.width();
    let h = img.height();
    let mut vec = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel_mut(x, y);
            let mean = 0.299 * (p[0] as f64) + 0.587 * (p[1] as f64) + 0.114 * (p[2] as f64);
            let mean = mean as u8;
            vec.push(mean);
            *p = image::Rgb([mean, mean, mean]);
        }
    }

    color_greyscale(&mut img, &inferno::COLOR_MAP);
    img.save("/tmp/inferno.png").expect("saving inferno from greyscale image");

    let img = color(w, h, &vec, &inferno::COLOR_MAP, DataRange::MinMax(0, 255));
    img.save("/tmp/inferno2.png").expect("saving inferno from bytes image");

    let img = color(w, h, &vec, &magma::COLOR_MAP, DataRange::MinMax(0, 255));
    img.save("/tmp/magma.png").expect("saving magma");

    let img = color(w, h, &vec, &plasma::COLOR_MAP, DataRange::Automatic);
    img.save("/tmp/plasma.png").expect("saving plasma");

    let img = color(w, h, &vec, &viridis::COLOR_MAP, DataRange::Automatic);
    img.save("/tmp/viridis.png").expect("saving viridis");
}
