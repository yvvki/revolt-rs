#[tokio::main]
async fn main() {
    let mut client = revolute::Client::new().await;

    let mut token = String::new();

    println!("Token: ");
    std::io::stdin().read_line(&mut token).unwrap();

    client.login_bot(token).await.unwrap();
}
