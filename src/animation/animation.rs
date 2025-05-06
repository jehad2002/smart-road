use rand::seq::SliceRandom;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use smartroad::vehicle::*;
// use std::collections::HashMap;
use std::time::Duration;

const WEST_RIGHT: i32 = 278;
const WEST_STRAIGHT: i32 = 320;
const WEST_LEFT: i32 = 362;

const EAST_RIGHT: i32 = 494;
const EAST_STRAIGHT: i32 = 452;
const EAST_LEFT: i32 = 410;

const SOUTH_RIGHT: i32 = 350;
const SOUTH_STRAIGHT: i32 = 405;
const SOUTH_LEFT: i32 = 460;

const NORTH_LEFT: i32 = 515;
const NORTH_STRAIGHT: i32 = 570;
const NORTH_RIGHT: i32 = 625;

const MIN_DISTANCE: f64 = 300.0;

const ACCELERATION_VELOCITY: f32 = 10.0;
const DECELERATION_VELOCITY: f32 = 1.0;
const NORMAL_VELOCITY: f32 = 5.0;


fn calculate_distance(point1: &Point, point2: &Point) -> f64 {
    let dx = point1.x - point2.x;
    let dy = point1.y - point2.y;
    ((dx * dx + dy * dy) as f64).sqrt()
}

fn is_on_same_lane_and_destination(car1: &Car, car2: &Car) -> bool {
    car1.direction == car2.direction && car1.destination == car2.destination
}

fn init_car<'a>(
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    direction: Direction,
    destination: Destination,
    image_path: &'a str,
    initial_position: Point,
    width: u32,
    height: u32,
    initial_angle: f64,
    h_t: bool,
    priority: i64, 
    _call_close:bool
) -> Result<(Car, sdl2::render::Texture<'a>, Rect), String> {
    let car_texture = texture_creator
        .load_texture(image_path)
        .expect("Err image");
    let car = Car::new(
        initial_position.clone(),
        direction,
        destination,
        NORMAL_VELOCITY,
        initial_angle,
        h_t,
        priority,
        _call_close,
    );
    let car_position = Rect::new(initial_position.x, initial_position.y, width, height);

    Ok((car, car_texture, car_position))
}

pub fn run() -> Result<(), String> {
    println!("intersection!");

    let sdl_context = match sdl2::init() {
        Ok(ctx) => ctx,
        Err(err) => return Err(format!("Err SDL: {}", err)),
    };
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)
        .expect("Errrr image");

    let window = match video_subsystem
        .window("projet rust Zone-01", 1000, 800)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(e) => return Err(format!("Err {}", e)),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(e) => return Err(format!("Err canvas {}", e)),
    };

    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator
        .load_texture("assets/roads/road.jpg")
        .expect("Err route");

    let mut cars: Vec<(Car, sdl2::render::Texture, Rect)> = Vec::new();

    let mut event_pump = match sdl_context.event_pump() {
        Ok(event) => event,
        Err(e) => return Err(format!("Err Events {}", e)),
    };
    let mut _exit = false;
    let mut stats = vec!["Statistiques : V",];

    'zone01: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    _exit = true;
                    break 'zone01;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    _exit = true;
                    break 'zone01;

                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    match keycode {
                        Keycode::Down => {
                            let y = -10;
                            let mut rng = rand::thread_rng();
                            let (x, dest, h_t, img) =
                                match *["r", "s", "l"].choose(&mut rng).unwrap() {
                                    "r" => (
                                        SOUTH_RIGHT,
                                        Destination::Right,
                                        true,
                                        "assets/vehicles/yellow_car_down.png",
                                    ),
                                    "l" => (
                                        SOUTH_LEFT,
                                        Destination::Left,
                                        true,
                                        "assets/vehicles/car-down.png",
                                    ),
                                    _ => (
                                        SOUTH_STRAIGHT,
                                        Destination::Straight,
                                        false,
                                        "assets/vehicles/blue_car_down.png",
                                    ),
                                };
                            let initial_car_position = Point { x, y };
                            let dest_clone = dest.clone();
                            if cars.iter().all(|(car, _, _)| {
                                !is_on_same_lane_and_destination(
                                    &car,
                                    &Car::new(
                                        initial_car_position.clone(),
                                        Direction::South,
                                        dest_clone.clone(),
                                        1.0,
                                        0.0,
                                        h_t,
                                        cars.len().try_into().unwrap(),
                                        false,
                                    ),
                                ) || calculate_distance(&car.coordinate, &initial_car_position)
                                    > MIN_DISTANCE
                            }) {
                                let car_data = init_car(
                                    &texture_creator,
                                    Direction::South,
                                    dest,
                                    img,
                                    initial_car_position,
                                    25,
                                    55,
                                    0.0,
                                    h_t,
                                    cars.len().try_into().unwrap(), 
                                    false,
                                )?;
                                cars.push(car_data);
                            }
                        }
                        Keycode::Up => {
                            let y = 800;
                            let mut rng = rand::thread_rng();
                            let (x, dest, h_t, img) =
                                match *["s", "l", "r"].choose(&mut rng).unwrap() {
                                    "r" => (
                                        NORTH_RIGHT,
                                        Destination::Right,
                                        true,
                                        "assets/vehicles/yellow_car_up.png",
                                    ),
                                    "l" => (
                                        NORTH_LEFT,
                                        Destination::Left,
                                        true,
                                        "assets/vehicles/car-up.png",
                                    ),
                                    _ => (
                                        NORTH_STRAIGHT,
                                        Destination::Straight,
                                        false,
                                        "assets/vehicles/blue_car_up.png",
                                    ),
                                };
                            let initial_car_position = Point { x, y };
                            let dest_clone = dest.clone(); 
                            if cars.iter().all(|(car, _, _)| {
                                !is_on_same_lane_and_destination(
                                    &car,
                                    &Car::new(
                                        initial_car_position.clone(),
                                        Direction::North,
                                        dest_clone.clone(),
                                        1.0,
                                        0.0,
                                        h_t,
                                        cars.len().try_into().unwrap(),
                                        false,
                                    ),
                                ) || calculate_distance(&car.coordinate, &initial_car_position)
                                    > MIN_DISTANCE
                            }) {
                                let car_data = init_car(
                                    &texture_creator,
                                    Direction::North,
                                    dest,
                                    img,
                                    initial_car_position,
                                    25,
                                    55,
                                    0.0,
                                    h_t,
                                    cars.len().try_into().unwrap(), 
                                    false,
                                )?;
                                cars.push(car_data);
                            }
                        }
                        Keycode::Right => {
                            let x = -10;
                            let mut rng = rand::thread_rng();
                            let (y, dest, h_t, img) =
                                match *["l", "r", "s"].choose(&mut rng).unwrap() {
                                    "r" => (
                                        EAST_RIGHT,
                                        Destination::Right,
                                        true,
                                        "assets/vehicles/yellow_car_right.png",
                                    ),
                                    "l" => (
                                        EAST_LEFT,
                                        Destination::Left,
                                        true,
                                        "assets/vehicles/car-right.png",
                                    ),
                                    _ => (
                                        EAST_STRAIGHT,
                                        Destination::Straight,
                                        false,
                                        "assets/vehicles/blue_car_right.png",
                                    ),
                                };
                            let initial_car_position = Point { x, y };
                            let dest_clone = dest.clone(); 
                            if cars.iter().all(|(car, _, _)| {
                                !is_on_same_lane_and_destination(
                                    &car,
                                    &Car::new(
                                        initial_car_position.clone(),
                                        Direction::East,
                                        dest_clone.clone(),
                                        1.0,
                                        0.0,
                                        h_t,
                                        cars.len().try_into().unwrap(),
                                        false,
                                    ),
                                ) || calculate_distance(&car.coordinate, &initial_car_position)
                                    > MIN_DISTANCE
                            }) {
                                let car_data = init_car(
                                    &texture_creator,
                                    Direction::East,
                                    dest,
                                    img,
                                    initial_car_position,
                                    55,
                                    25,
                                    0.0,
                                    h_t,
                                    cars.len().try_into().unwrap(), 
                                    false,
                                )?;
                                cars.push(car_data);
                            }
                        }
                        Keycode::Left => {
                            let x = 1000;
                            let mut rng = rand::thread_rng();
                            let (y, dest, h_t, img) =
                                match *["s", "r", "l"].choose(&mut rng).unwrap() {
                                    "l" => (
                                        WEST_LEFT,
                                        Destination::Left,
                                        true,
                                        "assets/vehicles/car-left.png",
                                    ),
                                    "r" => (
                                        WEST_RIGHT,
                                        Destination::Right,
                                        true,
                                        "assets/vehicles/yellow_car_left.png",
                                    ),
                                    _ => (
                                        WEST_STRAIGHT,
                                        Destination::Straight,
                                        false,
                                        "assets/vehicles/blue_car_left.png",
                                    ),
                                };
                            let initial_car_position = Point { x, y };
                            let dest_clone = dest.clone(); 
                            if cars.iter().all(|(car, _, _)| {
                                !is_on_same_lane_and_destination(
                                    &car,
                                    &Car::new(
                                        initial_car_position.clone(),
                                        Direction::West,
                                        dest_clone.clone(),
                                        1.0,
                                        0.0,
                                        h_t,
                                        cars.len().try_into().unwrap(),
                                        false,
                                    ),
                                ) || calculate_distance(&car.coordinate, &initial_car_position)
                                    > MIN_DISTANCE
                            }) {
                                let car_data = init_car(
                                    &texture_creator,
                                    Direction::West,
                                    dest,
                                    img,
                                    initial_car_position,
                                    55,
                                    25,
                                    0.0,
                                    h_t,
                                    cars.len().try_into().unwrap(),
                                    false,
                                )?;
                                cars.push(car_data);
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyUp {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let mut rng = rand::thread_rng();
                    let mut x = -10;
                    let mut y = -10;
                    let mut width = 25;
                    let mut height = 55;
                    let mut img = "assets/vehicles/blue_car_down.png";
                    let mut h_t: bool = true;
                    let mut direction: Direction = Direction::South;
                    let mut destination: Destination = Destination::Straight;
                    match *["e", "w", "n", "s"].choose(&mut rng).unwrap() {
                        "e" => {
                            direction = Direction::East;
                            width = 55;
                            height = 25;
                            match *["l", "r", "s", "r"].choose(&mut rng).unwrap() {
                                "r" => {
                                    destination = Destination::Right;
                                    y = EAST_RIGHT;
                                    img = "assets/vehicles/yellow_car_right.png";
                                }
                                "l" => {
                                    destination = Destination::Left;
                                    y = EAST_LEFT;
                                    img = "assets/vehicles/car-right.png";
                                }
                                _ => {
                                    y = EAST_STRAIGHT;
                                    h_t = false;
                                    img = "assets/vehicles/blue_car_right.png";
                                }
                            };
                            let initial_car_position = Point { x, y };
                            if cars.iter().all(|(car, _, _)| {
                                !is_on_same_lane_and_destination(
                                    &car,
                                    &Car::new(
                                        initial_car_position.clone(),
                                        direction.clone(),
                                        destination.clone(),
                                        1.0,
                                        0.0,
                                        h_t,
                                        cars.len().try_into().unwrap(),
                                        false,
                                    ),
                                ) || calculate_distance(&car.coordinate, &initial_car_position)
                                    > MIN_DISTANCE
                            }) {
                                let car_data = init_car(
                                    &texture_creator,
                                    direction.clone(),
                                    destination.clone(),
                                    img,
                                    initial_car_position,
                                    width,
                                    height,
                                    0.0,
                                    h_t,
                                    cars.len().try_into().unwrap(),
                                    false,
                                )?;
                                cars.push(car_data);
                            }
                        }
                        "w" => {
                            direction = Direction::West;
                            x = 1000;
                            width = 55;
                            height = 25;
                            match *["l", "r", "s", "r"].choose(&mut rng).unwrap() {
                                "r" => {
                                    destination = Destination::Right;
                                    y = WEST_RIGHT;
                                    img = "assets/vehicles/yellow_car_left.png";
                                }
                                "l" => {
                                    destination = Destination::Left;
                                    y = WEST_LEFT;
                                    img = "assets/vehicles/car-left.png";
                                }
                                _ => {
                                    y = WEST_STRAIGHT;
                                    h_t = false;
                                    img = "assets/vehicles/blue_car_left.png";
                                }
                            };
                        }
                        "n" => {
                            direction = Direction::North;
                            y = 800;
                            width = 25;
                            height = 55;
                            match *["l", "r", "s", "r"].choose(&mut rng).unwrap() {
                                "r" => {
                                    destination = Destination::Right;
                                    x = NORTH_RIGHT;
                                    img = "assets/vehicles/yellow_car_up.png";
                                }
                                "l" => {
                                    destination = Destination::Left;
                                    x = NORTH_LEFT;
                                    img = "assets/vehicles/car-up.png";
                                }
                                _ => {
                                    x = NORTH_STRAIGHT;
                                    h_t = false;
                                    img = "assets/vehicles/blue_car_up.png";
                                }
                            };
                        }
                        _ => {
                            match *["l", "r", "s", "r"].choose(&mut rng).unwrap() {
                                "r" => {
                                    destination = Destination::Right;
                                    x = SOUTH_RIGHT;
                                    img = "assets/vehicles/yellow_car_down.png";
                                }
                                "l" => {
                                    destination = Destination::Left;
                                    x = SOUTH_LEFT;
                                    img = "assets/vehicles/car-down.png";
                                }
                                _ => {
                                    x = SOUTH_STRAIGHT;
                                    h_t = false;
                                }
                            };
                        }
                    };
                    let initial_car_position = Point { x, y };
                    if cars.iter().all(|(car, _, _)| {
                        !is_on_same_lane_and_destination(
                            &car,
                            &Car::new(
                                initial_car_position.clone(),
                                direction.clone(),
                                destination.clone(),
                                1.0,
                                0.0,
                                h_t,
                                cars.len().try_into().unwrap(),
                                false,
                            ),
                        ) || calculate_distance(&car.coordinate, &initial_car_position)
                            > MIN_DISTANCE
                    }) {
                        let car_data = init_car(
                            &texture_creator,
                            direction,
                            destination,
                            img,
                            initial_car_position,
                            width,
                            height,
                            0.0,
                            h_t,
                            cars.len().try_into().unwrap(),
                            false,
                        )?;
                        cars.push(car_data);
                    }
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(&background_texture, None, None)?;
        let mut list_cars: Vec<Car> = Vec::new();

        for (car, _, _) in &cars {
            if !car.out_intersection {
                list_cars.push(car.clone());
            }
        }

        for (car, car_texture, car_position) in &mut cars {
            if !car.completed {
                let new_position = car.move_car();
                car_position.set_x(new_position.x);
                car_position.set_y(new_position.y);
                match car.direction {
                    Direction::South => {
                        if new_position.y >= (WEST_RIGHT - 15)
                            && car.destination == Destination::Right
                        {
                            car.angle = 90.0;
                            car.direction = Direction::West;
                            car.destination = Destination::Straight;
                        } else if new_position.y >= (EAST_LEFT - 15)
                            && car.destination == Destination::Left
                        {
                            car.angle = -90.0;
                            car.direction = Direction::East;
                            car.destination = Destination::Straight;
                        }
                    }
                    Direction::East => {
                        if new_position.x >= (SOUTH_RIGHT - 15)
                            && car.destination == Destination::Right
                        {
                            car.angle = 90.0;
                            car.direction = Direction::South;
                            car.destination = Destination::Straight;
                        } else if new_position.x >= (NORTH_LEFT - 15)
                            && car.destination == Destination::Left
                        {
                            car.angle = -90.0;
                            car.direction = Direction::North;
                            car.destination = Destination::Straight;
                        }
                    }
                    Direction::West => {
                        if new_position.x <= (NORTH_RIGHT - 15)
                            && car.destination == Destination::Right
                        {
                            car.angle = 90.0;
                            car.direction = Direction::North;
                            car.destination = Destination::Straight;
                        } else if new_position.x <= (SOUTH_LEFT - 15)
                            && car.destination == Destination::Left
                        {
                            car.angle = -90.0;
                            car.direction = Direction::South;
                            car.destination = Destination::Straight;
                        }
                    }
                    Direction::North => {
                        if new_position.y <= (EAST_RIGHT - 15)
                            && car.destination == Destination::Right
                        {
                            car.angle = 90.0;
                            car.direction = Direction::East;
                            car.destination = Destination::Straight;
                        } else if new_position.y <= (WEST_LEFT - 15)
                            && car.destination == Destination::Left
                        {
                            car.angle = -90.0;
                            car.direction = Direction::West;
                            car.destination = Destination::Straight;
                        }
                    }
                }
                if car.direction == Direction::East && car.destination == Destination::Right
                    || car.direction == Direction::South && car.destination == Destination::Right
                    || car.direction == Direction::North && car.destination == Destination::Right
                    || car.direction == Direction::West && car.destination == Destination::Right
                {
                    car.velocity = NORMAL_VELOCITY;
                } else {
                    if car.out_intersection {
                        car.velocity = NORMAL_VELOCITY;
                    } else if !car.out_intersection && car.in_intersection {
                        car.velocity = ACCELERATION_VELOCITY;
                    } else {
                        let mut is_on_line = true;
                        for col in car.get_car_collision(list_cars.clone()) {
                            if car.distance_before_line() >= col.distance_before_line() {
                                is_on_line = false;
                                break;
                            }
                        }
                        if is_on_line {
                            car.velocity = ACCELERATION_VELOCITY;
                        } else {
                            car._call_close=true;
                            car.velocity = DECELERATION_VELOCITY;
                        }
                    }
                }

                canvas.copy_ex(
                    car_texture,
                    None,
                    *car_position,
                    car.angle,
                    None,
                    false,
                    false,
                )?;
            }
            // if car.out_intersection{
            //     nbr_car += 1;
            // }
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));

        
    }
    let cars_completed = cars.into_iter().filter(|(car, _, _)| car.completed).collect::<Vec<(Car, sdl2::render::Texture, Rect)>>();
    let car_min_time = cars_completed.iter().map(|(car, _, _)| car).min_by_key(|car| car.time);
    let car_max_time = cars_completed.iter().map(|(car, _, _)| car).max_by_key(|car| car.time);
    let (car_min_velocity, car_max_velocity) = match min_max_velocity(&cars_completed) {
        Some((min, max)) => (min, max),
        None => (0.0, 0.0),
    };    
    let value = format!("Max nbr vehicles: {:?}", cars_completed.iter().filter(|(car, _, _)| car.out_intersection).count());
    let _call_close = format!("call close: {:?}", cars_completed.iter().filter(|(car, _, _)| car._call_close).count());

    let min_time = match car_min_time {
        Some(time) => format!("min time: {:?} ms", time.time),
        None => "min time: 0 ms".to_string(),
    };
    
    let max_time = match car_max_time {
        Some(time) => format!("max time: {:?} ms", time.time),
        None => "max time: 0 ms".to_string(),
    };
    let min_velocity =  format!("min velocity: {:?} m/s", car_min_velocity);
    let max_velocity =  format!("max velocity: {:?} m/s", car_max_velocity);
    stats.push(value.as_str());
    stats.push(max_time.as_str());
    stats.push(min_time.as_str());
    stats.push(min_velocity.as_str());
    stats.push(max_velocity.as_str());
    stats.push(_call_close.as_str());
    if _exit {
        
        write_stats(&mut canvas, &background_texture, &stats)?;
        
        'statistic: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'statistic,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'statistic,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

pub fn write_stats(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    background_texture: &sdl2::render::Texture,
    stats: &[&str],
) -> Result<(), String> {
    canvas.clear();
    canvas.copy(background_texture, None, None)?;

    let ttf_context = sdl2::ttf::init().map_err(|e| format!("Err TTF: {}", e))?;

    let font_path = "assets/fonts/font.ttf";
    let font = ttf_context
        .load_font(font_path, 32)
        .map_err(|e| format!("Err police: {}", e))?;

    let mut y_offset = 270;
    let line_spacing = 10;

    for &line in stats.iter() {
        let surface = font
            .render(line)
            .blended(sdl2::pixels::Color::RGBA(255, 255, 255, 255))
            .map_err(|e| format!("Err text: {}", e))?;

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| format!("Err create text: {}", e))?;

        let texture_query = texture.query();
        let dest = sdl2::rect::Rect::new(
            (canvas.viewport().width() - texture_query.width) as i32 / 2,
            y_offset,
            texture_query.width,
            texture_query.height,
        );

        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.fill_rect(dest)?;

        canvas.copy(&texture, None, dest)?;

        y_offset += texture_query.height as i32 + line_spacing;
    }

    canvas.present();

    Ok(())
}

fn min_max_velocity<'a>(cars: &[(Car, sdl2::render::Texture<'a>, Rect)]) -> Option<(f32, f32)> {
    if cars.is_empty() {
        return None;
    }

    let mut min_car_velocity = &cars[0].0;
    let mut max_car_velocity = &cars[0].0;

    for (car, _, _) in cars.iter() {
        if car.velocity < min_car_velocity.velocity {
            min_car_velocity = car;
        }
        if car.velocity > max_car_velocity.velocity {
            max_car_velocity = car;
        }
    }

    Some((min_car_velocity.clone().velocity_in, max_car_velocity.clone().velocity_in))
}
