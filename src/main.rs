extern crate docopt;
extern crate rustc_serialize;

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

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
                                    .and_then(|d| d.decode())
                                    .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
