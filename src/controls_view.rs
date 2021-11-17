use std::rc::Rc;
use std::default::default;

use test_engine::{Image, Level, screen::GameView, sprites::{DummyDrawer, SpritesDrawer}, tools::{Event, Rglica}, ui::{
        complex::{AnalogStickView, Slider},
        make_view_on, DPadView, View, ViewBase,
    }};
use crate::game_level::GameLevel;

pub struct ControlsView {
    base:         ViewBase,
    dpad:         Rglica<DPadView>,
    stick:        Rglica<AnalogStickView>,
    level:        GameLevel,
    scale_slider: Rglica<Slider>,
    drawer:       Rc<dyn SpritesDrawer>,
}

impl ControlsView {
    fn setup_dpad(&mut self) {
        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (280, 200).into();

            dpad.set_images(
                Image::load(&test_engine::paths::images().join("up.png")),
                Image::load(&test_engine::paths::images().join("down.png")),
                Image::load(&test_engine::paths::images().join("left.png")),
                Image::load(&test_engine::paths::images().join("right.png")),
            );
        });
    }

    fn setup_slider(&mut self) {
        let mut this = Rglica::from_ref(self);
        self.scale_slider = make_view_on(self, move |slider: &mut Slider| {
            slider.multiplier = 5.0;
            slider.frame_mut().size = (50, 280).into();
            slider.on_change.subscribe(move |scale| {
                this.drawer.set_scale(scale);
            });
        });
    }

    fn setup_stick(&mut self) {
        let mut level = Rglica::from_ref(&self.level);
        self.stick = make_view_on(self, |stick: &mut AnalogStickView| {
            stick.flaccid = true;

            stick.on_direction_change.subscribe(move |mut direction| {
                direction.invert_y();
                level.set_gravity(direction * 10);
                level.drawer().set_camera_rotation(direction.angle());
            });
        });
    }
}

impl View for ControlsView {
    fn setup(&mut self) {
        self.setup_dpad();
        self.setup_slider();
        self.setup_stick();
    }

    fn layout(&mut self) {
        self.place().as_background();
        self.dpad.place().bottom_left_margin(10);
        self.stick.place().bottom_right_margin(10);
        self.scale_slider.place().right();
    }

    fn view(&self) -> &ViewBase {
        &self.base
    }

    fn view_mut(&mut self) -> &mut ViewBase {
        &mut self.base
    }
}

impl GameView for ControlsView {
    fn level(&self) -> &dyn Level {
        &self.level
    }
    fn level_mut(&mut self) -> &mut dyn Level {
        &mut self.level
    }
    fn set_drawer(&mut self, drawer: Rc<dyn SpritesDrawer>) {
        self.drawer = drawer.clone();
        self.level.level_mut().drawer = drawer;
    }
}

impl Default for ControlsView {
    fn default() -> Self {
        Self {
            base:         default(),
            dpad:         default(),
            stick:        default(),
            level:        default(),
            scale_slider: default(),
            drawer: Rc::new(DummyDrawer::default()),
        }
    }
}