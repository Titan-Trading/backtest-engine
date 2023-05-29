use rlua::{Result as LuaResult, FromLuaMulti, ToLuaMulti, Value, Function, Context as LuaContext, ToLua};


pub struct LuaHook {}

impl LuaHook {
    // create a new rust hook called from within lua (call rust function in lua script)
    pub fn new_external<C, P, R>(lua_ctx: &LuaContext, name: &str, f: C)
    where
        C: 'static + Send + Fn(P) -> LuaResult<R>,
        P: for<'lua> FromLuaMulti<'lua> + 'static + Send,
        R: for<'lua> ToLuaMulti<'lua> + 'static + Send,
    {
        // create a rust hook to be called from with lua script
        let create_f_result = lua_ctx.create_function(move |ctx: LuaContext, arg: P| {
            f(arg)
        });

        // check for errors
        if let Err(e) = &create_f_result {
            println!("Error loading global from lua script: {:?}", e);
        }

        // unwrap the function
        let function = create_f_result.unwrap();

        // set the function as a global
        let set_result = lua_ctx.globals().set(name, function);
        if let Err(e) = set_result {
            println!("Error loading global from lua script: {:?}", e);
        }
    }

    // call a lua function from rust
    pub fn call<'lua>(lua_ctx: &Box<LuaContext<'lua>>, name: &String, params: Vec<Value<'lua>>) -> LuaResult<Value<'lua>> {
        // get the function from the lua script
        let get_result = lua_ctx.globals().get::<_, Function>(name.as_str());
        if let Err(e) = &get_result {
            println!("Error loading global from lua script: {:?}", e);
        }
        let f: Function = get_result.unwrap();

        // call the function
        let call_result: LuaResult<Value<'lua>> = f.call::<_, Value>(params);
        if let Err(e) = &call_result {
            println!("Error executing lua function: {} {:?}", name, e);
        }
        let value: _ = call_result.unwrap();
        let result = value.to_lua(**lua_ctx).unwrap();

        // return the result
        Ok(result)
    }
}