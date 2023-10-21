use std::io::prelude::Write;

/// Attempt to write to TcpStream and return true, if there were no errors
/// # Examples
/// ```no_run
/// 
///  use std::net::{SocketAddr, TcpListener, IpAddr};
///  use std::str::FromStr;
///
///  fn main() {
///      let addr = SocketAddr::new(IpAddr::from_str("192.168.1.1").unwrap(), 8080);
///      let listener = TcpListener::bind(addr).unwrap();
/// 
///      for stream in listener.incoming() {
///          let stream = stream.unwrap();
/// 
///          if !write_stream(stream.try_clone().unwrap(), "Hello there!\n".to_string()){
///              // Pipe broken - do some internal logic
///             stream.shutdown(std::net::Shutdown::Both);
///         }
///      }
///  }
/// ```
pub fn write_stream(mut stream: std::net::TcpStream, msg: String) -> bool {
    match stream.write_all(msg.as_bytes()) {
        Err(error) => {
            println!("Pipe is broken! Error: {}", error);
            return false;
        }

        Ok(_) => return true,
    }
}
