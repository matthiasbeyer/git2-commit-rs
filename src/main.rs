extern crate git2;
extern crate git2_commit;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use git2::Error;

const USAGE: &'static str = "
git2-commit

Usage:
  git2-commit add [--repository=<path>] <file>...
  git2-commit commit [--repository=<path>] <name> <email> <message>

Options:
  -r <path>, --repository=<path>  Path to the repository's working directory [default: .]
  -h, --help                      Show this screen.
";


#[derive(Debug,RustcDecodable)]
struct Args {
    arg_file: Vec<String>,

    arg_name: String,
    arg_email: String,
    arg_message: String,

    flag_repository: String,
    cmd_add: bool,
    cmd_commit: bool,
}

fn git_add(args: &Args) -> Result<(), Error> {
    let repo = &args.flag_repository;
    let files = &args.arg_file;

    git2_commit::add(repo, files)
}

fn git_commit(args: &Args) -> Result<(), Error> {
    let repo = &args.flag_repository;

    let name = &args.arg_name;
    let email = &args.arg_email;
    let message = &args.arg_message;

    git2_commit::commit(repo, name, email, message)
}

fn run(args: &Args) -> Result<(), Error> {

    if args.cmd_add {
        return git_add(args);
    }

    if args.cmd_commit {
        return git_commit(args);
    }

    Err(Error::from_str("Unknown command"))
}

fn main() {
    //add(".", &["foo.txt"]).unwrap();
    //commit(".", "jer", "foobar", "next commit").unwrap();

    let args : Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),

    }
}
