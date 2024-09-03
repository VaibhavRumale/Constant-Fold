extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub use ast::*;

pub mod parser;
pub use parser::*;

pub mod constant_folding;

pub mod error;

use clap::{App, Arg, SubCommand};

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[cfg(test)]
mod tests;

fn main() {
    // Parse command-line arguments
    let matches = App::new("Leo Code Generator")
        .version("1.0")
        .author("Vaibhav")
        .about("Generates Leo code with optional constant fold optimization")
        .subcommand(
            SubCommand::with_name("optimize")
                .about("Optimize and generate Leo code")
                .arg(
                    Arg::with_name("constant_fold")
                        .short('c')
                        .long("constant-fold")
                        .help("Apply constant fold optimization"),
                )
                .arg(
                    Arg::with_name("emit_leo")
                        .short('e')
                        .long("emit-leo")
                        .help("Emit generated Leo code"),
                )
                .arg(
                    Arg::with_name("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Specify output file for generated Leo code")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Specify input file for Leo code")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // Match the subcommand and perform actions accordingly
    if let Some(matches) = matches.subcommand_matches("optimize") {
        let input_file = matches.value_of("input").unwrap_or("src/files/before.leo");

        // Fetch file string
        let unparsed_file = fs::read_to_string(input_file).expect("cannot read file");
        println!("Unparsed file:\n{:?}\n", unparsed_file);

        // Create AST from file string
        let mut file = parse(&unparsed_file).expect("unsuccessful parse");

        // Perform constant folding if the flag is set
        if matches.is_present("constant_fold") {
            match constant_folding::fold_constants(&mut file) {
                Ok(_) => println!("Constant folding applied successfully."),
                Err(errors) => {
                    eprintln!("Constant folding encountered errors:");
                    for error in errors {
                        eprintln!("{}", error);
                    }
                }
            }
        }

        // If --emit-leo is set, generate and possibly save the output
        if matches.is_present("emit_leo") {
            let output = format!("{}", file);

            // Print the generated Leo code
            println!("Generated Leo code:\n\n{}", output);

            // Determine the output file path
            let output_file = if let Some(output_path) = matches.value_of("output") {
                Path::new(output_path).to_path_buf()
            } else {
                Path::new("src/files/generated.leo").to_path_buf()
            };

            // Ensure the directory exists
            if let Some(parent) = output_file.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent).expect("could not create directories");
                }
            }

            // Automatically append ".leo" if the extension is missing
            let output_file = if output_file.extension().is_none() {
                output_file.with_extension("leo")
            } else {
                output_file
            };

            // Write to the specified file or the default
            let mut file = File::create(&output_file).expect("could not create file");
            file.write_all(output.as_bytes())
                .expect("could not write to file");
            println!("Leo code has been written to {}", output_file.display());
        } else {
            // print the original parsed code if no flags are being used
            println!("Parsed Leo code:\n\n{}", file);
        }
    }
}
