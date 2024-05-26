use crate::{
    constants::{DASHBOARD_INFO_COLUMN, DASHBOARD_POS, FEEDBACK_POS, MENU_POS},
    models::{ParkingLotDataPayload, SpotDataPayload},
};
use std::{
    io::{Stdout, Write},
    sync::{Arc, Mutex, MutexGuard},
};
use termion::{clear, color, cursor, raw::RawTerminal};

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

    write!(stdout, "1. Fechar estacionamento").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "2. Fechar andar").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "3. Reabrir estacionamento").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "4. Reabrir andar").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "5. Resetar dados").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "0. Sair").unwrap();
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

    write!(stdout, "Escolha o andar:").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "1. Primeiro andar").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "2. Segundo andar").unwrap();
    new_line(&mut stdout, &mut line);

    write!(stdout, "0. Voltar").unwrap();
    new_line(&mut stdout, &mut line);

    stdout.flush().unwrap();
}

pub fn dashboard(
    stdout: &mut MutexGuard<RawTerminal<Stdout>>,
    parking_lot: &MutexGuard<ParkingLotDataPayload>,
) {
    /*
                  -----------------------------------------------------------------             Ocupação máxima: 14/24
        2° andar  | 0.0 | --- | 0.1 | --- | 0.2 | --- | 0.3 | --- | Aberto      Vagas de deficiente disponíveis: 2
                  -----------------------------------------------------------------             Vagas de idoso disponíveis: 3
        1° andar  | 1.0 | --- | 1.1 | --- | 1.2 | --- | 1.3 | --- | Fechado
                  -----------------------------------------------------------------
        Terreo    | 2.0 | --- | 2.1 | --- | 2.2 | --- | 2.3 | --- | Aberto
                  -----------------------------------------------------------------
    */

    // Get the total of occupied spaces
    let occupied_spots: u16 = parking_lot
        .floors
        .iter()
        .map(|floor| {
            floor
                .spots
                .iter()
                .filter(|spot| spot.parked_vehicle.is_some())
                .count() as u16
        })
        .sum();

    // Get the total of handiccaped and elderly spots
    let disabled_spots: u16 = parking_lot
        .floors
        .iter()
        .map(|floor| {
            floor
                .spots
                .iter()
                .filter(|spot| spot.spot_type == 1 && spot.parked_vehicle.is_none())
                .count() as u16
        })
        .sum();

    let elderly_spots: u16 = parking_lot
        .floors
        .iter()
        .map(|floor| {
            floor
                .spots
                .iter()
                .filter(|spot| spot.spot_type == 2 && spot.parked_vehicle.is_none())
                .count() as u16
        })
        .sum();

    // First line
    write!(stdout, "{}", cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1)).unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}-----------------------------------------------------------------{}Vagas disponíveis: {}",
        cursor::Goto(DASHBOARD_POS.0 + 10, DASHBOARD_POS.1),
        cursor::Goto(DASHBOARD_INFO_COLUMN, DASHBOARD_POS.1),
        24-occupied_spots,
    )
    .unwrap();

    // Second line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 1)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}2° andar  ",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 1)
    )
    .unwrap();

    write_floor(stdout, &parking_lot.floors[2].spots);

    write!(
        stdout,
        " {}",
        match parking_lot.floors[2].is_closed {
            true => "Fechado",
            false => "Aberto",
        }
    )
    .unwrap();

    write!(
        stdout,
        "{}Vagas de deficiente disponíveis: {}",
        cursor::Goto(DASHBOARD_INFO_COLUMN, DASHBOARD_POS.1 + 1),
        disabled_spots
    )
    .unwrap();

    // Third line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 2)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}-----------------------------------------------------------------{}Vagas de idoso disponíveis: {}",
        cursor::Goto(DASHBOARD_POS.0 + 10, DASHBOARD_POS.1 + 2),
        cursor::Goto(DASHBOARD_INFO_COLUMN, DASHBOARD_POS.1 + 2),
        elderly_spots
    )
    .unwrap();

    // Fourth line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 3)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}1° andar  ",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 3)
    )
    .unwrap();

    write_floor(stdout, &parking_lot.floors[1].spots);

    write!(
        stdout,
        " {}",
        match parking_lot.floors[1].is_closed {
            true => "Fechado",
            false => "Aberto",
        }
    )
    .unwrap();

    if let Some(last_vehicle) = parking_lot.exited_vehicles.first() {
        write!(
            stdout,
            "{}Último veículo que saiu pagou: R${}",
            cursor::Goto(DASHBOARD_INFO_COLUMN, DASHBOARD_POS.1 + 3),
            format!("{:.2}", last_vehicle.fee()),
        )
        .unwrap();
    }

    // Fifth line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 4)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}-----------------------------------------------------------------",
        cursor::Goto(DASHBOARD_POS.0 + 10, DASHBOARD_POS.1 + 4)
    )
    .unwrap();

    write!(
        stdout,
        "{}Total arrecadado: R${}",
        cursor::Goto(DASHBOARD_INFO_COLUMN, DASHBOARD_POS.1 + 4),
        format!(
            "{:.2}",
            parking_lot
                .exited_vehicles
                .iter()
                .map(|vehicle| vehicle.fee())
                .sum::<f64>()
        )
    )
    .unwrap();

    // Sixth line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 5)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}Terreo    ",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 5)
    )
    .unwrap();

    write_floor(stdout, &parking_lot.floors[0].spots);

    write!(
        stdout,
        " {}",
        match parking_lot.is_closed {
            true => "Fechado",
            false => "Aberto",
        }
    )
    .unwrap();

    // Seventh line
    write!(
        stdout,
        "{}",
        cursor::Goto(DASHBOARD_POS.0, DASHBOARD_POS.1 + 6)
    )
    .unwrap();
    write!(stdout, "{}", clear::CurrentLine).unwrap();

    write!(
        stdout,
        "{}-----------------------------------------------------------------",
        cursor::Goto(DASHBOARD_POS.0 + 10, DASHBOARD_POS.1 + 6)
    )
    .unwrap();

    stdout.flush().unwrap();
}

fn write_floor(stdout: &mut MutexGuard<RawTerminal<Stdout>>, spots: &Vec<SpotDataPayload>) {
    write!(stdout, "|").unwrap();

    for spot in spots {
        let parked_vehicle = match spot.parked_vehicle.as_ref() {
            Some(vehicle) => format!("R${:.1}", vehicle.fee()),
            None => "-----".to_string(),
        };

        let color = match spot.spot_type {
            // Green
            0 => color::Fg(color::Rgb(0, 255, 0)),
            // Blue
            1 => color::Fg(color::Rgb(0, 0, 255)),
            // Yellow
            2 => color::Fg(color::Rgb(255, 255, 0)),
            // White
            _ => color::Fg(color::Rgb(255, 255, 255)),
        };

        write!(
            stdout,
            " {}{}{}{} |",
            color::Fg(color::Reset),
            color,
            parked_vehicle,
            color::Fg(color::Reset)
        )
        .unwrap();
    }

    //write!(stdout, "{}", color::Fg(color::Reset)).unwrap();
}

fn new_line(stdout: &mut MutexGuard<RawTerminal<Stdout>>, line: &mut u16) {
    *line += 1;
    write!(stdout, "{}", cursor::Goto(1, *line)).unwrap();
}
