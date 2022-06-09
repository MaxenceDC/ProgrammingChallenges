use rand::{
  distributions::WeightedIndex,
  prelude::{Distribution, SliceRandom},
};
use regex::Regex;
use std::collections::HashMap;

type Transitions = HashMap<String, HashMap<String, usize>>;
type States = Vec<String>;

struct SentenceMarkovChain {
  states: States,
  transitions: Transitions,
}

impl SentenceMarkovChain {
  fn new(text: String) -> SentenceMarkovChain {
    // A regex matching everything that is not a letter, a punctuation sign,
    // or a space.
    let unwanted_chars = Regex::new(r"[^A-Za-zÀ-ÖØ-öø-ÿ\s!?\.,;-]").unwrap();

    // Removes all the unwanted characters using the regex and creates a vector
    // of all the words in the `text`, uppercase.
    let mut states: States = unwanted_chars
      .replace_all(text.as_str(), "")
      .to_uppercase()
      .split_whitespace()
      .map(|w| w.to_string())
      .collect();

    // Creates a default hashmap of all the states and their probability
    // with default value 0.
    let state = {
      let mut state = HashMap::new();
      states.iter().for_each(|x| {
        state.insert(x.to_string(), 0);
      });

      state
    };

    // Initializes all the transitions probability using the previously
    // created `state` hashmap.
    let mut transitions: Transitions = HashMap::new();
    states.iter().for_each(|x| {
      transitions.insert(x.to_string(), state.clone());
    });

    // Creates a vector of states shifted to the left, to easily find the
    // word following another word.
    let shifted_states: States = {
      let mut shifted = states.clone();
      shifted.rotate_left(1);

      shifted
    };

    // Inserts the probability of each possible state for each parent state.
    for (i, w) in states.iter().enumerate() {
      let next = (&shifted_states[i]).to_string();
      *transitions.get_mut(w).unwrap().entry(next).or_insert(0) += 1;
    }

    // Removes all duplicate words in the `states` vector.
    states.sort();
    states.dedup();

    // Returns the final SentenceMarkovChain!
    SentenceMarkovChain {
      states,
      transitions,
    }
  }

  fn generate_sentence(&self, n: usize) -> String {
    // Initializes a random number generator.
    let mut rng = rand::thread_rng();

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
        let mut weights: Vec<(String, usize)> = vec![];
        self.transitions[&current_word].iter().for_each(|x| {
          if x.1 > &0 {
            weights.push((x.0.to_string(), *x.1))
          } else {
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
        (&weighted_words[distribution.sample(&mut rng)].0).to_string();

      // Adds a whitespace followed by the chosen word to the result sentence.
      sentence.push(' ');
      sentence.push_str(current_word.as_str());
    }

    // Returns the final sentence!
    sentence
  }
}

fn main() {
  // Stores the input string that will be used to generate the Markov Chain
  // Sentence.
  let input_text = std::fs::read_to_string("input.txt").unwrap();

  // Creates this Markov Chain and generates a sentence of `n` words
  let n = 64;
  let markov_chain = SentenceMarkovChain::new(input_text);
  let markov_sentence = markov_chain.generate_sentence(n);

  // Prints the resulting sentence!
  println!("{markov_sentence}");
}
