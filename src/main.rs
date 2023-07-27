use rlua::prelude::*;
use rlua::{Lua, Result};
use std::fs;
fn main() -> Result<()> {
    //creat lua环境和 kissa_table
    let lua = unsafe { Lua::unsafe_new_with(rlua::StdLib::all()) };
    lua.context(|lua_ctx| {
        let kissa_table = lua_ctx.create_table()?;
        //注册 the function in the table
        kissa_table.set(
            "print",
            lua_ctx.create_function(|_lua: LuaContext<'_>, v: String| {
                print!("{}", v);
                Ok(())
            })?,
        )?;
        //将kissa_table存储到 lua环境全局kissa变量中
        lua_ctx.globals().set("kissa", kissa_table)?;
        let init_script = fs::read_to_string("./init.lua").expect("#114514 消失的init");
        let main_script = fs::read_to_string("./main.lua").expect("#1919810 消失的main");
        //在环境中执行init script
        lua_ctx.load(&init_script).exec()?;
        //在环境中执行main script
        lua_ctx.load(&main_script).exec()?;
        Ok(())
    })?;
    return Ok(());
}
