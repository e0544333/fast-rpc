// for reading the contents of a file to a string
use std::fs;
// get access to certain traits that let us read from and write to the stream.
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;
use fast_rpc::ThreadPool;

// Code adapted from https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn main() {
    // returns an iterator that gives us a sequence of streams. More specifically,
    //streams of type TcpStream.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    // When stream goes out of scope and is dropped at the end of the loop,
    // the connection is closed as part of the drop implementation.
}

fn handle_connection(mut stream: TcpStream) {
    // stream internal state might change. Therefore, declared as mut
    // declared a buffer on the stack to hold the data that is read in.
    // might expand when handling requests of arbitrary size.
    let mut buffer = [0; 1024];

    // read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();

    // transformed get into a byte string by adding the b"" byte string
    // syntax at the start of the content data.
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // reading the contents of a file to a string
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    // format! to add the file’s contents as the body of the success response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    // calling unwrap to terminate our program if the stream has any errors.
    // TODO: add error handling methods if stream has errors.
    stream.write(response.as_bytes()).unwrap();

    // flush will wait and prevent the program from continuing
    // until all the bytes are written to the connection.
    stream.flush().unwrap();
}
