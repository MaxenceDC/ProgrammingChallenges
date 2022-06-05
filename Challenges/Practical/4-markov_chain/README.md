# n°4 - Markov Chain Sentence Generator

Second challenge on the list, a Markov Chain Sentence Generator! This generator
will also be written in Rust. I have already heard about Markov Chains before
but I don't actually know how they work and what they are supposed to do. So
let's start doing some research and implementing an algorithm for generating
those Markov Chains!

## Starting the challenge

I First need to document myself on Markov Chains. A quick web search leads me to
[this](https://en.wikipedia.org/wiki/Markov_chain) Wikipedia page. Reading
through the article, I learn that there are two main types of Markov Chains:
Discrete and Continuous. As I will be generating a sentence consisting of
discrete words, I will need to implement a _Discrete-Time Markov Chain Sentence_
_Generator_. The purpose of such a chain is to predict a probable sequence
outcome based on an input. In the exemple of a sentence, each word is followed
by the next word, they are consecutives. A Markov Chain algorithm will look at
all those words and produce a probability for each word to be followed by
another specific word. It can seem a bit complicated at first, but you will
hopefully get the hang of it.

So, what does my progam needs to do? Well, it's a sentence generator, so it
needs to create a somewhat logic sentence with the use of a Markov Chain
(spoiler alert, it won't...). For this, the program will take a sentence as
input and will generate a new sentence (a Markov Chain) based on this input.

As usual, I create my new Rust project with `cargo new`. I name the folder
`4-markov_chain` and I start by opening the `main.rs` file in my IDE.

## Solving the challenge

