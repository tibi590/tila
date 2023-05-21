use std::fs::OpenOptions;
use std::io::{self, Write};
use std::error::Error;
use csv;

fn main() -> () {
    loop {
        let prompt = input_to_var("\nDo you want to Register(r), Login(l) or exit(e): ");

        if prompt == "r" {
            if let Err(e) = register() {
                eprintln!("{:?}", e);
            }
        } else if prompt == "l" {
            if let Err(e) = login() {
                eprintln!("{:?}", e);
            }
        } else if prompt == "e" {
            return ();
        } else {
            println!("Invalid Input\n");
        }
    }
}

fn register() -> Result<(), Box<dyn Error>> {
    println!("\nRegister");
    let mut name_taken: bool = false;

    loop {
        let name = input_to_var("Enter username: ");
        let password = input_to_var("Enter password: ");

        match username_taken(&name) {
            Ok(true) => name_taken = true,
            Ok(false) => name_taken = false,
            _ => println!("Error: Checking username."),
        }
        
        if !name_taken {
            if let Err(_e) = write_to_csv(&name, &password, "user".to_string()) {
                println!("Error: Writing to csv file.");
            }
            println!("\nREGISTRATION SUCCESSFUL");
            break;
        } else {
            println!("Username is already taken. Try again.");
        }
        name_taken = false;
    }

    Ok(())
}

fn login() -> Result<(), Box<dyn Error>> {
    println!("\nLogin");
    let mut x = csv::Reader::from_path("./profiles.csv")?;
    for _ in 0..3 {
        let name = input_to_var("Enter username: ");
        let password = input_to_var("Enter password: ");

        for result in x.records() {
            let record = result?;

            if name.trim() == record[0].to_string() && password.trim() == record[1].to_string() {
                println!("\nLOGIN SUCCESSFUL");
                if record[2].to_string() == "user" {
                    menu(Profile { name: record[0].to_string(), password: record[1].to_string(), privilege: Privilege::User });
                } else if record[2].to_string() == "admin" {
                    menu(Profile { name: record[0].to_string(), password: record[1].to_string(), privilege: Privilege::Admin });
                }
                return Ok(());
            }
        }
        println!("Wrong username or password.\n");
    };

    Ok(())  
}


fn menu(profile: Profile) -> (){
    println!("\nMenu");
    loop {
        let prompt = input_to_var("-> ");

        match prompt.trim() {
            "exit" => return (),
            "help" | "?" => profile.help(),
            "profile-info" => profile.profile_info(),
            "new-user" => profile.new_user(),
            "list-profiles" => {
                if let Err(e) = profile.list_profiles() {
                    eprintln!("{:?}", e);
                }
            },
            _ => println!("Invalid prompt"),
        }
    };
}


fn input_to_var(question: &str) -> String {
    let mut var = String::new();

    print!("{}", question);
    io::stdout()
        .flush()
        .expect("Error: Input");
    io::stdin()
        .read_line(&mut var)
        .expect("Error: Input");

    var.trim().to_string()
}

fn username_taken(name: &String) -> Result<bool, Box<dyn Error>> {
    for result in csv::Reader::from_path("./profiles.csv")?.records() { 
        if result?[0].to_string() == name.trim().to_string() {
            return Ok(true)
        }
    }
    Ok(false)
}

fn write_to_csv(name: &String, password: &String, privilege: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./profiles.csv")
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);

    writer.write_record(&[name.trim().to_string(), password.trim().to_string(), privilege.trim().to_lowercase().to_string()])?;
    Ok(())

}

#[derive(Debug)]
struct Profile {
    name: String,
    password: String,
    privilege: Privilege,
}

impl Profile {
    fn help(&self) {
        if self.privilege == Privilege::Admin {
            println!("Prompt list:
 -help or ?    |prompt list
 -exit         |close program
 -profile-info |shows username and password
 -new-user     |create a new user
 -list-profiles|list profiles from profiles.csv file");
        } else {
            println!("Prompt list:
 -help or ?    |prompt list
 -exit         |close program
 -profile-info |shows username and password");
        }   
    }

    fn profile_info(&self) {
        println!("Username: {}\nPassword: {}\nprivilege level: {:?}", self.name, self.password, self.privilege);
    }

    fn new_user(&self) {
        if self.privilege == Privilege::Admin {
            let name = input_to_var("Username: ");
            let password = input_to_var("Password: ");
            let privilege = input_to_var("Privilege level: (user, admin): ");
            let mut name_taken: bool = false;
            
            match username_taken(&name) {
                Ok(true) => name_taken = true,
                Ok(false) => name_taken = false,
                _ => println!("Error: Checking username."),
            }
            
            if !name_taken {
                if privilege.trim().to_lowercase() != "user" && privilege.trim().to_lowercase() != "admin" {
                    println!("Invalid privilege level.");
                } else {
                    if let Err(_e) = write_to_csv(&name, &password, privilege.trim().to_lowercase().to_string()) {
                        println!("Error: Writing to csv file");
                    }
                }
            } else {
                println!("Username is already taken.");
            }
        } else {
            println!("Permission denied.");
        }
    }

    fn list_profiles(&self) -> Result<(), Box<dyn Error>> {
        if self.privilege == Privilege::Admin {
            let mut reader = csv::Reader::from_path("./profiles.csv")?;

            let headers = reader.headers()?;
            println!("{}, {}, {}", headers[0].to_string().to_uppercase(), headers[1].to_string().to_uppercase(), headers[2].to_string().to_uppercase());

            for result in reader.records() {
                let record = result?;
                println!("{}, {}, {}", record[0].to_string(), record[1].to_string(), record[2].to_string());
            }
        } else {
            println!("Permission denied.");
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Privilege {
    User,
    Admin,
}
