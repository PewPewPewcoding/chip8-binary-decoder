use std::fs::File;
use std::io;
use std::io::{Read, Write};

// Struct to store the arguments
pub struct UserInput {
    pub input_filename: String,
    pub output_filename: String,
}

impl UserInput {
    // Handles the arguments supplies by the user
    pub fn new(mut args: std::env::Args) -> Result<UserInput, &'static str> {
        args.next();

        let input_filename = match args.next() {
            Some(d) => d,
            None => return Err("not enough arguments, 2 arguments needed"),
        };
        let output_filename = match args.next() {
            Some(d) => d,
            None => return Err("not enough arguments, 2 arguments needed"),
        };

        Ok(UserInput {input_filename, output_filename,})
    }
}

pub fn run(args: UserInput) -> Result<(), io::Error> {
    // Opens/creates input and output files needed
    let mut input_file = File::open(args.input_filename)?;
    let mut output_file = File::create(args.output_filename)?;

    // Makes variable we need to store the input files contents
    let mut tmp: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut tmp)?;

    // Puts processed input file contents into contents variable
    let contents = vec8_to_vec16(tmp);

    // Write to the output file the data in contents
    writeln!(
        output_file,
        "Number of instructions(2 bytes): {}\n",
        contents.len()
    )?;
    for item in contents.iter() {
        writeln!(output_file, "0x{:04X}", item)?;
    }
    Ok(())
}

// logic for putting the u8 vector values into the u16 vector, putting 2 bytes together.
fn vec8_to_vec16(input: Vec<u8>) -> Vec<u16> {
    let mut output = Vec::new();

    for (index, data) in input.iter().enumerate() {
        match index % 2 {
            0 => output.push(u16::from(*data) * 256),
            _ => output[index / 2] += u16::from(*data),
        }
    }
    output
}
