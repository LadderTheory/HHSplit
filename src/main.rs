use std::fs::File;
//use std::io::prelude::*;
use std::env;
use std::io;
use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len(){
        println!("{}", args[i]);
    }

    if args.len() == 2{     //split
        println!("What is the split size in MB's?");

        let mut input = String::new();
        let mut max: usize = 1;

        let mut valid_input = false;

        while !valid_input{
            io::stdin()
                .read_line(&mut input)
                .expect("Bad Input");

            let trimmed = input.trim();
            match trimmed.parse::<usize>() {
                Ok(_n) => valid_input = true,
                Err(..) => println!("this was not an integer: {}", trimmed),
            };

            max = trimmed.parse().unwrap();
        }

        max *= 1000000;

        let mut finput = File::open(&args[1])?;
        let mut buffer = Vec::new();
        finput.read_to_end(&mut buffer)?;

        let mut pos = 0;

        let mut current_section = 1;

        while pos < buffer.len(){
            println!("Current Section: {}", current_section);

            let mut section = Vec::new();
            while &section.len() <= &max && pos < buffer.len(){
                section.push(buffer[pos]);
                pos+=1;
            }

            let path = format!("{}.toohey", current_section);

            let mut foutput = File::create(&path)?;
            let section_pos = 0;
            foutput.write(&section[section_pos..])?;

            current_section += 1;
            println!("Section Complete");
        }
    }else if args.len() > 2{//join
        let mut out = Vec::new();

        for current_arg in 1..args.len(){
            let mut finput = File::open(&args[current_arg])?;
            let mut buffer = Vec::new();
            finput.read_to_end(&mut buffer)?;

            for i in 0..buffer.len(){
                out.push(buffer[i]);
            }
        }

        let mut foutput = File::create("rename this.toohey")?;
        let p = 0;
        foutput.write(&out[p..])?;
    }else{
        println!("Please read the readme");
    }

    pause();

    Ok(())
}
