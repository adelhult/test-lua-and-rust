use rlua::{Function, Lua};
use std::fs;

fn main() -> rlua::Result<()>{
    let filename = "test.lua";
    let lua_code = fs::read_to_string(&filename)
        .expect("Failed to read lua code from the file.");
    
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals.set("__author__", "Eli")?;

        lua_ctx.load(&lua_code).exec()?;
        
        let lua_main: Function = globals.get("main")?;
        lua_main.call(())?;
        
        Ok(())
    })?;

    Ok(())
}