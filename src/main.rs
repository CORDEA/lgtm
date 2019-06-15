#[macro_use]
extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use clap::App;
use std::iter::Iterator;
use image::{GenericImageView, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, FontCollection, Scale, Rect, point};

struct Size {
    width: i32,
    height: i32,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    let mut image = image::open(source).unwrap();

    let font = Vec::from(include_bytes!("Roboto-Black.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let text = "";

    let (w, h) = image.dimensions();
    let scale = h as f32 / 3.0;
    let scale = Scale {x: scale, y: scale};

    let Size {width, height} = get_text_size(&font, scale, text);
    let x = (w / 2) - (width as u32 / 2);
    let y = (h / 2) - (height as u32 / 2);

    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8, 1u8]), x, y, scale, &font, text);
    let _ = image.save(target).unwrap();
}

fn get_text_size(font: &Font, scale: Scale, text: &str) -> Size {
    let point = point(0.0, font.v_metrics(scale).ascent);
    let glyphs: Vec<Rect<i32>> = font.layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let width = glyphs.last().unwrap().max.x - glyphs.first().unwrap().min.x;
    let height = glyphs.iter().map(|b| b.height()).max().unwrap();
    return Size {width: width, height: height};
}
