use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::collections::{HashSet, LinkedList};
use std::iter::Scan;
use std::time::{Duration, Instant};

macro_rules! rect (
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32))
);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct State {
    status: Status,
    bounds: (i32, i32, i32, i32),
    food: (i32, i32),
    nutrition: f32,
    speed: f32,
    progress: f32,
    length: f32,
    direction: Direction,
    snake: LinkedList<(i32, i32)>,
    occupied: HashSet<(i32, i32)>,
}

#[derive(Debug)]
enum Status {
    Alive,
    Dead,
}

impl State {
    fn step_forward(&mut self) {
        if let Status::Dead = self.status {
            return;
        }
        self.progress += self.speed;
        let delta = self.progress.floor();
        self.progress -= delta;
        let delta = delta as i32;
        let (x, y) = self.snake.front().unwrap().clone();
        let new_pos = match self.direction {
            Direction::Up => (x, y + delta),
            Direction::Down => (x, y - delta),
            Direction::Left => (x - delta, y),
            Direction::Right => (x + delta, y),
        };
        if self.occupied.contains(&new_pos) {
            self.status = Status::Dead;
            return;
        }
        let (top, bottom, left, right) = self.bounds;
        let (x_p, y_p) = new_pos;
        if x_p > right || x_p < left || y_p < bottom || y_p > top {
            self.status = Status::Dead;
            return;
        }
        self.snake.push_front(new_pos);
        self.occupied.insert(new_pos);
        if new_pos == self.food {
            self.length += self.nutrition;
            loop {
                let new_y: i32 = thread_rng().gen_range(bottom..top);
                let new_x: i32 = thread_rng().gen_range(left..right);
                if !self.occupied.contains(&(new_x, new_y)) {
                    self.food = (new_x, new_y);
                    break;
                };
            }
        }
        if self.snake.len() as f32 > self.length {
            let tail = self.snake.pop_back().unwrap();
            self.occupied.remove(&tail);
        }
    }

    fn render(&self, canvas: &mut WindowCanvas) {
        const BLOCK_SIZE: i32 = 10;
        let (offset_x, offset_y) = (200, 200);
        let (top, bottom, left, right) = self.bounds;

        canvas.fill_rect(rect!(
            offset_x,
            offset_y,
            (right - left + 2) * BLOCK_SIZE,
            BLOCK_SIZE
        ));
        canvas.fill_rect(rect!(
            offset_x,
            offset_y + BLOCK_SIZE,
            BLOCK_SIZE,
            (top - bottom) * BLOCK_SIZE
        ));
        canvas.fill_rect(rect!(
            offset_x + (right - left + 1) * BLOCK_SIZE,
            offset_y + BLOCK_SIZE,
            BLOCK_SIZE,
            (top - bottom) * BLOCK_SIZE
        ));
        canvas.fill_rect(rect!(
            offset_x,
            offset_y + (top - bottom + 1) * BLOCK_SIZE,
            (right - left + 2) * BLOCK_SIZE,
            BLOCK_SIZE
        ));

        for (x, y) in self.snake.iter() {
            canvas.fill_rect(rect!(
                offset_x + BLOCK_SIZE + x * BLOCK_SIZE,
                offset_y + BLOCK_SIZE + y * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE
            ));
        }

        let (x, y) = self.food;
        canvas.fill_rect(rect!(
            offset_x + BLOCK_SIZE + x * BLOCK_SIZE + 1,
            offset_y + BLOCK_SIZE + y * BLOCK_SIZE + 1,
            BLOCK_SIZE - 2,
            BLOCK_SIZE - 2
        ));
    }
}

const DELTA_TIME: u128 = 1000 / 10;

fn main() {
    let mut snake: LinkedList<(i32, i32)> = LinkedList::new();
    let start = (1, 2);
    let tail = (1, 1);
    snake.push_back(start);
    snake.push_back(tail);
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    occupied.insert(start);
    occupied.insert(tail);
    let mut state = State {
        status: Status::Alive,
        bounds: (10, 0, 0, 10),
        food: (1, 3),
        nutrition: 1.0,
        speed: 1.0,
        progress: 0.0,
        length: snake.len() as f32,
        direction: Direction::Up,
        snake,
        occupied,
    };

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let t_0 = Instant::now();
    let mut t: u128 = 0;
    let mut current_time: u128 = 0;
    let mut accumulator: u128 = 0;

    println!("{:?}", state);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::W),
                    ..
                } => {
                    state.direction = Direction::Down;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::A),
                    ..
                } => {
                    state.direction = Direction::Left;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::S),
                    ..
                } => {
                    state.direction = Direction::Up;
                }
                Event::KeyDown {
                    scancode: Some(Scancode::D),
                    ..
                } => {
                    state.direction = Direction::Right;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        let new_time = Instant::now().duration_since(t_0).as_millis();
        let frame_time = new_time - current_time;
        current_time = new_time;
        accumulator += frame_time;

        while accumulator >= DELTA_TIME {
            state.step_forward();
            println!("{:?}", state);
            accumulator -= DELTA_TIME;
            t += DELTA_TIME;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        state.render(&mut canvas);
        canvas.present();
    }
}
