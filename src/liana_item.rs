use crate::item::Item;
use autopilot::geometry::Rect;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LianaItemType {
    TextInput,
    PrimaryButton(bool),
    SecondaryButton(bool),
    MenuButton(bool),
    CheckBox,
    SettingSection,
}

#[derive(Debug, Clone)]
pub struct LianaItem {
    pub text: Option<String>,
    pub position: Rect,
    pub kind: LianaItemType,
}

impl Item<LianaItemType> for LianaItem {
    fn position(&self) -> Rect {
        self.position
    }

    fn name(&self) -> String {
        self.text.clone().unwrap()
    }

    fn kind(&self) -> LianaItemType {
        self.kind
    }
}
