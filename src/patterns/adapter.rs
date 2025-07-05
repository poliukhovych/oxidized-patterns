pub trait Target {
    fn request(&self) -> String;
}

pub struct Adaptee {
    pub data: String,
}

impl Adaptee {
    pub fn new(data: impl Into<String>) -> Self {
        Adaptee { data: data.into() }
    }
    pub fn specific_request(&self) -> &str {
        &self.data
    }
}

pub struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    pub fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        format!("Adapter: [{}]", self.adaptee.specific_request())
    }
}

pub fn demo() {
    let adaptee = Adaptee::new("some legacy data");
    println!("[Adapter demo] Adaptee specific: {}", adaptee.specific_request());

    let target_obj: Box<dyn Target> = Box::new(Adapter::new(adaptee));
    println!("[Adapter demo] Adapter as Target: {}", target_obj.request());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptee_specific() {
        let a = Adaptee::new("xyz");
        assert_eq!(a.specific_request(), "xyz");
    }

    #[test]
    fn test_adapter_request() {
        let a = Adaptee::new("abc");
        let adapter = Adapter::new(a);
        assert_eq!(adapter.request(), "Adapter: [abc]");
    }

    #[test]
    fn test_demo_through_target() {
        let adaptee = Adaptee::new("123");
        let target: Box<dyn Target> = Box::new(Adapter::new(adaptee));
        assert_eq!(target.request(), "Adapter: [123]");
    }
}
