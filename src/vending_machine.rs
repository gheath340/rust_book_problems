use std::io;

pub enum VendingMachine {
    Soda,
    Chips,
    Candy,
}

impl VendingMachine {
    fn get_text(&self) -> &'static str {
        match self {
            VendingMachine::Soda => "Here is your soda",
            VendingMachine::Chips => "Here are your chips",
            VendingMachine::Candy => "Here is your candy",
        }
    }

    fn from_str(input: &str) -> Option<VendingMachine> {
        match input {
            "Soda" => Some(VendingMachine::Soda),
            "Chips" => Some(VendingMachine::Chips),
            "Candy" => Some(VendingMachine::Candy),
            _ => None,
        }
    }

    fn from_input(input: &str) -> Option<VendingMachine> {
        if input.eq_ignore_ascii_case("Soda") {
            Some(VendingMachine::Soda)
        } else if input.eq_ignore_ascii_case("Chips") {
            Some(VendingMachine::Chips)
        } else if input.eq_ignore_ascii_case("Candy") {
            Some(VendingMachine::Candy)
        } else {
            None
        }
    }
}

pub fn get_product() {
    println!("What product would you like?");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if let Some(product) = VendingMachine::from_input(input) {
            println!("{}", product.get_text());
            break;
        } else {
            println!("Please enter a valid option.");
        }
    }
}