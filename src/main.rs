use rustify::app::Model;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut model = Model::new().unwrap();

    model.application_loop().await;

    Ok(())
}
