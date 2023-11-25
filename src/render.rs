use maud::Markup;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn render_into(output: Markup, path: &Path) {
    let mut handle = fs::File::create(path).unwrap();
    handle.write_all(output.into_string().as_bytes()).unwrap();
}
