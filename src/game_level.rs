use std::ops::Deref;

use test_engine::{
    assets::Assets,
    sprites::{Control, SpritesDrawer},
    Image, Level, LevelBase, Sprite,
};

#[derive(Default)]
pub struct GameLevel {
    scale: f32,
    base:  LevelBase,
}

impl Level for GameLevel {
    fn setup(&mut self) {
        self.scale = 1.0;

        self.base.player = self.add_body((0, 10, 17.0 / 6.0, 28.0 / 6.0).into());
        self.base.player.set_image(Assets::image("frisk.png"));

        self.add_sprite((0, 0, 1, 1).into());

        self.make_walls();

        for i in 0..500 {
            self.add_body((0.1 * i as f32, i as f32 * 0.5, 0.5, 0.5).into());
        }
    }

    fn on_key_pressed(&mut self, key: String) {
        if key == "-" {
            self.scale /= 2.0;
        } else if key == "=" {
            self.scale *= 2.0;
        }

        self.drawer().set_scale(self.scale);
        self.player().move_by_key(key);
    }

    fn level(&self) -> &LevelBase {
        &self.base
    }

    fn level_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn drawer(&self) -> &dyn SpritesDrawer {
        self.base.drawer.deref()
    }
}

impl GameLevel {
    fn make_walls(&mut self) {
        let square = Image::load(&test_engine::paths::images().join("square.png"));

        let width = 280;
        let wall_width = 10;

        self.add_wall((width, wall_width).into())
            .set_image(square.clone());

        self.add_wall((-width, width, wall_width, width).into())
            .set_image(square.clone());

        self.add_wall((width, width, wall_width, width).into())
            .set_image(square.clone());

        self.add_wall((0, width * 2, width, 5).into())
            .set_image(square.clone());

        self.add_wall((40, 0, 5, 100).into())
            .set_image(square.clone());

        self.add_wall((-40, 0, 5, 100).into()).set_image(square);
    }
}
