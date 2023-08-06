use std::io;

pub fn main(args: Vec<String>) -> io::Result<()> {
    if args.len() != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "env command accepts no arguments",
        ));
    }

    _print_env(&mut std::env::vars(), &mut io::stdout())
}

fn _print_env(
    env_vars: &mut impl Iterator<Item = (String, String)>,
    stdout: &mut impl io::Write,
) -> io::Result<()> {
    // Print each environment variable's name and value
    for (name, value) in env_vars {
        writeln!(stdout, "{}={}", name, value).unwrap();
    }

    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _test_print_env_empty() {
        let mut stdout = Vec::new();
        let env_vars: Vec<(String, String)> = vec![];

        // Run _print_env() with an empty environment
        let ret = _print_env(&mut env_vars.into_iter(), &mut stdout);

        // Ensure stdout is empty
        assert!(ret.is_ok());
        assert_eq!(stdout, b"");
    }

    #[test]
    fn _test_print_env() {
        let mut stdout = Vec::new();
        let env_vars: Vec<(String, String)> = vec![
            ("name1".to_string(), "value1".to_string()),
            ("name2".to_string(), "value2".to_string()),
        ];

        // Run _print_env() with an empty environment
        let ret = _print_env(&mut env_vars.into_iter(), &mut stdout);

        // Ensure stdout is empty
        assert!(ret.is_ok());
        assert_eq!(stdout, b"name1=value1\nname2=value2\n");
    }

    #[test]
    fn _test_main_accepts_no_args() {
        // Current version accepts no args, simulate passing one...
        let ret = main(vec!["-i".to_string()]);

        // ...and ensure it returned an error
        assert!(ret.is_err());

        let error_message = ret.err().unwrap().to_string();
        assert_eq!(error_message.as_str(), "env command accepts no arguments");
    }
}
