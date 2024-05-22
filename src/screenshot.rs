use autopilot::geometry::Rect;
use image::{Rgba, RgbaImage};

use crate::error::Error;
use crate::graphical::Graphical;
use crate::items::*;

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct ScreenShot {
    pub frame: RgbaImage,
    pub image: Option<RgbaImage>,
    pub position: Rect,
    pub items: Vec<Item>,
    pub item_map: ItemMap,
}

#[allow(unused)]
impl ScreenShot {
    pub fn append_items(&mut self, items: Vec<Item>) {
        for item in items {
            self.append_item(item);
        }
    }

    pub fn append_item(&mut self, item: Item) {
        // Adjust item's position by adding the offset of the screenshot
        // item.position.origin.x = self.position.origin.x - item.position.origin.x;
        // item.position.origin.y = -(self.position.origin.y - item.position.origin.y);
        // item.position.size.height = - item.position.size.height;

        // Append the adjusted item to the items vector
        self.items.push(item);
    }

    #[allow(unused)]
    fn draw_item_box(&mut self, rect: &Rect, b: u32, color: Rgba<u8>) {
        let h = self.position.size.height as u32;
        if let Some(img) = &mut self.image {
            let xa: u32 = rect.origin.x as u32;
            let xb: u32 = (rect.origin.x + rect.size.width) as u32;
            let ya: u32 = h - (rect.origin.y as u32);
            let yb: u32 = h - ((rect.origin.y + rect.size.height) as u32);

            img.draw_rectangle(xa - b, xb + b, ya - b, ya, color);
            img.draw_rectangle(xa - b, xb + b, yb, yb + b, color);
            img.draw_rectangle(xa - b, xa, ya, yb, color);
            img.draw_rectangle(xb, xb + b, ya, yb, color);
        }
    }

    fn process(&mut self, _map: ItemMap) -> Result<(), Error> {
        // self.frame.save("frame.png").map_err(|e| Error::FailCapture(e.to_string()))?;
        // for (color, kind) in map {
        //     let frame = self.frame.filter_by_color(color);
        //     let filename = format!("{:?}.png", color);
        //     frame.save(filename).map_err(|e| Error::FailCapture(e.to_string()))?;
        //     Ocr::img_to_items(frame, self, kind)?;
        // }

        // --------------------- Display items -----------------------

        // self.image = Some(self.frame.clone());
        // // Draw boundary box of each item
        // for item in self.items.clone() {
        //     println!("{}", &item.text);
        //     self.draw_item_box(&item.position, 4, Rgba([255, 0, 0, 255]));
        // }
        //
        // // Save into a file
        // if let Some(image) = self.image.take() {
        //     image.save("output.png").map_err(|e| Error::FailCapture(e.to_string()))?;
        // }

        Ok(())
    }

    pub fn print_items(&self) {
        for item in &self.items {
            println!("{} => {:?}", item.text.as_ref().unwrap(), item.kind);
        }
    }
}
