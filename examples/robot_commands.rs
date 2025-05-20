use homework12::{command::*, robot::Robot};

fn main() {
    init_logger();

    let mut robot = Robot::default();

    let mut command_list = command_list();

    log::debug!("Robot state before executing commands: {:?}", robot);
    log::debug!("Command list: {:?}", command_list);
    log::debug!("Executing commands...");

    command_list.execute_all(&mut robot).unwrap();

    log::debug!("Robot state after executing commands: {:?}", robot);
    log::debug!("Rolling back commands...");

    command_list.rollback_all(&mut robot).unwrap();
}

fn init_logger() {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into());
    env_logger::Builder::new()
        .filter(None, log_level.parse().unwrap())
        .init();
}

fn command_list() -> CommandList {
    let mut command_list = CommandList::default();
    command_list.add_command(Box::new(MoveCommand::new(1)));
    command_list.add_command(Box::new(TurnLeftCommand::new(3)));
    command_list.add_command(Box::new(MoveCommand::new(2)));
    command_list.add_command(Box::new(TurnRightCommand::new(2)));
    command_list.add_command(Box::new(MoveCommand::new(3)));
    command_list.add_command(Box::new(DownPenCommand));
    command_list.add_command(Box::new(UpPenCommand));
    command_list.add_command(Box::new(MoveCommand::new(4)));
    command_list.add_command(Box::new(DownPenCommand));

    command_list
}
