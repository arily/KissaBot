#[macro_use]
mod kissa_plugin;

use mlua::prelude::*;
use std::fs;

fn main() -> LuaResult<()> {
    let lua = unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) };

    plugin_apply!(&lua);

    lua.load(fs::read("./lua/init.lua").expect("main.lua消失了？"))
        .exec()?;
    lua.load(fs::read("./lua/main.lua").expect("main.lua消失了？"))
        .exec()?;

    Ok(())
}
