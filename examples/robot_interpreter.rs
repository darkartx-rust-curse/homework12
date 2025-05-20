// Запускает интерактивную консоль для управления роботом с помощью интерпретатора.
// Возможные команды:
// - move <distance>: переместить робота на указанное расстояние
// - turn_left <angle>: повернуть робота налево на 90 градусов указанное количество раз
// - turn_right <angle>: повернуть робота направо на 90 градусов указанное количество раз
// - down_pen: опустить перо
// - up_pen: поднять перо

use std::{
    error,
    io::{self, BufRead, Write},
};

use homework12::{interpreter::Interpreter, robot::Robot};

fn main() {
    init_logger();

    let mut robot = Robot::default();
    run_prompt(&mut robot).unwrap();
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into());
    env_logger::Builder::new()
        .filter(None, log_level.parse().unwrap())
        .init();
}

fn run_prompt(mut robot: &mut Robot) -> Result<(), Box<dyn error::Error>> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        write!(stdout.lock(), "> ")?;
        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        let mut interpreter = Interpreter::new(&buffer);
        match interpreter.interpret() {
            Ok(mut commands) => {
                commands.execute_all(&mut robot)?;
            }
            Err(err) => eprintln!("{err}"),
        }
        buffer.clear();
    }
}
