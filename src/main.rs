use rlua::prelude::*;
use std::fs;
use uuid::Uuid;
fn main() -> LuaResult<()> {
    //creat lua环境和 kissa_table
    let lua = unsafe { Lua::unsafe_new_with_flags(rlua::StdLib::all(), rlua::InitFlags::DEFAULT) };

    lua.context(|lua_ctx| {
        let kissa_table = lua_ctx.create_table()?;
        //注册 the function in the table
        kissa_table.set(
            "print",
            lua_ctx.create_function(|_lua: LuaContext<'_>, v: LuaString| {
                print!("{}", v.to_str()?);
                Ok(())
            })?,
        )?;
        kissa_table.set(
            "newuuid",
            lua_ctx.create_function(|_lua: LuaContext<'_>, ()| {
                let id = Uuid::new_v4().to_string();
                Ok(id)
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
        
        //执行事件循环
        loop {
            let table: LuaTable = lua_ctx.globals().get("kissa").unwrap();
            let funs: LuaTable = table.get("__loop_event__")?;
            let iter = funs.pairs::<String, LuaFunction>();
            for pair in iter {
                match pair {
                    Ok((_k, v)) => {
                        v.call(())?;
                    }
                    _ => {
                        return Ok(());
                    }
                };
            }
        }
        Ok(())
    })?;
    return Ok(());
}
