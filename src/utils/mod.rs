use dotenv::dotenv;
use eyre::Result;
use std::{collections::HashMap, env, process::exit};

/// Function to check and get a list of required environment variables.
/// We use the dotenv library to collect them and panic if one of the
/// required variables is not set or retrieved.
/// https://crates.io/crates/dotenv
pub fn get_env_variables(required_vars: Vec<&str>) -> HashMap<String, String> {
    dotenv().ok();

    let mut vars: HashMap<String, String> = HashMap::new();

    required_vars.into_iter().for_each(|req_var| {
        match env::var(req_var) {
            Ok(var) => {
                if var.is_empty() {
                    panic!("\"{}\" must be set !", req_var)
                }
                vars.insert(req_var.to_string(), var)
            }
            Err(_) => panic!("\"{}\" must be set !", req_var),
        };
    });

    vars
}

pub fn confirm_swap(command: &str, amount: f64) -> Result<()> {
    let str_command = match command {
        "buy_eth" => format!("buy {} ETH", amount),
        "buy_usdc" => format!("buy {} USDC", amount),

        _ => {
            println!("Invalid command. Please provide a valid command.");
            return Ok(());
        }
    };

    println!("You are about to {}.", str_command);
    println!("Proceed? (y/n)");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;

    if input.trim() != "y" {
        println!("Operation cancelled.");
        exit(0)
    }

    Ok(())
}
