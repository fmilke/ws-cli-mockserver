#![allow(dead_code)]

use adapters::{test::TestAdapter, ws::WebSocketAdapter};
use app::App;
mod adapters;
mod parser;
mod mocks;
mod app;
mod ui;

fn main() -> anyhow::Result<()> {

    let mut app = App::default();
    //app.add(Box::new(WebSocketAdapter::create()?));
    app.add(Box::new(TestAdapter::default()));
    app.run();

    Ok(())
}
