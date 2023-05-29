use std::os::raw::{c_char, c_int, c_void};
use std::sync::{Arc, Mutex};
use std::panic::{catch_unwind, RefUnwindSafe};

#[repr(C)]
struct lua_State([u8; 496]);

#[link(name = "luajit-5.1")]
extern "C" {
    fn lua_newstate() -> *mut lua_State;
    fn lua_close(L: *mut lua_State);
}

type LuaState = *mut lua_State;

extern "C" {
    fn luaL_newstate() -> LuaState;
    fn luaL_openlibs(L: LuaState);
    fn luaL_loadbuffer(L: LuaState, buff: *const c_char, sz: usize, name: *const c_char) -> c_int;
    fn lua_pcall(L: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
    fn lua_tolstring(L: LuaState, idx: c_int, len: *mut usize) -> *const c_char;
    fn lua_gettop(L: LuaState) -> c_int;
    fn lua_settop(L: LuaState, idx: c_int);
}

fn catch_lua_panic<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce() -> R + RefUnwindSafe,
{
    match catch_unwind(f) {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("Panic in Lua script: {:?}", err)),
    }
}

pub struct LuaJIT {
    lua: Arc<Mutex<*mut lua_State>>,
}

impl LuaJIT {
    pub fn new() -> LuaJIT {
        let lua = unsafe { lua_newstate() };
        LuaJIT {
            lua: Arc::new(Mutex::new(lua)),
        }
    }

    pub fn exec<'a>(&mut self, script: &str) -> Result<String, String> {
        let lua = self.lua.lock().unwrap();
        let L = *lua;

        catch_lua_panic(|| {
            unsafe {
                luaL_openlibs(L);
                let cstr = std::ffi::CString::new(script).unwrap();
                luaL_loadbuffer(L, cstr.as_ptr(), script.len(), cstr.as_ptr());
                lua_pcall(L, 0, 1, 0);
                let len: usize = 0;
                let result = lua_tolstring(L, -1, &len);
                let s = std::slice::from_raw_parts(result as *const u8, len);
                String::from_utf8_lossy(s).to_string()
            }
        })
    }
}

impl Drop for LuaJIT {
    fn drop(&mut self) {
        let lua = self.lua.lock().unwrap();
        let L = *lua;
        unsafe {
            lua_close(L);
        }
    }
}

unsafe impl Send for LuaJIT {}
unsafe impl Sync for LuaJIT {}
