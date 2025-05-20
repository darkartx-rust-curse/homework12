// Пример реализации интерпретатора для языка команд
// Этот код включает в себя реализацию сканера, который разбирает входную строку на токены,
// а также интерпретатор, который собирает батч команд на основе токенов.
// В этом примере мы используем простые команды, такие как "move", "turn_left", "turn_right",
// "down_pen", "up_pen" и числа для указания расстояния или угла поворота.

use std::str;

use crate::{command::*, error::Error};

pub struct Interpreter<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &'a str) -> Self {
        let scanner = Scanner::new(input);
        Self { scanner }
    }

    pub fn interpret(&mut self) -> Result<CommandList, Error> {
        let mut command_list = CommandList::default();

        while let Some(token) = self.next_token()? {
            match token {
                Token::Move => {
                    let distance = match self.next_token()? {
                        Some(Token::Number(distance)) => distance,
                        Some(token) => return Err(Error::UnexpectedToken(token)),
                        None => return Err(Error::InvalidCommand),
                    };
                    command_list.add_command(Box::new(MoveCommand::new(distance)));
                }
                Token::TurnLeft | Token::TurnRight => {
                    let angle = match self.next_token()? {
                        Some(Token::Number(angle)) => angle,
                        Some(token) => return Err(Error::UnexpectedToken(token)),
                        None => return Err(Error::InvalidCommand),
                    };
                    match token {
                        Token::TurnLeft => {
                            command_list.add_command(Box::new(TurnLeftCommand::new(angle)))
                        }
                        Token::TurnRight => {
                            command_list.add_command(Box::new(TurnRightCommand::new(angle)))
                        }
                        _ => unreachable!(),
                    };
                }
                Token::DownPen => {
                    command_list.add_command(Box::new(DownPenCommand));
                }
                Token::UpPen => {
                    command_list.add_command(Box::new(UpPenCommand));
                }
                _ => return Err(Error::UnexpectedToken(token)),
            }
        }

        Ok(command_list)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        self.scanner.next_token()
    }
}

pub struct Scanner<'a> {
    source: str::Chars<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let source = input.chars();
        Self { source }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let token = loop {
            let ch = self.next_char();

            match ch {
                None => break None,
                Some(ch) if ch.is_alphabetic() => break Some(self.scan_keyword(ch)?),
                Some(ch) if ch.is_ascii_digit() => break Some(self.scan_number(ch)?),
                Some(ch) if ch.is_whitespace() => continue,
                Some(ch) => {
                    return Err(Error::UnexpectedCharacter(ch));
                }
            }
        };

        Ok(token)
    }

    fn next_char(&mut self) -> Option<char> {
        self.source.next()
    }

    fn scan_keyword(&mut self, ch: char) -> Result<Token, Error> {
        let mut buffer = ch.to_string();

        while let Some(next_ch) = self.next_char() {
            if !next_ch.is_whitespace() {
                buffer.push(next_ch);
            } else {
                break;
            }
        }

        match buffer.as_str() {
            "move" => Ok(Token::Move),
            "turn_left" => Ok(Token::TurnLeft),
            "turn_right" => Ok(Token::TurnRight),
            "down_pen" => Ok(Token::DownPen),
            "up_pen" => Ok(Token::UpPen),
            _ => Err(Error::UndefinedCommand(buffer)),
        }
    }

    fn scan_number(&mut self, ch: char) -> Result<Token, Error> {
        let mut buffer = ch.to_string();

        while let Some(next_ch) = self.next_char() {
            if !next_ch.is_whitespace() {
                buffer.push(next_ch);
            } else {
                break;
            }
        }

        match buffer.parse::<u32>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(Error::InvalidCommandParameter(buffer)),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Move,
    TurnLeft,
    TurnRight,
    DownPen,
    UpPen,
    Number(u32),
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_command() {
        let mut interpreter = Interpreter::new("move 10");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 1);
    }

    #[test]
    fn test_turn_left_command() {
        let mut interpreter = Interpreter::new("turn_left 90");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 1);
    }

    #[test]
    fn test_turn_right_command() {
        let mut interpreter = Interpreter::new("turn_right 45");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 1);
    }

    #[test]
    fn test_down_pen_command() {
        let mut interpreter = Interpreter::new("down_pen");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 1);
    }

    #[test]
    fn test_up_pen_command() {
        let mut interpreter = Interpreter::new("up_pen");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 1);
    }

    #[test]
    fn test_multiple_commands() {
        let mut interpreter = Interpreter::new("move 10 turn_left 90 move 5 down_pen up_pen");
        let commands = interpreter.interpret().unwrap();
        assert_eq!(commands.commands().len(), 5);
    }

    #[test]
    fn test_invalid_command() {
        let mut interpreter = Interpreter::new("fly 10");
        let result = interpreter.interpret();
        assert!(matches!(result, Err(Error::UndefinedCommand(_))));
    }

    #[test]
    fn test_missing_number_after_move() {
        let mut interpreter = Interpreter::new("move");
        let result = interpreter.interpret();
        assert!(matches!(result, Err(Error::InvalidCommand)));
    }

    #[test]
    fn test_unexpected_token() {
        let mut interpreter = Interpreter::new("move up_pen");
        let result = interpreter.interpret();
        assert!(matches!(result, Err(Error::UnexpectedToken(_))));
    }

    #[test]
    fn test_invalid_character() {
        let mut scanner = Scanner::new("move@10");
        let result = scanner.next_token();
        assert!(matches!(result, Err(Error::UndefinedCommand(_))));
    }

    #[test]
    fn test_invalid_number() {
        let mut scanner = Scanner::new("123abc");
        let token = scanner.next_token();
        assert!(matches!(token, Err(Error::InvalidCommandParameter(_))));
    }
}
