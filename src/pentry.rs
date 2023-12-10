use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::BufWriter;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo{
  pub service: String,
  pub username: String,
  pub password: String,
}

impl ServiceInfo {
  pub fn new(service:String,username:String,password:String) -> Self{
    ServiceInfo { 
      service: (service),
      username: (username),
      password: (password)
    }
  }

  pub fn from_json(json_string: &str) -> Result<Self,serde_json::Error>{
    serde_json::from_json(json_string)
  }
  #[allow(dead_code)]
  pub fn from_user_input() -> Self{
    print!("Enter Password Entry:");
    let mut service = String::new();
    io::stdin()
      .read_line(&mut service)
      .expect("Failed to read line");

    println!("Enter Username:");
    let mut username = String::new();
    io::stdin()
      .read_line(&mut username)
      .expect("Failed to read line");

    println!("Enter Password:");
    let mut password = String::new();
    io::stdin()
      .read_line(&mut password)
      .expect("Failed to read line");

    ServiceInfo::new(
      service.trim().to_string(),
      username.trim().to_string(),
      password.trim().to_string(),
    )
  }

  pub fn to_json(&self) -> String{
    serde_json::to_string(&self).expect("Failed to serialize to JSON")
  }

  pub fn write_to_file(&self){
    let json_output = format!("{}\n",self.to_json());

    match OpenOptions::new()
      .create(true)
      .append(true)
      .open("Passwords.json")
      {
        Ok(mut file) => {
          file.write_all(json_output.as_bytes()){
            eprintln!("Error writing to the file: {}", e);
          } else {
            println!("Successfully wrote to Passwords.json")
          }
          Err(e) => eprintln!("Error opening file: {}",e),
        }
      }
  }

}

pub fn read_passwords_from_file() ->  Result<Vec<ServiceInfo>, io::Error>{
  let file = File::open("Passwords.json");
  let reader = io::BufReader::new(file);
}

pub fn prompt(prompt &str) -> String{
  print!("{}",prompt);
  io::stdout().flush().unwrap();
  let mut input =  String::new();
  io::stdin().read_line(&mut input).unwrap();
  input.trim().to_String()
}