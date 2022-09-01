use crate::api::{search_location, Location};
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn ask_for_location(question: &str) -> Location {
    loop {
        let input: String = Input::new()
            .with_prompt(question)
            .interact_text()
            .expect("die");

        let api_result: Vec<Location> = search_location(&input).expect("die");

        let items = api_result
            .iter()
            .map(|loc: &Location| loc.name.to_string())
            .collect::<Vec<_>>();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .item("<New Search>")
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .expect("FAILED");

        let index = match selection {
            Some(0) => continue,
            Some(index) => index,

            _ => continue,
        };

        return api_result[index - 1].clone();
    }
}
