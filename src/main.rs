extern crate regex;

use regex::Regex;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::process::{Command, Stdio};
use std::env;

mod xdo_handler;

fn main() {
    let args: Vec<String> = env::args().collect();
    let acceleration: f32;
    if args.len() > 1 {
        acceleration = args[1].parse::<f32>().unwrap_or(1.0);
    } else {
        acceleration = 1.0;
    }

    let output = Command::new("stdbuf")
        .arg("-o0")
        .arg("libinput")
        .arg("debug-events")
        .stdout(Stdio::piped())
        .spawn()
        .expect("can not exec libinput")
        .stdout
        .expect("libinput has no stdout");

    let mut this_app_mouse_down = false;

    // GESTURE_SWIPE_BEGIN, GESTURE_SWIPE_UPDATE, GESTURE_SWIPE_END
    // event10  GESTURE_SWIPE_UPDATE +3.769s	4  0.25/ 0.48 ( 0.95/ 1.85 unaccelerated)
    
    let mut xdo_handler = xdo_handler::start_handler();

    let mut xsum: f32 = 0.0;
    let mut ysum: f32 = 0.0;
    let pattern = Regex::new(r"[\s]+|/|\(").unwrap();

    for line in io::BufReader::new(output).lines() {
        let line = line.unwrap();
        if let Some(_) = line.find("GESTURE_SWIPE_") {
            let parts: Vec<&str> = pattern.split(&line).filter(|c| !c.is_empty()).collect();
            let action = parts[1];
            let finger = parts[3];
            if finger != "3" {
                if this_app_mouse_down {
                    xdo_handler.mouse_up(1);
                    this_app_mouse_down = false;
                }
                continue;
            }
            match action {
                "GESTURE_SWIPE_BEGIN" => {
                    xsum = 0.0;
                    ysum = 0.0;
                    xdo_handler.mouse_down(1);
                    this_app_mouse_down = true;
                }
                "GESTURE_SWIPE_UPDATE" => {
                    let x: f32 = parts[4].parse().unwrap();
                    let y: f32 = parts[5].parse().unwrap();
                    xsum += x * acceleration;
                    ysum += y * acceleration;
                    if xsum.abs() > 1.0 || ysum.abs() > 1.0 {
                        xdo_handler.move_mouse_relative(xsum as i32, ysum as i32);
                        xsum = 0.0;
                        ysum = 0.0;
                    }
                }
                _ => {
                    xdo_handler.move_mouse_relative(xsum as i32, ysum as i32);
                    if parts.len() > 4 && parts[4] == "cancelled" {
                        xdo_handler.mouse_up(1);
                    } else {
                        xdo_handler.mouse_up_delay(1, 750);
                    }
                }
            }
        } else if this_app_mouse_down {
            xdo_handler.mouse_up(1);
            this_app_mouse_down = false;
        }
    }
}