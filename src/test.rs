use rlua::{Function, Error, Lua, MultiValue};
use rustyline::Editor;
use std::fs;

fn main() {
    let lua_content = fs::read_to_string("test.lua").unwrap();

    Lua::new().context(|lua| {
        
        lua
        .load(
            r#"
            function print_cool()
                print("cool")
            end
        "#,
        )
        .set_name("Testing")
        .unwrap()
        .exec()
        .unwrap();

        let mut editor = Editor::<()>::new();
        
        let globals = lua.globals();
        
        let cool = lua.create_function(|lua, ()| {
            let t = lua.create_table()?;
            t.set(1, "Eli")?;
            t.set(2, "Adelhult")?;
            Ok(t)
        }).unwrap();
        
        globals.set("cool", cool).unwrap();

        lua.load(&lua_content).exec().unwrap();
        
        loop {
            let mut prompt = "> ";
            let mut line = String::new();

            loop {
                match editor.readline(prompt) {
                    Ok(input) => line.push_str(&input),
                    Err(_) => return,
                }

                match lua.load(&line).eval::<MultiValue>() {
                    Ok(values) => {
                        editor.add_history_entry(line);
                        println!(
                            "{}",
                            values
                                .iter()
                                .map(|value| format!("{:?}", value))
                                .collect::<Vec<_>>()
                                .join("\t")
                        );
                        break;
                    }
                    Err(Error::SyntaxError {
                        incomplete_input: true,
                        ..
                    }) => {
                        // continue reading input and append it to `line`
                        line.push_str("\n"); // separate input lines
                        prompt = ">> ";
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        }
    });
}