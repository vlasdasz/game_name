use test_engine::{
    assets::Assets,
    gm::{Color, Point, Rect},
    maze::{
        cell::{Cell, CellSide},
        Grid,
    },
    rtools::Rglica,
    sprites::{Control, Player, Wall},
    Image, Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct GameLevel {
    base:  LevelBase,
    cells: Vec<Rglica<Wall>>,
}

impl Level for GameLevel {
    fn setup(&mut self) {
        self.set_scale(2.0);

        self.base.player =
            Player::make((0, 10, 17.0 / 16.0, 28.0 / 16.0).into(), self.level_mut()).into();

        self.base.player.weapon.set_image(Assets::image("ak.png"));

        self.base.player.set_image(Assets::image("frisk.png"));

        for i in 0..500 {
            self.add_body((0.1 * i as f32, i as f32 * 0.5, 0.2, 0.2).into());
        }
    }

    fn on_key_pressed(&mut self, key: String) {
        if key == "-" {
            self.set_scale(self.scale() / 2.0);
        } else if key == "=" {
            self.set_scale(self.scale() * 2.0);
        }

        self.player().move_by_key(key);
    }

    fn level(&self) -> &LevelBase {
        &self.base
    }

    fn level_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }
}

impl GameLevel {
    fn _make_walls(&mut self) {
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
        self.cells.iter_mut().for_each(|a| a.remove());
        self.cells.clear();

        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                self.add_cell(cell, x, y);
            }
        }
    }

    fn add_cell(&mut self, cell: &Cell, x: usize, y: usize) {
        if !cell.visited {
            let frame = visited_frame(x, y);
            let mut wall = self.add_wall(frame.into());
            wall.set_color(Color::BLACK);
            self.cells.push(wall);
        }

        cell.all_sides(|side| {
            let frame = frame_for_side(side, x, y);
            let mut wall = self.add_wall(frame.into());
            wall.set_color(Color::BLACK);
            self.cells.push(wall);
        })
    }
}

const BIG: f32 = 10.0;
const SMALL: f32 = 1.0;

fn origin(x: usize, y: usize) -> Point {
    (BIG * 2.0 * x as f32, BIG * 2.0 * y as f32).into()
}

fn visited_frame(x: usize, y: usize) -> Rect {
    let origin = origin(x, y);
    const SIZE: f32 = BIG / 2.0;
    (origin.x, origin.y, SIZE, SIZE).into()
}

fn frame_for_side(side: CellSide, x: usize, y: usize) -> Rect {
    let origin = origin(x, y);

    match side {
        CellSide::Down => (origin.x, origin.y + BIG, BIG, SMALL),
        CellSide::Up => (origin.x, origin.y - BIG, BIG, SMALL),
        CellSide::Left => (origin.x - BIG, origin.y, SMALL, BIG),
        CellSide::Right => (origin.x + BIG, origin.y, SMALL, BIG),
    }
    .into()
}
