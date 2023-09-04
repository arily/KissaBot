use std::any::Any;

pub enum EventType<'a> {
    Message(&'a str),
    Unknown(&'a str, Box<dyn Any>),
}

pub trait KissaPlugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn load(&self) -> Vec<EventType> {
        vec![]
    }
    fn on_event(&self, event: &EventType) -> Vec<EventType>;
}

pub type PluginCreator = fn() -> Box<dyn KissaPlugin>;
