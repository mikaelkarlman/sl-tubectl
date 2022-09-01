pub struct Location {}

pub struct Travel {}
pub struct Api {}

impl Api {
    pub fn search_location(search_term: &str) -> Location {}
    pub fn search_travels(orig_location: &Location, dest_location: &Location) -> Vec<Travel> {}
}
