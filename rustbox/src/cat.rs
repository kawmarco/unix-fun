use std::fs::File;
use std::io;

const READ_BUFFER_SIZE: usize = 4096;

pub fn main(args: Vec<String>) -> io::Result<()> {
    // if no arguments are passed, read from stdin
    if args.len() == 0 {
        return _write(&mut io::stdin(), &mut io::stdout());
    }

    // else, open each file and pipe contents to stdout
    _catenate(args, &mut io::stdout())
}

fn _write(f_in: &mut impl io::Read, f_out: &mut impl io::Write) -> io::Result<()> {
    let mut buf = [0; READ_BUFFER_SIZE];

    loop {
        // read up to READ_BUFFER_SIZE bytes
        let bytes_read = f_in.read(&mut buf)?;

        // Check if entire file was consumed
        if bytes_read == 0 {
            break;
        }

        // Write to stdout
        match f_out.write(&buf[..bytes_read]) {
            Err(e) => return Err(e),
            Ok(_) => (),
        }
    }

    Ok(())
}

fn _catenate(filenames: Vec<String>, f_out: &mut impl io::Write) -> io::Result<()> {
    // Concatenate all files in filenames and write them to f_out

    for filename in filenames {
        match File::open(&filename) {
            Err(e) => {
                // On errors, add filename to the error message
                return Err(std::io::Error::new(
                    e.kind(),
                    format!("Failed to open file '{}': {}", filename, e),
                ));
            }
            Ok(mut f) => _write(&mut f, f_out)?,
        };
    }

    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fs::File;
    use std::io::{BufReader, Write};
    use tempfile::tempdir;

    #[test]
    fn _test_write() {
        // Simple test checking if stdin is being copied to stdout
        // Reference: https://stackoverflow.com/a/48393114
        let mut stdin = BufReader::new("Hello!".as_bytes());
        let mut stdout = Vec::new();

        let ret = _write(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout, b"Hello!");
    }

    #[test]
    fn _test_write_large() {
        // Simple test checking if stdin is being copied to stdout
        // for stdin sizes greater than READ_BUFFER_SIZE
        let stdin_contents = vec![b'a'; READ_BUFFER_SIZE + 123];

        let mut stdin = BufReader::new(stdin_contents.as_slice());
        let mut stdout = Vec::new();

        let ret = _write(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout[0..3], *b"aaa");
        assert_eq!(stdout[READ_BUFFER_SIZE + 122], b'a');
        assert_eq!(stdout, stdin_contents);
    }

    #[test]
    fn _test_write_empty() {
        // Ensure it behaves correctly if input is empty
        let mut stdin = BufReader::new("".as_bytes());
        let mut stdout = Vec::new();

        let ret = _write(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout, b"");
    }

    #[test]
    fn _test_catenate_empty() {
        // Zero concatenated files should result in an empty stdout
        let filenames: Vec<String> = Vec::new();
        let mut stdout = Vec::new();

        let ret = _catenate(filenames, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout, b"");
    }

    #[test]
    fn _test_catenate_error() {
        // Try opening a non-existing file
        let filenames: Vec<String> = vec![(*"/sakljdslakd/asdkljaslkdj").to_string()];
        let mut stdout = Vec::new();

        let ret = _catenate(filenames, &mut stdout);

        // Ensure it returned an error and printed nothing to stoud
        assert!(ret.is_err());
        assert_eq!(stdout, b"");

        // Ensure it printed friendly error message
        let re_error = Regex::new(r"^Failed to open file '/sakljdslakd/asdkljaslkdj': .*").unwrap();
        let error_message = ret.err().unwrap().to_string();
        assert!(re_error.is_match(error_message.as_str()));
    }

    #[test]
    fn _test_catenate_files() -> Result<(), std::io::Error> {
        // Create temporary directory and test files
        let dir = tempdir()?;

        let filename_a = dir.path().join("filename_a");
        let mut file = File::create(&filename_a)?;
        write!(file, "AAA")?;

        let filename_b = dir.path().join("filename_b");
        let mut file = File::create(&filename_b)?;
        write!(file, "BBB")?;

        // Build temporary file list
        let filenames: Vec<String> = vec![
            filename_a.into_os_string().into_string().unwrap(),
            filename_b.into_os_string().into_string().unwrap(),
        ];
        let mut stdout = Vec::new();

        // Concatenate files
        let ret = _catenate(filenames, &mut stdout);

        // Ensure stdout is as expected
        assert!(ret.is_ok());
        assert_eq!(stdout, b"AAABBB");

        Ok(())
    }
}
