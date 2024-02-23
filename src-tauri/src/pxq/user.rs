use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    client::{get, post},
    error::PXQError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerificationCodeResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhotoCodeData {
    #[serde(rename = "baseCode")]
    base_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratePhoneCodeResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: PhotoCodeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<LoginData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub nickname: String,
    pub avatar: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<UserProfile>,
}


#[tauri::command(async)]
pub async fn send_verification_code(
    app: tauri::Window,
    mobile: String,
    token: String,
) -> Result<SendVerificationCodeResult, PXQError> {
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/send_verify_code";

    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "verifyCodeUseType": "USER_LOGIN",
        "cellphone": mobile,
        "messageType": "MOBILE",
        "token": token
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value::<SendVerificationCodeResult>(data)
        .map_err(|_| PXQError::ParseDataError)?;
    Ok(result)
}

#[tauri::command(async)]
pub async fn generate_photo_code(
    app: tauri::Window,
    mobile: String,
) -> Result<GeneratePhoneCodeResult, PXQError> {
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/generate_photo_code";
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "cellphone": mobile,
        "verifyCodeUseType": "USER_LOGIN",
        "messageType": "MOBILE"
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    let result = serde_json::from_value::<GeneratePhoneCodeResult>(data)
        .map_err(|_| PXQError::ParseDataError)?;
    Ok(result)
}

#[tauri::command(async)]
pub async fn login_by_mobile(
    app: tauri::Window,
    mobile: String,
    sms_code: String,
) -> Result<LoginResult, PXQError> {
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/pub/v3/login_or_register";
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.10-20240221091017",
        "cellphone": mobile,
        "verifyCode": sms_code
    });
    let data = post(app, url, json_data)
        .await
        .map_err(|_| PXQError::ReqwestError)?;
    println!("{:?}", data);
    let login_result =
        serde_json::from_value::<LoginResult>(data).map_err(|_| PXQError::ReqwestError)?;
    Ok(login_result)
}

#[tauri::command(async)]
pub async fn get_user_profile(app: tauri::Window) -> Result<UserProfileResult, PXQError> {
    let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/buyer/v3/profile";
    let form = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920"
    });
    let data = get(app, url, form)
        .await
        .map_err(|_| PXQError::GetUserProfileError)?;
    let user_profile_result =
        serde_json::from_value(data).map_err(|_| PXQError::GetUserProfileError)?;
    Ok(user_profile_result)
}
