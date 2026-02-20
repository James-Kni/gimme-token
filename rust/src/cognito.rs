use crate::config::Profile;
use aws_cognito_srp::{SrpClient, User};
use serde_json::{json, Value};

const COGNITO_SERVICE: &str = "AWSCognitoIdentityProviderService";

fn cognito_post(region: &str, target: &str, body: &Value) -> Result<Value, String> {
    let url = format!("https://cognito-idp.{region}.amazonaws.com/");
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .header("Content-Type", "application/x-amz-json-1.1")
        .header("X-Amz-Target", format!("{COGNITO_SERVICE}.{target}"))
        .json(body)
        .send()
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    let json: Value = resp.json().map_err(|e| e.to_string())?;

    if !status.is_success() {
        let msg = json["message"].as_str().unwrap_or("Unknown error");
        return Err(msg.to_string());
    }

    Ok(json)
}

pub fn authenticate(profile: &Profile) -> Result<String, String> {
    let user = User::new(
        &profile.aws_user_pool,
        &profile.username,
        &profile.password,
    );
    let srp = SrpClient::new(user, &profile.aws_client_id, None);
    let auth_params = srp.get_auth_parameters();

    // Step 1: InitiateAuth
    let init_body = json!({
        "AuthFlow": "USER_SRP_AUTH",
        "ClientId": profile.aws_client_id,
        "AuthParameters": {
            "USERNAME": auth_params.username,
            "SRP_A": auth_params.a
        }
    });

    let init_resp = cognito_post(&profile.aws_region, "InitiateAuth", &init_body)?;

    let challenge = &init_resp["ChallengeParameters"];
    let secret_block = challenge["SECRET_BLOCK"].as_str().ok_or("Missing SECRET_BLOCK")?;
    let user_id = challenge["USER_ID_FOR_SRP"].as_str().ok_or("Missing USER_ID_FOR_SRP")?;
    let salt = challenge["SALT"].as_str().ok_or("Missing SALT")?;
    let srp_b = challenge["SRP_B"].as_str().ok_or("Missing SRP_B")?;

    // Step 2: Verify and build challenge response
    let verification = srp
        .verify(secret_block, user_id, salt, srp_b)
        .map_err(|e| format!("{e:?}"))?;

    let respond_body = json!({
        "ChallengeName": "PASSWORD_VERIFIER",
        "ClientId": profile.aws_client_id,
        "ChallengeResponses": {
            "USERNAME": user_id,
            "PASSWORD_CLAIM_SECRET_BLOCK": verification.password_claim_secret_block,
            "PASSWORD_CLAIM_SIGNATURE": verification.password_claim_signature,
            "TIMESTAMP": verification.timestamp
        }
    });

    let auth_resp = cognito_post(&profile.aws_region, "RespondToAuthChallenge", &respond_body)?;

    let id_token = auth_resp["AuthenticationResult"]["IdToken"]
        .as_str()
        .ok_or("Missing IdToken in response")?;

    Ok(id_token.to_string())
}
