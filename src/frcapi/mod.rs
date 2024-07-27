use sled::Db;

compile_error!("Don't hardcode API keys, you absolute buffoon");
const API_KEY: &'static str = "";
mod updater;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchesRes {
    #[serde(rename = "Matches")]
    pub matches: Vec<Match>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub is_replay: bool,
    pub match_video_link: Option<String>,
    pub description: String,
    pub match_number: i64,
    pub score_red_final: Option<i64>,
    pub score_red_foul: Option<i64>,
    pub score_red_auto: Option<i64>,
    pub score_blue_final: Option<i64>,
    pub score_blue_foul: Option<i64>,
    pub score_blue_auto: Option<i64>,
    pub auto_start_time: Option<String>,
    pub actual_start_time: Option<String>,
    pub tournament_level: String,
    pub post_result_time: Option<String>,
    pub teams: Vec<MatchTeam>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTeam {
    pub team_number: u32,
    pub station: String,
    pub dq: bool,
}

async fn updater(db: Db) {
    tokio::spawn(async {
        loop {

    });
}

async fn update_matches(db: Db, event_code: impl Into<String>) {
    let event_code = event_code.into();
    let tree = db.open_tree("matches").unwrap();

    let res = reqwest::get("https://frc-api.firstinspires.org/v3.0/{SEASON}/matches/{event_code}").await.unwrap();

    let data: MatchesRes = serde_json::from_slice(&res.bytes().await.unwrap()).unwrap();

    for m in &data.matches {
        tree.insert(format!("{event_code}_{level}{match_num}", level = m.tournament_level, match_num = m.match_number), serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
    }
}

/*
async fn update_events(db: Db, event_code: impl Into<String>) {
    let event_code = event_code.into();

    let res = reqwest::get("https://frc-api.firstinspires.org/v3.0/{SEASON}/matches/{event_code}").await.unwrap();

    let data: MatchesRes = serde_json::from_slice(&res.bytes().await.unwrap()).unwrap();

    let tree = db.open_tree("events").unwrap();
    tree.insert("", value)

}
*/
