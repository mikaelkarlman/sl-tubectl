use reqwest;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub place_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub planned: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TravelResult {
    pub travels: Vec<Travel>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Travel {
    pub legs: Vec<Leg>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    pub origin: Origin,
    pub destination: Destination,
    pub transport: Transport,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    pub name: String,
    pub departure_time: Time,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    pub name: String,
    pub arrival_time: Time,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub name: String,
    pub direction: String,
}

pub fn search_location(search_term: &str) -> Result<Vec<Location>, reqwest::Error> {
    let body: Vec<Location> = reqwest::blocking::get(format!(
        "https://webcloud.sl.se/api/travellocations?search={}&mode=cors",
        search_term,
    ))?
    .json()?;

    Ok(body)
}
pub fn search_travels(
    orig_loc: &Location,
    dest_loc: &Location,
) -> Result<TravelResult, reqwest::Error> {
    let body: TravelResult = reqwest::blocking::get(format!(
        "https://webcloud.sl.se/v2/api/travels?mode=travelPlanner&origPlaceId={}&destPlaceId={}&searchForArrival=false&transportTypes=111&desiredResults=3&useCache=false",
        orig_loc.place_id, dest_loc.place_id,
    ))?
    .json()?;

    Ok(body)
}
