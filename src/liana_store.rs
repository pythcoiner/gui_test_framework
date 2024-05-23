use crate::liana_item::{LianaItem, LianaItemType};
use crate::store::Store;

#[allow(unused)]
#[derive(Debug)]
pub struct LianaStore {
    text_inputs: Vec<LianaItem>,
    primary_buttons: Vec<LianaItem>,
    secondary_buttons: Vec<LianaItem>,
    menu_buttons: Vec<LianaItem>,
    checkboxes: Vec<LianaItem>,
    setting_section: Vec<LianaItem>,
}

impl LianaStore {
    pub fn new() -> Self {
        LianaStore {
            text_inputs: Vec::new(),
            primary_buttons: Vec::new(),
            secondary_buttons: Vec::new(),
            menu_buttons: Vec::new(),
            checkboxes: Vec::new(),
            setting_section: Vec::new(),
        }
    }
}

impl Store<LianaItem, LianaItemType> for LianaStore {
    fn bucket(&mut self, kind: LianaItemType) -> &mut Vec<LianaItem> {
        match kind {
            LianaItemType::TextInput => &mut self.text_inputs,
            LianaItemType::PrimaryButton(_) => &mut self.primary_buttons,
            LianaItemType::SecondaryButton(_) => &mut self.secondary_buttons,
            LianaItemType::MenuButton(_) => &mut self.menu_buttons,
            LianaItemType::CheckBox => &mut self.checkboxes,
            LianaItemType::SettingSection => &mut self.setting_section,
        }
    }
}
