use std::{io::ErrorKind, net::{TcpStream, ToSocketAddrs}};
use websocket::{server::{sync::Server, NoTlsAcceptor}, sync::Client, WebSocketError};
use super::common::{Adapter, Direction, Line};

pub struct WebSocketAdapter {
    server: Server<NoTlsAcceptor>,
    streams: Vec<Client<TcpStream>>,
}

const LOG_PREFIX: &str = "ws-adapter:";

impl WebSocketAdapter {
    pub fn from_addr(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {

        let server = Server::bind(addr)?;
        server.set_nonblocking(true)?;

        Ok(WebSocketAdapter{
            server,
            streams: vec![],
        })
    }

    pub fn accept_connections(&mut self) {
        match self.server.accept() {
            Ok(stream) => {
                match stream.accept() {
                    Ok(s) => {
                        s.set_nonblocking(true).unwrap();
                        self.streams.push(s);
                    },
                    Err((_, e)) => {
                        // todo
                    },
                }
            },
            Err(e) => {
                match e {
                    _ => {},
                }
            },
        }
    }

    pub fn check_connections(&mut self) -> Option<Vec<Line>> {
        let mut lines = vec![];

        let mut i = 0;
        let mut len = self.streams.len();

        eprintln!("{} checking connections", LOG_PREFIX);

        while i < len {
            let mut remove_stream = false;
            let s = self.streams.get_mut(i).unwrap();
            match s.recv_message() {
                Ok(message) => {
                    match message {
                        websocket::OwnedMessage::Text(text) => {
                            let log_line = Line::new_json(text, Direction::Incoming);
                            lines.push(log_line);
                        },
                        websocket::OwnedMessage::Binary(_) => {
                            lines.push(Line::new_log(String::from("received a binary message")));
                            eprintln!("{} received binary", LOG_PREFIX);
                        }, websocket::OwnedMessage::Close(_) => { remove_stream = true;
                        },
                        websocket::OwnedMessage::Ping(_) => {
                            eprintln!("{} received ping", LOG_PREFIX);
                        },
                        websocket::OwnedMessage::Pong(_) => {
                            eprintln!("{} received pong", LOG_PREFIX);
                        },
                    }
                },
                Err(e) => {

                    let mut would_block = false;
                    if let WebSocketError::IoError(ref ws_err) = e {
                        would_block = ws_err.kind() == ErrorKind::WouldBlock;
                    }

                    if !would_block {
                        eprintln!("{} encountered error: {}", LOG_PREFIX, e);
                        remove_stream = true;
                    }
                },
            }

            if remove_stream {
                eprintln!("{} removing stream", LOG_PREFIX);
                self.streams.swap_remove(i);
                len = len - 1
            } else {
                i = i + 1
            }
        }

        if lines.len() > 0 {
            Some(lines)
        } else {
            None
        }
    }

}

impl Adapter for WebSocketAdapter {
    fn get_lines(&mut self) -> Option<Vec<Line>> {
        self.accept_connections();
        self.check_connections()
    }
}

