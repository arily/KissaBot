use std::any::Any;

pub enum EventType<'a> {
    Message(&'a str),
    Unknown(&'a str, *mut dyn Any),
}
pub trait KissaPlugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn load(&mut self) -> Vec<EventType> {
        vec![]
    }
    fn on_event(&mut self, event: EventType) -> Vec<EventType>;
}

pub type PluginCreater = fn() -> *mut dyn KissaPlugin;

pub struct TestEvent {
    pub message: String,
}