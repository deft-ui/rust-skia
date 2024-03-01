use crate::DrawingDriver;
use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Canvas, FontMgr, Paint, Point,
};
use std::path;

pub fn draw(driver: &mut impl DrawingDriver, path: &path::Path) {
    let path = path.join("SkParagraph-Example");
    driver.draw_image_256(&path, "lorem-ipsum", draw_lorem_ipsum);
    driver.draw_image_256(&path, "lorem-ipsum-ubuntu", draw_lorem_ipsum_ubuntu_font);
}

fn draw_lorem_ipsum(canvas: &Canvas) {
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let paragraph_style = ParagraphStyle::new();
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&Paint::default());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(LOREM_IPSUM);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(256.0);
    paragraph.paint(canvas, Point::default());
}

/// Draws a paragraph with a custom font.
fn draw_lorem_ipsum_ubuntu_font(canvas: &Canvas) {
    let font_mgr = FontMgr::empty();
    let typeface = font_mgr
        .new_from_data(UBUNTU_REGULAR, None)
        .expect("Failed to load Ubuntu font");
    println!("{}", font_mgr.count_families());

    {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(font_mgr, None);
        let paragraph_style = ParagraphStyle::new();
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
        let mut ts = TextStyle::new();
        ts.set_foreground_paint(&Paint::default());
        ts.set_typeface(typeface);
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(LOREM_IPSUM);
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(256.0);
        paragraph.paint(canvas, Point::default());
    }
}

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur at leo at nulla tincidunt placerat. Proin eget purus augue. Quisque et est ullamcorper, pellentesque felis nec, pulvinar massa. Aliquam imperdiet, nulla ut dictum euismod, purus dui pulvinar risus, eu suscipit elit neque ac est. Nullam eleifend justo quis placerat ultricies. Vestibulum ut elementum velit. Praesent et dolor sit amet purus bibendum mattis. Aliquam erat volutpat.";

static UBUNTU_REGULAR: &[u8] = include_bytes!("fonts/Ubuntu-Regular.ttf");
