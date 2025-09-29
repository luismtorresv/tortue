use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

fn main() {
    let mut stdout = stdout();
    let stdin = stdin();

    println!("tortue shell");
    loop {
        print!("$ ");
        let _ = stdout.flush();

        let mut input = String::new();
        let line = stdin.read_line(&mut input);
        match line {
            Ok(0) => {
                println!();
                exit(0);
            }

            Ok(_) => {}

            Err(error) => {
                println!("error: {error}");
                exit(1);
            }
        }
    }
}
