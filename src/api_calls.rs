use reqwest::Response;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MSAuthorizationTokenResponse {
    token_type: String,
    expires_in: i32,
    scope: String,
    acess_token: String,
    refresh_token: String,
    user_id: String,
    foci: String,
}

pub async fn get_authorization_token_from_code(
    code: String,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    let res: Response = reqwest::get(format!("https://login.live.com/oauth2_token.srf?")).await?;
    let j = res.json::<MSAuthorizationTokenResponse>().await?;

    println!("json = {:?}", j);
    Ok("salut")
}
