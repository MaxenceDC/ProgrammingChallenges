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

I start by creating a new structure for the Markov Chain. I will use a Vector of
Strings to represent the words of the sentence, and a `HashMap` to represent the
transition probabilities:

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
represents the number of times the word is followed by another word (the
"*probability*").

### Implementing the `new` Method

I then need to implement the `new` method for the `SentenceMarkovChain`. This
function will need to take a sentence as input, and return a new Option for a
`SentenceMarkovChain` with `states` containing every possible words, and
`transitions` containing the transitions probabilities. To do that, we start by
removing any non-alphabetic/punctuation characters from the sentence using this
regex:

`/[^A-Za-zÀ-ÖØ-öø-ÿ\s!?\.,;-]/`

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

We start by iterating over all the `states` (words) in the sentence, and we add each word not previously added to the `transitions` `HashMap`:

```rs
for (i, word) in states.iter().enumerate() {
  if transitions.get(word).is_none() {
    transitions.insert(String::from(word), HashMap::new());
  }
  // - Snip -
}
```

Then, we find the `next` word in the sentence by getting the word at the index `i + 1`. If the index is out of bounds (and returns a `None` type), we instead get the first word in the states, using `unwrap_or_else` and a closure. We then add the `next` word to the `transitions` using the `entry` method, and modify the value of the `HashMap` entry by incrementing it by 1, or insert 1 if it doesn't exist yet. I like how with the Rust syntax, this code can nearly be read as a normal sentence!

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

And the final step for the `new` method is to remove the duplicates in the states and return the `SentenceMarkovChain` with the `states` and `transitions` filled. The reason why we sort the `states` is that the `dedup` method removes only consecutive duplicates, and we want to remove all duplicates.

```rs
states.sort();
states.dedup();

Ok(Self {
  states,
  transitions,
})
```

Okay, so now we have a working `new` method. But let's visualize what it does. Consider the following sentence:

```rs
"Hello you How are you today"
```

Obviously this sentence is very short, so the output will be very similar to the input. But if we look at the `new` method, we can see that it creates a new `SentenceMarkovChain` with the following `states` and `transitions`:

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

We can see that the `states` contains each word without any duplicates, and the `transitions` contains all the probabilities. This can be better visualized with a graph:

![Graph of the Markov Chain](images/Graph.png#gh-dark-mode-only)
![Graph of the Markov Chain](images/GraphL.png#gh-light-mode-only)
