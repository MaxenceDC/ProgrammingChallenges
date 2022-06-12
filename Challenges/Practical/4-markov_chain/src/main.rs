use rand::{
  distributions::WeightedIndex,
  prelude::{Distribution, SliceRandom},
  thread_rng,
};
use regex::Regex;
use std::{
  collections::HashMap,
  env::args,
  fs::read_to_string,
  io::{Error, ErrorKind},
  path::PathBuf,
  process::exit,
};

type Transitions = HashMap<String, HashMap<String, usize>>;
type States = Vec<String>;

const HELP_MESSAGE: &str = "Markov Chain Sentence Generator - A quick tool used to generate sentences based on an input using a discreet Markov chain algorithm.
 
Usage: markov_chain_sentence_generator (flags) [path] [size]

Flags/Parameters :
  --help (-h)        Prints this help message!
  path               Specify the path of the file containaing the input text that will be fed to the program to generate a sentence from.
  size               Specify the number of words in the resulting sentence. Must be a valid, strictly-positive number.";

struct Parameters {
  input_file: PathBuf,
  sentence_size: usize,
}

struct SentenceMarkovChain {
  states: States,
  transitions: Transitions,
}

impl SentenceMarkovChain {
  pub fn new(text: String) -> Result<Self, Error> {
    // A regex matching everything that is not a letter (can have diacritics),
    // a punctuation sign, or a space.
    let unwanted_chars = Regex::new(r"[^A-Za-zÀ-ÖØ-öø-ÿ\s!?\.,;-]").unwrap();

    // Removes all the unwanted characters using the regex and creates a vector
    // of all the words in the `text`, uppercase.
    let mut states: States = unwanted_chars
      .replace_all(text.as_str(), "")
      .to_uppercase()
      .split_whitespace()
      .map(String::from)
      .collect();

    // If there are no words, returns an error.
    if states.is_empty() {
      return Err(Error::new(
        ErrorKind::Other,
        "The input text does not contain any words.",
      ));
    }

    // Creates a vector of states shifted to the left, to easily find the word
    // following another word.
    let next_states: States = {
      let mut shifted = states.clone();
      shifted.rotate_left(1);
      shifted
    };

    let transitions: Transitions = {
      let mut transitions: Transitions = HashMap::new();
      for (i, word) in states.iter().enumerate() {
        let next = String::from(&next_states[i]);
        if transitions.get(word).eq(&None) {
          transitions.insert(String::from(word), HashMap::new());
        }

        if transitions[word].get(&next).eq(&None) {
          transitions.get_mut(word).unwrap().insert(next, 1);
        } else {
          *transitions.get_mut(word).unwrap().entry(next).or_default() += 1;
        }
      }
      transitions
    };

    // Removes all duplicate words in the `states` vector.
    states.sort();
    states.dedup();

    // Returns the final SentenceMarkovChain!
    Ok(Self {
      states,
      transitions,
    })
  }

  fn generate_sentence(&self, n: usize) -> String {
    // Initializes a random number generator.
    let mut rng = thread_rng();

    // Choses a random word from all the possible states to use it as the
    // first word.
    let start_word = self.states.choose(&mut rng).unwrap();

    // Initializes the current word and the result sentence.
    let mut current_word = start_word.clone();
    let mut sentence = String::from(&current_word);

    // Appends `n` words following Markov Chain's rules using a loop.
    for _ in 1..n {
      let weighted_words = {
        // Creates a weighted vector of possible words based on their
        // probability to be the next word.
        let mut weights = vec![];
        self.transitions[&current_word].iter().for_each(|x| {
          if x.1 > &0 {
            weights.push((String::from(x.0), *x.1))
          }
        });

        weights
      };

      // Creates a distribution for the random number generator.
      let distribution =
        WeightedIndex::new(weighted_words.iter().map(|x| x.1)).unwrap();

      // Chose a random possible word based on the weighted distribution and
      // updates the current word with this new word.
      current_word =
        String::from(&weighted_words[distribution.sample(&mut rng)].0);

      // Adds a whitespace followed by the chosen word to the result sentence.
      sentence.push_str(format!(" {current_word}").as_str());
    }

    // Returns the final sentence!
    sentence
  }
}

fn get_args() -> Parameters {
  // Stores the arguments provided by the user in a Vector of Strings
  let args = args().collect::<Vec<String>>();

  if args.contains(&String::from("--help"))
    || args.contains(&String::from("-h"))
  {
    println!("{HELP_MESSAGE}");
    exit(0)
  }

  // If no arguments are provided or the arguments provided are wrong, prints
  // the help or the program.
  // (exit code 22 means "Invalid argument")
  if args.len() < 2 || args.len() > 3 {
    println!("{HELP_MESSAGE}");
    exit(22)
  }

  // Initializes the `path` parameter, and exits if the file is inexistant.
  // (exit code 2 means "No such file or directory")
  let path = PathBuf::from(&args[1]);
  if !path.exists() || !path.is_file() {
    eprintln!("File `{}` was not found!", path.display());
    exit(2)
  }

  // Initializes the `size` parameter, and exits if the value is unvalid.
  // (exit code 22 means "Invalid argument")
  let size = &args[2].parse::<usize>().unwrap_or_default();
  if size.eq(&0) {
    eprintln!("The size provided is not a valid, strictly positive number!");
    exit(22)
  }

  // Returns the parameters!
  Parameters {
    input_file: path,
    sentence_size: size.to_owned(),
  }
}

fn main() {
  // Gets the user-provided parameters.
  let parameters = get_args();

  let input_text = match read_to_string(parameters.input_file) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("There was a problem trying to read the file: {e}");
      exit(1)
    }
  };

  // Creates a Sentence Markov Chain based on the content of the file and
  // generates a sentence of `n` words
  let markov_chain = match SentenceMarkovChain::new(input_text) {
    Ok(m) => m,
    Err(e) => {
      eprintln!(
        "There was a problem trying to create the Markov Chain Sentence: {e}"
      );
      exit(22)
    }
  };
  let markov_sentence =
    markov_chain.generate_sentence(parameters.sentence_size);

  // Prints the resulting sentence!
  println!("Result:\n{markov_sentence}");
}
