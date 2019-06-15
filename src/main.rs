#[macro_use]
extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use clap::App;
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    let mut image = image::open(source).unwrap();

    let font = Vec::from(include_bytes!("ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let scale = Scale {x: 100.0, y: 100.0};
    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8, 1u8]), 0, 0, scale, &font, "");

    let _ = image.save(target).unwrap();
}
