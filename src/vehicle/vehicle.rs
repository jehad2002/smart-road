#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    West,
    South,
    North,
    East,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Destination {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone)]
pub struct Car {
    pub coordinate: Point,
    pub direction: Direction,
    pub destination: Destination,
    pub velocity: f32,
    pub angle: f64,
    pub completed: bool,
    pub in_intersection: bool,
    pub out_intersection: bool,
    pub have_turn: bool,
    pub priority: i64,
    pub _call_close:bool,
    pub time: u128,
    pub entry_time: Option<u128>,
    pub exit_time: Option<u128>,
    pub velocity_in: f32,
}

impl Car {
    pub fn new(
        c: Point,
        d: Direction,
        dest: Destination,
        v: f32,
        a: f64,
        h_t: bool,
        priority: i64,
        _call_close:bool
    ) -> Car {
        Car {
            coordinate: c,
            direction: d,
            destination: dest,
            velocity: v,
            angle: a,
            completed: false,
            in_intersection: false,
            out_intersection: false,
            have_turn: h_t,
            priority: priority,
            _call_close:false,
            time: 0 as u128,
            entry_time: Some(0 as u128),
            exit_time: Some(0 as u128),
            velocity_in: 0.0,
        }
    }

    pub fn accelerate(&mut self) {
        if self.velocity < 5.0 {
            self.velocity += 0.5;
        }
    }

    pub fn decelerate(&mut self) {
        if self.velocity > 0.0 {
            self.velocity -= 0.5;
        }
    }

    pub fn move_car(&mut self) -> Point {
        match self.direction {
            Direction::West => {
                self.coordinate.x -= self.velocity as i32;
                if self.coordinate.x <= 665 {
                    if !self.in_intersection {
                        self.entry_time = Some(get_current_time());
                    }
                    self.in_intersection = true;
                }
                if self.coordinate.x <= 280 {
                    if !self.out_intersection{
                        self.exit_time = Some(get_current_time());
                    }
                    self.out_intersection = true;
                }
                if self.coordinate.x <= -30 {
                    self.completed = true;
                    self.time = self.exit_time.unwrap() - self.entry_time.unwrap();
                }
                if self.in_intersection && !self.out_intersection {
                    self.velocity_in = self.velocity;
                }
            }
            Direction::East => {
                self.coordinate.x += self.velocity as i32;
                if self.coordinate.x >= 275 {
                    if !self.in_intersection {
                        self.entry_time = Some(get_current_time());
                    }
                    self.in_intersection = true;
                }
                if self.coordinate.x > 660 {
                    if !self.out_intersection{
                        self.exit_time = Some(get_current_time());
                    }
                    self.out_intersection = true;
                }
                if self.coordinate.x >= 1000 {
                    self.time = self.exit_time.unwrap() - self.entry_time.unwrap();
                    self.completed = true;
                }
                if self.in_intersection && !self.out_intersection {
                    self.velocity_in = self.velocity;
                }
            }
            Direction::North => {
                self.coordinate.y -= self.velocity as i32;

                if self.coordinate.y <= 535 {
                    if !self.in_intersection {
                        self.entry_time = Some(get_current_time());
                    }
                    self.in_intersection = true;
                }
                if self.coordinate.y <= 220 {
                    if !self.out_intersection{
                        self.exit_time = Some(get_current_time());
                    }
                    self.out_intersection = true;
                }
                if self.coordinate.y <= 0 {
                    self.time = self.exit_time.unwrap() - self.entry_time.unwrap();
                    self.completed = true;
                }
                if self.in_intersection && !self.out_intersection {
                    self.velocity_in = self.velocity;
                }
            }
            Direction::South => {
                self.coordinate.y += self.velocity as i32;
                if self.coordinate.y >= 214 {
                    if !self.in_intersection {
                        self.entry_time = Some(get_current_time());
                    }
                    self.in_intersection = true;
                }
                if self.coordinate.y > 545 {
                    if !self.out_intersection{
                        self.exit_time = Some(get_current_time());
                    }
                    self.out_intersection = true;
                }
                if self.coordinate.y >= 800 {
                    self.time = self.exit_time.unwrap() - self.entry_time.unwrap();
                    self.completed = true;
                }
                if self.in_intersection && !self.out_intersection {
                    self.velocity_in = self.velocity;
                }
            }
        }
        
        self.coordinate.clone()
    }

    pub fn get_car_collision<'a>(&self, cars: Vec<Car>) -> Vec<Car> {
        let mut cars_collision = Vec::new();
        for car in cars {
            if !car.out_intersection {
                match (self.direction.clone(), self.destination.clone()) {
                    (Direction::East, Destination::Straight) => {
                        if (car.direction == Direction::South
                            && car.destination == Destination::Straight
                            && car.coordinate.x > 360)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Straight
                                && !car.have_turn)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Left)
                        //    Pourront ignorer ce test
                        //    (car.direction == Direction::West && car.destination == Destination::Straight && car.have_turn)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::East, Destination::Left) => {
                        if (car.direction == Direction::South
                            && car.destination == Destination::Straight
                            && car.coordinate.x > 360)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Straight
                                && car.coordinate.y > 278)
                        //    Pourront ignorer ce test
                        //    (car.direction == Direction::East && car.destination == Destination::Straight && car.have_turn)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::South, Destination::Straight) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Straight
                            && !car.have_turn)
                            || (car.direction == Direction::East
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Straight
                                && car.coordinate.y > 270)
                        //    Pourront ignorer ce test
                        //    (car.direction == Direction::North && car.destination == Destination::Straight && car.have_turn)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::South, Destination::Left) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Straight
                                && car.coordinate.x < 610)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Straight
                                && car.coordinate.y > 290)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Left)
                        //    Pourront ignorer ce test
                        //    (car.direction == Direction::South && car.destination == Destination::Straight && car.coordinate.x >= 440)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::West, Destination::Straight) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Left)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Straight
                                && !car.have_turn)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Straight
                                && car.coordinate.x < 600)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::West, Destination::Left) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Straight
                            && car.coordinate.y < 470)
                            || (car.direction == Direction::East
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Straight
                                && car.coordinate.x < 600)
                            || (car.direction == Direction::North
                                && car.destination == Destination::Left)
                        //    Cant wait
                        //    (car.direction == Direction::West && car.destination == Destination::Straight && car.have_turn)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::North, Destination::Straight) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Straight
                            && car.coordinate.y < 470)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Straight
                                && !car.have_turn)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Left)
                        {
                            cars_collision.push(car);
                        }
                    }
                    (Direction::North, Destination::Left) => {
                        if (car.direction == Direction::East
                            && car.destination == Destination::Left)
                            || (car.direction == Direction::East
                                && car.destination == Destination::Straight
                                && car.coordinate.y < 470)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Left)
                            || (car.direction == Direction::South
                                && car.destination == Destination::Straight
                                && car.coordinate.x > 350)
                            || (car.direction == Direction::West
                                && car.destination == Destination::Left)
                        //    (car.direction == Direction::North && car.destination == Destination::Straight && car.have_turn)
                        {
                            cars_collision.push(car);
                        }
                    }
                    _ => {}
                }
            }
        }
        cars_collision
    }

    pub fn distance_before_line(&self) -> i32 {
        match self.direction {
            Direction::East => 274 - self.coordinate.x,
            Direction::West => self.coordinate.x - 665,
            Direction::North => self.coordinate.y - 535,
            Direction::South => 214 - self.coordinate.y,
        }
    }
}


fn get_current_time() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}