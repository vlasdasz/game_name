use test_engine::{
    game_view::GameView,
    rtools::{data_manager::DataManager, Rglica, ToRglica},
    sprites::{Control, SpritesDrawer},
    ui::{
        complex::{AnalogStickView, Slider},
        view_base::{add_view_with_frame, make_view_on, ViewBase},
        DPadView, Label, View,
    },
    Image, Level,
};

use crate::game_level::GameLevel;

#[derive(Debug, Default)]
pub struct ControlsView {
    base:          ViewBase,
    dpad:          Rglica<DPadView>,
    stick:         Rglica<AnalogStickView>,
    level:         GameLevel,
    scale_slider:  Rglica<Slider>,
    gravity_label: Rglica<Label>,
}

impl ControlsView {
    fn setup_dpad(&mut self) {
        self.dpad = make_view_on(self, |dpad: &mut DPadView| {
            dpad.frame_mut().size = (280, 200).into();

            dpad.set_images(
                Image::get("up.png"),
                Image::get("down.png"),
                Image::get("left.png"),
                Image::get("right.png"),
            );
        });
    }

    fn setup_slider(&mut self) {
        let mut this = Rglica::from_ref(self);
        self.scale_slider = make_view_on(self, move |slider: &mut Slider| {
            slider.set_multiplier(5);
            slider.frame_mut().size = (50, 280).into();
            slider.on_change.subscribe(move |scale| {
                this.level.drawer_mut().set_scale(scale);
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

        let mut player = self.level.player;
        self.dpad
            .on_press
            .subscribe(move |dir| player.move_by_direction(dir));
    }

    fn setup_ui(&mut self) {
        self.gravity_label = add_view_with_frame(self, (100, 100));
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
        self.dpad.place().bottom_left(10);
        self.stick.place().bottom_right(10);
        self.scale_slider.place().right();
        self.gravity_label.place().top_right(10);
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
    fn set_sprites_drawer(&mut self, drawer: Rglica<dyn SpritesDrawer>) {
        self.level.base_mut().drawer = drawer;
    }
}
