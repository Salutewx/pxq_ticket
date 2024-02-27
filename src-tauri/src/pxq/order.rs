use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    client::{get, post},
    error::PXQError,
};

// #[tauri::command(async)]
// pub async fn get_pending_order_list(app: tauri::Window, page: u8) -> Result<UserProfileResult, PXQError> {
//     let url = "https://m.piaoxingqiu.com/cyy_gatewayapi/user/buyer/v3/profile";
//     let form = json!({
//         "src": "WEB",
//         "ver": "4.0.13-20240223084920"
//     });
//     let data = get(app, url, form)
//         .await
//         .map_err(|_| PXQError::GetUserProfileError)?;
//     let user_profile_result =
//         serde_json::from_value(data).map_err(|_| PXQError::GetUserProfileError)?;
//     Ok(user_profile_result)
// }
