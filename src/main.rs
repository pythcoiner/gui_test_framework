mod capture;
mod error;
mod graphical;
mod item;
mod liana_item;
mod liana_store;
mod ocr;
mod screenshot;
mod store;
mod widget_detector;

use crate::capture::Capture;
use crate::graphical::Color;
use crate::graphical::{CORAL, DARK_CYAN, HOT_PINK, LIME, MEDIUM_BLUE, ORANGE, PINK, PURPLE, RED};
use crate::liana_item::LianaItemType::{
    CheckBox, MenuButton, PrimaryButton, SecondaryButton, SettingSection, TextInput,
};
use error::Error;
use liana_item::LianaItemType;
use screenshot::ScreenShot;
use std::collections::HashMap;

fn screenshot() -> Result<ScreenShot, Error> {
    let mut item_map: HashMap<Color, LianaItemType> = HashMap::new();
    item_map.insert(LIME, TextInput);
    item_map.insert(RED, PrimaryButton(true));
    item_map.insert(ORANGE, PrimaryButton(false));
    item_map.insert(PURPLE, SecondaryButton(true));
    item_map.insert(MEDIUM_BLUE, SecondaryButton(false));
    item_map.insert(HOT_PINK, MenuButton(false));
    item_map.insert(PINK, MenuButton(true));
    item_map.insert(CORAL, CheckBox);
    item_map.insert(DARK_CYAN, SettingSection);

    Capture::from_named_window("Liana", item_map)
}

fn main() -> Result<(), Error> {
    let _page = screenshot()?;

    Ok(())
}
