use rand::{distributions, prelude::*};
use regex::Regex;
use std::{collections::HashMap, fs, io, process};

mod args;

type Transitions = HashMap<String, HashMap<String, usize>>;
type States = Vec<String>;

struct SentenceMarkovChain {
  states: States,
  transitions: Transitions,
}

impl SentenceMarkovChain {
  pub fn new(text: String) -> Result<Self, io::Error> {
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
      return Err(io::Error::new(
        io::ErrorKind::Other,
        "The input text does not contain any words.",
      ));
    }

    // Crates the HashMap of all the states and their probability to be followed
    // by any other state.
    let mut transitions: Transitions = HashMap::new();

    // For every words, gets the next word in the text, and adds one to its
    // probability.
    for (i, word) in states.iter().enumerate() {
      // If the word is new in the transitions, inserts it.
      transitions.entry(word.to_owned());

      if transitions.get(word).is_none() {
        transitions.insert(word.to_owned(), HashMap::new());
      }

      // Finds the word followed by the current word. If the current word is
      // the last, gets the first element instead.
      let next = states
        .get(i + 1)
        .unwrap_or_else(|| states.first().unwrap())
        .to_owned();

      // If the next word is new in the transtitions of `word`, inserts it
      // with a default value of 1. Else, adds 1 to the probability of this
      // word.
      transitions
        .get_mut(word)
        .unwrap()
        .entry(next)
        .and_modify(|x| *x += 1)
        .or_insert(1);
    }

    // Removes all duplicate words in the `states` vector.
    states.sort();
    states.dedup();

    // Returns the final SentenceMarkovChain!
    Ok(Self {
      states,
      transitions,
    })
  }

  fn generate(&self, n: usize) -> String {
    // Initializes a random number generator.
    let mut rng = rand::thread_rng();

    // Choses a random word from all the possible states to use it as the
    // first word.
    let start_word = self.states.choose(&mut rng).unwrap();

    // Initializes the current word and the result sentence.
    let mut current_word = start_word.clone();
    let mut sentence = current_word.to_owned();

    // Appends `n` words following Markov Chain's rules using a loop.
    for _ in 1..n {
      let weighted_words = {
        // Creates a weighted vector of possible words based on their
        // probability to be the next word.
        let mut weights = vec![];
        self.transitions[&current_word].iter().for_each(|x| {
          if x.1 > &0 {
            weights.push((x.0.to_owned(), *x.1))
          }
        });

        weights
      };

      // Creates a distribution for the random number generator.
      let distribution =
        distributions::WeightedIndex::new(weighted_words.iter().map(|x| x.1))
          .unwrap();

      // Chose a random possible word based on the weighted distribution and
      // updates the current word with this new word.
      current_word =
        (&weighted_words[distribution.sample(&mut rng)].0).to_owned();

      // Adds a whitespace followed by the chosen word to the result sentence.
      sentence.push_str(format!(" {current_word}").as_str());
    }

    // Returns the final sentence!
    sentence
  }
}

fn main() {
  // Gets the user-provided parameters.
  let parameters = args::get_args();

  let input_text = match fs::read_to_string(parameters.input_file) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("There was a problem trying to read the file: {e}");
      process::exit(1)
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
      process::exit(22)
    }
  };
  let markov_sentence = markov_chain.generate(parameters.sentence_size);

  println!("{:#?}", markov_chain.transitions);
  // Prints the resulting sentence!
  println!("Result:\n{markov_sentence}");
}
