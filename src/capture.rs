use crate::error::Error;
use crate::graphical::Graphical;
use crate::liana_item::LianaItemType;
use crate::liana_store::LianaStore;
use crate::screenshot::{ScreenShot, ScreenShotTrait};
use crate::widget_detector::detect_items;
use crate::Color;
use autopilot::geometry::{Point, Rect, Size};
use image::RgbaImage;
use std::collections::HashMap;
use std::process::Command;
use std::str;

#[allow(unused)]
pub struct Capture {}

impl Capture {
    pub fn get_window_position(output: &str) -> Result<Rect, Error> {
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

    pub fn find_named_window(window_name: &str) -> Result<Rect, Error> {
        let output = Command::new("xwininfo")
            .args(["-name", window_name])
            .output()
            .map_err(|_| Error::FailFindWindow(window_name.to_string()))?;

        if output.status.success() {
            if let Ok(str_out) = str::from_utf8(&output.stdout) {
                Ok(Self::get_window_position(str_out)?)
            } else {
                Err(Error::FailFetchPosition)
            }
        } else {
            Err(Error::FailFindWindow(window_name.to_string()))
        }
    }

    pub fn from_named_window(
        window_name: &str,
        map: HashMap<Color, LianaItemType>,
    ) -> Result<ScreenShot, Error> {
        let rect = Self::find_named_window(window_name)?;
        let frame = autopilot::bitmap::capture_screen_portion(rect)
            .map_err(|e| Error::FailCapture(e.to_string()))?;
        let frame = *RgbaImage::from_bitmap(&frame)?;
        let mut s = ScreenShot {
            frame,
            image: None,
            position: rect,
            item_map: map,
            store: LianaStore::new(),
        };
        if let Ok(items) = detect_items(&s.item_map, &s.frame) {
            s.append_items(items);
            Ok(s)
        } else {
            println!("Fail to detect items!");
            Err(Error::DetectItem)
        }
    }
}
