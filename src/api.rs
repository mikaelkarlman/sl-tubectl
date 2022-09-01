use reqwest;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub place_id: String,
    pub name: String,
}

pub struct Travel {}

pub fn search_location(search_term: &str) -> Result<Vec<Location>, reqwest::Error> {
    let body: Vec<Location> = reqwest::blocking::get(format!(
        "https://webcloud.sl.se/api/travellocations?search={}&mode=cors",
        search_term,
    ))?
    .json()?;

    Ok(body)
}
//pub fn search_travels(orig_location: &Location, dest_location: &Location) -> Vec<Travel> {}
