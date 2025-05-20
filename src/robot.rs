// Имеем некого робота, который может двигаться по координатной сетке.
// Он может двигаться в четырех направлениях но только вперед: вверх, вниз, влево и вправо.
// Он может поворачивать налево и направо.
// Он может поднимать и опускать перо, чтобы рисовать линии.

use std::fmt;

use super::error::Error;

#[derive(Debug, Clone)]
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
    drawing: bool,
}

impl Default for Robot {
    fn default() -> Self {
        Self::new(0, 0, Direction::Up, false)
    }
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction, drawing: bool) -> Self {
        Self {
            x,
            y,
            direction,
            drawing,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn is_drawing(&self) -> bool {
        self.drawing
    }

    pub fn move_forward(&mut self) -> Result<(), Error> {
        match self.direction {
            Direction::Up => {
                if self.y == i32::MAX {
                    return Err(Error::OutOfBounds);
                }

                self.y += 1
            }
            Direction::Right => {
                if self.x == i32::MAX {
                    return Err(Error::OutOfBounds);
                }

                self.x += 1
            }
            Direction::Down => {
                if self.y == i32::MIN {
                    return Err(Error::OutOfBounds);
                }

                self.y -= 1
            }
            Direction::Left => {
                if self.x == i32::MIN {
                    return Err(Error::OutOfBounds);
                }

                self.x -= 1
            }
        }

        log::info!("Move to forward at ({}, {})", self.x, self.y);
        if self.drawing {
            log::info!("Drawing at ({}, {})", self.x, self.y);
        }

        Ok(())
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
        log::info!("Turn left to {}", self.direction);
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        log::info!("Turn right to {}", self.direction);
    }

    pub fn down_pen(&mut self) {
        if !self.drawing {
            log::info!("Pen down");
            self.drawing = true;
        }
    }

    pub fn up_pen(&mut self) {
        if self.drawing {
            log::info!("Pen up");
            self.drawing = false;
        }
    }
}

#[derive(Debug, Clone)]
pub struct RobotBuilder {
    x: i32,
    y: i32,
    direction: Direction,
    drawing: bool,
}

impl Default for RobotBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RobotBuilder {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::Up,
            drawing: false,
        }
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn drawing(mut self, drawing: bool) -> Self {
        self.drawing = drawing;
        self
    }

    pub fn build(self) -> Robot {
        Robot::new(self.x, self.y, self.direction, self.drawing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "up"),
            Direction::Down => write!(f, "down"),
            Direction::Left => write!(f, "left"),
            Direction::Right => write!(f, "right"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_new() {
        let robot = Robot::new(1, 2, Direction::Left, true);
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 2);
        assert_eq!(robot.direction, Direction::Left);
        assert!(robot.drawing);
    }

    #[test]
    fn test_robot_move_forward_up() {
        let mut robot = Robot::new(0, 0, Direction::Up, false);
        robot.move_forward().unwrap();
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 1);
    }

    #[test]
    fn test_robot_move_forward_right() {
        let mut robot = Robot::new(0, 0, Direction::Right, false);
        robot.move_forward().unwrap();
        assert_eq!(robot.x, 1);
        assert_eq!(robot.y, 0);
    }

    #[test]
    fn test_robot_move_forward_down() {
        let mut robot = Robot::new(0, 0, Direction::Down, false);
        robot.move_forward().unwrap();
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, -1);
    }

    #[test]
    fn test_robot_move_forward_left() {
        let mut robot = Robot::new(0, 0, Direction::Left, false);
        robot.move_forward().unwrap();
        assert_eq!(robot.x, -1);
        assert_eq!(robot.y, 0);
    }

    #[test]
    fn test_robot_turn_left() {
        let mut robot = Robot::default();
        robot.turn_left();
        assert_eq!(robot.direction, Direction::Left);
        robot.turn_left();
        assert_eq!(robot.direction, Direction::Down);
        robot.turn_left();
        assert_eq!(robot.direction, Direction::Right);
        robot.turn_left();
        assert_eq!(robot.direction, Direction::Up);
    }

    #[test]
    fn test_robot_turn_right() {
        let mut robot = Robot::default();
        robot.turn_right();
        assert_eq!(robot.direction, Direction::Right);
        robot.turn_right();
        assert_eq!(robot.direction, Direction::Down);
        robot.turn_right();
        assert_eq!(robot.direction, Direction::Left);
        robot.turn_right();
        assert_eq!(robot.direction, Direction::Up);
    }

    #[test]
    fn test_robot_pen_down_and_up() {
        let mut robot = Robot::default();
        assert!(!robot.drawing);
        robot.down_pen();
        assert!(robot.drawing);
        robot.up_pen();
        assert!(!robot.drawing);
    }

    #[test]
    fn test_robot_builder_defaults() {
        let robot = RobotBuilder::default().build();
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        assert_eq!(robot.direction, Direction::Up);
        assert!(!robot.drawing);
    }

    #[test]
    fn test_robot_builder_custom() {
        let robot = RobotBuilder::new()
            .x(5)
            .y(-3)
            .direction(Direction::Down)
            .drawing(true)
            .build();
        assert_eq!(robot.x, 5);
        assert_eq!(robot.y, -3);
        assert_eq!(robot.direction, Direction::Down);
        assert!(robot.drawing);
    }
}
