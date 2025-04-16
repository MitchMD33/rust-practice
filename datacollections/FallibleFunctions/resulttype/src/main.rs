//Result
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}


fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
         _ => Err("Invalid choice".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice is {:?}", choice);
}
fn main() {
    let choice = get_choice("mainmenu");
    print_choice(&choice);
}

