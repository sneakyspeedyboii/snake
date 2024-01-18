use std::thread::sleep;
use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIN_X: u32 = 500;
const WIN_Y: u32 = 500;

const MAP_X_BOUND: u32 = 10;
const MAP_Y_BOUND: u32 = 10;

const UPDATE_SPEED: u32 = 30;

#[inline] //W performance gains because they were really needed
pub fn main() -> Result<(), String> {
    //Render init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut rand = rand::thread_rng();

    let window = video_subsystem
        .window("Snake", WIN_X, WIN_Y)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut tick = 0;

    //Player init
    let mut body_positions: Vec<(u32, u32)> = vec![(MAP_X_BOUND / 2, MAP_Y_BOUND / 2)];
    let mut player_facing = PlayerFacing::East;

    //Cryptographically secure apple placement
    let mut apple = (
        rand.gen_range(1..=MAP_X_BOUND),
        rand.gen_range(1..=MAP_Y_BOUND),
    );

    'main: loop {
        //Clear canvas with specified colour
        canvas.set_draw_color(Color::RGB(40, 44, 52));
        canvas.clear();

        //Kill program if close requested
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        //User input check
        for key in event_pump.keyboard_state().pressed_scancodes() {
            if key == Scancode::Up {
                player_facing = PlayerFacing::North;
            }
            if key == Scancode::Right {
                player_facing = PlayerFacing::East;
            }
            if key == Scancode::Down {
                player_facing = PlayerFacing::South;
            }
            if key == Scancode::Left {
                player_facing = PlayerFacing::West;
            }

            if key == Scancode::Escape {
                break 'main;
            }
        }

        //Allows for user input every UPDATE_SPEEDms, while snake speed can be controlled via the UPDATE_SPEED constant
        if tick % UPDATE_SPEED == 0 {
            //Clone head and add new position
            let mut new_head_pos = body_positions[0];

            //Ugly - no ternary
            match player_facing {
                PlayerFacing::North => {
                    if new_head_pos.1 == 1 {
                        new_head_pos.1 = MAP_Y_BOUND
                    } else {
                        new_head_pos.1 -= 1
                    }
                }
                PlayerFacing::East => {
                    if new_head_pos.0 == MAP_X_BOUND {
                        new_head_pos.0 = 1
                    } else {
                        new_head_pos.0 += 1
                    }
                }
                PlayerFacing::South => {
                    if new_head_pos.1 == MAP_Y_BOUND {
                        new_head_pos.1 = 1
                    } else {
                        new_head_pos.1 += 1
                    }
                }
                PlayerFacing::West => {
                    if new_head_pos.0 == 1 {
                        new_head_pos.0 = MAP_X_BOUND
                    } else {
                        new_head_pos.0 -= 1
                    }
                }
            }

            //Insert head into body positions array
            body_positions.insert(0, new_head_pos);

            //If head on apple, then do not pop last value, to increase snake size, then reposition apple
            if body_positions[0].0 == apple.0 && body_positions[0].1 == apple.1 {
                //Check if user beat the game
                if body_positions.len() as u32 == MAP_X_BOUND * MAP_Y_BOUND {
                    panic!("HOORAY");
                }

                let mut new_pos;

                //Check generated apple pos is not in the body of the snake
                'apple_placement: loop {
                    new_pos = (
                        rand.gen_range(1..=MAP_X_BOUND),
                        rand.gen_range(1..=MAP_Y_BOUND),
                    );

                    for item in body_positions.clone() {
                        if new_pos == item {
                            continue 'apple_placement;
                        }
                    }

                    break;
                }

                apple = new_pos;
            } else {
                body_positions.pop();
            }

            //Game end checks
            for (segment_index, (segment_x, segment_y)) in body_positions.clone().iter().enumerate()
            {
                for (other_segment_index, (other_segment_x, other_segment_y)) in
                    body_positions.clone().iter().enumerate()
                {
                    if segment_x == other_segment_x
                        && segment_y == other_segment_y
                        && segment_index != other_segment_index
                    {
                        panic!("Snake collided with itself. W game end strat");
                    }
                }
            }

            //Render
            let x_box_size = WIN_X / MAP_X_BOUND;
            let y_box_size = WIN_Y / MAP_Y_BOUND;

            for x in 1..=MAP_X_BOUND {
                for y in 1..=MAP_Y_BOUND {
                    for (segment_x, segment_y) in body_positions.clone() {
                        if segment_x == x && segment_y == y {
                            let x_segment_size = x_box_size / 10 * 8;
                            let y_segment_size = y_box_size / 10 * 8;

                            canvas.set_draw_color(Color::RGB(255, 0, 0));
                            canvas.fill_rect(Rect::new(
                                ((x - 1) * x_box_size + x_box_size / 10) as i32,
                                ((y - 1) * y_box_size + y_box_size / 10) as i32,
                                x_segment_size,
                                y_segment_size,
                            ))?;
                        }
                    }

                    if apple.0 == x && apple.1 == y {
                        let x_segment_size = x_box_size / 10 * 8;
                        let y_segment_size = y_box_size / 10 * 8;

                        canvas.set_draw_color(Color::RGB(0, 255, 0));
                        canvas.fill_rect(Rect::new(
                            ((x - 1) * x_box_size + x_box_size / 10) as i32,
                            ((y - 1) * y_box_size + y_box_size / 10) as i32,
                            x_segment_size,
                            y_segment_size,
                        ))?;
                    }
                }
            }

            canvas.present();
        }

        tick += 1;
        sleep(Duration::from_millis(10));
    }
    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PlayerFacing {
    North,
    East,
    South,
    West,
}


//havent read assign brief yet - probably requires improvements
// implement this for improvements or smth idk

// add sdl2 text lib for easy text rendering - grab silkscreen ttf/otf files

// Start screen:
//  Game title
//  High score
//  Previous score
//  Start game
//  Exit game


// Main game:
//  Basically done
//  Add snake length to reach
//  Add current snake length

// Pause screen
//  Snake length to reach
//  Current snake length
//  Main menu
//  Exit game

// Dead screen - temp screen 3s??
//  Show score reached
//  New high score???