use std::io::stdin;

fn main() {
    let mut _name: String = String::new();
    let mut _date_of_birth: String = String::new();
    let mut _sex: String = String::new();
    let mut _country_of_origin: String = String::new();
    let mut _confirmation: String = String::new();

    println!("Hello! Welcome to HatToving Airlines. Please give out the following info below:\n");

    please_state(&mut _name, 
                "Please state your full name.".to_string(), 
                "Please type in your name.".to_string());

    please_state(&mut _date_of_birth, 
                "\nPlease state your date of birth. (DD/MM/YYYY)".to_string(), 
                "Please type in your date of birth.".to_string());
                        
    please_state(&mut _sex, 
        "\nPlease state your sex.".to_string(), 
        "Please type in your sex.".to_string());

    please_state(&mut _country_of_origin, 
        "\nPlease state your country of origin.".to_string(), 
        "Please type in your country of origin.".to_string());

    println!("\nHere's your passport!\n");
    print!("Name: {_name}Date of Birth: {_date_of_birth}Sex: {_sex}Country of Origin: {_country_of_origin}\n");

    println!("Is this okay?");

    loop {
        stdin().read_line(&mut _confirmation)
            .expect("Please type in yes or no.");

        if _confirmation.trim().to_uppercase().contains("YES") {
            println!("\nCool! Identity stolen.");
            break;
        } else if _confirmation.trim().to_uppercase().contains("NO") {
            println!("\nShucks, better luck next time...");
            break;
        } else {
            println!("\nYou have to type in either {} or {}, please.", r#""yes""#, r#""no""#);
        }
    }
}

fn please_state(what: &mut String, prompt: String, expect: String) {
    println!("{prompt}");
    stdin().read_line(what).expect(&expect);
}