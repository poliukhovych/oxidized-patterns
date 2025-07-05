pub trait Chair {
    fn assemble(&self) -> String;
}

pub trait Sofa {
    fn assemble(&self) -> String;
}

pub trait FurnitureFactory {
    type C: Chair;
    type S: Sofa;
    fn create_chair(&self) -> Self::C;
    fn create_sofa(&self) -> Self::S;
}

pub struct ModernChair;
impl Chair for ModernChair {
    fn assemble(&self) -> String {
        "Assembling modern chair".into()
    }
}

pub struct ModernSofa;
impl Sofa for ModernSofa {
    fn assemble(&self) -> String {
        "Assembling modern sofa".into()
    }
}

pub struct VictorianChair;
impl Chair for VictorianChair {
    fn assemble(&self) -> String {
        "Assembling Victorian chair".into()
    }
}

pub struct VictorianSofa;
impl Sofa for VictorianSofa {
    fn assemble(&self) -> String {
        "Assembling Victorian sofa".into()
    }
}

pub struct ModernFactory;
impl FurnitureFactory for ModernFactory {
    type C = ModernChair;
    type S = ModernSofa;
    fn create_chair(&self) -> Self::C {
        ModernChair
    }
    fn create_sofa(&self) -> Self::S {
        ModernSofa
    }
}

pub struct VictorianFactory;
impl FurnitureFactory for VictorianFactory {
    type C = VictorianChair;
    type S = VictorianSofa;
    fn create_chair(&self) -> Self::C {
        VictorianChair
    }
    fn create_sofa(&self) -> Self::S {
        VictorianSofa
    }
}

pub trait FactoryDyn {
    fn create_chair(&self) -> Box<dyn Chair>;
    fn create_sofa(&self) -> Box<dyn Sofa>;
}

impl FactoryDyn for ModernFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(ModernChair)
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(ModernSofa)
    }
}

impl FactoryDyn for VictorianFactory {
    fn create_chair(&self) -> Box<dyn Chair> {
        Box::new(VictorianChair)
    }
    fn create_sofa(&self) -> Box<dyn Sofa> {
        Box::new(VictorianSofa)
    }
}

pub fn demo() {
    let modern = ModernFactory;
    let chair = <ModernFactory as FurnitureFactory>::create_chair(&modern);
    let sofa = <ModernFactory as FurnitureFactory>::create_sofa(&modern);
    println!("[AbstractFactory demo] {}", chair.assemble());
    println!("[AbstractFactory demo] {}", sofa.assemble());

    let dyn_factories: Vec<Box<dyn FactoryDyn>> = vec![Box::new(ModernFactory), Box::new(VictorianFactory)];
    for factory in dyn_factories {
        let chair = factory.create_chair();
        let sofa = factory.create_sofa();
        println!("[AbstractFactory demo] dyn {}", chair.assemble());
        println!("[AbstractFactory demo] dyn {}", sofa.assemble());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_factory() {
        let factory = VictorianFactory;
        let chair = <VictorianFactory as FurnitureFactory>::create_chair(&factory);
        let sofa = <VictorianFactory as FurnitureFactory>::create_sofa(&factory);
        assert_eq!(chair.assemble(), "Assembling Victorian chair");
        assert_eq!(sofa.assemble(), "Assembling Victorian sofa");
    }

    #[test]
    fn test_dynamic_factory() {
        let factory: Box<dyn FactoryDyn> = Box::new(ModernFactory);
        let chair = factory.create_chair();
        let sofa = factory.create_sofa();
        assert_eq!(chair.assemble(), "Assembling modern chair");
        assert_eq!(sofa.assemble(), "Assembling modern sofa");
    }
}
