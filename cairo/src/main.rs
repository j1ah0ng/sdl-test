extern crate cairo;

use cairo::{ImageSurface,
            Format,
            Context};
use std::fs::File;

fn main() {
    let s = ImageSurface::create(Format::ARgb32, 600, 600)
        .expect("ohno");
    let ctx = Context::new(&s);
    ctx.set_source_rgb(1.0, 0.0, 0.0);
    ctx.paint();

    let mut file = File::create("out.png")
        .expect("Allow write perms!");
    s.write_to_png(&mut file)
        .expect("Couldn't write to file");
}
