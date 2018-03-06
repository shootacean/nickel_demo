#[macro_use]    // nickelクレイトのマクロを全てロードする
extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

/// 参考: https://github.com/nickel-org/nickel.rs/blob/master/examples/template.rs
/// 'mv や 'conn はどういうものか？
fn handler<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("message", "Hello, world!");
    // render()が MiddlewareResultを返す。
    // MiddlewareResult とは何なのか？
    return res.render("assets/hello.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();
    server.get("/hello", handler);
    // なぜ、こっちだと動かない？
    // server.get("/", middleware! {|_, res|
    //     let mut data = HashMap::<&str, &str>::new();
    //     data.insert("message", "Hello, world!");
    //     return res.render("assets/hello.tpl", &data);
    // });
    server.listen("127.0.0.1:6767").unwrap();
}
