pub mod exercise_020 {
    pub fn run() {
        let mut name = String::new();

        println!("Enter your name:");
        std::io::stdin().read_line(&mut name).unwrap();
        let length = name.trim().len();

        println!("Your name is {length} characters long");
    }
}

pub mod exercise_021 {
    pub fn run() {
        let mut first_name = String::new();
        let mut last_name = String::new();

        println!("Enter your first name:");
        std::io::stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim();

        println!("Enter your last name:");
        std::io::stdin().read_line(&mut last_name).unwrap();
        let last_name = last_name.trim();

        let full_name = first_name.to_string() + " " + last_name;
        let length = full_name.len();

        println!("Your full name is {full_name} and it is {length} characters long");
    }
}

pub mod exercise_022 {
    use inflections::case;

    pub fn run() {
        let mut first_name = String::new();
        let mut last_name = String::new();

        println!("Enter your first name in lower case:");
        std::io::stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim();

        println!("Enter your last name in lower case:");
        std::io::stdin().read_line(&mut last_name).unwrap();
        let last_name = last_name.trim();

        let full_name = first_name.to_string() + " " + last_name;
        let full_name = case::to_title_case(&full_name[..]);

        println!("Your name in title case is {full_name}");
    }
}

pub mod exercise_023 {
    pub fn run() {

    }
}
