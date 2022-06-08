use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

use actix_web::{App, get, HttpServer, Responder, web};
use tokio::io;

use updater::*;

#[get("/{project}/{server}/index.html")]
async fn index(params: web::Path<(String, String)>) -> impl Responder {
	let (project, server) = params.into_inner();
	let path = format!("./logs/{}_{}.txt", project, server);
	return cat(path);
}

#[tokio::main]
async fn main() -> io::Result<()> {
	tokio::spawn(async {
		loop {
			updater_logs().await;
			thread::sleep(time::Duration::from_millis(500));
		}
	});
	HttpServer::new(|| App::new().service(index))
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}

fn cat(path: String) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// use chrono::{Datelike, NaiveDate};

// use std::slice::Split;
// use std::str::SplitWhitespace;
//
// fn main() {
// 	// let d = NaiveDate::from_ymd(2015, 3, 14);
// 	let d = chrono::offset::Local::now().to_string();
// 	let q: Vec<&str> = d.split(" ").collect();
// 	let t: Vec<&str> = q[0].split("-").collect();
// 	println!("{}-{}-{}", t[2], t[1], t[0]);
// 	// println!("{:?}", d.to_string());
// 	let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
//     println!("{:?}", v);
// }