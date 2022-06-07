use tokio::fs;

pub async fn updater_logs() {
	let resp = reqwest::get("https://logs3.shadowcraft.ru/Hitech_public_logs/08-06-2022.txt")
		.await
		.expect("pizda1")
		.text()
		.await
		.expect("pizda2");
	println!("proshlo!");
	fs::write("logs/shadowcraft_magic1710.txt", resp).await.unwrap();
}