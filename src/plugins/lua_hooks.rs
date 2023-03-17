use mlua::{Function, Lua, ToLuaMulti, Value, Result, ToLua, FromLuaMulti};


pub struct LuaHook {}

impl LuaHook {
    // create a new rust hook called from within lua (call rust function in lua script)
    pub fn new_external<C, P, R>(lua: &Lua, name: &str, f: C)
    where
        C: 'static + Send + Fn(P) -> Result<R>,
        P: for<'lua> FromLuaMulti<'lua> + 'static + Send,
        R: for<'lua> ToLuaMulti<'lua> + 'static + Send,
    {
        // create a rust hook to be called from with lua script
        let create_f_result = lua.create_function(move |_, arg: P| {
            f(arg)
        });

        // check for errors
        if let Err(e) = &create_f_result {
            println!("Error loading global from lua script: {:?}", e);
        }

        // unwrap the function
        let function = create_f_result.unwrap();

        // set the function as a global
        let set_result = lua.globals().set(name, function);
        if let Err(e) = set_result {
            println!("Error loading global from lua script: {:?}", e);
        }
    }

    // call a lua function from rust
    pub fn call<'lua>(lua: &'lua Box<Lua>, name: &String, params: Value<'lua>) -> Result<Value<'lua>> {
        let name = name.as_str();
        // get the function from the lua script
        let get_result = lua.globals().get(name);
        if let Err(e) = &get_result {
            println!("Error loading global from lua script: {:?}", e);
        }
        let f: Function = get_result.unwrap();

        // call the function
        let call_result: Result<Value<'lua>> = f.call(params);
        if let Err(e) = &call_result {
            println!("Error executing lua function: {} {:?}", name, e);
        }
        let value: _ = call_result.unwrap();
        let result = value.to_lua(lua).unwrap();

        // return the result
        Ok(result)
    }
}