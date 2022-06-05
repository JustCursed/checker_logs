use actix_web::{get, App, HttpServer, Responder};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tokio::io;

#[get("/{project}/{server}/index.html")]
async fn index() -> impl Responder {
	let path = Path::new("./logs/");
	return cat(path);
}

#[actix_web::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| App::new().service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
//
// fn main() {
// 	let path = "paaaaa";
// 	println!("{:?} ", cat(".gitignore".as_ref()));
// }
//
// fn cat(path: &Path) -> io::Result<File> {
//     let mut f = File::open(path);
//     let mut s = String::new();
//     return f.into_boxed_str();
// }