use lux_camp_badge::led::{Animation, LedMatrix};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use smart_leds_trait::{SmartLedsWrite, RGB, RGB8};
use std::time::Duration;

pub struct Gol<Color, const X: usize, const Y: usize> {
    rng: SmallRng,
    cells: [[bool; Y]; X],
    color: Color,
    alive: bool,
    generations: usize,
    max_generations: Option<usize>,
    start_probability: f64,
    frame_rate: Option<Duration>,
}

/// `max_generations` defines how many generation a playout has before the
/// game will reset. This is useful in that if the game reaches oscillators,
/// the playout won't just last forever.
///
/// `start_probability` is the probably of a cell being alive at the start.
///
/// `frame_rate` is the refresh rate; defaults to matrix FPS rate if `None`.
impl<Color, const X: usize, const Y: usize> Gol<Color, X, Y> {
    pub fn build<Matrix, Driver>(
        prng_seed: u64,
        start_probability: f64,
        max_generations: Option<usize>,
        frame_rate: Option<Duration>,
    ) -> Box<dyn Animation<Matrix> + Send>
    where
        Matrix: LedMatrix<Driver = Driver>,
        Driver: SmartLedsWrite<Color = Color>,
        Color: Default + Copy + Send + 'static,
        Gol<Color, X, Y>: Animation<Matrix>,
    {
        Box::new(Self {
            rng: SmallRng::seed_from_u64(prng_seed),
            cells: [[false; Y]; X],
            color: Default::default(),
            alive: true,
            generations: 0,
            max_generations,
            start_probability,
            frame_rate,
        })
    }
}

impl<Color, const X: usize, const Y: usize> Gol<Color, X, Y> {
    pub fn neighbors<C>(&self, x: usize, y: usize) -> usize
    where
        C: LedMatrix,
    {
        let mut alive = 0;

        if self.cells[y][if x > 0 { x - 1 } else { X - 1 }] {
            alive += 1
        }
        if self.cells[y][if x < X - 1 { x + 1 } else { 0 }] {
            alive += 1
        }
        if self.cells[if y > 0 { y - 1 } else { Y - 1 }][x] {
            alive += 1
        }
        if self.cells[if y < Y - 1 { y + 1 } else { 0 }][x] {
            alive += 1
        }
        if self.cells[if y > 0 { y - 1 } else { Y - 1 }][if x > 0 { x - 1 } else { X - 1 }] {
            alive += 1
        }
        if self.cells[if y < Y - 1 { y + 1 } else { 0 }][if x > 0 { x - 1 } else { X - 1 }] {
            alive += 1
        }
        if self.cells[if y > 0 { y - 1 } else { Y - 1 }][if x < X - 1 { x + 1 } else { 0 }] {
            alive += 1
        }
        if self.cells[if y < Y - 1 { y + 1 } else { 0 }][if x < X - 1 { x + 1 } else { 0 }] {
            alive += 1
        }

        alive
    }
}

impl<const X: usize, const Y: usize, B, C: LedMatrix<Driver = B>> Animation<C>
    for Gol<RGB<u8>, X, Y>
where
    B: SmartLedsWrite<Color = RGB<u8>>,
{
    fn init(&mut self, matrix: &mut C) -> Option<Duration> {
        self.alive = true;
        self.generations = 0;
        self.color = RGB8::new(
            self.rng.gen_range(0..255),
            self.rng.gen_range(0..255),
            self.rng.gen_range(0..255),
        );

        for y in 0..<C as LedMatrix>::Y {
            for x in 0..<C as LedMatrix>::X {
                let cell = if self.rng.gen_bool(self.start_probability) {
                    self.cells[y][x] = true;
                    self.color.into()
                } else {
                    self.cells[y][x] = false;
                    None
                };
                matrix.set_2d(x, y, &cell.unwrap_or_default());
            }
        }

        self.frame_rate
    }

    fn update(&mut self, _tick: Duration, matrix: &mut C) {
        if !self.alive || self.max_generations.is_some_and(|n| self.generations >= n) {
            self.init(matrix);
            return;
        }

        self.alive = false;
        let mut next_gen = [[false; Y]; X];
        for y in 0..<C as LedMatrix>::Y {
            for x in 0..<C as LedMatrix>::X {
                let new_state = match self.neighbors::<C>(x, y) {
                    3 => Some(self.color),
                    2 if self.cells[y][x] => Some(self.color),
                    _ => None,
                };
                next_gen[y][x] = new_state.is_some();
                self.alive |= self.cells[y][x] != next_gen[y][x];
                matrix.set_2d(x, y, &new_state.unwrap_or_default());
            }
        }

        self.cells = next_gen;
        self.generations += 1;
    }
}
