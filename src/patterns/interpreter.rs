pub trait Expression {
    fn interpret(&self) -> i32;
}

pub struct Number(i32);

impl Number {
    pub fn new(value: i32) -> Self {
        Number(value)
    }
}

impl Expression for Number {
    fn interpret(&self) -> i32 {
        self.0
    }
}

pub struct Plus {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Plus {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Plus { left, right }
    }
}

impl Expression for Plus {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}

pub struct Minus {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Minus {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Minus { left, right }
    }
}

impl Expression for Minus {
    fn interpret(&self) -> i32 {
        self.left.interpret() - self.right.interpret()
    }
}

pub fn demo() {
    let expr = Plus::new(
        Box::new(Minus::new(
            Box::new(Number::new(10)),
            Box::new(Number::new(4)),
        )),
        Box::new(Number::new(3)),
    );
    println!("[Interpreter demo] result = {}", expr.interpret());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let n = Number::new(5);
        assert_eq!(n.interpret(), 5);
    }

    #[test]
    fn test_plus() {
        let left = Box::new(Number::new(2));
        let right = Box::new(Number::new(3));
        let plus = Plus::new(left, right);
        assert_eq!(plus.interpret(), 5);
    }

    #[test]
    fn test_minus() {
        let left = Box::new(Number::new(5));
        let right = Box::new(Number::new(3));
        let minus = Minus::new(left, right);
        assert_eq!(minus.interpret(), 2);
    }

    #[test]
    fn test_complex() {
        let expr = Plus::new(
            Box::new(Minus::new(
                Box::new(Number::new(10)),
                Box::new(Number::new(4)),
            )),
            Box::new(Number::new(3)),
        );
        assert_eq!(expr.interpret(), 9);
    }
}
