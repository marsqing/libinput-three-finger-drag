extern crate libxdo;
extern crate regex;

use libxdo::XDo;
use regex::Regex;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::process::{Command, Stdio};

fn main() {
    let output = Command::new("stdbuf")
        .arg("-o0")
        .arg("libinput")
        .arg("debug-events")
        .stdout(Stdio::piped())
        .spawn()
        .expect("can not exec libinput")
        .stdout
        .expect("libinput has no stdout");

    let xdo = XDo::new(None).expect("can not initialize libxdo");

    // GESTURE_SWIPE_BEGIN, GESTURE_SWIPE_UPDATE, GESTURE_SWIPE_END
    // event10  GESTURE_SWIPE_UPDATE +3.769s	4  0.25/ 0.48 ( 0.95/ 1.85 unaccelerated)
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
                continue;
            }
            match action {
                "GESTURE_SWIPE_BEGIN" => {
                    xsum = 0.0;
                    ysum = 0.0;
                    xdo.mouse_down(1).unwrap();
                }
                "GESTURE_SWIPE_UPDATE" => {
                    let x: f32 = parts[6].parse().unwrap();
                    let y: f32 = parts[7].parse().unwrap();
                    xsum += x;
                    ysum += y;
                    if xsum.abs() > 1.0 || ysum.abs() > 1.0 {
                        xdo.move_mouse_relative(xsum as i32, ysum as i32).unwrap();
                        xsum = 0.0;
                        ysum = 0.0;
                    }
                }
                _ => {
                    xdo.move_mouse_relative(xsum as i32, ysum as i32).unwrap();
                    xdo.mouse_up(1).unwrap();
                }
            }
        }
    }
}
