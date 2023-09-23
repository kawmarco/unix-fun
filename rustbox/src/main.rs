use std::env as std_env;
use std::path::Path;

// subcommand modules
mod cat;
mod echo;
mod env;
mod ls;
mod nc;

fn main() {
    let argv: Vec<String> = std_env::args().collect();

    // Get executable basename (equivalent to `basename $0`)
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
        "cat" => match cat::main(args) {
            Err(reason) => fail(format!("{}", reason).as_str(), 1),
            Ok(_) => (),
        },
        "ls" => match ls::main(args) {
            Err(reason) => fail(format!("{}", reason).as_str(), 1),
            Ok(_) => (),
        },
        "env" => match env::main(args) {
            Err(reason) => fail(format!("{}", reason).as_str(), 1),
            Ok(_) => (),
        },
        "nc" => match nc::main(args) {
            Err(reason) => fail(format!("{}", reason).as_str(), 1),
            Ok(_) => (),
        },
        unknown => fail(format!("Unknown subcommand: '{}'", unknown).as_str(), 2),
    }
}

fn fail(reason: &str, exit_code: i32) {
    eprintln!("ERROR: {}", reason);
    std::process::exit(exit_code);
}
