use std::{fs::{self, File}, io::{BufRead, BufReader}, path::Path, process::exit};

pub fn check_exist(path: &str, messages: &str, exist:bool) -> Option<bool>{
    if Path::new(&path).exists() && exist{
        if messages == ""{
            return Some(true);
        }                           
        println!("{}", messages);
        exit(1)
    }
    if !Path::new(&path).exists() && !exist{                           
        println!("{}", messages);
        exit(1)
    }
    return None;
}

pub fn check_args(length: usize,messages: &str, expected_length: usize ){
    if length < expected_length{
        println!("{}", messages);
        exit(1);
    }
} 
pub fn create_file(path:&str, messages: &str){
    if check_exist(path, messages,true) == Some(true){
        return;        
    };  
    let _file = File::create(path).expect("something went wrong");
    println!("succesfully added {path}");     
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    check_args(args.len(),"no arguments",2);
    let command: &String = &args[1];
    match &command[..] {
        "create"=>{
            check_args(args.len(), &format!("Usage: {} create <filename>", &args[0]), 3);
            create_file(&args[2], "file already exist");
        }
        "add-td"=>{
            check_args(args.len(), &format!("Usage: {} write <messages>", &args[0]), 3);
            create_file("todo.txt", "");
            let  to_do = fs::read_to_string("todo.txt").expect(&format!("no to do list exist"));
            let result = to_do + "\n" + &args[2];
            let _ = fs::write("todo.txt", result).expect("something gone wrong");
            print!("succesfully writen");
        }"write"=>{
            check_args(args.len(), &format!("Usage: {} write <messages>", &args[0]), 3);
            create_file(&args[2], "");
            let  read= fs::read_to_string(&args[2]).expect(&format!("no to do list exist"));
            let result = read + "\n" + &args[3];
            let _ = fs::write(&args[2], result).expect("something gone wrong");
            print!("succesfully writen");
        }
        
        "delete-line"=>{
            let mut count =0;
            let mut result = String::new();  
            check_args(args.len(), &format!("Usage: {} write <messages>", &args[0]), 3);
            create_file(&args[2], "");
            let file = File::open(&args[2]).expect("cant open file");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.expect("Unable to read line");
                if line != args[3]{
                    result.push_str(&(line + "\n"));  
                }
                count = count + 1 ;
                println!("{}", result);
            }   
            println!("last result : {}", result);
            let _ = fs::write(&args[2], result).expect("something gone wrong");   
        }
        
        "td"=>{
            check_exist("todo.txt", "no to do exist", false);             
            let to_do = fs::read_to_string("todo.txt").expect(&format!("no to do list exist"));
            print!("{}" ,to_do);
        }"read"=> {
            check_args(args.len(), &format!("Usage: {} read <filename>", &args[0]), 3);
            check_exist(&args[2], "file doesnt exist", false);             
            let read = fs::read_to_string(& args[2]).expect(&format!("file doesnt {} exist",args[2]));
            print!("{read}")
        }
        "remove"=>{
            fs::remove_file(&args[2]).expect("could not remove file");
            println!("Removed file data.txt");
        }"find"=>{
            fs::remove_file(&args[2]).expect("could not remove file");
            
        }
        _ => {
            println!("not a valid command");
        }
    }
}




