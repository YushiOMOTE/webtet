use crate::sprite::{Map, Sprite};
use quicksilver::graphics::Color;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub enum Tetrimino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Distribution<Tetrimino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetrimino {
        match rng.gen_range(0, 7) {
            0 => Tetrimino::I,
            1 => Tetrimino::O,
            2 => Tetrimino::T,
            3 => Tetrimino::J,
            4 => Tetrimino::L,
            5 => Tetrimino::S,
            6 => Tetrimino::Z,
            _ => unreachable!(),
        }
    }
}

pub fn gen_tetrimino() -> Tetrimino {
    rand::random()
}

fn make_map(map: Vec<Vec<bool>>, color: Color) -> Map<Color> {
    let map = map
        .iter()
        .map(|i| {
            i.iter()
                .map(|b| if *b { color } else { color.with_alpha(0.0) })
                .collect()
        })
        .collect();
    Map::new(map)
}

impl From<Tetrimino> for Sprite<Color> {
    fn from(t: Tetrimino) -> Self {
        let (map, axis, color) = match t {
            Tetrimino::I => (vec![vec![true, true, true, true]], (1.0, 0.0), Color::CYAN),
            Tetrimino::O => (
                vec![vec![true, true], vec![true, true]],
                (0.5, 0.5),
                Color::YELLOW,
            ),
            Tetrimino::T => (
                vec![vec![true, true, true], vec![false, true, false]],
                (1.0, 1.0),
                Color::MAGENTA,
            ),
            Tetrimino::J => (
                vec![vec![true, true, true], vec![false, false, true]],
                (1.0, 0.0),
                Color::BLUE,
            ),
            Tetrimino::L => (
                vec![vec![false, false, true], vec![true, true, true]],
                (1.0, 1.0),
                Color::ORANGE,
            ),
            Tetrimino::S => (
                vec![vec![false, true, true], vec![true, true, false]],
                (1.0, 1.0),
                Color::GREEN,
            ),
            Tetrimino::Z => (
                vec![vec![true, true, false], vec![false, true, true]],
                (1.0, 1.0),
                Color::RED,
            ),
        };

        Sprite::new_with_axis(0.0, 0.0, make_map(map, color), axis)
    }
}
