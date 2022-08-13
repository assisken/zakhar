use std::{cell::RefCell, collections::HashMap};

pub trait MarkovChains {
    fn insert_words(&mut self, words: impl Into<Vec<String>>);
    fn get_words(&self) -> Vec<String>;
}

pub struct MarkovChainsStruct {
    links: HashMap<String, RefCell<Vec<String>>>,
}

impl MarkovChainsStruct {
    pub fn new() -> Self {
        return MarkovChainsStruct {
            links: HashMap::new(),
        };
    }
}

impl MarkovChains for MarkovChainsStruct {
    fn insert_words(&mut self, words: impl Into<Vec<String>>) {
        let words: Vec<String> = words.into();
        for i in 0..words.len() - 1 {
            let first = words[i].clone().to_lowercase();
            let second = words[i + 1].clone().to_lowercase();

            match self.links.get(&first) {
                Some(link) => link.borrow_mut().push(second),
                None => drop(self.links.insert(first, RefCell::new(vec![second]))),
            };
        }
    }

    fn get_words(&self) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{MarkovChains, MarkovChainsStruct};
    use test_case::test_case;

    #[test_case("" => HashMap::new(); "when empty")]
    #[test_case("A purple a green and a kite" => HashMap::from([
        (String::from("a"), String::from("purple green kite")),
        (String::from("purple"), String::from("a")),
        (String::from("green"), String::from("and")),
        (String::from("and"), String::from("a")),
    ]); "with some words")]
    #[test_case("foo" => HashMap::new(); "the last word are ignored")]
    fn word_adding(sentence: &str) -> HashMap<String, String> {
        let mut mc = MarkovChainsStruct::new();
        mc.insert_words(
            sentence
                .split(' ')
                .map(String::from)
                .collect::<Vec<String>>(),
        );

        let mut map = HashMap::new();
        for (k, v) in mc.links {
            map.insert(k, v.borrow().join(" "));
        }
        map
    }
}
