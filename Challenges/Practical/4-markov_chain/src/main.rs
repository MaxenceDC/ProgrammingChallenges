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

    // Initializes the current word and the result sentence.
    let mut current_word = self.states.choose(&mut rng).unwrap();
    let mut sentence = vec![current_word.to_owned()];

    // Appends `n` words following Markov Chain's rules using a loop.
    for _ in 1..n {
      // Creates a weighted vector of possible words based on their
      // probability to be the next word, based on the current word.
      let weighted_words = {
        let mut weights = Vec::new();
        self.transitions[current_word]
          .iter()
          .for_each(|x| weights.push((x.0, x.1)));

        weights
      };
      // Creates a distribution for the random number generator.
      let distribution =
        distributions::WeightedIndex::new(weighted_words.iter().map(|x| x.1))
          .unwrap();

      // Chose a random possible word based on the weighted distribution and
      // updates the current word with this new word.
      current_word = weighted_words[distribution.sample(&mut rng)].0;

      // Pushes the word to the sentence vector.
      sentence.push(current_word.to_owned());
    }

    // Returns the final sentence separated by spaces!
    sentence.join(" ")
  }
}

fn main() {
  // Gets the user-provided parameters.
  let params = args::get_args();

  let input_text = match fs::read_to_string(params.input_file) {
    Ok(content) => content,
    Err(e) => {
      eprintln!("There was a problem trying to read the file: {e}");
      process::exit(1)
    }
  };

  // Creates a Sentence Markov Chain based on the content of the file and
  // generates a sentence of `n` words
  let markov_chain = match SentenceMarkovChain::new(input_text) {
    Ok(s) => s,
    Err(e) => {
      eprintln!(
        "There was a problem trying to create the Markov Chain Sentence: {e}"
      );
      process::exit(22)
    }
  };

  // Prints the resulting sentence!
  let sentence = markov_chain.generate(params.sentence_size);
  println!("Result:\n{sentence}");
}
