use std::path::Path;
use std::process::{Command, Stdio};
use autopilot::geometry::{Point};
use serde::{Deserialize, Serialize};
use crate::graphical::Color;
use crate::items::{Item, ItemMap};
use rusty_tesseract::image::RgbaImage;
use crate::ocr::Ocr;

#[derive(Debug, Clone, Copy)]
pub enum DetectError {
    CannotStartProcessDw,
    ExecuteDw,
    ParseUtf8,
    ParseResponse,
    WriteTempFile,

}

fn color_arg(map: &ItemMap) -> String {
    let mut colors = String::new();

    let color_list = map
        .keys()
        .collect::<Vec<_>>();

    for (i, c) in color_list.iter().enumerate() {
        colors = format!("{}[{}, {}, {}]", colors, c.r, c.g, c.b);
        if  i < color_list.len() - 1 {
            colors.push_str(", ")
        }
    }

    let head = r#"{ "colors": ["#;
    let tail = r#"]}"#;

    colors = format!("{}{}{}", head, colors, tail);

    colors
}

fn detect(colors: &str, path: &Path) -> Result<String, DetectError> {
    let mut dw = Command::new("dw");
    let dw = dw.arg("-f")
        .arg(path)
        .arg("-c")
        .arg(colors)
        .stdout(Stdio::piped())
        .output()
        .map_err(|_| DetectError::CannotStartProcessDw)?;

    if dw.status.success() {
        String::from_utf8(dw.stdout).map_err(|_| DetectError::ParseUtf8)
    } else {
       Err(DetectError::ExecuteDw)
    }

}

#[allow(unused)]
pub fn detect_items(map: &ItemMap, frame: &RgbaImage) -> Result<Vec<Item>, DetectError> {
    
    // get current dir
    let mut dir = std::env::current_dir()
        .expect("Should no fait getting cwd!");
    dir.push("frame.png");
    
    // save frame in temp `frame.png` file
    frame.save(&dir).map_err(|e| DetectError::WriteTempFile)?;
    
    let colors = color_arg(map);

    let r = detect(&colors, &dir)?;

    let out: Response = serde_json::from_str(&r)
        .map_err(|_| DetectError::ParseResponse)?;

    if out.status != *"OK" {
        return Err(DetectError::ExecuteDw);
    }
    
    let items = out.items.unwrap();
    

    
    items.iter().for_each(|i| println!("{:?}", i));
    
    let items = items
        .into_iter()
        .filter_map(|item| {
            let color = item.color();
            let position = item.position();
            if let (Some(color), Some(position)) = (color, position) {
                map.get(&color).map(|kind| Item{
                            text: None,
                            position,
                            kind: *kind,
                        })
                
            } else { None }
        })
        .filter_map(|mut item| {
            if let Some(label) = Ocr::read_label(frame, &item.position) {
                item.text = Some(label);
                Some(item)
            } else { None }
        })
        .collect::<Vec<_>>();
    
    Ok(items)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct DetectedItem {
    color: Vec<u8>,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl DetectedItem {

    pub fn is_valid(&self) -> bool {
        #[allow(clippy::if_same_then_else, clippy::needless_bool)]
        if self.top > self.bottom { false }
        else if self.left > self.right { false }
        else if self.color.len() != 3 { false }
        else { true }
    }
    
    pub fn color(&self) -> Option<Color> {
        if !self.is_valid() {
            None
        } else {
            Some(Color {
                r: self.color[0],
                g: self.color[1],
                b: self.color[2],
            })
        }

    }

    pub fn position(&self) -> Option<Position> {
        if !self.is_valid() {
            None
        } else {
            Some(
                Position{
                        x: self.left, 
                        y: self.top,
                        width: self.right - self.left, 
                        height: self.bottom - self.top,
                }
            )
        }
    }
    
    #[allow(unused)]
    pub fn center(&self) -> Point {
        if !self.is_valid() {
            panic!("Invalid item!")
        }
        
        Point {
            x: ((self.left + self.right) / 2) as f64,
            y: ((self.top + self.bottom) / 2) as f64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Response {
    status: String,
    items: Option<Vec<DetectedItem>>,
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::Path;
    use crate::graphical::Color;
    use crate::items::ItemKind;
    use super::*;
    #[test]
    fn test_deserialize_response() {
        const RESPONSE: &str = r#"{ "status": "OK", "items": [{ "color": [254, 167, 0], "top": 696, "bottom": 734, "left": 871, "right": 891}, { "color": [226, 78, 27], "top": 697, "bottom": 741, "left": 748, "right": 768}, { "color": [0, 255, 0], "top": 546, "bottom": 670, "left": 350, "right": 371}, { "color": [0, 255, 0], "top": 406, "bottom": 444, "left": 886, "right": 906}, { "color": [0, 255, 0], "top": 406, "bottom": 513, "left": 404, "right": 424}, { "color": [0, 255, 0], "top": 356, "bottom": 461, "left": 405, "right": 425}, { "color": [0, 255, 0], "top": 306, "bottom": 370, "left": 404, "right": 424}, { "color": [127, 0, 127], "top": 486, "bottom": 608, "left": 812, "right": 835}, { "color": [255, 105, 180], "top": 772, "bottom": 836, "left": 43, "right": 65}, { "color": [255, 105, 180], "top": 442, "bottom": 493, "left": 44, "right": 65}, { "color": [255, 105, 180], "top": 392, "bottom": 489, "left": 43, "right": 63}, { "color": [255, 105, 180], "top": 343, "bottom": 389, "left": 43, "right": 62}, { "color": [255, 105, 180], "top": 292, "bottom": 352, "left": 44, "right": 65}, { "color": [255, 105, 180], "top": 195, "bottom": 243, "left": 44, "right": 60}, { "color": [228, 171, 183], "top": 242, "bottom": 285, "left": 43, "right": 63} ] }"#;
        let response: Response = serde_json::from_str(RESPONSE).unwrap();
        assert_eq!(response.status , "OK".to_string());
        let items = response.items.unwrap();
        assert_eq!(items.len() , 15);
        assert_eq!(items[0], DetectedItem{ color: vec![254, 167, 0, ], top: 696, bottom: 734, left: 871, right: 891 })
    }

    #[test]
    fn test_colors_arg() {
        let mut map = HashMap::<Color, ItemKind>::new();
        map.insert(Color{r:226, g:78, b:27}, ItemKind::CheckBox);
        map.insert(Color{r:254, g:167, b:0}, ItemKind::CheckBox);

        let arg = color_arg(&map);

        // FIXME: idk why sometime ordering change, should be related to hash function
        let fw = r#"{ "colors": [[226, 78, 27], [254, 167, 0]]}"#.to_string();
        let rv = r#"{ "colors": [[254, 167, 0], [226, 78, 27]]}"#.to_string();
        assert!(arg == fw || arg == rv )

    }

    #[test]
    fn test_dw() {
        // get absolute path of test frame
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(manifest_dir)
            .join("src")
            .join("test")
            .join("assets")
            .join("frame.png");

        println!("{path:?}");

        let mut map = HashMap::<Color, ItemKind>::new();
        map.insert(Color{r:226, g:78, b:27}, ItemKind::CheckBox);

        let colors = color_arg(&map);

        let ret = detect(&colors, &path);
        assert!(ret.is_ok());
        let stdout = ret.unwrap();
        let response: Response = serde_json::from_str(&stdout).unwrap();
        assert_eq!(response.status, "OK".to_string());

        let items = response.items.unwrap();

        let item = DetectedItem { color: vec![226, 78, 27], top: 709, bottom: 753, left: 760, right: 780 };
        assert_eq!(items[0], item);

        let item = DetectedItem { color: vec![226, 78, 27], top: 534, bottom: 544, left: 1042, right: 1055 };
        assert_eq!(items[1], item);
    }

}


