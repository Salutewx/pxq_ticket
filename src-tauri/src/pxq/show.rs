use std::fmt::format;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::form_urlencoded;

use super::{
    client::{get, post},
    error::PXQError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    #[serde(rename = "searchType")]
    pub search_type: String,

    #[serde(rename = "showId")]
    pub show_id: String,

    #[serde(rename = "showName")]
    pub show_name: String,

    #[serde(rename = "showDate")]
    pub show_date: String,

    #[serde(rename = "cityName")]
    pub city_name: String,

    #[serde(rename = "showStatus")]
    pub show_status: String,

    #[serde(rename = "minOriginalPrice")]
    pub min_original_price: f64,

    #[serde(rename = "posterUrl")]
    pub poster_url: String,

    #[serde(rename = "venueName")]
    pub venue_name: String,

    #[serde(rename = "firstShowTime")]
    pub first_show_time: i128,

    #[serde(rename = "lastShowTime")]
    pub last_show_time: i128,

    #[serde(rename="latestSaleTime")]
    pub latest_sale_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowData {
    #[serde(rename = "isLastPage")]
    pub is_last_page: bool,

    #[serde(rename = "searchData")]
    pub show_list: Vec<Show>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchShowResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<ShowData>,
}

#[tauri::command(async)]
pub async fn search_show_list(
    app: tauri::Window,
    keyword: String,
    sort_type: String,
    page: u8,
) -> Result<SearchShowResult, PXQError> {
    let keyword: String = form_urlencoded::byte_serialize(keyword.as_bytes()).collect();
    let offset = (page - 1) * 10;
    let url = format!("https://m.piaoxingqiu.com/cyy_gatewayapi/home/pub/v3/show_list/search?backendCategoryCode=ALL&cityId=4455&keyword={}&length=10&offset={}&pageType=SEARCH_PAGE&sortType={}&src=WEB&ver=4.0.13-20240223084920", 
    keyword, offset, sort_type);
    let form = json!({});
    let data = get(app, url.as_str(), form)
        .await
        .map_err(|_| PXQError::SearchShowError)?;

    let result =
        serde_json::from_value::<SearchShowResult>(data).map_err(|_| PXQError::SearchShowError)?;
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeatPlan {

    #[serde(rename="seatPlanId")]
    pub seat_plan_id: String,

    #[serde(rename="stdSeatPlanId")]
    pub std_seat_plan_id: String,

    #[serde(rename="originalPrice")]
    pub original_price: f64,

    #[serde(rename="seatPlanName")]
    pub seat_plan_name: String,    

    #[serde(rename="hasActivity")]
    pub has_activity: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Session {

    #[serde(rename="showLimit")]
    pub show_limit: i32,

    #[serde(rename="showId")]
    pub show_id: String,

    #[serde(rename="showName")]
    pub show_name: String,


    #[serde(rename="bizShowSessionId")]
    pub session_id: String,

    #[serde(rename="sessionName")]
    pub session_name: String,

    #[serde(rename="hasActivity")]
    pub has_activity: bool,

    #[serde(rename="hasSessionSoldOut")]
    pub has_session_sold_out: bool,

    #[serde(rename="seatPlans")]
    pub seat_plans: Vec<SeatPlan>

}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryShowSessionsResult{
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: Option<Vec<Session>>
}

#[tauri::command(async)]
pub async fn query_show_sessions(app: tauri::Window, show_id: String) -> Result<QueryShowSessionsResult, PXQError>  {
    let url = format!("https://m.piaoxingqiu.com/cyy_gatewayapi/show/pub/v5/show/{}/sessions", show_id);
    let form = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "source": "FROM_QUICK_ORDER",
        "isQueryShowBasicInfo": true,
    });
    let data = get(app, url.as_str(), form).await.map_err(|_|PXQError::ReqwestError)?;

    let result =
    serde_json::from_value::<QueryShowSessionsResult>(data).map_err(|_| PXQError::QueryShowSessionsError)?;
Ok(result)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  AddReminderData {
    pub subscribed: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddReminderResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: AddReminderData,
}

#[tauri::command(async)]
pub async fn add_reminder(app: tauri::Window, show_id: String, session_id: String) -> Result<AddReminderResult, PXQError> {
    let url = format!("https://m.piaoxingqiu.com/cyy_gatewayapi/show/buyer/v3/shows/{}/subscribe?showSessionId={}", show_id, session_id);
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "openId": "",
        "appId": "",
        "showId": show_id,
        "subscribeTargetType": "SHOW_SESSION",
        "showSessionId": session_id,
        "remindType": "SALE_REMIND"
    });
    let data = post(app, url.as_str(), json_data).await.map_err(|_|PXQError::ReqwestError)?;

    let result = serde_json::from_value::<AddReminderResult>(data).map_err(|_|PXQError::AddReminderError)?;

    Ok(result)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct  TicketWaitlistData {
    pub subscribed: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketWaitlistResult {
    #[serde(rename = "statusCode")]
    status_code: i32,
    comments: String,
    data: TicketWaitlistData,
}

#[tauri::command(async)]
pub async fn ticket_waitlist(app: tauri::Window, show_id: String, session_id: String, seat_plan_id: String) -> Result<TicketWaitlistResult, PXQError> {
    let url = format!("https://m.piaoxingqiu.com/cyy_gatewayapi/show/buyer/v3/shows/{}/subscribe?showSessionId={}", show_id, session_id);
    let json_data = json!({
        "src": "WEB",
        "ver": "4.0.13-20240223084920",
        "openId": "",
        "appId": "",
        "showId": show_id,
        "subscribeTargetType": "SEAT_PLAN",
        "showSessionId": session_id,
        "remindType": "OOS",
        "seatPlanId": seat_plan_id
    });
    let data = post(app, url.as_str(), json_data).await.map_err(|_|PXQError::ReqwestError)?;

    let result = serde_json::from_value::<TicketWaitlistResult>(data).map_err(|_|PXQError::TicketWaitlistError)?;

    Ok(result)
}