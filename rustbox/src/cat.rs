use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};

const READ_BUFFER_SIZE: usize = 4096;

pub fn main(args: Vec<String>) -> io::Result<()> {
    // if no arguments are passed, read from stdin
    if args.len() == 0 {
        return write_to_stdout(BufReader::new(io::stdin()));
    }

    // else, open each file and pipe contents to stdout
    for filename in args {
        match File::open(&filename) {
            Err(e) => {
                return Err(std::io::Error::new(
                    e.kind(),
                    format!("Failed to open file '{}': {}", filename, e),
                ))
            }
            Ok(f) => write_to_stdout(BufReader::new(f))?,
        };
    }

    Ok(())
}

pub fn write_to_stdout<R>(mut f: BufReader<R>) -> io::Result<()>
where
    R: std::io::Read,
{
    let mut buf = [0; READ_BUFFER_SIZE];

    loop {
        // read up to READ_BUFFER_SIZE bytes
        let bytes_read = f.read(&mut buf)?;

        // Check if entire file was consumed
        if bytes_read == 0 {
            break;
        }

        // Write to stdout
        match io::stdout().write(&buf[..bytes_read]) {
            Err(e) => return Err(e),
            Ok(_) => (),
        }
    }

    Ok(())
}
