use crate::{register::Registers, instructions::OpCode};
use std::env;
use std::fs::File;
use termios::*;
use std::io::BufReader;
use byteorder::{BigEndian, ReadBytesExt};

fn main()
{
    let stdin_fd = 0;
    
    let termios = Termios::from_fd(stdin_fd).expect("Failed to get terminal attributes");
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin_fd, TCSANOW, &mut new_termios).expect("Failed to set terminal attributes");

    let mut vm = VM::new();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        eprintln!("Usage: cargo run <program-file.obj>");
        return;
    }

    let filename = &args[1];
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };
    let mut reader = BufReader::new(file);

    let mut address = 0;
    loop 
    {
        match reader.read_u16::<BigEndian>()
        {
            Ok(instruction) => {
                vm.write_memory(address, instruction);
                address += 1;
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("End of file reached");
                } else {
                    eprintln!("Error reading file: {}", err);
                }
                break;
            }
        }
    }

    vm.run();

    tcsetattr(stdin_fd, TCSANOW, &termios).expect("Failed to restore terminal attributes");

}
