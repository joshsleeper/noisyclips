// https://github.com/aweinstock314/rust-clipboard/blob/master/src/x11_clipboard.rs
// https://github.com/aweinstock314/rust-clipboard/blob/master/src/x11_clipboard.rs#L45
// https://doc.rust-lang.org/core/marker/struct.PhantomData.html
// https://docs.rs/x11-clipboard/0.3.0/x11_clipboard/
// https://github.com/cdown/clipnotify/blob/master/clipnotify.c
// https://github.com/erlepereira/x11-rs/blob/master/src/xfixes.rs

#[link(name = "clipnotify")]
extern "C" {
    fn wait_for_clipboard_event() -> i32;
}

fn main() {
    unsafe {
        println!("Return value of wait_for_clipboard_event: {}", wait_for_clipboard_event());
    }
    // println!("wow")
}
// https://blog.jfo.click/calling-a-c-function-from-rust/
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

// https://github.com/fitzgen/bindgen-tutorial-bzip2-sys/tree/master/src
