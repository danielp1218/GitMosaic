use image::{DynamicImage, GenericImageView, ImageBuffer, open, Luma};
use colored::{ColoredString, Colorize};

pub fn valid_image_path(path: &str) -> bool {
    match open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn process_image(image_path: &str) -> Vec<Vec<u8>> {
    // Placeholder for image processing logic
    let img: DynamicImage = open(image_path)
        .expect("Failed to open image");

    const GIT_GRAPH_WIDTH: u32 = 52;
    const GIT_GRAPH_HEIGHT: u32 = 7;
    const LEVELS: u8 = 5;


    let img = img.grayscale();
    let new_image = image::DynamicImage::resize(&img, GIT_GRAPH_WIDTH, GIT_GRAPH_HEIGHT, image::imageops::FilterType::Lanczos3);
    let mut quantized = vec![vec![0u8; new_image.width() as usize]; new_image.height() as usize];

    for (x, y, pixel) in new_image.to_luma8().enumerate_pixels() {
        let idx = pixel_to_activity(*pixel, LEVELS);
        quantized[y as usize][x as usize] = idx;
    }

    println!("Size {} x {}", quantized[0].len(), quantized.len());

    print_git_preview(&quantized);
    quantized
}

fn print_git_preview(image: &Vec<Vec<u8>>) {
    let (width, height) = (image[0].len(), image.len());
    for y in (0..height).step_by(2) {
        for x in 0..width {
            let pixel_top = image[y][x];
            let pixel_bot = image.get(y + 1).and_then(|row| row.get(x));
            let coloured_pixel = pixel_to_colour(pixel_top, pixel_bot);
            print!("{}", coloured_pixel);
        }
        println!();
    }
}

fn pixel_to_colour(pixel_top: u8, pixel_bot: Option<&u8>) -> ColoredString {
    let ret = match pixel_top {
        0 => "▄".on_truecolor(21, 27, 35),
        1 => "▄".on_truecolor(3,58,22),
        2 => "▄".on_truecolor(25,108,46),
        3 => "▄".on_truecolor(46,160,67),
        4 => "▄".on_truecolor(86, 211, 100),
        _ => "▄".on_truecolor(255, 0, 0),
    };

    if let Some(pixel_bot) = pixel_bot {
        return match *pixel_bot {
            0 => ret.truecolor(21, 27, 35),
            1 => ret.truecolor(3,58,22),
            2 => ret.truecolor(25,108,46),
            3 => ret.truecolor(46,160,67),
            4 => ret.truecolor(86, 211, 100),
            _ => ret.truecolor(255, 0, 0),
        }
    }
    return ret.truecolor(0, 0, 0);
}

// linear quantization, map 0-255 to 0-(levels-1)
// may want to change to something else later
fn pixel_to_activity(pixel: image::Luma<u8>, levels: u8) -> u8 {
    return (pixel[0] as f32 / 256.0 * levels as f32).floor() as u8;
}
