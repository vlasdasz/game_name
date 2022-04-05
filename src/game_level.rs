use test_engine::{
    assets::Assets,
    gm::{volume::GyroData, Color, Point, Rect},
    maze::{
        cell::{Cell, CellSide},
        Grid,
    },
    rtools::{Rglica, ToRglica},
    sprites::{add_sprite, Control, Player, SpriteData, Unit, Wall},
    Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct GameLevel {
    base:        LevelBase,
    cells:       Vec<Rglica<Wall>>,
    gyro_sprite: Rglica<SpriteData>,
    pub player:  Rglica<Player>,
}

impl Level for GameLevel {
    fn setup(&mut self) {
        self.set_scale(2.0);
        self.make_walls();
        self.setup_player();
        self.setup_enemies();

        let gyro = SpriteData::make((0, 20, 2, 0.8).into(), self.rglica());
        let rglyro = gyro.to_rglica();
        self.add_sprite(gyro);
        self.gyro_sprite = rglyro;
        self.gyro_sprite.set_image(Assets::image("arrow.png"));
    }

    fn update(&mut self) {
        let pos = self.player.position();
        self.set_camera_position(pos);
    }

    fn on_key_pressed(&mut self, key: String) {
        if key == "-" {
            self.set_scale(self.scale() / 2.0);
        } else if key == "=" {
            self.set_scale(self.scale() * 2.0);
        }

        self.player.move_by_key(key);
    }

    fn on_gyro_changed(&mut self, gyro: GyroData) {
        dbg!(&gyro);
        self.gyro_sprite
            .set_rotation(gyro.pitch - std::f32::consts::PI / 2.0);
    }

    fn base(&self) -> &LevelBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LevelBase {
        &mut self.base
    }

    fn rglica(&self) -> Rglica<dyn Level> {
        (self as &dyn Level).to_rglica()
    }
}

impl GameLevel {
    fn setup_player(&mut self) {
        self.player = add_sprite((0, 5, 2, 2), self);

        self.player.set_image(Assets::image("frisk.png"));

        self.player.weapon.set_image(Assets::image("ak.png"));
        self.player.weapon.bullet_image = Assets::image("bullet.png").into();
        self.player.weapon.bullet_speed = 100.0;

        let mut player = self.player;
        self.base
            .on_tap
            .subscribe(move |pos| player.weapon.shoot_at(pos));
    }

    fn setup_enemies(&mut self) {
        let mut enemy = Unit::make((0, 10, 2, 2).into(), self.rglica());
        enemy.set_image(Assets::image("chmonya.png"));
        enemy.enable_collision_detection();
        enemy.data_mut().on_collision.subscribe(|sprite| {
            dbg!(sprite);
        });
        self.add_sprite(enemy);
    }

    fn make_walls(&mut self) {
        // let square =
        // Image::load(&test_engine::paths::images().join("square.png"));
        //
        // let width = 280;
        // let wall_width = 10;
        //
        // self.add_wall((0, 0, width, wall_width).into())
        //     .set_image(square.clone());
        //
        // self.add_wall((-width, width, wall_width, width).into())
        //     .set_image(square.clone());
        //
        // self.add_wall((width, width, wall_width, width).into())
        //     .set_image(square.clone());
        //
        // self.add_wall((50, 0, 5, 100).into())
        //     .set_image(square.clone());
        //
        // self.add_wall((-50, 0, 5, 100).into()).set_image(square);
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
            let mut wall = add_sprite::<Wall>(visited_frame(x, y), self);
            wall.set_color(Color::BLACK);
            self.cells.push(wall);
        }

        cell.all_sides(|side| {
            let mut wall = add_sprite::<Wall>(frame_for_side(side, x, y), self);
            wall.set_color(Color::BLACK);
            self.cells.push(wall);
        })
    }
}

const BIG: f32 = 100.0;
const SMALL: f32 = 2.0;

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
