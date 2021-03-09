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
    speed: i32,
    length: usize,
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
        let (x, y) = self.snake.front().unwrap().clone();
        let new_pos = match self.direction {
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
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
            self.length += 1;
            loop {
                let new_y: usize = thread_rng().gen_range(bottom..top);
                let new_x: usize = thread_rng().gen_range(left..right);
                if !self.occupied.contains(&(new_x, new_y)) {
                    self.food = (new_x, new_y);
                    break;
                };
            }
        }
        if self.snake.len() > self.length {
            let tail = self.snake.pop_back().unwrap();
            self.occupied.remove(&tail);
        }
    }
}

fn main() {
    let mut snake: LinkedList<(usize, usize)> = LinkedList::new();
    let start = (1, 2);
    snake.push_front(start);
    let mut occupied: HashSet<(usize, usize)> = HashSet::new();
    occupied.insert(start);
    let mut state = State {
        status: Status::Alive,
        bounds: (10, 0, 0, 10),
        food: (1, 3),
        speed: 1,
        length: 1,
        direction: Direction::Up,
        snake,
        occupied,
    };
    for _ in 0..10 {
        println!("{:?}", &state);
        state.step_forward();
    }
}
