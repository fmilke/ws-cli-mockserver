use std::{io::{ErrorKind, Read, Write}, net::{TcpListener, ToSocketAddrs}};
use std::net::TcpStream;
use super::common::{Adapter, Line};

#[derive(Debug)]
pub struct TcpAdapter {
    listener: TcpListener,
    streams: Vec<TcpStream>,
    lines: Vec<Line>,
    buffer: Box<[u8;1024]>,
}

impl Adapter for TcpAdapter {
    fn get_lines(&mut self) -> Option<Vec<Line>> {
        self.update_connections();
        self.check_streams();

        if self.lines.len() > 0 {
            let v = std::mem::replace(&mut self.lines, vec![]);
            Some(v)
        } else {
            None
        }
            
    }

    fn status(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn send_message(&mut self, input: &String) {
        let mut len = self.streams.len();
        let mut i = 0;
        let mut lines = vec![];
        eprintln!("trying to write to {} streams", len);

        while i < len {
            let mut s = self.streams.get(i).unwrap();

            match s.write_all(input.as_str().as_bytes()) {
                Ok(_) => {
                    lines.push(format!("sent message"));
                    i = i + 1;
                },
                Err(e) => {
                    eprintln!("error writing to stream {}", e);
                    self.streams.remove(i);
                    len = len - 1;
                },
            }
        }
    }
}

impl TcpAdapter {
    pub fn from_addr(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
        // we need to move this into a thread
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let intro = Line::new_log(format!("listening at {}", listener.local_addr().unwrap()));

        Ok(Self {
            buffer: Box::new([0u8;1024]),
            streams: vec![],
            lines: vec![intro],
            listener,
        })
    }

    fn update_connections(&mut self) {
        match self.listener.accept() {
            Ok((stream, addr)) => {
                eprintln!("accepting connection");
                self.lines.push(Line::new_log(format!("connected with {}", addr)));
                stream.set_nonblocking(true).expect("to enable non-blocking");
                self.streams.push(stream);
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::WouldBlock => {},
                    _ => {
                        self.lines.push(Line::new_log(format!("unexpected error: (kind: {}) {}", e.kind(), e)));
                    },
                }
            }
        }
    }

    fn check_streams(&mut self) {
        let mut len = self.streams.len();
        let mut i = 0;
        while i < len {
            let s = self.streams.get_mut(i).unwrap();
            match s.read(&mut *self.buffer) {
                Ok(mut bytes) => {
                    eprintln!("read {} bytes from stream", bytes);
                    if bytes == 0 {
                        eprintln!("closing {}-th stream", i);
                        self.streams.remove(i);
                        len = len - 1;
                        continue;
                    } else {
                        i = i + 1;
                    }

                    if self.buffer[bytes] == ('\0' as u8) && bytes > 0 {
                        eprintln!("byte was {}", self.buffer[bytes-1]);
                        bytes = bytes - 1;
                    }

                    self.lines.push(Line::new_log(format!("read {} bytes from stream", bytes)));
                    let msg = String::from_utf8_lossy(&self.buffer.as_slice()[..bytes]);
                    self.lines.push(Line::new_log(msg.to_string()));
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::WouldBlock => {},
                        _ => {
                            self.lines.push(Line::new_log(format!("could not read from stream: (kind: {}) {}", e.kind(), e)));
                        }
                    };
                },
            }
            i = i + 1
        }
    }
}
