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

    kissa_core_event::apply(&lua)?;

    lua.load(fs::read("./lua/init.lua").expect("main.lua消失了？"))
        .exec()?;
    lua.load(fs::read("./lua/main.lua").expect("main.lua消失了？"))
        .exec()?;

    return Ok(());
}
