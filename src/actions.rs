use std::fs::File;
use std::io::prelude::*;

use gtk::prelude::*;
use gtk::*;

pub fn save_file(text_view: &TextView) -> std::io::Result<()> {
    let mut outfile = File::create("rjournal.txt")?;

    if let Some(text_buf) = text_view.get_buffer() {
        let (start, end) = text_buf.get_bounds();
        if let Some(text) = text_buf.get_text(&start, &end, false) {
            outfile.write_all(text.as_bytes())?
        }
    }
    Ok(())
}
