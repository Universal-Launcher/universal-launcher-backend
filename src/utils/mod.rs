pub mod errors;
pub mod hash;

#[derive(Clone)]
pub struct Data {
    pub hash: Box<hash::Hash>,
    pub app_secret: String,
}

pub fn load_data() -> Result<Data, String> {
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not set");

    let data = Data {
        hash: Box::new(hash::Hash::new()),
        app_secret,
    };

    Ok(data)
}
