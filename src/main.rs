#[macro_use]
extern crate clap;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use clap::App;
use rusttype::{FontCollection};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    let mut image = image::open(source).unwrap();

    let font = Vec::from(include_bytes!("ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let _ = image.save(target).unwrap();
}
