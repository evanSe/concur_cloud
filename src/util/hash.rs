use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn hash<T: Hash>(t: &T) -> u64
{
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn hash_time() -> u64
{
    let now: Duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    return hash(&now.as_secs());
}

// create a hash from a file which we can reference
pub fn hash_file(file: File) -> u64
{
    let mut buf_reader = BufReader::new(file);
    let mut file_ontents = String::new();
    buf_reader.read_to_string(&mut file_ontents);
    return hash(&file_ontents);
}