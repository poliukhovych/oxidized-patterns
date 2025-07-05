pub trait Strategy {
    fn execute(&self, value: i32) -> i32;
}

impl<F> Strategy for F
where
    F: Fn(i32) -> i32,
{
    fn execute(&self, value: i32) -> i32 {
        (self)(value)
    }
}

pub struct ContextGeneric<S: Strategy> {
    strategy: S,
}

impl<S: Strategy> ContextGeneric<S> {
    pub fn new(strategy: S) -> Self {
        ContextGeneric { strategy }
    }

    pub fn execute(&self, value: i32) -> i32 {
        self.strategy.execute(value)
    }
}

pub struct ContextDyn {
    strategy: Box<dyn Strategy>,
}

impl ContextDyn {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        ContextDyn { strategy }
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    pub fn execute(&self, value: i32) -> i32 {
        self.strategy.execute(value)
    }
}

pub struct AddStrategy {
    pub amount: i32,
}

impl Strategy for AddStrategy {
    fn execute(&self, value: i32) -> i32 {
        value + self.amount
    }
}

pub struct MultiplyStrategy {
    pub factor: i32,
}

impl Strategy for MultiplyStrategy {
    fn execute(&self, value: i32) -> i32 {
        value * self.factor
    }
}

pub fn demo() {
    let static_ctx = ContextGeneric::new(AddStrategy { amount: 5 });
    println!("[Strategy demo] static AddStrategy: {}", static_ctx.execute(10));

    let mut dyn_ctx = ContextDyn::new(Box::new(MultiplyStrategy { factor: 3 }));
    println!("[Strategy demo] dyn MultiplyStrategy: {}", dyn_ctx.execute(10));
    dyn_ctx.set_strategy(Box::new(|v| v - 2));
    println!("[Strategy demo] dyn closure subtract 2: {}", dyn_ctx.execute(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_strategy() {
        let ctx = ContextGeneric::new(AddStrategy { amount: 2 });
        assert_eq!(ctx.execute(3), 5);
    }

    #[test]
    fn test_dynamic_strategy() {
        let mut ctx = ContextDyn::new(Box::new(MultiplyStrategy { factor: 4 }));
        assert_eq!(ctx.execute(3), 12);
        ctx.set_strategy(Box::new(AddStrategy { amount: 1 }));
        assert_eq!(ctx.execute(3), 4);
    }

    #[test]
    fn test_closure_strategy() {
        let ctx = ContextDyn::new(Box::new(|v| v * v));
        assert_eq!(ctx.execute(4), 16);
    }
}
