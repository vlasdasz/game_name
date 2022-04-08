use test_engine::{
    assets::ImageManager,
    gm::volume::GyroData,
    rtools::{Rglica, ToRglica},
    sprites::{add_sprite, Control, Player, SpriteData, Unit},
    Image, Level, LevelBase, Sprite,
};

#[derive(Default, Debug)]
pub struct GameLevel {
    base:        LevelBase,
    gyro_sprite: Rglica<SpriteData>,
    pub player:  Rglica<Player>,
}

impl Level for GameLevel {
    fn setup(&mut self) {
        self.set_scale(2.0);
        self.make_walls();
        self.setup_player();
        self.setup_enemies();

        let gyro = SpriteData::make((2, 0.8).into(), (0, 20).into(), self.rglica());
        let rglyro = gyro.to_rglica();
        self.add_sprite(gyro);
        self.gyro_sprite = rglyro;
        self.gyro_sprite.set_image(Image::get("arrow.png"));
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
        self.player = add_sprite((2, 2), (0, 5), self);

        self.player.set_image(Image::get("frisk.png"));

        self.player.weapon.set_image(Image::get("ak.png"));
        self.player.weapon.bullet_image = Image::get("bullet.png").into();
        self.player.weapon.bullet_speed = 100.0;

        let mut player = self.player;
        self.base
            .on_tap
            .subscribe(move |pos| player.weapon.shoot_at(pos));
    }

    fn setup_enemies(&mut self) {
        let mut enemy = Unit::make((2, 2).into(), (0, 10).into(), self.rglica());
        enemy.set_image(Image::get("chmonya.png"));
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
}
