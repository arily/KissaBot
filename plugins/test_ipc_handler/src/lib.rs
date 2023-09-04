use kissa::{EventType, KissaPlugin};

extern crate test_ipc_events;

#[derive(Default, Debug)]
struct TestIPCHandler;

impl KissaPlugin for TestIPCHandler {
    fn name(&self) -> &'static str {
        "test_ipc_handler"
    }

    fn load(&self) -> Vec<EventType<'_>> {
        vec![]
    }

    fn on_event(&self, event: &EventType<'_>) -> Vec<EventType<'_>> {
        match event {
            EventType::Unknown(e_name, e_data) => {
                println!("received event {}", e_name);
                if let Some(custom_event) = e_data
                    .as_ref()
                    .downcast_ref::<test_ipc_events::CustomEvents>()
                {
                    match custom_event {
                        test_ipc_events::CustomEvents::CustomEvent2(str) => {
                            println!("CustomEvent2 content: {}", str)
                        }
                        test_ipc_events::CustomEvents::CustomEvent1 => todo!(),
                        test_ipc_events::CustomEvents::CustomEvent3(_, _, _) => todo!(),
                    }
                }
            }
            _ => {}
        }
        vec![]
    }
}

#[no_mangle]
pub fn _create() -> Box<dyn KissaPlugin> {
    Box::new(TestIPCHandler::default())
}
