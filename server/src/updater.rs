use tokio::fs;

pub async fn updater_logs() {
	let d = chrono::offset::Local::now().to_string();
	let q: Vec<&str> = d.split(" ").collect();
	let t: Vec<&str> = q[0].split("-").collect();
	let resp = reqwest::get(format!("https://logs3.shadowcraft.ru/Hitech_public_logs/{}-{}-{}\
	.txt", t[2], t[1], t[0]))
		.await
		.expect("pizda1")
		.text()
		.await
		.expect("pizda2");
	println!("proshlo!");
	fs::write("logs/shadowcraft_magic1710.log", resp).await.unwrap();
}