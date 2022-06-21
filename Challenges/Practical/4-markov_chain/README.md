# N°4 - Markov Chain Sentence Generator

Second challenge on the list, a Markov Chain Sentence Generator! This generator
will also be written in Rust. I have already heard about Markov Chains before,
but I don't actually know how they work and what they are supposed to do. So
let's start doing some research and implementing an algorithm for generating
those Markov Chains!

## Starting the challenge

I First need to document myself on Markov Chains. A quick web search leads me to
[this](https://en.wikipedia.org/wiki/Markov_chain) Wikipedia page. Reading
through the article, I learn that there are two main types of Markov Chains:
Discrete and Continuous. As I will be generating a sentence consisting of
discrete words, I will need to implement a *Discrete-Time Markov Chain Sentence*
*Generator*. The purpose of such a chain is to predict a probable sequence
outcome based on an input. In the example of a sentence, each word is followed
by the next word, they are consecutive. A Markov Chain algorithm will look at
all those words and produce a probability for each word to be followed by
another specific word. It can seem a bit complicated at first, but I will
hopefully get the hang of it.

So, what does my program needs to do? Well, it's a sentence generator, so it
needs to create a somewhat syntactically *logic* sentence with the use of a
Markov Chain (spoiler alert, it won't be logically correct...). For this, the
program will take a sentence as input and will generate a new sentence (a Markov
Chain) based on this input.

As usual, I create my new Rust project with `cargo new`. I name the folder
`4-markov_chain` and I start by opening the `main.rs` file in my IDE.

## Solving the challenge

### The Markov Chain Structure

We start by creating a new structure for the Markov Chain. We will use a Vector
of Strings to represent the words of the sentence, and a `HashMap` to represent
the transition probabilities:

```rs
type States = Vec<String>;
type Transitions = HashMap<String, HashMap<String, usize>>;

struct SentenceMarkovChain {
  states: States,
  transitions: Transitions,
}
```

The `Transitions` contains a `String` as a key, and a `HashMap` as a value. The
`HashMap` contains a `String` as a key, and an `usize` as a value, which
represents the number of times the word is followed by another word (the weight
or *probability*).

#### Implementing the `new` Method

We then need to implement the `new` method for the `SentenceMarkovChain`. This
function will need to take a sentence as input, and return a new Option for a
`SentenceMarkovChain` with `states` containing every possible words, and
`transitions` containing the transitions probabilities. To do that, we start by
removing any non-alphabetic/punctuation characters from the sentence using this
regex (and the `regex` crate):

```regex
/[^A-Za-zÀ-ÖØ-öø-ÿ\s!?\.,;-]/
```

Then, we will split the sentence into words, and add each word to the `states`
Vector. Finally, we will make sure the sentence contains at least one word. If
not, we return an `Err`.

Here is the first part of the `new` method:

```rs
let unwanted_chars = Regex::new(r"[^A-Za-zÀ-ÖØ-öø-ÿ\s!?\.,;-]").unwrap();
let mut states: States = unwanted_chars
  .replace_all(text.as_str(), "")
  .to_uppercase()
  .split_whitespace()
  .map(String::from)
  .collect();

if states.is_empty() {
  return Err(io::Error::new(
    io::ErrorKind::Other,
    "The input text does not contain any words.",
  ));
} 
```

Then, we will need to generate the `transitions`. The first step is to create
an empty `HashMap` matching the `Transitions` type. Then, we will iterate over
the `states` Vector, and for each word, we will find the next word in the
sentence.

We start by iterating over all the `states` (words) in the sentence, and we add
each word not previously added to the `transitions` `HashMap`:

```rs
for (i, word) in states.iter().enumerate() {
  if transitions.get(word).is_none() {
    transitions.insert(String::from(word), HashMap::new());
  }
  // - Snip -
}
```

Then, we find the `next` word in the sentence by getting the word at the index
`i + 1`. If the index is out of bounds (and returns a `None` type), we instead
get the first word in the states, using `unwrap_or_else` and a closure. We then
add the `next` word to the `transitions` using the `entry` method, and modify
the value of the `HashMap` entry by incrementing it by 1, or insert 1 if it
doesn't exist yet. I like how with the Rust syntax, this code can nearly be read
as a normal sentence!

```rs
for (i, word) in states.iter().enumerate() {
  // - Snip -
  let next = String::from(
    states.get(i + 1).unwrap_or_else(|| states.first().unwrap()),
  );
  
  transitions
    .get_mut(word)
    .unwrap()
    .entry(next)
    .and_modify(|x| *x += 1)
    .or_insert(1);
}
```

And the final step for the `new` method is to remove the duplicates in the
states and return the `SentenceMarkovChain` with the `states` and `transitions`
filled. The reason why we sort the `states` is that the `dedup` method removes
only consecutive duplicates, and we want to remove all duplicates.

```rs
states.sort();
states.dedup();

Ok(Self {
  states,
  transitions,
})
```

Okay, so now we have a working `new` method. But let's visualize what it does.
Consider the following sentence:

```rs
"Hello you How are you today"
```

Obviously this sentence is very short, so the output will be very similar to the
input. But if we look at the `new` method, we can see that it creates a new
`SentenceMarkovChain` with the following `states` and `transitions`:

```rs
states: ["HELLO", "YOU", "HOW", "ARE", "TODAY"],
transitions: {
  "HELLO": {
    "YOU": 1,
  },
  "HOW": {
    "ARE": 1,
  },
  "ARE": {
    "YOU": 1,
  },
  "YOU": {
    "TODAY": 1,
    "HOW": 1,
  },
  "TODAY": {
    "HELLO": 1,
  },
},
```

We can see that the `states` contains each word without any duplicates, and the
`transitions` contains all the probabilities. This can be better visualized with
a graph:

![Graph of the Markov Chain](images/Graph.png#gh-dark-mode-only)
![Graph of the Markov Chain](images/GraphL.png#gh-light-mode-only)

Here, all the nodes representing all the possible `states` are connected
together with the edges representing the probabilities. On this graph, the
probabilities are between 0 and 1, but in my code, they are integers
representing the number of times a word is followed by another word (so they are
more like weights).

#### Implementing the `generate` Method

Now that we have this transition structure, we can implement the `generate`
method. This method will take a `usize` (number of words) as an input, and will
return a `String` (the resulting sentence). Because probabilities involves
randomness (in this context), we will need to use the
[`rand`crate](https://crates.io/crates/rand). I add it to the `Cargo.toml` file,
and then import it using `use`. Because we will be using weighted randomness, we
also import the `distributions`
  
```rs
use rand::{distributions, prelude::*};
```

We first need to create a new random number generator thread using `thread_rng`:

```rs
let mut rng = rand::thread_rng();
```

Then, we randomly chose a starting word, and initialize the `sentence` vector
that will contain all the words. We start by adding the starting word to the
`sentence` vector:

```rs
let mut current_word = self.states.choose(&mut rng).unwrap().to_owned();
let mut sentence = vec![current_word.to_owned()];
```

Then we will loop `n` times (the number of words we want to generate). For each
iteration, we will get a list of possible (weighted) words based on the current
word, and then chose a random word from that list. We then add the word to the
`sentence` vector. Finally, we return the `sentence` vector as a `String` by
joining the words with a space.

```rs
for _ in 1..n {
  let weighted_words = {
    let mut weights = Vec::new();
    self.transitions[current_word]
      .iter()
      .for_each(|x| weights.push((x.0, x.1)));
    weights
  };

  let distribution =
    distributions::WeightedIndex::new(weighted_words.iter().map(|x| x.1))
      .unwrap();

  current_word = weighted_words[distribution.sample(&mut rng)].0;
  sentence.push(current_word.to_owned());
}

sentence.join(" ")
```

We now have a working Markov Chain Sentence Generator!

### Using the Generator

Inside the `main` function, we need to create a new Markov Chain based on an
input text. To do that, we use the `new` method we created earlier:
  
```rs
let markov_chain = MarkovChain::new(text).unwrap();
```

> For the moment, we don't have a text so this `text` variable is a placeholder.

Then, we can print a sentence generated from this chain by using the `generate`
method:

```rs
let sentence = markov_chain.generate(9).unwrap();
println!("{sentence}");
```

> Here, we generate a sentence with 9 words.

If the text is, for example:

```txt
But I must explain to you how all this mistaken idea of denouncing pleasure and
praising pain was born, and I will give you a complete account of the system,
and expound the actual teachings of the great explorer of the truth, the
master-builder of human happiness. No one rejects, dislikes, or avoids pleasure
itself, because it is pleasure, but because those who do not know how to pursue
pleasure rationally encounter consequences that are extremely painful. Nor again
is there anyone who loves or pursues or desires to obtain pain of itself,
because it is pain, but because occasionally circumstances occur in which toil
and pain can procure him some great pleasure. To take a trivial example, which
of us ever undertakes laborious physical exercise, except to obtain some
advantage from it? But who has any right to find fault with a man who chooses to
enjoy a pleasure that has no annoying consequences, or one who avoids a pain
that produces no resultant pleasure?
```

> This text is just a *Lorem-like* text in English

We could get:
  
```txt
BECAUSE IT IS THERE ANYONE WHO HAS ANY RIGHT
```

Which makes no sense, but is still (kind of) syntactically correct. Plus, this
sentence does not appear anywhere in the input text, so it created a brand-new
sentence! *The bigger the input is, the better the sentence will be.*

With that, we can say the challenge is completed!

### Final Code

The final code can be found in the [`main.rs`](src/main.rs) file, inside the `src`
folder.

## Going further: Using arguments

To make this tool easier to use, we can implement the possibility to use
arguments to specify the input text and number of words to generate. When
running the program, the user will need to append two arguments like this:

```sh
./markov_chain_sentence_generator <path> <number>
```

> `path` is the path to the file containing the input text and `number` is the
> number of words to generate.

### The `args` module

To handle those arguments, we create a new module, `args` in another file
(`src/args.rs`), and then import it in the main file using the `mod` keyword.
This module contains one public function called `get_args` that will return a
`Parameters` structure with the path and the number in it. In this function, we
start by collecting the arguments in a `Vec` of `Strings` using the `env::args`
method of the standard library. We need to do four checks before processing the
arguments:

  1. Check if the arguments contain **-h** or **--help**.
  2. Check if there are not enough or too many arguments.
  3. Check if the path leads to an invalid file.
  4. Check if the number is invalid.

In the first two cases, we print the help message and in the two remaining
cases, we print an error message. Of course, we exit the program if any of those
checks is true.

Finally, if all the checks are passed, we return the `Parameters` structure with
the path and the parsed number in it.

Here are the four checks and the return statement of the `get_args` function:

```rs
pub fn get_args() -> Parameters {
  let args = std::env::args().collect::<Vec<String>>();

  // Check 1
  if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
    print!("{HELP_MESSAGE}");
    process::exit(0)
  }

  // Check 2
  if args.len() < 2 || args.len() > 3 {
    print!("{HELP_MESSAGE}");
    process::exit(22)
  }
  
  // Check 3
  let path = path::PathBuf::from(&args[1]);
  if !path.exists() || !path.is_file() {
    eprintln!("File `{}` was not found!", path.display());
    process::exit(2)
  }
  
  // Check 4
  let size = &args[2].parse::<usize>().unwrap_or_default();
  if size.eq(&0) {
    eprintln!("The size provided is not a valid, strictly positive number!");
    process::exit(22)
  }

  Parameters {
    input_file: path,
    sentence_size: size.to_owned(),
  }
}
```

### In the `main` function

Back inside the main function, we call the `get_args` and store the result in
a variable called `params`. Then, we read the file using the path of the
`params` and store it in a variable called `input_text`. If the file is not
readable, we print an error message and exit the program.

```rs
let params = args::get_args();

let input_text = match fs::read_to_string(params.input_file) {
  Ok(content) => content,
  Err(e) => {
    eprintln!("There was a problem trying to read the file: {e}");
    process::exit(1)
  }
};
```

Finally, we pass the text into the `SentenceMarkovChain` constructor and print
the generated sentence (while not forgetting to handle the possible errors)!

```rs
let markov_chain = match SentenceMarkovChain::new(input_text) {
  Ok(s) => s,
  Err(e) => {
    eprintln!(
      "There was a problem trying to create the Markov Chain Sentence: {e}"
    );
    process::exit(22)
  }
};


let sentence = markov_chain.generate(params.sentence_size);
println!("Result:\n{sentence}");
```

## Challenge finished

The final entirely commented source code can be found in the [`src`](./src/)
folder.
