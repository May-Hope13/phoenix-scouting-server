use reqwest::Client;
use sled::Db;

/// Data updater
///
/// Manages updating of data from the FIRST Events API.
///
pub struct Updater {
    db: Db,
    client: Client,
}
impl Updater {
    pub fn new(db: Db) -> Self {
        let client = Client::builder().default_headers(reqwest::header::HeaderMap::from_iter([(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Basic {API_KEY}")).unwrap(),
        )])).build().unwrap();

        Self {
            db,
            client,
        }
    }

    async fn update_events(&mut self) {
        let tree = self.db.open_tree("events").unwrap();

        let res = self.client.get("https://frc-api.firstinspires.org/v3.0/{SEASON}/events").send().await.unwrap();

        let data: EventsRes = serde_json::from_slice(&res.bytes().await.unwrap()).unwrap();

        for e in &data.events {

    async fn update_matches(&self, event_code: impl Into<String>) {
        let event_code = event_code.into();
        let tree = db.open_tree("matches").unwrap();
    
        let res = reqwest::get("https://frc-api.firstinspires.org/v3.0/{SEASON}/matches/{event_code}").await.unwrap();
    
        let data: MatchesRes = serde_json::from_slice(&res.bytes().await.unwrap()).unwrap();
    
        for m in &data.matches {
            tree.insert(format!("{event_code}_{level}{match_num}", level = m.tournament_level, match_num = m.match_number), serde_json::to_string(&data).unwrap().as_bytes()).unwrap();
        }
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
