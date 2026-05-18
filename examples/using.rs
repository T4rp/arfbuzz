use arfbuzz::harfbuzz::{self, hb_buffer_create, hb_buffer_destroy};

fn main() {
    let buffer = unsafe { hb_buffer_create() };
    unsafe {
        hb_buffer_destroy(buffer);
    }
}
