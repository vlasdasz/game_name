use std::rc::Rc;

use test_engine::{
    maze::{maker::Maker, Grid},
    sprites::{Control, DummyDrawer, SpritesDrawer},
    tools::{Rglica, ToRglica},
    ui::{
        complex::{AnalogStickView, Slider},
        init_view_with_frame, make_view_on, DPadView, Label, View, ViewBase,
    },
    ui_layer::GameView,
    Image, Level,
};
use tokio::sync::mpsc::Receiver;

use crate::game_level::GameLevel;

pub struct ControlsView {
    base:          ViewBase,
    dpad:          Rglica<DPadView>,
    stick:         Rglica<AnalogStickView>,
    level:         GameLevel,
    scale_slider:  Rglica<Slider>,
    gravity_label: Rglica<Label>,
    drawer:        Rc<dyn SpritesDrawer>,

    grid_recv: Receiver<Grid>,
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
        let this = Rglica::from_ref(self);
        self.scale_slider = make_view_on(self, move |slider: &mut Slider| {
            slider.multiplier = 5.0;
            slider.frame_mut().size = (50, 280).into();
            slider.on_change.subscribe(move |scale| {
                this.drawer.set_scale(scale);
            });
        });
    }

    fn setup_stick(&mut self) {
        let mut this = self.to_rglica();
        self.stick = make_view_on(self, |stick: &mut AnalogStickView| {
            stick.flaccid = true;

            stick.on_direction_change.subscribe(move |mut direction| {
                direction.invert_y();
                this.level.set_gravity(direction * 10);
                this.level.drawer().set_camera_rotation(direction.angle());
                let gravity = this.level().gravity();
                this.gravity_label
                    .set_text(format!("gravity: {:?}", gravity));
            });
        });
    }

    fn setup_level(&mut self) {
        self.level_mut().setup();

        let mut player = self.level_mut().player().clone();
        self.dpad
            .on_press
            .subscribe(move |dir| player.move_by_direction(dir));
    }

    fn setup_ui(&mut self) {
        self.gravity_label = init_view_with_frame((100, 100).into(), self);
    }
}

impl View for ControlsView {
    fn setup(&mut self) {
        self.setup_dpad();
        self.setup_slider();
        self.setup_stick();
        self.setup_level();
        self.setup_ui();
    }

    fn layout(&mut self) {
        self.place().as_background();
        self.dpad.place().bottom_left_margin(10);
        self.stick.place().bottom_right_margin(10);
        self.scale_slider.place().right();
        self.gravity_label.place().top_right_margin(10);
    }

    fn update(&mut self) {
        let _ = self
            .grid_recv
            .try_recv()
            .inspect(|val| self.level.display_grid(val));
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
            base:          Default::default(),
            dpad:          Default::default(),
            stick:         Default::default(),
            level:         Default::default(),
            scale_slider:  Default::default(),
            gravity_label: Default::default(),
            drawer:        Rc::new(DummyDrawer::default()),
            grid_recv:     Maker::generate(),
        }
    }
}
