extern crate pulldown_cmark as cmark;
extern crate pdf_canvas;

use pdf_canvas::{Pdf, BuiltinFont, FontSource};
use cmark::*;
use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::borrow::Cow;

/// PAGE_SIZE is the size of a sheet of A4 paper in pt
const PAGE_SIZE: (f32, f32) = (595.0, 842.0);
const MARGIN: (f32, f32) = (20.0, 20.0);
const DEFAULT_FONT: BuiltinFont = BuiltinFont::Times_Roman;
const DEFAULT_FONT_SIZE: f32 = 12.0;
const BOLD_FONT: BuiltinFont = BuiltinFont::Times_Bold;
const ITALIC_FONT: BuiltinFont = BuiltinFont::Times_Italic;

const DEFAULT_OUTPUT_FILENAME: &str = "test.pdf";

struct Span<'txt> {
    text: Cow<'txt, str>,
    font_type: BuiltinFont,
    font_size: f32,
}

impl<'txt> Span<'txt> {
    pub fn new(text: Cow<'txt, str>, font_type: BuiltinFont, font_size: f32) -> Self {
        Self {
            text, font_type, font_size
        }
    }
}

fn main() {
    let mut doc = Pdf::create(DEFAULT_OUTPUT_FILENAME).unwrap();

    let mut markdown_file = File::open("test.md").unwrap();
    let mut markdown = String::new();
    markdown_file.read_to_string(&mut markdown).unwrap();

    let parser = Parser::new(&markdown);

    let mut lines = VecDeque::new();
    let mut x = 0.0;
    let mut current_line = vec![];
    let max_width = PAGE_SIZE.0 - MARGIN.0 - MARGIN.0;
    let mut current_font = DEFAULT_FONT;

    for event in parser {
        match event {
            Event::Start(Tag::Strong) => current_font = BOLD_FONT,
            Event::End(Tag::Strong) => current_font = DEFAULT_FONT,
            Event::Start(Tag::Emphasis) => current_font = ITALIC_FONT,
            Event::End(Tag::Emphasis) => current_font = DEFAULT_FONT,

            Event::Start(Tag::Item) => current_line.push(Span::new(" - ".into(), current_font, DEFAULT_FONT_SIZE)),
            Event::End(Tag::Item) => {
                lines.push_back(current_line);
                current_line = vec![];
            }

            Event::Text(text) => {
                let width = current_font.get_width(DEFAULT_FONT_SIZE, &text);
                if x + width > max_width {
                    lines.push_back(current_line);
                    x = 0.0;
                    current_line = vec![];
                }
                x += width;
                current_line.push(Span::new(text, current_font, DEFAULT_FONT_SIZE));
            },

            Event::End(Tag::Paragraph) => {
                lines.push_back(current_line);
                current_line = vec![];
            }

            Event::SoftBreak => current_line.push(Span::new(" ".into(), current_font, DEFAULT_FONT_SIZE)),

            _ => {}
        }
    }

    doc.render_page(PAGE_SIZE.0, PAGE_SIZE.1, |canvas| {
        let regular = canvas.get_font(DEFAULT_FONT);
        let bold = canvas.get_font(BOLD_FONT);
        let italic = canvas.get_font(ITALIC_FONT);
        canvas.text(|t| {
            t.set_font(&regular, DEFAULT_FONT_SIZE)?;
            t.set_leading(18.0)?;
            t.pos(MARGIN.0, PAGE_SIZE.1-MARGIN.1)?;

            for line in lines {
                for span in line {
                    let font = match span.font_type {
                        BuiltinFont::Times_Roman => &regular,
                        BuiltinFont::Times_Bold => &bold,
                        BuiltinFont::Times_Italic => &italic,
                        _ => &regular,
                    };
                    t.set_font(font, span.font_size)?;
                    t.show(&span.text)?;
                }
                t.show_line("")?;
            }
            Ok(())
        })
    }).unwrap();

    doc.finish().unwrap();
}
