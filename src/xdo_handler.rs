extern crate libxdo;
extern crate timer;
extern crate chrono;

use libxdo::XDo;
use std::sync::mpsc;
use std::thread;
use timer::Timer;
use chrono::Duration;

pub enum XDoCommand {
    MouseUp,
    MouseDown,
    MoveMouseRelative,
}

pub struct XDoHandler {
    tx: mpsc::Sender<(XDoCommand, i32, i32)>,
    timer: Timer,
    guard: Option<timer::Guard>,
    handler_mouse_down: bool,
}

pub fn start_handler() -> XDoHandler {
    let (tx, rx) = mpsc::channel();
    let timer = Timer::new();

    thread::spawn(move || {
        let xdo = XDo::new(None).expect("can not initialize libxdo");
        loop {
            let (command, param1, param2) = rx.recv().unwrap();
            match command {
                XDoCommand::MouseDown => {
                    xdo.mouse_down(param1).unwrap();
                }
                XDoCommand::MouseUp => {
                    xdo.mouse_up(param1).unwrap();
                }
                XDoCommand::MoveMouseRelative => {
                    xdo.move_mouse_relative(param1, param2).unwrap();
                }
            }
        }
    });

    return XDoHandler {
        tx: tx, 
        timer: timer, 
        guard: None,
        handler_mouse_down: false,
    };
}

impl XDoHandler {
    pub fn mouse_down(&mut self, button: i32) {
        self.cancel_timer_if_present();
        self.tx.send((XDoCommand::MouseDown, button, 255)).unwrap();
        self.handler_mouse_down = true;
    }

    pub fn mouse_up(&mut self, button: i32) {
        self.cancel_timer_if_present();
        if self.handler_mouse_down {
            self.tx.send((XDoCommand::MouseUp, button, 255)).unwrap();
            self.handler_mouse_down = false;
        }
    }

    pub fn mouse_up_force(&mut self, button: i32) {
        self.cancel_timer_if_present();
        self.tx.send((XDoCommand::MouseUp, button, 255)).unwrap();
        self.handler_mouse_down = false;
    }

    pub fn mouse_up_delay(&mut self, button: i32, delay_ms: i64) {
        let tx_clone = self.tx.clone();
        self.guard = Some(self.timer.schedule_with_delay(Duration::milliseconds(delay_ms), move || {
            tx_clone.send((XDoCommand::MouseUp, button, 255)).unwrap();
        }));
        self.handler_mouse_down = false;
    }

    pub fn move_mouse_relative(&mut self, x_val: i32, y_val: i32) {
        self.cancel_timer_if_present();
        self.tx.send((XDoCommand::MoveMouseRelative, x_val, y_val)).unwrap();
    }

    pub fn cancel_timer_if_present(&mut self) {
        match &self.guard {
            Some(_) => {
                self.guard = None;
                self.handler_mouse_down = true;
            }
            None => return,
        }
    }
}