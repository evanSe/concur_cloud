use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::string::String;
use std::fs::File;
use std::fs::ReadDir;
use std::ffi::OsStr;
use util::hash;
// returns file
pub fn read_file(path: &Path) -> Option<File>
{
    let display = path.display();
    let f = match File::open(path) {
        Err(_) => return None,
        Ok(file) => return Some(file),
    };
}

// returns ReadDir
pub fn read_directory(path: &Path) -> ReadDir
{
    let display = path.display();
    match fs::read_dir(path) {
        Err(_) => panic!("couldn't open {}:", display),
        Ok(file) => return file,
    };
}

// pub fn file_exists(dir: ReadDir, name: &String, sub: bool) -> Option<PathBuf>
// {
//     for entry in dir {
//         match entry {
//             Ok (e) => {
//                 let path = &e.path();
//                 let file_name = path.file_name().unwrap().to_str().unwrap();
//                 if e.path().is_file() 
//                     && String::from(file_name).contains(OsStr::new(&name).to_str().unwrap()) {
//                     println!("Found file this is the hash:{}", hash::hash_file(read_file(&e.path())));
//                     let test = String::from(e.path().to_str().unwrap());
//                     Some(e.path().to_path_buf());
//                 }
//                 if e.path().is_dir() && sub
//                 {
//                     Some(file_exists(read_directory(&e.path()), name, sub).unwrap());
//                 }
//             },
//             Err(why) => {
//                 println!("! {:?}", why.kind());
//             },
//         }
//     }
//     None
// }