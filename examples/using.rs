use arfbuzz::harfbuzz::{self, hb_buffer_create};

fn main() {
    let buffer = unsafe { hb_buffer_create() };
}
