// utility focused modules being used by application  but not a feature of application
pub mod fs;
pub mod hash;

use std::path::PathBuf;
use std::env;

pub fn curr_dir() -> PathBuf
{
	return env::current_dir().unwrap();
}