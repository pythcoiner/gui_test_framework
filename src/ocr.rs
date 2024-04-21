use std::collections::HashMap;
use autopilot::geometry::{Point, Rect, Size};
use rusty_tesseract::{Args, image_to_boxes};
use rusty_tesseract::image::{DynamicImage, RgbaImage};
use rusty_tesseract::tesseract::input::Image as TessImage;
use crate::error::Error;
use crate::items::{Item, ItemKind};
use crate::ScreenShot;

#[derive(Debug, Clone)]
struct Char {
    text: String,
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    center: i32,
}

impl Char {
    pub fn intersect(&self, other: &Self, f: i32) -> bool {

        self.expanded(f).right >= other.expanded(f).left && other.expanded(f).right >= self.expanded(f).left
    }

    fn expanded(&self, factor: i32) -> Char {
        let expanded_width = (self.right - self.left) /2 * factor;
        let mut out = self.clone();
        out.right += expanded_width;
        out.left -= expanded_width;
        out



    }
}

#[allow(unused)]
pub struct Ocr{}

impl Ocr {
    pub fn chars_to_lines(mut chars: Vec<Char>) -> Vec<Vec<Char>> {
        // Sort chars by their vertical center position
        chars.sort_by(|a, b| a.center.partial_cmp(&b.center).unwrap());

        let mut lines: Vec<Vec<Char>> = Vec::new();
        let mut current_line: Vec<Char> = Vec::new();

        for char in chars {
            if current_line.is_empty() || Self::chars_are_on_same_line(current_line.last().unwrap(), &char) {
                current_line.push(char);
            } else {

                current_line.sort_by(|a,b|(a.left+a.right).partial_cmp(&(b.left + b.right)).unwrap());
                lines.push(current_line);
                current_line = vec![char];
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    pub fn chars_are_on_same_line(a: &Char, b: &Char) -> bool {
        // Check if the vertical center of one char is within the vertical range of the other
        (b.center <= a.top  && b.center >= a.bottom) ||
            (a.center <= b.top && a.center >= b.bottom)
    }

    pub fn line_to_items(chars: Vec<Char>, factor: i32, kind: Option<ItemKind>) -> Vec<Item> {
        let mut words: Vec<Item> = Vec::new();
        let mut current_word_chars: Vec<Char> = Vec::new();

        for char in chars {
            if current_word_chars.is_empty() || current_word_chars.last().unwrap().intersect(&char, factor) {
                current_word_chars.push(char);
            } else {
                words.push(Self::chars_to_item(current_word_chars, kind.clone()));
                current_word_chars = vec![char];
            }
        }

        if !current_word_chars.is_empty() {
            words.push(Self::chars_to_item(current_word_chars, kind));
        }

        words
    }

    pub fn chars_to_item(chars: Vec<Char>, kind: Option<ItemKind>) -> Item {
        let mut text = String::new();
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        let mut top = i32::MIN;
        let mut bottom = i32::MAX;
        let offset = 6.0f64;

        for char in chars {
            text.push_str(&char.text);
            left = left.min(char.left);
            right = right.max(char.right);
            top = top.max(char.top);
            bottom = bottom.min(char.bottom);
        }

        Item {
            text,
            position: Rect::new(
                Point::new((left as f64) - offset, (bottom as f64) - offset),
                Size::new(((right - left) as f64) + 2.0 * offset, ((top - bottom) as f64) + 2.0 * offset)
            ),
            kind,
        }
    }

    pub fn img_to_items(frame: RgbaImage, screenshot: &mut ScreenShot, kind: ItemKind) -> Result<(), Error> {

        let dynamic_image = DynamicImage::ImageRgba8(frame);

        let tess_image = TessImage::from_dynamic_image(&dynamic_image)?;

        let args = Args {
            lang: "eng".into(),
            config_variables: HashMap::from([(
                "tessedit_char_whitelist".into(),
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.0123456789".into(),
            )]),
            // 15" screen 1920x1080 = 140DPI
            dpi: Some(140),
            // Page segmentation modes (PSM):
            // 0    Orientation and script detection (OSD) only.
            // 1    Automatic page segmentation with OSD.
            // 2    Automatic page segmentation, but no OSD, or OCR.
            // 3    Fully automatic page segmentation, but no OSD. (Default)
            // 4    Assume a single column of text of variable sizes.
            // 5    Assume a single uniform block of vertically aligned text.
            // 6    Assume a single uniform block of text.
            // 7    Treat the image as a single text line.
            // 8    Treat the image as a single word.
            // 9    Treat the image as a single word in a circle.
            // 10    Treat the image as a single character.
            // 11    Sparse text. Find as much text as possible in no particular order.
            // 12    Sparse text with OSD.
            // 13    Raw line. Treat the image as a single text line, bypassing hacks that are Tesseract-specific.
            psm: Some(3),
            // OCR Engine modes: (see https://github.com/tesseract-ocr/tesseract/wiki#linux)
            // 0    Legacy engine only.
            // 1    Neural nets LSTM engine only.
            // 2    Legacy + LSTM engines.
            // 3    Default, based on what is available.
            oem: Some(3),
        };

        // OCR Recognition
        let box_output = image_to_boxes(&tess_image, &args)?;
        let mut chars: Vec<Char> = Vec::new();
        for char in box_output.boxes {
            chars.push(Char {
                text: char.symbol,
                top: char.top,
                bottom: char.bottom,
                left: char.left,
                right: char.right,
                center: (char.top + char.bottom) / 2
            });
        }

        // Separate by lines
        let lines = Ocr::chars_to_lines(chars);

        // Separate & load items
        for line in lines{
            screenshot.append_items(Ocr::line_to_items(line, 1, Some(kind.clone())));
        }

        Ok(())
    }

}

