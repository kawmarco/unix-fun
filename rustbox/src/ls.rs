use std::fs;
use std::io;

pub fn main(args: Vec<String>) -> io::Result<()> {
    // if no arguments are passed, use current working dir
    let cwd = vec![".".to_string()];
    let mut files: &Vec<String> = &args;
    if files.len() == 0 {
        files = &cwd;
    }

    _ls(files, &mut io::stdout())
}

fn _ls(files: &Vec<String>, stdout: &mut impl io::Write) -> io::Result<()> {
    for filename in files {
        // Check whether this is a regular file or a directory
        let stat = std::fs::metadata(filename)?;

        if stat.is_dir() {
            // If it's a directory, list every entry inside it
            for entry in fs::read_dir(filename)? {
                writeln!(stdout, "{}", entry?.path().display())?;
            }
        } else {
            // Just print out the filename for all other file types
            writeln!(stdout, "./{}", filename)?
        }
    }

    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn _test_ls_cwd_empty() -> Result<(), std::io::Error> {
        // Create empty temporary directory
        let dir = tempdir()?;
        std::env::set_current_dir(&dir)?;

        let mut stdout = Vec::new();
        let ret = _ls(&vec![".".to_string()], &mut stdout);

        // Ensure stdout is empty
        assert!(ret.is_ok());
        assert_eq!(stdout, b"");

        Ok(())
    }

    #[test]
    fn _test_ls_cwd() -> Result<(), std::io::Error> {
        // Create temporary directory and test files
        let dir = tempdir()?;
        std::env::set_current_dir(&dir)?;

        let filename_a = dir.path().join("filename_a");
        File::create(&filename_a)?;

        let filename_b = dir.path().join("filename_b");
        File::create(&filename_b)?;

        let mut stdout = Vec::new();
        let ret = _ls(&vec![".".to_string()], &mut stdout);

        // Ensure stdout is as expected
        assert!(ret.is_ok());
        assert_eq!(stdout, b"./filename_a\n./filename_b\n");

        Ok(())
    }

    #[test]
    fn _test_ls_regular_file() -> Result<(), std::io::Error> {
        // Create temporary directory and test files
        let dir = tempdir()?;
        std::env::set_current_dir(&dir)?;

        let filename_a = dir.path().join("filename_a");
        File::create(&filename_a)?;

        let mut stdout = Vec::new();
        let ret = _ls(&vec!["filename_a".to_string()], &mut stdout);

        // Ensure stdout is as expected
        assert!(ret.is_ok());
        assert_eq!(stdout, b"./filename_a\n");

        Ok(())
    }
}
