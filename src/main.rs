use std::fs::{self, File};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = &args[1];

    match command.as_str() {
        "greet" => {
            let default_name = "World".to_string();
            let name = args.get(2).unwrap_or(&default_name);
            println!("Hello, {}!", name);
        }
        "repeat" => {
            if args.len() < 4 {
                println!("Usage: {} repeat <message> <count>", args[0]);
                return;
            }
            let message = &args[2];
            let count: u32 = args[3].parse().unwrap_or(1);
            for _ in 0..count {
                println!("{}", message);
            }
        }
        "whatislife"=>{
            println!("life is sunshine and rainbow");
            let message = fs::read_to_string(&args[2]).expect("error");
            println!("{}", message);
        }
        "create-to-do"=>{
            let _file = File::create("todo.txt").expect("please create a todo first");                
            
        }
        "create-file"=>{
            let _file = File::create(&args[2]).expect("error");
        }
        "add"=>{
            let _ = fs::write("todo.txt", &args[2]).expect("what?");
        }
        "to-do"=>{
            let to_do = fs::read_to_string("todo.txt").expect("no todo exist");
            print!("{}" ,to_do);
        }
        _ => {
            println!("not a valid command");
        }
    } 

}



