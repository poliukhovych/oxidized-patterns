use std::marker::PhantomData;

pub struct Product {
    pub name: String,
    pub quantity: u32,
}

pub struct Missing;
pub struct Set;

pub struct Builder<NameState, QuantityState> {
    name: Option<String>,
    quantity: Option<u32>,
    _marker: PhantomData<(NameState, QuantityState)>,
}

impl Builder<Missing, Missing> {
    pub fn new() -> Self {
        Builder { name: None, quantity: None, _marker: PhantomData }
    }
}

impl<Q> Builder<Missing, Q> {
    pub fn name(self, name: impl Into<String>) -> Builder<Set, Q> {
        Builder { name: Some(name.into()), quantity: self.quantity, _marker: PhantomData }
    }
}

impl<N> Builder<N, Missing> {
    pub fn quantity(self, qty: u32) -> Builder<N, Set> {
        Builder { name: self.name, quantity: Some(qty), _marker: PhantomData }
    }
}

impl Builder<Set, Set> {
    pub fn build(self) -> Product {
        Product { name: self.name.unwrap(), quantity: self.quantity.unwrap() }
    }
}

pub fn demo() {
    let prod = Builder::new()
        .name("DemoProduct")
        .quantity(5)
        .build();
    println!("[Builder demo] name='{}', quantity={}", prod.name, prod.quantity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_product_success() {
        let p = Builder::new()
            .name("Widget")
            .quantity(10)
            .build();
        assert_eq!(p.name, "Widget");
        assert_eq!(p.quantity, 10);
    }
}
