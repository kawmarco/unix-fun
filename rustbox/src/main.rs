use std::env;
use std::path::Path;

// subcommand modules
mod echo;

fn main() {
    let argv: Vec<String> = env::args().collect();

    // Get executable basename (i.e. `basename $0`)
    let executable_name: &str = Path::new(&argv[0]).file_name().unwrap().to_str().unwrap();

    // Check if this was invoked as `rustbox $subcommand` (e.g. `rustbox echo "blah"`) or
    // if the executable is named as $subcommand (e.g. `echo "blah"`)
    match (executable_name, argv.len() >= 2) {
        ("rustbox", false) => fail("Expected subcommand", 2), // TODO: print usage
        ("rustbox", true) => dispatch_subcommand(argv[1].as_str(), argv[2..].to_vec()),
        (_, _) => dispatch_subcommand(executable_name, argv[1..].to_vec()),
    }
}

fn dispatch_subcommand(executable_name: &str, args: Vec<String>) {
    // Dispatch subcommand corresponding to executable_name
    match executable_name {
        "echo" => echo::main(args),
        unknown => fail(format!("Unknown subcommand: '{}'", unknown).as_str(), 2),
    }
}

fn fail(reason: &str, exit_code: i32) {
    eprintln!("ERROR: {}", reason);
    std::process::exit(exit_code);
}
