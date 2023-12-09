mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;

fn clr(){
    print!("{}[2J",27 as char);
}

fn main(){
    clr()
    
}
