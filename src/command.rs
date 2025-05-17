// Команды для управления роботом
// Команды реализуют паттерн Command, который позволяет инкапсулировать запросы в объекты
// и передавать их как параметры другим объектам. Это позволяет реализовать такие
// паттерны, как Undo/Redo, логирование и т.д.
// Команды могут быть выполнены, отменены и повторно выполнены.
// В этом файле определены команды для перемещения робота, поворота налево и направо,
// а также для включения и выключения режима рисования.

use std::fmt;

use super::{error::Error, robot::Robot};

pub trait Command: fmt::Debug {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error>;
    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error>;
    fn box_clone(&self) -> Box<dyn Command>;
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

// Команда для перемещения робота на заданное количество шагов
#[derive(Debug, Clone)]
pub struct MoveCommand {
    distance: u32,
}

impl Command for MoveCommand {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Moving robot {} steps", self.distance);

        for _ in 0..self.distance {
            robot.move_forward()?;
        }

        Ok(())
    }

    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Rolling back moving robot {} steps", self.distance);

        robot.turn_left();
        robot.turn_left();
        for _ in 0..self.distance {
            robot.move_forward()?;
        }
        robot.turn_left();
        robot.turn_left();
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl MoveCommand {
    pub fn new(distance: u32) -> Self {
        Self { distance }
    }
}

// Команда для поворота робота на лево заданное количество раз
#[derive(Debug, Clone)]
pub struct TurnLeftCommand {
    times: u8,
}

impl Command for TurnLeftCommand {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Turning robot left {} times", self.times);

        for _ in 0..self.times {
            robot.turn_left();
        }

        Ok(())
    }

    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Rolling back turning robot left {} times", self.times);

        for _ in 0..self.times {
            robot.turn_right();
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl TurnLeftCommand {
    pub fn new(times: u32) -> Self {
        let times = (times % 4) as u8;
        Self { times }
    }
}

// Команда для поворота робота на право заданное количество раз
#[derive(Debug, Clone)]
pub struct TurnRightCommand {
    times: u8,
}

impl Command for TurnRightCommand {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Turning robot right {} times", self.times);

        for _ in 0..self.times {
            robot.turn_right();
        }

        Ok(())
    }

    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Rolling back turning robot right {} times", self.times);

        for _ in 0..self.times {
            robot.turn_left();
        }

        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl TurnRightCommand {
    pub fn new(times: u32) -> Self {
        let times = (times % 4) as u8;
        Self { times }
    }
}

// Команда для включения режима рисования
#[derive(Debug, Clone)]
pub struct DownPenCommand;

impl Command for DownPenCommand {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Pen down");

        robot.down_pen();
        Ok(())
    }

    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Rolling back pen down");

        robot.up_pen();
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

// Команда для выключения режима рисования
#[derive(Debug, Clone)]
pub struct UpPenCommand;

impl Command for UpPenCommand {
    fn execute(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Pen up");

        robot.up_pen();
        Ok(())
    }

    fn rollback(&mut self, robot: &mut Robot) -> Result<(), Error> {
        log::debug!("Rolling back pen up");

        robot.down_pen();
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CommandList {
    commands: Vec<Box<dyn Command>>,
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn execute_all(&mut self, robot: &mut Robot) -> Result<(), Error> {
        for command in &mut self.commands {
            command.execute(robot)?;
        }
        Ok(())
    }

    pub fn rollback_all(&mut self, robot: &mut Robot) -> Result<(), Error> {
        for command in self.commands.iter_mut().rev() {
            command.rollback(robot)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{*, super::robot::Direction};

    #[test]
    fn test_move_command_execute_and_rollback() {
        let mut robot = Robot::default();
        let mut cmd = MoveCommand::new(3);

        // Move forward 3 steps
        assert!(cmd.execute(&mut robot).is_ok());
        assert_eq!(robot.x(), 0);
        assert_eq!(robot.y(), 3);

        // Rollback: should return to original position
        assert!(cmd.rollback(&mut robot).is_ok());
        assert_eq!(robot.x(), 0);
        assert_eq!(robot.y(), 0);
    }

    #[test]
    fn test_turn_left_command_execute_and_rollback() {
        let mut robot = Robot::default();
        let mut cmd = TurnLeftCommand::new(1);

        // Turn left once
        assert!(cmd.execute(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Left);

        // Rollback: should turn right, back to up
        assert!(cmd.rollback(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Up);
    }

    #[test]
    fn test_turn_right_command_execute_and_rollback() {
        let mut robot = Robot::default();
        let mut cmd = TurnRightCommand::new(2);

        // Turn right twice
        assert!(cmd.execute(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Down);

        // Rollback: should turn left twice, back to up
        assert!(cmd.rollback(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Up);
    }

    #[test]
    fn test_down_pen_command_execute_and_rollback() {
        let mut robot = Robot::default();
        let mut cmd = DownPenCommand;

        // Pen down
        assert!(cmd.execute(&mut robot).is_ok());
        assert!(robot.is_drawing());

        // Rollback: pen up
        assert!(cmd.rollback(&mut robot).is_ok());
        assert!(!robot.is_drawing());
    }

    #[test]
    fn test_up_pen_command_execute_and_rollback() {
        let mut robot = Robot::default();
        robot.down_pen();
        let mut cmd = UpPenCommand;

        // Pen up
        assert!(cmd.execute(&mut robot).is_ok());
        assert!(!robot.is_drawing());

        // Rollback: pen down
        assert!(cmd.rollback(&mut robot).is_ok());
        assert!(robot.is_drawing());
    }

    #[test]
    fn test_turn_left_command_wraps_around() {
        let mut robot = Robot::default();
        let mut cmd = TurnLeftCommand::new(5); // 5 % 4 == 1
        assert_eq!(cmd.times, 1);
        assert!(cmd.execute(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Left);
    }

    #[test]
    fn test_turn_right_command_wraps_around() {
        let mut robot = Robot::default();
        let mut cmd = TurnRightCommand::new(8); // 8 % 4 == 0
        assert_eq!(cmd.times, 0);
        assert!(cmd.execute(&mut robot).is_ok());
        assert_eq!(robot.direction(), Direction::Up);
    }
}
