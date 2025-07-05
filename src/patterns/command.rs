use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Receiver { log: Rc<RefCell<Vec<String>>> }

impl Receiver {
    pub fn new() -> Self {
        Receiver { log: Rc::new(RefCell::new(Vec::new())) }
    }
    pub fn action(&self, info: &str) {
        self.log.borrow_mut().push(info.to_string());
    }
    pub fn get_log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }
}

pub trait Command {
    fn execute(&self);
}

pub struct ConcreteCommand { receiver: Receiver, info: String }

impl ConcreteCommand {
    pub fn new(receiver: Receiver, info: impl Into<String>) -> Self {
        ConcreteCommand { receiver, info: info.into() }
    }
}

impl Command for ConcreteCommand {
    fn execute(&self) {
        self.receiver.action(&self.info);
    }
}

pub struct Invoker { command: Box<dyn Command> }

impl Invoker {
    pub fn new(command: Box<dyn Command>) -> Self {
        Invoker { command }
    }
    pub fn invoke(&self) {
        self.command.execute();
    }
}

pub fn demo() {
    let receiver = Receiver::new();
    let cmd1 = ConcreteCommand::new(receiver.clone(), "Command 1");
    let cmd2 = ConcreteCommand::new(receiver.clone(), "Command 2");
    let invoker1 = Invoker::new(Box::new(cmd1));
    let invoker2 = Invoker::new(Box::new(cmd2));
    invoker1.invoke();
    invoker2.invoke();
    println!("[Command demo] history={:?}", receiver.get_log());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_history() {
        let receiver = Receiver::new();
        let cmd = ConcreteCommand::new(receiver.clone(), "Hello");
        let invoker = Invoker::new(Box::new(cmd));
        invoker.invoke();
        assert_eq!(receiver.get_log(), vec!["Hello".to_string()]);
    }
}
