use openapi::apis::{
    configuration::Configuration,
    crate_api::get_joke,
};
    
#[tokio::main]
async fn main() {
    let c = Configuration::new();
    let joke = get_joke(&c, "boo").await.unwrap();
    println!("{:?}", joke);
}
