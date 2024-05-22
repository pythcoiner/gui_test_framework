use std::collections::HashMap;

use crate::graphical::Color;
use crate::graphical::Graphical;
use crate::item::Item;
use crate::liana_item::LianaItemType;
use crate::liana_store::LianaStore;
use autopilot::geometry::Rect;
use image::{Rgba, RgbaImage};

#[derive(Debug)]
pub struct ScreenShot {
    pub frame: RgbaImage,
    pub image: Option<RgbaImage>,
    pub position: Rect,
    pub item_map: HashMap<Color, LianaItemType>,
    pub store: LianaStore,
}

#[allow(unused)]
impl<LianaItem: Item<LianaItemType>> ScreenShotTrait<LianaItem, LianaItemType> for ScreenShot {
    fn position(&self) -> Rect {
        todo!()
    }

    fn store(&mut self) -> &mut Vec<LianaItem> {
        todo!()
    }

    fn image(&self) -> Option<RgbaImage> {
        todo!()
    }
}

#[allow(unused)]
pub trait ScreenShotTrait<I, K>
where
    I: Item<K>,
{
    fn position(&self) -> Rect;

    fn store(&mut self) -> &mut Vec<I>;

    fn image(&self) -> Option<RgbaImage>;

    fn append_items(&mut self, items: Vec<I>) {
        for item in items {
            self.push(item);
        }
    }

    fn push(&mut self, item: I) {
        // Adjust item's position by adding the offset of the screenshot
        // item.position.origin.x = self.position.origin.x - item.position.origin.x;
        // item.position.origin.y = -(self.position.origin.y - item.position.origin.y);
        // item.position.size.height = - item.position.size.height;

        // Append the adjusted item to the items vector
        self.store().push(item);
    }

    fn draw_item_box(&mut self, rect: &Rect, b: u32, color: Rgba<u8>) {
        let h = self.position().size.height as u32;
        if let Some(img) = &mut self.image() {
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
}
