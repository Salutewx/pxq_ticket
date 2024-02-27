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
