use proc_macro::Boxed;
use screen::GameView;
use test_engine::*;
use tools::{Boxed, Rglica, ToRglica};
use ui::{DPadView, View, ViewBase};

#[derive(Boxed)]
pub struct ControlsView {
    base: ViewBase,
    dpad: Rglica<DPadView>,
}

impl ControlsView {
    fn setup_dpad(&mut self) {
        let dpad = DPadView::boxed();
        self.dpad = dpad.to_rglica();
        self.dpad.frame_mut().size.width = 280.0;
        self.dpad.frame_mut().size.height = 200.0;
        self.add_subview(dpad);

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
        self.dpad.place().lb();
    }

    fn view(&self) -> &ViewBase { &self.base }

    fn view_mut(&mut self) -> &mut ViewBase { &mut self.base }
}

impl GameView for ControlsView {
    fn level(&self) -> &dyn Level { todo!() }

    fn level_mut(&mut self) -> &mut dyn Level { todo!() }
}
