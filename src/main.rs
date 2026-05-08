// use core::num;
use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Ticket {
    number_of_ticket: u8,
}
#[allow(dead_code)]
enum Transport {
    Car,
    Train {
        owner: String,
        ticket: Ticket,
        car_model: String,
        year: u32,
        price: f64,
    },
    Airplane,
}
#[allow(dead_code)]
enum LoginState {
    LoggedIn { user: String },
    EnterName { name: String },
    LoggedOut(),
}
#[allow(dead_code)]
#[derive(Debug)]
struct Car {
    owner: String,
    car_model: String,
    year: u32,
    fuel_level: f64,
    price: f64,
}
fn main() {
    let _transportation = Transport::Train {
        owner: "benjamin".to_owned(),
        ticket: Ticket {
            number_of_ticket: 4,
        },
        car_model: "merceded".to_owned(),
        year: 2012,
        price: 99.9,
    };
    let mut _car_collection: Vec<Car> = Vec::new();
    // let mut borrower = Vec::new();
    // let table_borrow: HashMap<Car::owner, Car::car_model> = HashMap::new();
    //
    let msg = "cannot display user entry";
    let _table_borrow: HashMap<String, String> = HashMap::new();

    let _new_car = Car::new(
        "benjamin".to_owned(),
        String::from("mercedes"),
        2022,
        60.3,
        90.0,
    )
    .expect(msg);
    let _car_2 = Car::new("carson".to_owned(), "toyota".to_owned(), 2014, 80.3, 44.3).expect(msg);
    // let _new_car =
    //     Car::new("benjamin".to_owned(), 2022, 60.3, 90.0).expect("error cannot handle user input");
    _car_collection.push(_new_car);
    _car_collection.push(_car_2);

    // println!("Hello, world!");
}

impl Car {
    fn new(
        owner: String,
        car_model: String,
        year: u32,
        fuel_level: f64,
        price: f64,
    ) -> Result<Self, io::Error> {
        if owner.is_empty() || year < 2000 || fuel_level < 0.0 || price <= 0.0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "nan"));
        }
        Ok(Self {
            owner,
            car_model,
            year,
            fuel_level,
            price,
        })
    }
    #[allow(dead_code)]
    fn display_car_information(&self) {
        println!(
            "owner:{}\ncar_model:{}\n,year:{}\n,fue_level:{}\nprice:{}\n",
            self.owner, self.car_model, self.year, self.fuel_level, self.price
        );
    }

    #[allow(dead_code)]
    fn refuel(&mut self, fuel_added: f64) {
        self.fuel_level += fuel_added;
        println!("fuel added:{}", self.fuel_level)
    }

    #[allow(dead_code)]
    fn fixed_monthly_insurance() -> f64 {
        0.32
    }

    #[allow(dead_code)]
    fn change_ownership(self) -> Self {
        self
    }
    #[allow(dead_code)]
    fn selling_price(&self) -> f64 {
        self.price * Car::fixed_monthly_insurance()
    }
}

impl Transport {
    #[allow(dead_code)]
    // #[allow(unused_variables)]
    fn allowance(&self, miles: f64) {
        let mut i = 0;
        let allowance = match &self {
            Transport::Car => println!("miles:{}", miles * 2.0),

            Transport::Train {
                owner,
                ticket,
                car_model,
                year,
                price,
            } => {
                println!("enter name:");
                println!("owner:{}", owner);
                while i < ticket.number_of_ticket {
                    let customer_names = String::new();
                }
                // let mut owner=String::new();
                // std::io::stdin().read_line(&mut owner).expect("cannot collect user name");
                println!("miles:{}", miles * 20.0)
            }
            // Transport::Train => println!("miles:{}", miles * 2.0),
            Transport::Airplane => println!("miles:{}", miles * 2.0),
        };
        allowance
    }
}

#[allow(dead_code)]
fn login_parameter() -> Result<LoginState, io::Error> {
    loop {
        let mut _state = LoginState::LoggedOut();
        match &mut _state {
            LoginState::LoggedOut() => {
                println!("thank you for using our service:");
                let mut user_entry = String::new();
                std::io::stdin().read_line(&mut user_entry)?;
                // .expect("cannot enter user entry");
                let user_entry = user_entry.trim();
                if user_entry == "login" {
                    _state = LoginState::EnterName {
                        name: String::new(),
                    }
                } else if user_entry == "quit" {
                }
            }
            LoginState::EnterName { name } => {
                if name.is_empty() {
                    print!("please enter name\n");
                    let result = std::io::stdout().flush();
                    match result {
                        Ok(name) => println!("welcome,{:?}", name),
                        Err(_) => println!("cannot collect user name and name is empty"),
                    }
                } else {
                    println!("welcome,{}", name);
                    _state = LoginState::LoggedIn { user: name.clone() }
                }
            }
            LoginState::LoggedIn { user } => {
                print!("enter command:>");
                std::io::stdout().flush()?;
                let user_input = String::new();
                let user_input = user_input.trim();
                match user_input {
                    "whoami" => println!("helloo user,{}", user),

                    "logoout" => _state = LoginState::LoggedOut(),
                    // "quit" => break,
                    _ => println!("wrong entry"),
                };
                // _state = LoginState::LoggedOut();
            } // _ => println!("invalid entry"),
        }
    }
}

#[allow(dead_code)]
fn user_query(prompt: &str) -> Result<String, io::Error> {
    println!("enter name to buy ticket::{}", prompt);
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input)?;

    if user_input.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "nan"))?;
    }
    Ok(user_input.to_owned())
}

#[allow(dead_code)]
fn atoi(prompt: &str) -> Result<i32, io::Error> {
    println!("{}", prompt);
    let user_query = user_query("enter nums:")?;
    let num_conv = user_query
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "nan"))?;
    Ok(num_conv)
}

#[allow(dead_code)]
fn atof64(prompt: &str) -> Result<f64, io::Error> {
    println!("{}", prompt);
    let user_query = user_query("enter nums:")?;
    let num_conv = user_query
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "nan"))?;
    Ok(num_conv)
}
