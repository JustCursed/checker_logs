use actix_web::{get, web, App, HttpServer, Responder};
use std::fs::File;
use std::io::prelude::*;
use std::{ thread, time };
use tokio::{ io, fs };

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
			thread::sleep(time::Duration::from_millis(1000));
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

async fn updater_logs() {
	let resp = reqwest::get("https://logs3.shadowcraft.ru/Hitech_public_logs/08-06-2022.txt")
		.await
		.expect("pizda1")
		.text()
		.await
		.expect("pizda2");
	println!("proshlo!");
	fs::write("logs/shadowcraft_magic1710.txt", resp).await.unwrap();
}