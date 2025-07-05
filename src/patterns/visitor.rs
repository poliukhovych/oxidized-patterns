pub trait Visitor {
    fn visit_element_a(&mut self, element: &ElementA);
    fn visit_element_b(&mut self, element: &ElementB);
}

pub trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct ElementA {
    pub value: i32,
}

impl Element for ElementA {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_element_a(self);
    }
}

pub struct ElementB {
    pub text: String,
}

impl Element for ElementB {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_element_b(self);
    }
}

pub struct ConcreteVisitor {
    pub sum: i32,
    pub texts: Vec<String>,
}

impl ConcreteVisitor {
    pub fn new() -> Self {
        ConcreteVisitor { sum: 0, texts: Vec::new() }
    }
}

impl Visitor for ConcreteVisitor {
    fn visit_element_a(&mut self, element: &ElementA) {
        self.sum += element.value;
    }

    fn visit_element_b(&mut self, element: &ElementB) {
        self.texts.push(element.text.clone());
    }
}

pub fn demo() {
    let elements: Vec<Box<dyn Element>> = vec![
        Box::new(ElementA { value: 2 }),
        Box::new(ElementB { text: String::from("foo") }),
        Box::new(ElementA { value: 5 }),
        Box::new(ElementB { text: String::from("bar") }),
    ];
    let mut visitor = ConcreteVisitor::new();
    for element in &elements {
        element.accept(&mut visitor);
    }
    println!("[Visitor demo] sum={}, texts={:?}", visitor.sum, visitor.texts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_accumulates() {
        let elements: Vec<Box<dyn Element>> = vec![
            Box::new(ElementA { value: 1 }),
            Box::new(ElementB { text: String::from("x") }),
        ];
        let mut visitor = ConcreteVisitor::new();
        for element in &elements {
            element.accept(&mut visitor);
        }
        assert_eq!(visitor.sum, 1);
        assert_eq!(visitor.texts, vec![String::from("x")]);
    }
}
