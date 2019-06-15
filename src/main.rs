#[macro_use]
extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use clap::App;
use std::iter::Iterator;
use image::{DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, FontCollection, Scale, Rect, point};

const TITLE: &str = "LGTM";
const DESCRIPTION: &str = "Looks Good To Me";

struct Size {
    first_padding: u32,
    width: u32,
    height: u32,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    let mut image = image::open(source).unwrap();

    let font = Vec::from(include_bytes!("Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let (w, h) = image.dimensions();

    let title_scale = h as f32 / 5.0;
    let title_scale = Scale {x: title_scale, y: title_scale};
    let title_size = get_text_size(&font, title_scale, TITLE);

    let title_x = (w / 2) - (title_size.width / 2) - title_size.first_padding;
    let title_y = (h * 2/3) - (title_size.height / 2);
    let padding = h / 30;
    let desc_y = title_y  + title_size.height + padding;

    let color = Rgba([0u8, 0u8, 0u8, 1u8]);
    draw_description(&mut image, color, desc_y, &font);
    draw_text_mut(&mut image, color, title_x, title_y, title_scale, &font, TITLE);

    let _ = image.save(target).unwrap();
}

fn draw_description(image: &mut DynamicImage, color: Rgba<u8>, y: u32 , font: &Font) {
    let (w, h) = image.dimensions();
    let desc_scale = h as f32 / 10.0;
    let margin = w / 20;

    let desc_scale = Scale {x: desc_scale, y: desc_scale};

    let splitted: Vec<&str> = DESCRIPTION.split(" ").collect();
    let sizes: Vec<Size> = splitted.iter()
        .map(|s| get_text_size(&font, desc_scale, s))
        .collect();

    let width: u32 = sizes.iter().map(|s| s.width).sum::<u32>()
        + ((splitted.len() - 1) as u32 * margin);
    let mut current_x = (w / 2) - (width / 2);

    for (s, size) in splitted.iter().zip(sizes.iter()) {
        current_x -= size.first_padding;
        draw_text_mut(image, color, current_x, y, desc_scale, &font, s);
        current_x += margin + size.width + size.first_padding;
    }
}

fn get_text_size(font: &Font, scale: Scale, text: &str) -> Size {
    let point = point(0.0, font.v_metrics(scale).ascent);
    let glyphs: Vec<Rect<i32>> = font.layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let first_x = glyphs.first().unwrap().min.x;
    let width = glyphs.last().unwrap().max.x - first_x;
    let height = glyphs.iter().map(|b| b.height()).max().unwrap();
    return Size {first_padding: first_x as u32, width: width as u32, height: height as u32};
}
