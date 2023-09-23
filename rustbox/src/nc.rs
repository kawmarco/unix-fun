use std::io;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

/*
 * Simple nc implementation, TCP client only (for now)
 */

const BUFFER_SIZE: usize = 4096;

pub fn main(args: Vec<String>) -> io::Result<()> {
    // Argument parsing, only supports client mode for now
    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Usage: nc <destination> <port>",
        ));
    }
    let destination = &args[0];
    let port = &args[1];

    // Setup socket stream
    let stream = TcpStream::connect(format!("{}:{}", destination, port))?;

    // Setup channel to signal if either reader/writer flow has
    // finished
    let (ch_finished_tx, ch_finished_rx): (Sender<io::Result<()>>, Receiver<io::Result<()>>) =
        channel();

    // Spawn two threads for both ingress -> stdout and stdin -> egress flows

    // socket -> stdout thread
    let mut stream_reader = stream.try_clone()?;
    let _reader_finished = ch_finished_tx.clone();

    let _reader_thread = thread::spawn(move || {
        let result = _pipe_stream(&mut stream_reader, &mut io::stdout());
        _reader_finished.send(result).unwrap();
    });

    // stdin -> socket thread
    let mut stream_writer = stream.try_clone()?;
    let _writer_finished = ch_finished_tx.clone();

    let _writer_thread = thread::spawn(move || {
        let result = _pipe_stream(&mut io::stdin(), &mut stream_writer);
        _writer_finished.send(result).unwrap();
    });

    // Wait for either reader or writer threads to finish
    let result = ch_finished_rx.recv().unwrap();

    stream.shutdown(Shutdown::Both)?;

    return result;
}

pub fn _pipe_stream(f_in: &mut impl io::Read, f_out: &mut impl io::Write) -> io::Result<()> {
    // This is the same as _write in cat.rs (might make it a common function later)
    let mut buf = [0; BUFFER_SIZE];

    loop {
        // read up to BUFFER_SIZE bytes
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn _test_write() {
        // Simple test checking if stdin is being copied to stdout
        // Reference: https://stackoverflow.com/a/48393114
        let mut stdin = BufReader::new("Hello!".as_bytes());
        let mut stdout = Vec::new();

        let ret = _pipe_stream(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout, b"Hello!");
    }

    #[test]
    fn _test_pipe_stream_large() {
        // Simple test checking if f_in is being copied to f_out
        // for stdin sizes greater than BUFFER_SIZE
        let stdin_contents = vec![b'a'; BUFFER_SIZE + 123];

        let mut stdin = BufReader::new(stdin_contents.as_slice());
        let mut stdout = Vec::new();

        let ret = _pipe_stream(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout[0..3], *b"aaa");
        assert_eq!(stdout[BUFFER_SIZE + 122], b'a');
        assert_eq!(stdout, stdin_contents);
    }

    #[test]
    fn _test_pipe_stream_empty() {
        // Ensure it behaves correctly if input is empty
        let mut stdin = BufReader::new("".as_bytes());
        let mut stdout = Vec::new();

        let ret = _pipe_stream(&mut stdin, &mut stdout);

        assert!(ret.is_ok());
        assert_eq!(stdout, b"");
    }
}
