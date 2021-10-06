use test_engine::{
    screen::GameView,
    tools::Rglica,
    ui::{complex::AnalogStickView, make_view_on, DPadView, View, ViewBase},
    Image, Level,
};

use crate::game_level::GameLevel;

#[derive(Default)]
pub struct ControlsView {
    base:  ViewBase,
    dpad:  Rglica<DPadView>,
    stick: Rglica<AnalogStickView>,

    level: GameLevel,
}

impl ControlsView {
    fn setup_dpad(&mut self) {
        self.dpad = make_view_on(self);

        self.dpad.frame_mut().size = (280, 200).into();

        self.dpad.set_images(
            Image::load(&test_engine::paths::images().join("up.png")),
            Image::load(&test_engine::paths::images().join("down.png")),
            Image::load(&test_engine::paths::images().join("left.png")),
            Image::load(&test_engine::paths::images().join("right.png")),
        );
    }

    fn setup_stick(&mut self) {
        self.stick = make_view_on(self);
        self.stick.flaccid = true;

        let mut level = Rglica::from_ref(&self.level);
        self.stick.on_direction_change.subscribe(move |mut direction| {
            direction.invert_y();
            level.set_gravity(direction * 10)
        });
    }
}

impl View for ControlsView {
    fn setup(&mut self) {
        self.setup_dpad();
        self.setup_stick();
    }

    fn layout(&mut self) {
        self.place().as_background();
        self.dpad.place().bottom_left_margin(10);
        self.stick.place().bottom_right_margin(10);
    }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}

impl GameView for ControlsView {
    fn level(&self) -> &dyn Level { &self.level }
    fn level_mut(&mut self) -> &mut dyn Level { &mut self.level }
}
