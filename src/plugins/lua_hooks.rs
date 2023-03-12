use rlua::{Function, Lua, Context, ToLuaMulti, Value, Result, ToLua, MultiValue, UserData, FromLuaMulti};



pub struct LuaHook {

}

impl LuaHook {

    // create a new rust hook called from within lua
    // pub fn new_external<C, R>(lua_ctx: &Context, name: &str, f: C)
    // where
    //     C: 'static + Send + Fn(Context, R) -> Result<()>,
    //     R: for<'lua> rlua::FromLua<'lua> + 'static + Send,
    // {
    //     let create_f_result = lua_ctx.create_function(move |ctx, arg: R| {
    //         f(ctx, arg)
    //     });

    //     if let Err(e) = create_f_result {
    //         println!("Error loading global from Lua script: {}", e);
    //     }

    //     let function = create_f_result.unwrap();

    //     let set_result = lua_ctx.globals().set(name, function);
    //     if let Err(e) = set_result {
    //         println!("Error loading global from Lua script: {}", e);
    //     }
    // }

    // // call a lua function from rust
    // pub fn call<'lua>(lua_ctx: &'lua Context, name: &str, params: MultiValue<'lua>) -> Result<Value<'lua>> {
    //     let get_result = lua_ctx.globals().get(name);
    //     if let Err(e) = get_result {
    //         println!("Error loading global from Lua script: {}", e);
    //     }
    //     let f: Function = get_result.unwrap();

    //     let ret: Result<Value<'lua>> = f.call(params);
    //     if let Err(e) = ret {
    //         println!("Error executing Lua function: {}", e);
    //     }

    //     let result: _ = ret.unwrap();

    //     Ok(result.to_lua(*lua_ctx).unwrap())
    // }
}