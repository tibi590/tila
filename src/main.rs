use std::fs::OpenOptions;
use std::io::{self, Write};
use std::error::Error;
use csv;

fn main() -> () {
    let mut prompt = String::new();

    loop {
        print!("\nDo you want to Register(r), Login(l) or exit(e): ");
        io::stdout()
            .flush()
            .expect("Error: Main");
        io::stdin()
            .read_line(&mut prompt)
            .expect("Error: Main");

        if prompt.trim() == "r" {
            if let Err(e) = register() {
                eprintln!("{:?}", e);
            }
        } else if prompt.trim() == "l" {
            if let Err(e) = login() {
                eprintln!("{:?}", e);
            }
        } else if prompt.trim() == "e" {
            return ();
        } else {
            println!("Invalid Input\n");
        }
        prompt.clear();
    }
}

fn register() -> Result<(), Box<dyn Error>> {
    println!("\nRegister");
    let mut name = String::new();
    let mut password = String::new();
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./profiles.csv")
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);
    let mut name_taken: bool = false;

    loop {
        print!("Enter username: ");
        io::stdout()
            .flush()
            .expect("Error: Register");
        io::stdin()
            .read_line(&mut name)
            .expect("Error: Register");

        print!("Enter password: ");
        io::stdout()
            .flush()
            .expect("Error: Register");
        io::stdin()
            .read_line(&mut password)
            .expect("Error: Register");

        for result in csv::Reader::from_path("./profiles.csv")?.records() { 
            if result?[0].to_string() == name.trim().to_string() {
                name_taken = true;
                break;
            }
        }
        
        if !name_taken {
            writer.write_record(&[name.trim().to_string(), password.trim().to_string()])?;
            writer.flush()?;
            println!("\nREGISTRATION SUCCESFULL");
            break;
        } else {
            println!("Username already taken. Try again.\n");
        }
        name.clear();
        password.clear();
        name_taken = false;
    }

    Ok(())
}

fn login() -> Result<(), Box<dyn Error>> {
    println!("\nLogin");
    let mut name = String::new();
    let mut password = String::new();
    let mut reader = csv::Reader::from_path("./profiles.csv")?;

    for _i in 0..3 {
        print!("Enter username: ");
        io::stdout()
            .flush()
            .expect("Error: Login");
        io::stdin()
            .read_line(&mut name)
            .expect("Error: Login");

        print!("Enter password: ");
        io::stdout()
            .flush()
            .expect("Error: Login");
        io::stdin()
            .read_line(&mut password)
            .expect("Error: Login");

        for result in reader.records() {
            let record = result?;

            if name.trim() == record[0].to_string() && password.trim() == record[1].to_string() {
                println!("\nREGISTRATION SUCCESFULL");
                menu(Profile { name: record[0].to_string(), password: record[1].to_string() });
                return Ok(());
            }
        }
        println!("Wrong username or password.\n");
        name.clear();
        password.clear();
    };

    Ok(())  
}

fn menu(profile: Profile) -> (){
    println!("\nMenu");
    loop {
        let mut prompt = String::new();

        print!("-> ");
        io::stdout()
            .flush()
            .expect("Error: Menu");
        io::stdin()
            .read_line(&mut prompt)
            .expect("Error: Menu");

        match prompt.trim() {
            "sex" => println!("SEXY SEX"),
            "exit" => return (),
            "profile-info" => profile.profile_info(),
            "help" => help(),
            _ => println!("No"),
        }
    };
}

fn help() {
    println!("Prompt list:
 -sex          |SEXY SEX
 -help         |prompt list
 -profile-info |shows username and password
 -exit         |close program");
}

#[derive(Debug)]
struct Profile {
    name: String,
    password: String,
}

impl Profile {
    fn profile_info(&self) {
        println!("Username: {}\nPassword: {}", self.name, self.password);
    }
}
