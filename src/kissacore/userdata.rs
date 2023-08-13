use mlua::prelude::*;
pub struct KissaEventNode {
    name: String,
}

impl LuaUserData for KissaEventNode {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_function_get("listeners", |lua, this| {
            let list = this.user_value::<LuaValue>()?;
            let list = match list {
                LuaNil => {
                    let new_list = lua.create_table()?;
                    let list = new_list.clone();
                    this.set_user_value(new_list)?;
                    list
                }
                LuaValue::Table(t) => t,
                _ => panic!("listeners? no!"),
            };
            Ok(list)
        });
        fields.add_field_method_set("name", |_, this, value: String| {
            this.name = value;
            Ok(())
        });
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function(
            "addlistener",
            |lua, (this, fun): (LuaAnyUserData, LuaFunction)| {
                let list = this.user_value::<LuaValue>()?;
                let list = match list {
                    LuaNil => {
                        let new_list = lua.create_table()?;
                        let list = new_list.clone();
                        this.set_user_value(new_list)?;
                        list
                    }
                    LuaValue::Table(t) => t,
                    _ => panic!("listeners? no!"),
                };
                list.raw_insert(list.raw_len() + 1, fun)?;
                Ok(())
            },
        );
        methods.add_function(
            "invoke",
            |lua, (this, param): (LuaAnyUserData, LuaTable)| {
                let list = this.user_value::<LuaValue>()?;
                let list = match list {
                    LuaNil => {
                        let new_list = lua.create_table()?;
                        let list = new_list.clone();
                        this.set_user_value(new_list)?;
                        list
                    }
                    LuaValue::Table(t) => t,
                    _ => panic!("listeners? no!"),
                };
                for v in list.pairs::<usize, LuaFunction>() {
                    let (_, v) = v?;
                    v.call::<LuaTable, _>(param.clone())?;
                }
                Ok(())
            },
        );
    }
}
impl KissaEventNode {
    pub fn new(event_name: String) -> Self {
        KissaEventNode { name: event_name }
    }
}
