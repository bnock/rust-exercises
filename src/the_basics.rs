pub mod exercise_001 {
    pub fn run() {
        let mut first_name = String::new();

        println!("Enter your first name:");

        std::io::stdin().read_line(&mut first_name).unwrap();

        let first_name = first_name.trim();

        println!("Hello {first_name}");
    }
}

pub mod exercise_002 {
    pub fn run() {
        let mut first_name = String::new();
        let mut last_name = String::new();

        println!("Enter your first name:");
        std::io::stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim();

        println!("Enter your last name:");
        std::io::stdin().read_line(&mut last_name).unwrap();
        let last_name = last_name.trim();

        println!("Hello {first_name} {last_name}");
    }
}

pub mod exercise_003 {
    pub fn run() {
        println!("What do you call a bear with no teeth?\nA gummy bear!");
    }
}

pub mod exercise_004 {
    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the first number:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let num1: usize = input1.trim().parse().unwrap();

        println!("Enter the second number:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let num2: usize = input2.trim().parse().unwrap();

        let sum = num1 + num2;

        println!("The total is {sum}");
    }
}

pub mod exercise_005 {
    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();

        println!("Enter the first number:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let num1: usize = input1.trim().parse().unwrap();

        println!("Enter the second number:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let num2: usize = input2.trim().parse().unwrap();

        println!("Enter the third number:");
        std::io::stdin().read_line(&mut input3).unwrap();
        let num3: usize = input3.trim().parse().unwrap();

        let result = (num1 + num2) * num3;

        println!("The answer is {result}");
    }
}

pub mod exercise_006 {
    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("How many pizza slices did you start with:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let total_slices: usize = input1.trim().parse().unwrap();

        println!("How many pizza slices did you eat:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let eaten_slices: usize = input2.trim().parse().unwrap();

        let result = total_slices - eaten_slices;

        println!("You have {result} pizza slices left.");
    }
}

pub mod exercise_007 {
    pub fn run() {
        let mut name = String::new();
        let mut input2 = String::new();

        println!("Enter your name:");
        std::io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();

        println!("Enter your age:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let mut age: usize = input2.trim().parse().unwrap();

        age += 1;

        println!("{name}, on your next birthday you will be {age} years old.");
    }
}

pub mod exercise_008 {
    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the total bill amount:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let total: f64 = input1.trim().parse().unwrap();

        println!("Enter the number of diners:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let diners: usize = input2.trim().parse().unwrap();

        let cost = total as f64 / diners as f64;

        println!("Each diner must pay {:.2}", cost);
    }
}

pub mod exercise_009 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter how many days:");
        std::io::stdin().read_line(&mut input).unwrap();
        let days: usize = input.trim().parse().unwrap();

        let hours = days * 24;
        let minutes = hours * 60;
        let seconds = minutes * 60;

        println!("In {days} days, there are\n{hours} hours\n{minutes} minutes\n{seconds} seconds");
    }
}

pub mod exercise_010 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter a weight in kg:");
        std::io::stdin().read_line(&mut input).unwrap();
        let kilos: f64 = input.trim().parse().unwrap();

        let pounds = kilos * 2.204;

        println!("The weight in pounds is {:.2}.", pounds);
    }
}

pub mod exercise_011 {
    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter a number over 100:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let num1: usize = input1.trim().parse().unwrap();

        println!("Enter a number less than 10:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let num2: usize = input2.trim().parse().unwrap();

        let result = num1 as f64 / num2 as f64;

        println!("The second number goes into the first number {:.2} times.", result);
    }
}
