use std::io::{self, BufRead, Write};


fn main() {
    let mut line = String::new();
    let stdin = io::stdin();

    loop {
        print!(">> ");
        let mut line = String::new();
        io::stdout().flush().expect("Error flushing stdout");
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        let line = line.trim();
        if(line == "exit"){
            break;
        }

        println!("{}", line);

    }
}
