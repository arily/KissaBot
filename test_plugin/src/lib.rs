use kissa::{EventType, KissaPlugin, TestEvent};
#[derive(Default)]
struct TestPlugin;

impl KissaPlugin for TestPlugin {
    fn name(&self) -> &'static str {
        "test_plugin"
    }
    fn load(&mut self) -> Vec<kissa::EventType<'_>> {
        vec![EventType::Unknown(
            "hello".into(),
            Box::into_raw(Box::new(TestEvent {
                message: "hello world".to_string(),
            })),
        )]
    }
    fn on_event(&mut self, _: EventType<'_>) -> Vec<EventType<'_>> {
        vec![]
    }
}

#[no_mangle]
pub fn _create() -> *mut dyn KissaPlugin {
    let object = TestPlugin::default();
    let boxed: Box<dyn KissaPlugin> = Box::new(object);
    Box::into_raw(boxed)
}
