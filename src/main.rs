mod capture;
mod error;
mod graphical;
mod items;
mod ocr;
mod screenshot;
mod widget_detector;

use crate::capture::Capture;
use crate::graphical::{CORAL, DARK_CYAN, HOT_PINK, LIME, MEDIUM_BLUE, ORANGE, PINK, PURPLE, RED};
use crate::items::ItemKind::{
    CheckBox, MenuButton, PrimaryButton, SecondaryButton, SettingSection, TextInput,
};
use error::Error;
use items::*;

fn main() -> Result<(), Error> {
    let mut item_map: ItemMap = ItemMap::new();
    item_map.insert(LIME, TextInput);
    item_map.insert(RED, PrimaryButton(true));
    item_map.insert(ORANGE, PrimaryButton(false));
    item_map.insert(PURPLE, SecondaryButton(true));
    item_map.insert(MEDIUM_BLUE, SecondaryButton(false));
    item_map.insert(HOT_PINK, MenuButton(false));
    item_map.insert(PINK, MenuButton(true));
    item_map.insert(CORAL, CheckBox);
    item_map.insert(DARK_CYAN, SettingSection);
    let page = Capture::from_named_window("Liana", item_map)?;
    page.print_items();

    Ok(())
}
