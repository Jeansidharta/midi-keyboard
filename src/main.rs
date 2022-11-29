use midi::main as crate_main;

#[tokio::main]
async fn main() {
    match crate_main().await {
        Ok(()) => {}
        Err(err) => println!("{}", err),
    }
}
