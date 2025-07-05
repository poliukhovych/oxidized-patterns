use std::collections::HashMap;

pub struct EventEmitter<E> {
    next_id: u64,
    observers: HashMap<u64, Box<dyn Fn(&E)>>,
}

impl<E> EventEmitter<E> {
    pub fn new() -> Self {
        EventEmitter { next_id: 0, observers: HashMap::new() }
    }

    pub fn subscribe<F>(&mut self, handler: F) -> u64
    where
        F: Fn(&E) + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.observers.insert(id, Box::new(handler));
        id
    }

    pub fn unsubscribe(&mut self, id: u64) -> bool {
        self.observers.remove(&id).is_some()
    }

    pub fn emit(&self, event: &E) {
        for handler in self.observers.values() {
            handler(event);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Started,
    Data(String),
    Finished,
}

pub fn demo() {
    let mut emitter = EventEmitter::new();
    let id1 = emitter.subscribe(|e: &Event| {
        println!("[Observer demo] Handler1 got event: {:?}", e);
    });
    let _id2 = emitter.subscribe(|e: &Event| {
        if let Event::Data(d) = e {
            println!("[Observer demo] Data handler got: {}", d);
        }
    });

    emitter.emit(&Event::Started);
    emitter.emit(&Event::Data("hello".into()));
    emitter.emit(&Event::Finished);

    emitter.unsubscribe(id1);
    emitter.emit(&Event::Data("world".into()));
}
