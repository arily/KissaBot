use kissa::{EventType, KissaPlugin};

#[derive(Default, Debug)]
struct TestIPCPub;

pub enum CustomEvents {
    CustomEvent1,
    CustomEvent2(&'static str), // Change the lifetime to 'static
    CustomEvent3(&'static str, &'static str, &'static str), // Also change here
}

impl KissaPlugin for TestIPCPub {
    fn name(&self) -> &'static str {
        "test_ipc_publish"
    }

    fn load(&self) -> Vec<EventType> {
        vec![EventType::Unknown(
            self.name(),
            Box::new(CustomEvents::CustomEvent2("hello from publisher!!")), // Corrected
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
