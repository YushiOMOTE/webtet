use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Map<T>(Vec<Vec<T>>);

impl<T> Map<T> {
    pub fn new(map: Vec<Vec<T>>) -> Self {
        Self(map)
    }

    pub fn fill(width: f32, height: f32, default: T) -> Self
    where
        T: Clone,
    {
        Self(vec![vec![default; width as usize]; height as usize])
    }

    pub fn width(&self) -> f32 {
        self.0.get(0).map(|v| v.len()).unwrap_or(0) as f32
    }

    pub fn height(&self) -> f32 {
        self.0.len() as f32
    }

    pub fn get(&self, x: f32, y: f32) -> &T {
        assert!(0.0 <= x && x < self.width());
        assert!(0.0 <= y && y < self.height());
        &(self.0)[y as usize][x as usize]
    }

    pub fn set(&mut self, x: f32, y: f32, v: T) {
        assert!(0.0 <= x && x < self.width());
        assert!(0.0 <= y && y < self.height());

        (self.0)[y as usize][x as usize] = v;
    }
}

#[derive(Debug, Clone)]
pub struct Sprite<T> {
    pub x: f32,
    pub y: f32,
    pub map: Map<T>,
    pub axis: (f32, f32),
}

fn rotate(p: (f32, f32), axis: (f32, f32), v: (f32, f32)) -> (f32, f32) {
    (v.0 * (p.1 - axis.1) + axis.0, v.1 * (p.0 - axis.0) + axis.1)
}

impl<T> Sprite<T> {
    pub fn new(x: f32, y: f32, map: Map<T>) -> Self {
        Self::new_with_axis(x, y, map, (0.0, 0.0))
    }

    pub fn new_with_axis(x: f32, y: f32, map: Map<T>, axisoff: (f32, f32)) -> Self {
        Self {
            x,
            y,
            map,
            axis: (axisoff.0 + x, axisoff.1 + y),
        }
    }

    pub fn at(mut self, x: f32, y: f32) -> Self {
        let axisoff = (self.axis.0 - self.x, self.axis.1 - self.y);
        self.x = x;
        self.y = y;
        self.axis = (axisoff.0 + self.x, axisoff.1 + self.y);
        self
    }

    pub fn width(&self) -> f32 {
        self.map.width()
    }

    pub fn height(&self) -> f32 {
        self.map.height()
    }

    pub fn move_by(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.axis.0 += x;
        self.axis.1 += y;
    }

    pub fn rotate_left(&mut self)
    where
        T: Clone + Default + Debug,
    {
        self.rotate_left_with_axis(self.axis)
    }

    pub fn rotate_right(&mut self)
    where
        T: Clone + Default + Debug,
    {
        self.rotate_right_with_axis(self.axis)
    }

    pub fn rotate_left_with_axis(&mut self, axis: (f32, f32))
    where
        T: Clone + Default + Debug,
    {
        self.rotate(axis, (1.0, -1.0), (1.0, 0.0));
    }

    pub fn rotate_right_with_axis(&mut self, axis: (f32, f32))
    where
        T: Clone + Default + Debug,
    {
        self.rotate(axis, (-1.0, 1.0), (0.0, 1.0));
    }

    fn rotate(&mut self, axis: (f32, f32), v: (f32, f32), pos: (f32, f32))
    where
        T: Clone + Default + Debug,
    {
        // Calculate new position
        let (x, y) = (
            self.x + pos.0 * (self.map.width() - 1.0),
            self.y + pos.1 * (self.map.height() - 1.0),
        );
        let (nx, ny) = rotate((x, y), axis, v);

        // Calculate new size
        let (nw, nh) = (self.map.height(), self.map.width());
        let mut map = vec![vec![T::default(); nw as usize]; nh as usize];

        for (x, y, p) in self
            .map
            .0
            .iter()
            .enumerate()
            .map(|(y, i)| i.iter().enumerate().map(move |(x, p)| (x, y, p)))
            .flatten()
        {
            // To absolute position
            let (x, y) = (x as f32 + self.x, y as f32 + self.y);
            // Rotate
            let (x, y) = rotate((x, y), axis, v);
            // To relative position
            let (x, y) = ((x - nx) as usize, (y - ny) as usize);
            map[y as usize][x as usize] = p.clone();
        }

        self.x = nx;
        self.y = ny;
        self.map.0 = map;
    }
}
