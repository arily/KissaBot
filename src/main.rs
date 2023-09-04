#![feature(type_name_of_val)]
use dynamic_reload::DynamicReload;
use kissa::{EventType, KissaPlugin, PluginCreator};
use std::time::Duration;
fn main() {
    let mut reload_handler = DynamicReload::new(
        Some(vec![
            "./plugins/test_ipc_handler/target/debug",
            "./plugins/test_ipc_publish/target/debug",
            "./plugins/test_clashed_publish/target/debug",
        ]),
        Some("./"),
        dynamic_reload::Search::Default,
        Duration::from_secs(2),
    );

    let plugins = &vec![
        "test_ipc_handler",
        "test_ipc_publish",
        "test_clashed_publish",
    ];

    let mut instances: Vec<Box<dyn KissaPlugin>> = vec![];

    for plugin in plugins {
        match unsafe { reload_handler.add_library(plugin, dynamic_reload::PlatformName::Yes) } {
            Ok(lib) => {
                let creator: dynamic_reload::Symbol<PluginCreator> =
                    unsafe { lib.lib.get(b"_create").unwrap() };
                let plugin = creator();
                instances.push(plugin);
            }
            Err(e) => panic!("{}", e),
        }
    }

    let mut created_events: Vec<EventType> = vec![];
    for instance in &instances {
        created_events.append(&mut instance.load())
    }
    for instance in &instances {
        for e in &created_events {
            instance.on_event(e);
        }
    }
}
