use std::io;
use std::env;
use std::io::Read;
use encoding_rs::SHIFT_JIS;

const VERSION: &str = "1.0";

fn main() -> io::Result<()>{
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.len() == 2 && args[1] == "/?"{
        println!("Usage: {} [/u] [/i] [/n] <search_string1> [<search_string2> ...]\nOptions:\n  /u: Display without color.\n  /i: Search ignoring case.\n  /n: Show number of lines.\n", args[0]);
        return Ok(());
    }else if args.len() == 2 && args[1] == "--version"{
        println!("version: {}", VERSION);
        return Ok(());
    }

    args.remove(0);
    let mut count: i32 = 0;
    let limit = if 3 > args.len() { args.len() } else { 3 };
    let mut is_colored: bool = true;
    let mut is_ignore_case: bool = false;
    let mut is_show_line: bool = false;

    for arg in args.iter().take(limit){
        match &arg[..]{
            "/u" => {is_colored = false; count += 1},
            "/i" => {is_ignore_case = true; count += 1},
            "/n" => {is_show_line = true; count += 1},
            _ => ()
        }
    }
    for _ in 0..count { args.remove(0); }

    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer).expect("Failed to read from stdin");

    let (text, _, _) = SHIFT_JIS.decode(&buffer);
    {
        let decoded_text = text.trim();
        let lines: Vec<&str> = decoded_text.lines().collect();

        for (i, line) in lines.iter().enumerate(){
            let line_count: usize = i + 1;

            for i in 0..args.len(){
                let start: Option<usize> = if is_ignore_case {line.to_lowercase().find(&args[i].to_lowercase())} else {line.find(&args[i])};
                if let Some(position) = start {
                        if is_show_line{
                            print!("{:>3}|", &line_count);
                        }
                        if !is_colored{
                            println!("{line}");
                        }else{
                            let remainder = i % 5;
                            let color_code = match remainder  {
                                0 => 32,
                                1 => 33, 
                                2 => 35, 
                                3 => 36, 
                                4 => 31, 
                                _ => 37, 
                            };
                            print!("{}", &line[0..position]);
                            print!("\x1B[{}m{}\x1B[0m", color_code, &line[position..position + args[i].len()]);
                            println!("{}", &line[position + args[i].len()..])
                        }
                    }
                }
            }
        }
        Ok(())
    }
