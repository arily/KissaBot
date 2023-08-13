pub mod kissacore;

use kissacore::userdata::*;
use mlua::prelude::*;
use std::fs;

fn main() -> LuaResult<()> {
    //creat lua环境和 kissa_table
    let lua = unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) };
    let t_kissa = lua.create_table()?;
    let f_print = lua.create_function(|_lua, v: String| {
        print!("{}", v);
        Ok(())
    })?;
    t_kissa.set("print", f_print)?;
    lua.globals().set("kissa", t_kissa)?;

    let f_createeventnode =
        lua.create_function(|lua, name: String| Ok(lua.create_userdata(KissaEventNode::new(name))?))?;
    lua.globals().set("EventNode", f_createeventnode)?;

    lua.load(fs::read("./init.lua").expect("init.lua消失了？"))
        .exec()?;
    lua.load(fs::read("./main.lua").expect("main.lua消失了？"))
        .exec()?;

    return Ok(());
}
