use std::net::{Ipv4Addr, SocketAddrV4};
use websocket::server::{sync::Server, NoTlsAcceptor};
use super::common::Adapter;

pub struct WebSocketAdapter {
    server: Server<NoTlsAcceptor>,
}

impl WebSocketAdapter {
    pub fn create() -> anyhow::Result<Self> {

        let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
        let server = Server::bind(addr)?;

        Ok(WebSocketAdapter{
            server,
        })
    }
}

impl Adapter for WebSocketAdapter {

    fn status(&mut self) -> anyhow::Result<()> {


        Ok(())
    }

    fn get_lines(&mut self) -> Option<Vec<String>> {
        None
    }
}

