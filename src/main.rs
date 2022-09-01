mod api;
mod cli;

fn main() {
    let orig_loc = cli::ask_for_location("What station do you want to go from?");

    let dest_loc = cli::ask_for_location("What station do you want to go to?");
    dbg!("orig_loc {:?}", orig_loc);
    dbg!("dest_loc {:?}", dest_loc);
}
