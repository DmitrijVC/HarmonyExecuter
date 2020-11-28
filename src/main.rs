extern crate libloading as lib;
use std::process::Command;

fn pause() {
    print!("\n");
    let _ = Command::new("cmd.exe").arg("/C").arg("pause").status();
}

fn main() {
    println!("Starting Harmony");
    let dll = match lib::Library::new(r"HarmonyRS.dll") {
        Ok(result) => result,
        Err(error) => {
            println!("Error: {}, can't find HarmonyRS.dll file!", error.to_string());
            pause();
            return;
        }
    };

    unsafe {
        let func: lib::Symbol<unsafe extern "C" fn()> = dll.get(b"Start").unwrap();
        func();
    }

    pause();
}
