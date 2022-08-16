use std::{
    fs::{read_to_string, File},
    io::Write,
};

use zephyr::{scraping::get_classes, *};

fn main() {
    tracing_subscriber::fmt::init();

    let html = read_to_string("./website/index.html").unwrap();

    let classes = get_classes(&html);
    let z = Zephyr::new();
    let css = z.generate_classes(classes.iter().map(String::as_str));

    let mut file = File::create("./website/style.css").unwrap();
    file.write_all(css.as_bytes()).unwrap();
}
