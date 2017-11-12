use std::fs::File;
use std::io::Write;

// write string to file
pub fn write_to_file(mut file: &File, s: String)
{
    file.write_all(s.as_bytes()).expect("Unable to write data");
}
