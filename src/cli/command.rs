pub trait CliCommand {
    fn new(name: &str) -> Self;
    fn action();
}

pub struct Command {
    name: String,
}

impl CliCommand for Command {
    fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
        }
    }
    fn action() {
        todo!("implement some action");
    }
}
