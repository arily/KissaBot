#![feature(type_name_of_val)]
use dynamic_reload::DynamicReload;
use kissa::{PluginCreater, TestEvent};
use std::time::Duration;
fn main() {
    let mut reload_handler = DynamicReload::new(
        Some(vec!["./test_plugin/target/debug"]),
        Some("./"),
        dynamic_reload::Search::Default,
        Duration::from_secs(2),
    );

    match unsafe { reload_handler.add_library("test_plugin", dynamic_reload::PlatformName::Yes) } {
        Ok(lib) => {
            let creater: dynamic_reload::Symbol<PluginCreater> =
                unsafe { lib.lib.get(b"_create").unwrap() };
            let plugin = creater();
            let mut plugin = unsafe { Box::from_raw(plugin) };
            let event = plugin.load();
            match &event[0] {
                kissa::EventType::Message(_) => {}
                kissa::EventType::Unknown(_, b) => {
                    let dynany = unsafe { Box::from_raw(*b) };
                    let event = dynany.downcast::<TestEvent>().unwrap();
                    println!("{}", event.message);
                }
            }
        }
        Err(e) => panic!("{}", e),
    }
}
