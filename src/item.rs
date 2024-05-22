use crate::widget_detector::Position;

#[allow(unused)]
fn click_mouse() {
    autopilot::mouse::click(autopilot::mouse::Button::Left, None);
}
#[allow(unused)]
fn move_mouse(pos: Position) {
    autopilot::mouse::move_to(autopilot::geometry::Point {
        x: pos.x as f64,
        y: pos.y as f64,
    })
    .unwrap();
}
#[allow(unused)]
fn input_text(text: String) {
    autopilot::key::type_string(&text, &[], 30.0, 0.0);
}
#[allow(unused)]
pub trait Item<K> {
    // To be reimplemented
    fn position(&self) -> Position;
    fn name(&self) -> String;
    fn kind(&self) -> K;

    // To not be reimplemented
    fn click(&self) {
        self.hover();
        click_mouse();
    }
    fn insert(&self, text: String) {
        self.click();
        input_text(text);
    }
    fn hover(&self) {
        let position = self.position();
        move_mouse(position);
    }
}
