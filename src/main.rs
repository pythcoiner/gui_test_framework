use std::collections::HashMap;
use autopilot::bitmap::Bitmap;
use autopilot::geometry::{Point, Rect, Size};
use std::process::{Command};
use std::str;
use rusty_tesseract::image::{DynamicImage, Rgba, RgbaImage};
use rusty_tesseract::tesseract::input::Image as TessImage;
use rusty_tesseract::tesseract::output_boxes::{image_to_boxes,};
use rusty_tesseract::{Args, TessError};

use image::Rgba as ImageRgba;
use rusty_tesseract::image::Rgba as TessRgba; 

fn convert_rgba(image_rgba: ImageRgba<u8>) -> TessRgba<u8> {
    TessRgba([image_rgba.0[0], image_rgba.0[1], image_rgba.0[2], image_rgba.0[3]])
}

#[derive(Debug)]
pub enum Error {
    FailFindWindow(String),
    FailFetchPosition,
    FailCapture(String),
    Tesseract(TessError),
}

impl From<TessError> for Error {
    fn from(err: TessError) -> Self {
        Error::Tesseract(err)
    }
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Button,
    InputField,
}

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
#[derive(Debug, Clone)]
struct Item {
    text: String,
    position: Rect,
    kind: Option<ItemKind>,
}

#[derive(Debug)]
struct ScreenShot {
    pub frame: Bitmap,
    pub image: Option<RgbaImage>,
    pub position: Rect,
    pub items: Vec<Item>
}
impl ScreenShot {
    fn get_window_position(output: &str) -> Result<Rect, Error> {
        let mut x = None;
        let mut y = None;
        let mut width = None;
        let mut height = None;

        for line in output.lines() {
            if line.starts_with("  Absolute upper-left X:") {
                x = line
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<i32>().ok());
            } else if line.starts_with("  Absolute upper-left Y:") {
                y = line
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<i32>().ok());
            } else if line.starts_with("  Width:") {
                width = line
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<i32>().ok());
            } else if line.starts_with("  Height:") {
                height = line
                    .split_whitespace()
                    .last()
                    .and_then(|s| s.parse::<i32>().ok());
            }
        }
        if let (Some(x), Some(y), Some(width), Some(height)) = (x, y, width, height) {
            Ok(Rect::new(
                Point::new(x as f64, y as f64),
                Size::new(width as f64, height as f64),
            ))
        } else {
            Err(Error::FailFetchPosition)
        }
    }

    fn find_named_window(window_name: &str) -> Result<Rect, Error> {
        let output = Command::new("xwininfo")
            .args(["-name", window_name])
            .output().map_err(|_| Error::FailFindWindow(window_name.to_string()))?;

        if output.status.success() {
            if let Ok(str_out) = str::from_utf8(&output.stdout) {
                Ok(ScreenShot::get_window_position(str_out)?)
            } else {
                Err(Error::FailFetchPosition)
            }
        } else {
            Err(Error::FailFindWindow(window_name.to_string()))
        }
    }

    fn bitmap_to_image_buffer(bitmap: &Bitmap) -> Result<RgbaImage, Error> {
        let (width, height) = (bitmap.size.width as u32, bitmap.size.height as u32);
        let mut imgbuf = RgbaImage::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pos = Point::new(x as f64, y as f64);
            *pixel = convert_rgba(bitmap.get_pixel(pos));
        }

        Ok(imgbuf)
    }

    pub fn from_named_window(window_name: &str) -> Result<Self, Error> {
        let rect = ScreenShot::find_named_window(window_name)?;
        let frame = autopilot::bitmap::capture_screen_portion(rect).map_err(|e| Error::FailCapture(e.to_string()))?;
        Ok(
            ScreenShot {
                frame,
                image: None,
                position: rect,
                items: Vec::new(),
            }
        )
    }

    fn chars_to_lines(mut chars: Vec<Char>) -> Vec<Vec<Char>> {
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

    fn chars_are_on_same_line(a: &Char, b: &Char) -> bool {
        // Check if the vertical center of one char is within the vertical range of the other
        (b.center <= a.top  && b.center >= a.bottom) ||
            (a.center <= b.top && a.center >= b.bottom)
    }

    fn chars_to_words(chars: Vec<Char>) -> Vec<Item> {
        let mut words: Vec<Item> = Vec::new();
        let mut current_word_chars: Vec<Char> = Vec::new();

        for char in chars {
            if current_word_chars.is_empty() || current_word_chars.last().unwrap().intersect(&char, 1) {
                current_word_chars.push(char);
            } else {
                words.push(Self::chars_to_item(current_word_chars));
                current_word_chars = vec![char];
            }
        }

        if !current_word_chars.is_empty() {
            words.push(Self::chars_to_item(current_word_chars));
        }

        words
    }

    fn chars_to_item(chars: Vec<Char>) -> Item {
        let mut text = String::new();
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        let mut top = i32::MIN;
        let mut bottom = i32::MAX;

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
                Point::new(left as f64, bottom as f64),
                Size::new((right - left) as f64, (top - bottom) as f64)
            ),
            kind: None,
        }
    }

    pub fn append_items(&mut self, items: Vec<Item>) {
        for item in items {
            self.append_item(item);
        }
    }

    pub fn append_item(&mut self, mut item: Item) {
        // Adjust item's position by adding the offset of the screenshot
        item.position.origin.x += self.position.origin.x;
        item.position.origin.y += self.position.origin.y;

        // Append the adjusted item to the items vector
        self.items.push(item);
    }

    fn draw_item_box(&mut self, rect: &Rect, border_thickness: i32, color: Rgba<u8>) {
        if let Some(img) = &mut self.image {
            // Adjust rectangle coordinates to be relative to the frame
            let adjusted_x = rect.origin.x as i32 - self.position.origin.x as i32;
            let adjusted_y = (self.frame.size.height as i32 - rect.origin.y as i32) - self.position.origin.y as i32 + rect.size.height as i32;


            for x in (adjusted_x - border_thickness)..(adjusted_x + rect.size.width as i32 + border_thickness) {
                for y in (adjusted_y - border_thickness)..(adjusted_y + rect.size.height as i32 + border_thickness) {
                    if x >= adjusted_x && x < adjusted_x + rect.size.width as i32 &&
                        (y < adjusted_y + border_thickness || y >= adjusted_y + rect.size.height as i32 - border_thickness) ||
                        y >= adjusted_y && y < adjusted_y + rect.size.height as i32 &&
                            (x < adjusted_x + border_thickness || x >= adjusted_x + rect.size.width as i32 - border_thickness) {
                        // Check bounds to prevent out-of-bounds drawing
                        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
                            img.put_pixel(x as u32, y as u32, color);
                        }
                    }
                }
            }
        }
    }

    fn process(&mut self) -> Result<(), Error> {
        let dynamic_image = DynamicImage::ImageRgba8(ScreenShot::bitmap_to_image_buffer(&self.frame)?);
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
        let lines = Self::chars_to_lines(chars);

        // Separate in items
        for line in lines{
            self.append_items(Self::chars_to_words(line));
        }

        self.image = Some(Self::bitmap_to_image_buffer(&self.frame)?);
        // Draw boundary box of each item
        for item in self.items.clone() {
            self.draw_item_box(&item.position, 5, Rgba([255, 0, 0, 255]));
        }
        
        // Save into a file
        if let Some(image) = self.image.take() {
            image.save("output.png").map_err(|e| Error::FailCapture(e.to_string()))?;
        }

        Ok(())
    }
}


fn main() -> Result<(), Error> {
    let mut frame = ScreenShot::from_named_window("Liana")?;
    frame.process()?;
    // println!("------------------------");
    // println!("{:?}", frame);
    // println!("------------------------");
    // for word in frame.items {
    //     println!("{:?}", word)
    // }


    Ok(())
}
