use kissa::{EventType, KissaPlugin};

#[derive(Default, Debug)]
struct TestIPCPub;

pub enum ClashedEvent {
    ClashedEvent(&'static str), // Change the lifetime to 'static
}

impl KissaPlugin for TestIPCPub {
    fn name(&self) -> &'static str {
        "test_clashed_publish"
    }

    fn load(&self) -> Vec<EventType> {
        vec![EventType::Unknown(
            self.name(),
            Box::new(ClashedEvent::ClashedEvent("hello from clashed!!")), // Corrected
        )]
    }

    fn on_event(&self, _: &EventType) -> Vec<EventType> {
        vec![]
    }
}

#[no_mangle]
pub fn _create() -> Box<dyn KissaPlugin> {
    Box::new(TestIPCPub::default())
}
