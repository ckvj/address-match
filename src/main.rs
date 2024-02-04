use std::process;
use colored::Colorize;

fn main() {
    println!("Enter First Address");
    let string1 = get_string_input(); 

    let interm_string = "Interm string copied to Clipboard";
    cli_clipboard::set_contents(interm_string.to_owned()).unwrap();

    println!("Enter Second Address");
    let string2 = get_string_input(); 

    if str_comp_sensitive(&string1, &string2) {
            println!("{}", "Case Sensitive: MATCH".on_green());
            println!("Matched String: {}",&string1.green());
            process::exit(0)
    };

    match str_comp_insensitive(string1, string2) {
        true => println!("{}", "Case Insensitive: MATCH".on_yellow()),
        false => println!("{}", "Case Insensitive: NO MATCH".on_red()),
    }
}

fn get_string_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn str_comp_sensitive(string1: &str, string2: &str) -> bool {
    string1.eq(string2)
}

fn str_comp_insensitive(string1: String, string2: String) -> bool {
    string1.to_lowercase().eq(&string2.to_lowercase())
}