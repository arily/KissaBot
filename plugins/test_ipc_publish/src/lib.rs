use kissa::{EventType, KissaPlugin};

extern crate test_ipc_events;

#[derive(Default, Debug)]
struct TestIPCPub;

impl KissaPlugin for TestIPCPub {
    fn name(&self) -> &'static str {
        "test_ipc_publish"
    }

    fn load(&self) -> Vec<EventType> {
        vec![EventType::Unknown(
            self.name(),
            Box::new(test_ipc_events::CustomEvents::CustomEvent2(
                "hello from publisher!!",
            )), // Corrected
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
