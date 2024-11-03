#![allow(dead_code)]

use adapters::{tcp::TcpAdapter, test::TestAdapter, ws::WebSocketAdapter};
use app::App;
mod adapters;
mod parser;
mod app;
mod ui;
mod json;

fn main() -> anyhow::Result<()> {

    let mut app = App::default();
    app.add(Box::new(WebSocketAdapter::from_addr("127.0.0.1:8080")?));
    //app.add(Box::new(TcpAdapter::from_addr("127.0.0.1:8080")?));
    //app.add(Box::new(TestAdapter::default()));
    app.run();

    Ok(())
}

