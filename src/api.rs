use rocket::http::{Cookie, CookieJar};
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;

#[get("/api/getActiveBets")]
pub fn get_active_bets() -> Json<Vec<()>> {
    Json(vec![])
}

#[get("/api/getUser")]
pub fn get_user(cookies: &CookieJar<'_>) -> Json<Value> {
    Json(json!({
        "id": "be457c4e-79e6-4016-94f5-76c6705741bb",
        "email": "before@sibr.dev",
        // disable ability to change email on frontend
        "appleId": "what's umpdog",
        "lightMode": cookies.get_pending("light_mode")
            .and_then(|s| bool::from_str(s.value()).ok())
            .unwrap_or(false),
        "verified": true,
        "coins": 0,
        "idol": "placeholder-idol",
        "unlockedShop": true,
        "unlockedElection": true,
        "snacks": {
            "Forbidden_Knowledge_Access": 1,
            "Stadium_Access": 1,
            "Wills_Access": 1,
        },
        "snackOrder": [
            "Forbidden_Knowledge_Access",
            "Stadium_Access",
            "Wills_Access",
            "E",
            "E",
            "E",
            "E",
            "E",
        ],
        "packSize": 8,
        // set all these to reasonably high values to avoid rendering the "what to do next" actions
        // in the bulletin
        "trackers": {
            "BEGS": 3,
            "BETS": 10,
            "VOTES_CAST": 1,
            "SNACKS_BOUGHT": 2,
            "SNACK_UPGRADES": 3,
        },
    }))
}

#[get("/api/getUserRewards")]
pub fn get_user_rewards() -> NoContent {
    NoContent
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub light_mode: bool,
}

#[post("/api/updateSettings", data = "<settings>")]
pub fn update_settings(cookies: &CookieJar<'_>, settings: Json<Settings>) -> Json<Value> {
    cookies.add(Cookie::new("light_mode", settings.light_mode.to_string()));
    Json(json!({ "message": "Settings updated" }))
}
