use std::collections::{LinkedList, HashSet};
use rand::{thread_rng, Rng};

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
    bounds: (usize, usize, usize, usize),
    food: (usize, usize),
    nutrition: f32,
    speed: f32,
    progress: f32,
    length: f32,
    direction: Direction,
    snake: LinkedList<(usize, usize)>,
    occupied: HashSet<(usize, usize)>,
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
        let delta = delta as usize;
        let (x, y) = self.snake.front().unwrap().clone();
        let new_pos = match self.direction {
            Direction::Up => (x, y + delta),
            Direction::Down => (x, y - delta),
            Direction::Left => (x - delta, y),
            Direction::Right => (x + delta, y),
        };
        let (top, bottom, left, right) = self.bounds;
        let (x_p, y_p) = new_pos;
        if x_p >= right || x_p <= left || y_p <= bottom || y_p >= top {
            self.status = Status::Dead;
            return;
        }
        self.snake.push_front(new_pos);
        self.occupied.insert(new_pos);
        if new_pos == self.food {
            self.length += self.nutrition;
            loop {
                let new_y: usize = thread_rng().gen_range(bottom..top);
                let new_x: usize = thread_rng().gen_range(left..right);
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
}

fn main() {
    let mut snake: LinkedList<(usize, usize)> = LinkedList::new();
    let start = (1, 2);
    let tail = (1, 1);
    snake.push_back(start);
    snake.push_back(tail);
    let mut occupied: HashSet<(usize, usize)> = HashSet::new();
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
    for _ in 0..10 {
        println!("{:?}", &state);
        state.step_forward();
    }
}
