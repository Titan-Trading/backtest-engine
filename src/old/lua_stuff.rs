// fn get_setting_value<'lua>(lua_ctx: rlua::Context<'lua>, strategy: Strategy, key: String) -> Result<String> {
        //     let value = strategy.settings.get(&mut key).unwrap();
        //     Ok(value.to_string())
        //     // Ok("test".to_string())
        // }


        // setup the lua script
        // - setup Rust functions that can be called from Lua
        // - setup Lua functions that can be called from Rust
        // - load the lua script
        // lua.context(|lua_ctx| {
        //     let print_order = lua_ctx.create_function(|_, s: String| {
        //         println!("order: {}", s);
        //         Ok(())
        //     }).unwrap();

        //     let get_setting = lua_ctx.create_function(|_, setting_key: String| {
        //         let setting_value = get_setting_value(&mut settings, &setting_key);
        //         Ok(setting_value)
        //     }).unwrap();

        //     lua_ctx.globals().set("print_order", print_order).unwrap();
        //     lua_ctx.globals().set("get_setting", get_setting).unwrap();

        //     lua_ctx.load(&strategy.lua_script).exec().unwrap();
        // });

        // load the lua stript for the strategy
        /*lua.context(|lua_context| {

            let print_order = lua_context.create_function(|_, s: String| {
                println!("order: {}", s);
                Ok(())
            }).unwrap();

            // let get_setting = lua_context.create_function(|_, key: String| {
            //     let value = &strategy.settings.get(&key).unwrap();
            //     Ok(value)
            // }).unwrap();

            // let get_setting = lua_context.create_function(|_, key: String| {
            //     let value = strategy.settings.get(&key).unwrap();
            //     Ok(value)
            // }).unwrap();

            lua_context.globals().set("print_order", print_order).unwrap();

            lua_context.load(&strategy.lua_script).exec().unwrap();
        });*/