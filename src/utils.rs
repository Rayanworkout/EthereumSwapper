use dotenv::dotenv;
use std::{collections::HashMap, env};

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
