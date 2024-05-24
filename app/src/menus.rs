use std::{
    io::{Stdout, Write},
    sync::{Arc, Mutex, MutexGuard},
};

use termion::{clear, cursor, raw::RawTerminal};

use crate::constants::{FEEDBACK_POS, MENU_POS};

pub fn init_console(stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    let mut stdout = stdout.lock().unwrap();

    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    stdout.flush().unwrap();
}

pub fn finalize_console(stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    let mut stdout = stdout.lock().unwrap();

    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
    write!(stdout, "{}", clear::All).unwrap();
    write!(stdout, "{}", cursor::Show).unwrap();

    stdout.flush().unwrap();
}

pub fn main_menu(stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    let mut stdout = stdout.lock().unwrap();

    write!(stdout, "{}", cursor::Goto(MENU_POS.0, MENU_POS.1)).unwrap();
    write!(stdout, "{}", clear::AfterCursor).unwrap();

    let mut line = MENU_POS.1;

    write!(stdout, "1. close parking lot").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "2. close floor").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "3. open parking lot").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "4. open floor").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "5. reset database").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "0. exit").unwrap();
    new_line(&mut stdout, &mut line);

    stdout.flush().unwrap();
}

pub fn feedback(stdout: &Arc<Mutex<RawTerminal<Stdout>>>, message: &str) {
    let mut stdout = stdout.lock().unwrap();

    write!(stdout, "{}", cursor::Goto(FEEDBACK_POS.0, FEEDBACK_POS.1)).unwrap();
    write!(stdout, "{}", clear::AfterCursor).unwrap();
    write!(stdout, "{}", message).unwrap();

    stdout.flush().unwrap();
}

pub fn floor_menu(stdout: &Arc<Mutex<RawTerminal<Stdout>>>) {
    let mut stdout = stdout.lock().unwrap();

    write!(stdout, "{}", cursor::Goto(FEEDBACK_POS.0, FEEDBACK_POS.1)).unwrap();
    write!(stdout, "{}", clear::AfterCursor).unwrap();

    let mut line = FEEDBACK_POS.1;

    write!(stdout, "Choose floor:").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "1. First floor").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "2. Second floor").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "0. Cancel").unwrap();
    new_line(&mut stdout, &mut line);

    stdout.flush().unwrap();
}

fn new_line(stdout: &mut MutexGuard<RawTerminal<Stdout>>, line: &mut u16) {
    *line += 1;
    write!(stdout, "{}", cursor::Goto(1, *line)).unwrap();
}
