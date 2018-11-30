use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = "/home/jsleeper/repos/noisyclips";

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    // cc  -c  clipnotifylib.c -o clipnotifylib -I/usr/X11R6/include -L/usr/X11R6/lib -lX11 -lXfixes
    Command::new("gcc").args(&["clipnotify/clipnotify.c", "-c", "-I/usr/X11R6/include", "-L/usr/X11R6/lib", "-lX11", "-lXfixes"])
                       .arg(&format!("{}/clipnotify.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libclipnotify.a", "clipnotify.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=clipnotify");
}
