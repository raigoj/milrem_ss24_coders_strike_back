use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}



#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Vector2 { x, y }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vector2::new((self.x as f64 * scalar) as i32, (self.y as f64 * scalar) as i32)
    }
}

fn dist_sqr(a: Vector2, b: Vector2) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    dx * dx + dy * dy
}

struct CheckpointManager {
    checkpoints: Vec<Vector2>,
    current_lap: usize,
    checkpoint_index: usize,
    best_boost_index: usize,
}

impl CheckpointManager {
    fn new() -> Self {
        CheckpointManager {
            checkpoints: Vec::new(),
            current_lap: 0,
            checkpoint_index: 0,
            best_boost_index: 0,
        }
    }

    fn should_use_boost(&self) -> bool {
        self.checkpoint_index == self.best_boost_index
    }

    fn update(&mut self, next_checkpoint: Vector2) {
        if self.checkpoints.is_empty() {
            self.checkpoints.push(next_checkpoint);
            return;
        }

        if next_checkpoint == self.checkpoints[self.checkpoint_index] {
            return;
        }

        if next_checkpoint == self.checkpoints[0] {
            if self.current_lap == 0 {
                self.compute_best_boost_index();
            }
            self.current_lap += 1;
            self.checkpoint_index = 0;
        } else {
            self.checkpoint_index += 1;
            if self.current_lap == 0 {
                self.checkpoints.push(next_checkpoint);
            }
        }
    }

    fn compute_best_boost_index(&mut self) {
        if self.checkpoints.len() < 2 {
            return;
        }

        let mut longest_dist = 0.0;
        let mut best_boost_index = 0;

        for i in 0..self.checkpoints.len() {
            let j = (i + 1) % self.checkpoints.len();
            let cur_dist = dist_sqr(self.checkpoints[i], self.checkpoints[j]);

            if cur_dist > longest_dist {
                longest_dist = cur_dist;
                best_boost_index = j;
            }
        }

        self.best_boost_index = best_boost_index;
    }
}

fn main() {
    let mut can_boost = true;
    let mut checkpoints = CheckpointManager::new();
    let mut prev_pos = Vector2::new(-1, -1);

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<i32> = input_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let x = inputs[0];
        let y = inputs[1];
        let next_checkpoint_x = inputs[2];
        let next_checkpoint_y = inputs[3];
        let next_checkpoint_dist = inputs[4];
        let next_checkpoint_angle = inputs[5];

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<i32> = input_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let opponent_x = inputs[0];
        let opponent_y = inputs[1];

        let pos = Vector2::new(x, y);
        let cp = Vector2::new(next_checkpoint_x, next_checkpoint_y);
        let angle = next_checkpoint_angle.abs();

        if prev_pos.x < 0 {
            prev_pos = pos;
        }

        checkpoints.update(cp);

        let mut thrust = 100;
        let mut use_boost = false;

        if angle < 1 {
            thrust = 100;
            use_boost = can_boost && checkpoints.should_use_boost();
        } else {
            let d_pos = pos - prev_pos;
            let target = cp - (Vector2::new(3 * d_pos.x, 3 * d_pos.y));
            let dist_to_cp_sqr = dist_sqr(pos, cp);
            let distance_slowdown_factor = dist_to_cp_sqr.min(360000.0) / 360000.0;
            let angle_slowdown_factor = 1.0 - angle.min(90).max(0) as f64 / 90.0;

            thrust = (100.0 * distance_slowdown_factor * angle_slowdown_factor) as i32;
        }

        prev_pos = pos;
        if use_boost {
            can_boost = false;
        }
        
        let thrust_str = thrust.to_string();
        let thrust_string = if use_boost { "BOOST" } else { &thrust_str };

        println!("{} {} {} {} {} {} {}", next_checkpoint_x, next_checkpoint_y, thrust_string, next_checkpoint_dist, next_checkpoint_angle, opponent_x, opponent_y);
    }
}

