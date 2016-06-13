extern crate docopt;
extern crate rustc_serialize;

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

const USAGE: &'static str = "
Recursively search files for a given pattern.

Usage:
  gimme [options] <pattern>
  gimme --help

Options:
  --help                Show this screen.
  -l --list-filenames   Only show matching filenames, not the lines.
  -w --word             Only match whole words.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_pattern: String,
    flag_list_filenames: bool,
    flag_word: bool,
}

fn visit_dirs<P: AsRef<Path>>(dir: P, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir.as_ref())).is_dir() {
        for entry in try!(fs::read_dir(dir.as_ref())) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
                                    .and_then(|d| d.decode())
                                    .unwrap_or_else(|e| e.exit());

    fn callback(de: &DirEntry) {
        println!("{}", de.path().strip_prefix(".").unwrap().to_str().unwrap());
    }
    visit_dirs(Path::new("."), &callback);
}
