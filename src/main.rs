mod api;
mod cli;

use colored::Colorize;

fn main() {
    let orig_loc = cli::ask_for_location("What station do you want to go from?");

    let dest_loc = cli::ask_for_location("What station do you want to go to?");

    let travel_result: api::TravelResult = api::search_travels(&orig_loc, &dest_loc).expect("die");

    for travel in travel_result.travels.iter() {
        let leg_length = travel.legs.len();
        println!(
            "{}: {} {} {}: {}",
            travel.legs[0].origin.name.red(),
            travel.legs[0].origin.departure_time.planned.cyan(),
            "=>".green(),
            travel.legs[leg_length - 1].destination.name.red(),
            travel.legs[leg_length - 1]
                .destination
                .arrival_time
                .planned
                .cyan()
        );
    }
}
