use test_engine::{
    assets::Assets,
    gm::{Point, Rect},
    maze::{cell::Cell, Grid},
    Image, Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct GameLevel {
    scale: f32,
    base:  LevelBase,
}

impl Level for GameLevel {
    fn setup(&mut self) {
        self.scale = 1.0;

        self.base.player = self.add_body((0, 10, 17.0 / 6.0, 28.0 / 6.0).into());
        self.base.player.set_image(Assets::image("frisk.png"));

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

        let scale = self.scale;
        self.drawer().set_scale(scale);
        //self.player().move_by_key(key);
    }

    fn level(&self) -> &LevelBase {
        &self.base
    }

    fn level_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }
}

impl GameLevel {
    fn make_walls(&mut self) {
        let square = Image::load(&test_engine::paths::images().join("square.png"));

        let width = 280;
        let wall_width = 10;

        self.add_wall((0, 0, width, wall_width).into())
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

    pub fn display_grid(&mut self, grid: &Grid) {
        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                self.add_cell(cell, x, y);
            }
        }
    }

    fn add_cell(&mut self, cell: &Cell, x: usize, y: usize) {
        const LENGHT: f32 = 10.0;
        const WIDTH: f32 = 0.5;

        let origin: Point = (LENGHT * x as f32, LENGHT * y as f32).into();

        if cell.left {
            let _rect: Rect = (
                origin.x + LENGHT / 2.0,
                origin.y + WIDTH / 2.0,
                LENGHT,
                WIDTH,
            )
                .into();

            // self.add_sprite(rect);
        }
    }
}
