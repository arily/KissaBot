use ezlua::luaapi::UnsafeLuaApi;
use ezlua::prelude::*;
fn main() -> LuaResult<()> {
    //creat lua环境和 kissa_table
    let lua = Lua::with_open_libs();
    let kissa_table = lua.new_table()?;

    //注册 the function in the table
    kissa_table.set_closure("log", |content: &str| println!("{}", content))?;

    //将kissa_table存储到 lua环境全局kissa变量中
    lua.global().set("kissa", kissa_table.to_lua(&lua)?)?;

    //在环境中执行init script
    lua.do_file("./init.lua");
    //在环境中执行main script
    lua.do_file("./main.lua");
    return Ok(());
}
