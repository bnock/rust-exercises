pub mod exercise_012 {
    use std::cmp::Ordering;

    pub fn run() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Enter the first number:");
        std::io::stdin().read_line(&mut input1).unwrap();
        let num1: usize = input1.trim().parse().unwrap();

        println!("Enter the second number:");
        std::io::stdin().read_line(&mut input2).unwrap();
        let num2: usize = input2.trim().parse().unwrap();

        match num1.cmp(&num2) {
            Ordering::Greater => println!("{num2} {num1}"),
            Ordering::Less => println!("{num1} {num2}"),
            Ordering::Equal => panic!("How could two numbers possibly be equal?!"),
        }
    }
}

pub mod exercise_013 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter a number less than 20:");
        std::io::stdin().read_line(&mut input).unwrap();
        let num: usize = input.trim().parse().unwrap();

        if num >= 20 {
            println!("Too high");
        } else {
            println!("Thank you");
        }
    }
}

pub mod exercise_014 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter a number between 10 and 20:");
        std::io::stdin().read_line(&mut input).unwrap();
        let num: usize = input.trim().parse().unwrap();

        if num >= 10 && num <= 20 {
            println!("Thank you");
        } else {
            println!("Incorrect value");
        }
    }
}

pub mod exercise_015 {
    pub fn run() {
        let mut color = String::new();

        println!("Enter your favorite color:");
        std::io::stdin().read_line(&mut color).unwrap();
        let color = color.trim();

        if color.to_lowercase() == "red" {
            println!("I like red too");
        } else {
            println!("I don't like {color}, I prefer red");
        }
    }
}

pub mod exercise_016 {
    pub fn run() {
        let mut raining = String::new();

        println!("Is it raining:");
        std::io::stdin().read_line(&mut raining).unwrap();
        let raining = raining.trim().to_lowercase();

        if raining == "yes" {
            let mut windy = String::new();

            println!("Is it windy:");
            std::io::stdin().read_line(&mut windy).unwrap();
            let windy = windy.trim().to_lowercase();

            if windy == "yes" {
                println!("It is too windy for an umbrella");
            } else {
                println!("Take an umbrella");
            }
        } else {
            println!("Enjoy your day");
        }
    }
}

pub mod exercise_017 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter your age:");
        std::io::stdin().read_line(&mut input).unwrap();
        let age: usize = input.trim().parse().unwrap();

        if age >= 18 {
            println!("You can vote");
        } else if age == 17 {
            println!("You can learn to drive");
        } else if age == 16 {
            println!("You can buy a lottery ticket");
        } else {
            println!("You can go trick-or-treating");
        }
    }
}

pub mod exercise_018 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter a number:");
        std::io::stdin().read_line(&mut input).unwrap();
        let num: usize = input.trim().parse().unwrap();

        if num < 10 {
            println!("Too low");
        } else if num < 20 {
            println!("Correct");
        } else {
            println!("Too high");
        }
    }
}

pub mod exercise_019 {
    pub fn run() {
        let mut input = String::new();

        println!("Enter 1, 2, or 3:");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => println!("Thanks you"),
            "2" => println!("Well done"),
            "3" => println!("Correct"),
            _ => println!("Error message"),
        }
    }
}
