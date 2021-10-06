use test_engine::{
    screen::GameView,
    tools::Rglica,
    ui::{complex::AnalogStickView, make_view_on, DPadView, View, ViewBase},
    Image, Level,
};

use crate::game_level::GameLevel;

#[derive(Default)]
pub struct ControlsView {
    base:   ViewBase,
    dpad:   Rglica<DPadView>,
    _stick: Rglica<AnalogStickView>,

    level: GameLevel,
}

impl ControlsView {
    fn setup_dpad(&mut self) {
        self.dpad = make_view_on::<DPadView>(self);

        self.dpad.frame_mut().size.width = 280.0;
        self.dpad.frame_mut().size.height = 200.0;

        self.dpad.set_images(
            Image::load(&test_engine::paths::images().join("up.png")),
            Image::load(&test_engine::paths::images().join("down.png")),
            Image::load(&test_engine::paths::images().join("left.png")),
            Image::load(&test_engine::paths::images().join("right.png")),
        );
    }
}

impl View for ControlsView {
    fn setup(&mut self) { self.setup_dpad(); }

    fn layout(&mut self) {
        self.place().as_background();
        self.dpad.place().bottom_left_margin(10);
    }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}

impl GameView for ControlsView {
    fn level(&self) -> &dyn Level { &self.level }
    fn level_mut(&mut self) -> &mut dyn Level { &mut self.level }
}
