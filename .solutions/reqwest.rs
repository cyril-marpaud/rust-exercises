const BASE_URL: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let r = reqwest::get(format!("{BASE_URL}/")).await?.text().await?;
	println!("{r}");

	let client = reqwest::Client::new();

	let r = client
		.post(format!("{BASE_URL}/explore"))
		.body("north")
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	let r = client
		.put(format!("{BASE_URL}/cave"))
		.json(&serde_json::json!({"password": "coconut"}))
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	let r = client
		.get(format!("{BASE_URL}/chest"))
		.header("X-CHEST", "inspect")
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	let r = client
		.post(format!("{BASE_URL}/open_chest"))
		.form(&[("code", "3141")])
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	let r = client
		.delete(format!("{BASE_URL}/boat"))
		.basic_auth("captain", Some("crustacean"))
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	let r = client
		.get(format!("{BASE_URL}/return"))
		.header("X-CAP", "home")
		.send()
		.await?
		.text()
		.await?;
	println!("{r}");

	Ok(())
}
