use rand::{
  distributions::WeightedIndex,
  prelude::{Distribution, SliceRandom},
  thread_rng,
};
use regex::Regex;
use std::{
  collections::HashMap,
  fs,
  io::{Error, ErrorKind},
  process::exit,
};

mod args;

type Transitions = HashMap<String, HashMap<String, usize>>;
type States = Vec<String>;

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

fn main() {
  // Gets the user-provided parameters.
  let parameters = args::get_args();

  let input_text = match fs::read_to_string(parameters.input_file) {
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
