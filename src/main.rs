use markov_chains::markov_chains::{MarkovChains, MarkovChainsStruct};

mod markov_chains;

fn main() {
    let mut mc = MarkovChainsStruct::new();
    let words = ["a", "b", "c"].map(String::from);
    mc.insert_words(words);

    let res = mc.get_words();
    println!("{:?}", res);
}
