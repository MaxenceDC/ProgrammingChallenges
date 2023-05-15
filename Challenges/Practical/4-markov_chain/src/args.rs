use std::{path, process};

pub struct Parameters {
    pub input_file: path::PathBuf,
    pub sentence_size: usize,
}

const HELP_MESSAGE: &str = "
Markov Chain Sentence Generator - A quick tool used to generate sentences based on an input using a discreet Markov chain algorithm.
 
Usage: markov_chain_sentence_generator (flags) [path] [size]

Flags/Parameters :
  --help (-h)        Prints this help message!
  path               Specify the path of the file containaing the input text that will be fed to the program to generate a sentence from.
  size               Specify the number of words in the resulting sentence. Must be a valid, strictly-positive number.
";

pub fn get_args() -> Parameters {
    // Stores the arguments provided by the user in a Vector of Strings
    let args = std::env::args().collect::<Vec<String>>();

    if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
        print!("{HELP_MESSAGE}");
        process::exit(0)
    }

    // If no arguments are provided or the arguments provided are wrong, prints
    // the help or the program.
    // (exit code 22 means "Invalid argument")
    if args.len() < 2 || args.len() > 3 {
        print!("{HELP_MESSAGE}");
        process::exit(22)
    }

    // Initializes the `path` parameter, and exits if the file is inexistant.
    // (exit code 2 means "No such file or directory")
    let path = path::PathBuf::from(&args[1]);
    if !path.exists() || !path.is_file() {
        eprintln!("File `{}` was not found!", path.display());
        process::exit(2)
    }

    // Initializes the `size` parameter, and exits if the value is unvalid.
    // (exit code 22 means "Invalid argument")
    let size = &args[2].parse::<usize>().unwrap_or_default();
    if size.eq(&0) {
        eprintln!("The size provided is not a valid, strictly positive number!");
        process::exit(22)
    }

    // Returns the parameters!
    Parameters {
        input_file: path,
        sentence_size: size.to_owned(),
    }
}
