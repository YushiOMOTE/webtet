mod sprite;
mod tetrimino;

use crate::sprite::{Map, Sprite as S};
use crate::tetrimino::gen_tetrimino;

type Sprite = S<Color>;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

struct Game {
    sprite: Option<Sprite>,
    frozen: Sprite,
    frame: (f32, f32, f32, f32),
    time: usize,
    scale: f32,
    gameover: bool,
}

impl Game {
    fn new(width: f32, height: f32) -> Self {
        let mut game = Self {
            sprite: None,
            frozen: Sprite::new(
                1.0,
                0.0,
                Map::fill(width, height, Color::WHITE.with_alpha(0.0)),
            ),
            frame: (1.0, 0.0, width, height),
            time: 0,
            scale: 20.0,
            gameover: false,
        };

        game.gen_next();

        game
    }

    fn draw(&mut self, window: &mut Window) {
        self.draw_frame(window);

        if let Some(s) = self.sprite.as_ref() {
            draw_sprite(window, s, self.scale);
        }

        draw_sprite(window, &self.frozen, self.scale);
    }

    fn draw_frame(&self, window: &mut Window) {
        window.draw(
            &Rectangle::new(
                (self.frame.0 * self.scale, self.frame.1 * self.scale),
                (self.frame.2 * self.scale, self.frame.3 * self.scale),
            ),
            Col(Color::BLACK.with_alpha(0.5)),
        );
    }

    fn collide(&self, sprite: &Sprite) -> bool {
        if collide(sprite, &self.frozen) {
            return true;
        }

        if sprite.x < self.frame.0 || sprite.x + sprite.width() > self.frame.0 + self.frame.2 {
            return true;
        }
        if sprite.y < self.frame.1 || sprite.y + sprite.height() > self.frame.1 + self.frame.3 {
            return true;
        }

        false
    }

    fn try_update<F: Fn(&mut Sprite)>(&mut self, op: F) -> bool {
        let mut collided = false;

        if let Some(old) = self.sprite.take() {
            let mut new = old.clone();
            op(&mut new);
            self.sprite = if self.collide(&new) {
                collided = true;
                Some(old)
            } else {
                Some(new)
            };
        }

        collided
    }

    fn try_shift(&mut self, x: f32, y: f32) -> bool {
        self.try_update(|s| s.move_by(x, y))
    }

    fn try_rotate_left(&mut self) -> bool {
        self.try_update(|s| s.rotate_left())
    }

    fn try_rotate_right(&mut self) -> bool {
        self.try_update(|s| s.rotate_right())
    }

    fn gen_next(&mut self) -> Sprite {
        Sprite::from(gen_tetrimino()).at(self.frame.2 / 2.0, -1.0)
    }

    fn freeze(&mut self, s: Sprite) {
        for y in 0..(s.height() as usize) {
            for x in 0..(s.width() as usize) {
                let p = s.map.get(x as f32, y as f32);
                if p.a == 0.0 {
                    continue;
                }
                let fx = x as f32 + s.x - self.frozen.x;
                let fy = y as f32 + s.y - self.frozen.y;
                self.frozen.map.set(fx, fy, *p);
            }
        }

        let mut y = self.frozen.height() - 1.0;
        while y >= 0.0 {
            let mut filled = true;
            for x in 0..(self.frozen.width() as usize) {
                if self.frozen.map.get(x as f32, y as f32).a == 0.0 {
                    filled = false;
                    break;
                }
            }
            if filled {
                for yy in (1..=(y as usize)).rev() {
                    for x in 0..(self.frozen.width() as usize) {
                        let p = *self.frozen.map.get(x as f32, yy as f32 - 1.0);
                        self.frozen.map.set(x as f32, yy as f32, p);
                    }
                }
            } else {
                y -= 1.0;
            }
        }
    }
}

fn draw_sprite(window: &mut Window, sprite: &Sprite, scale: f32) {
    for x in 0..(sprite.map.width() as usize) {
        for y in 0..(sprite.map.height() as usize) {
            let col = sprite.map.get(x as f32, y as f32);
            window.draw(
                &Rectangle::new(
                    (
                        (sprite.x as f32 + x as f32) * scale,
                        (sprite.y as f32 + y as f32) * scale,
                    ),
                    (scale, scale),
                ),
                Col(*col),
            );
        }
    }
}

fn collide(s1: &Sprite, s2: &Sprite) -> bool {
    if (s1.x + s1.width() <= s2.x || s2.x + s2.width() <= s1.x)
        && (s1.y + s1.height() <= s2.y || s2.y + s2.height() <= s1.y)
    {
        return false;
    }

    for x in 0..(s1.width() as usize) {
        for y in 0..(s1.height() as usize) {
            let c1 = s1.map.get(x as f32, y as f32);
            let (x2, y2) = (x as f32 + s1.x - s2.x, y as f32 + s1.y - s2.y);
            if x2 < 0.0 || x2 >= s2.width() || y2 < 0.0 || y2 >= s2.height() {
                continue;
            }
            let c2 = s2.map.get(x2 as f32, y2 as f32);
            if c1.a != 0.0 && c2.a != 0.0 {
                return true;
            }
        }
    }

    false
}

impl State for Game {
    fn new() -> Result<Self> {
        Ok(Self::new(10.0, 29.0))
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Up, ButtonState::Pressed) => {
                while self.sprite.is_some() && !self.try_shift(0.0, 1.0) {}
            }
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.try_shift(-1.0, 0.0);
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.try_shift(1.0, 0.0);
            }
            Event::Key(Key::Down, ButtonState::Pressed) => {
                self.try_shift(0.0, 1.0);
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                self.try_rotate_left();
            }
            Event::Key(Key::X, ButtonState::Pressed) => {
                self.try_rotate_right();
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        if self.gameover {
            return Ok(());
        }

        self.time = self.time.wrapping_add(1);

        if self.time % 10 == 0 {
            if self.sprite.is_some() {
                if self.try_shift(0.0, 1.0) {
                    let s = self.sprite.take().unwrap();
                    if s.y == -1.0 {
                        self.gameover = true;
                    } else {
                        self.freeze(s);
                    }
                }
            } else {
                self.sprite = Some(self.gen_next());
            }
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.draw(window);
        Ok(())
    }
}

fn main() {
    run::<Game>("Tetris", Vector::new(800, 600), Settings::default());
}
