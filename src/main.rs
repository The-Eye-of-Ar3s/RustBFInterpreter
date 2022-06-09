use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;
use std::io::stdin;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;


fn main() {
    let filename = arg_collector();
    //println!("Filename: {}", filename);
    let rawcode = file_reader(filename);
    //println!("{}", rawcode);
    let cleancode = clean(rawcode);
    //println!("{}", cleancode);
    runner(cleancode);
}

fn arg_collector() -> String{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("NO FILE SPECIFIED!");
        process::exit(1);
    }
    let arg = args.remove(1);
    return arg;
}

fn file_reader(filename: String) -> String{
    if Path::new(&filename).is_file() {
        let raw = fs::read_to_string(filename).expect("Failed to read File!");
        return raw;
    } else {
        eprintln!("Specified File was not found!");
        process::exit(1);
    }
}

fn clean(raw: String) -> String{
    let expr = Regex::new(r"[^<|>|\+|\-|,|\.|\[|\]]").unwrap();
    let result = expr.replace_all(&raw, "").to_string();
    return [result, "!".to_string()].join("").to_string();
}

fn runner(codestr: String) {
    let code: Vec<char> = codestr.chars().collect();
    let loops = loopfinder(codestr);
    let oplmap = loops.0;
    let cllmap = loops.1;
    let mut memory = vec![0];
    let mut pointer = 0;
    let mut index = 0;
    let mut command;
    loop {
        command = code[index].to_string();
        if command == ">" {
            if pointer+1 == memory.len() {
                memory.push(0);
                pointer += 1;
            } else {
                pointer += 1;
            }
            index += 1;
        } else if command == "<" {
            if pointer != 0 {
                pointer -= 1;
            }
            index += 1;
        } else if command == "+" {
            if memory[pointer] == 255 {
                memory[pointer] = 0;
            } else {
                memory[pointer] += 1;
            }
            index += 1;
        } else if command == "-" {
            if memory[pointer] == 0 {
                memory[pointer] = 255;
            } else {
                memory[pointer] -= 1;
            }
            index += 1;
        } else if command == "." {
            print!("{}", memory[pointer] as u8 as char);
            io::stdout().flush().unwrap();
            index += 1;
        } else if command == "," {
                let mut input_string = String::new();
                stdin().read_line(&mut input_string)
    	        .ok()
                .expect("Failed to read line");
                let mut num = input_string.chars().nth(0).expect("AAA1") as u32;
                if num == 13 {
                    num = 0;
                }
                memory[pointer] = num as u32;
            index += 1;
        } else if command == "[" {
            if memory[pointer] == 0 {
                index = oplmap.get(&index.to_string()).unwrap().to_string().parse::<usize>().unwrap() + 1;
            } else {
                index += 1;
            }
        } else if command == "]" {
            if memory[pointer] != 0 {
                index = cllmap.get(&index.to_string()).unwrap().to_string().parse::<usize>().unwrap() + 1;
            } else {
                index += 1;
            }
        } else if command == "!" {
            process::exit(0);
        }
    }
}

fn loopfinder(codestr: String) -> (HashMap<String, String>, HashMap<String, String>){
    let code: Vec<char> = codestr.chars().collect();
    let mut oplmap: HashMap<String, String> = HashMap::new();
    let mut cllmap: HashMap<String, String> = HashMap::new();
    let mut openstack = Vec::new();
    for (i, command) in code.iter().enumerate() {
        if command==&'[' {
            openstack.push(i);
        } else if command==&']' {
            oplmap.insert(openstack.last().unwrap().to_string(), i.to_string());
            cllmap.insert(i.to_string(), openstack.last().unwrap().to_string());
            openstack.pop();
        }
    }
    //println!("{}", codestr);
    return (oplmap, cllmap);
}