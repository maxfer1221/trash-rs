// extern crate fs_extra;
use std::str::FromStr;
// use std::io;
use std::env;
// use fs_extra::dir::copy;
//
#[derive(Debug, PartialEq)]
enum Function {
    Delete,
    List,
    Empty,
    ChangeDir(Option<String>),
    Restore(Option<Box<[String]>>),
}

impl FromStr for Function {
    type Err = ();

    fn from_str(input: &str) -> Result<Function, Self::Err> {
        match input {
            "delete"     => Ok(Function::Delete),
            "list"       => Ok(Function::List),
            "empty"      => Ok(Function::Empty),
            "change-dir" => Ok(Function::ChangeDir(None)),
            "Restore"    => Ok(Function::Restore(None)),
            _            => Err(()),
        }
    }
}

fn main() {
    let mut args: &[String] = &env::args().collect::<Vec<String>>();

    //let mut err_str: String = args[1].clone();
    //err_str.push_str(" is not a function.");

    let function = Function::from_str(&args[1]);
    // args = &args[2..];
    match function {
        Ok(Function::Delete) => {
            println!("Delete!");       
        }
        Ok(Function::List) => {
            println!("List!");
        }
        Ok(Function::Empty) => {
            println!("Empty!");
        }
        Ok(Function::ChangeDir(_)) => {
            println!("Change dir!");
        }
        Ok(Function::Restore(_)) => {
            println!("Restore!");
        }
        Err(()) => {
            println!("{} is not a function.", args[1]);
        }
    }
  
}
