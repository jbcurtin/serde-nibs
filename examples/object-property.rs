use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Content {
	#[serde(rename="Name")]
	name: String,
}
#[tokio::main]
async fn main() {
	let content = r#"{"Name": "my name is, serde"}"#;
	let content: Content = serde_json::from_str(content).unwrap();
	println!("{:?}", content.name);
}