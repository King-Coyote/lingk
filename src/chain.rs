use std::collections::HashMap;
use std::cmp::max;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use crate::token::Token;
use crate::util::*;
use crate::error::Error;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct Node {
    data: Token,
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    links: HashMap<Token, i32>,
    weights: Option<Vec<(Token, f32)>>,
    weights_map: Option<HashMap<Token, f32>>,
    total: i32,
}

impl Node {
    pub fn new(data: Token) -> Self {
        Node {
            data,
            links: HashMap::default(),
            weights: None,
            weights_map: None,
            total: 0,
        }
    }

    pub fn link(&mut self, token: Token) {
        *self.links
            .entry(token)
            .or_insert(0)
            += 1;
        self.total += 1;
    }

    pub fn unlink(&mut self, token: &Token, amount: i32) -> Result<(), Error> {
        let current = self.links.get_mut(token).ok_or(Error::TokenNotFound(token.clone()))?;
        if amount <= *current {
            self.total = max(self.total - amount, 0);
        } else {
            self.total -= *current;
        }
        *current = max(*current - amount, 0);
        if *current == 0 {
            self.links.remove(token);
        }
        Ok(())
    }

    pub fn calculate(&mut self) {
        let mut counts: Vec<(Token, i32)> = self.links.iter()
            .map(|(t, c)| (t.clone(), *c))
            .collect::<Vec<(Token, i32)>>();
        counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut acc = 0;
        let weights: Vec<(Token, f32)> = counts
            .into_iter()
            .map(|(token, count)| {
                acc += count;
                (token, acc as f32 / self.total as f32)
            })
            .collect();
        let mut last_weight = 0.0;
        let mut weights_map: HashMap<Token, f32> = HashMap::new();
        for (token, weight) in weights.iter() {
            weights_map.insert(token.clone(), weight - last_weight);
            last_weight = *weight;
        }
        self.weights_map = Some(weights_map);
        self.weights = Some(weights);
    }

    pub fn choose(&self) -> Option<Token> {
        let n: f32 = rand::thread_rng().gen();
        self.weights.as_ref().expect("Weights requested before calculation")
            .iter()
            .find(|(_, w)| n <= *w)
            .map(|(t, _)| t.clone())
    }

    pub fn query_on(&self, b: char) -> Option<f32> {
        self.weights_map.as_ref()?
            .get(&Token::Char(b))
            .copied()
    }
}

#[serde_with::serde_as]
#[derive(Default, Serialize, Deserialize)]
pub struct Chain {
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    nodes: HashMap<Token, Node>,
    is_calculated: bool,
}

impl Chain {
    pub fn feed(&mut self, data: Vec<Token>) {
        let mut prev: Option<&mut Node> = None;
        self.is_calculated = false;
        for token in data.iter() {
            if let Some(ref mut prev_node) = prev {
                prev_node.link(token.clone());
            }
            let node = self.nodes
                .entry(token.clone())
                .or_insert_with(|| Node::new(token.clone()));
            prev = Some(node);
        }
    }

    pub fn calculate(&mut self) {
        for node in self.nodes.values_mut() {
            node.calculate();
        }
        self.is_calculated = true;
    }

    pub fn generate(&self) -> String {
        assert!(self.is_calculated);
        let mut token = Token::Start;
        let mut value = "".to_string();
        let mut last_token_val: String = "".to_owned();
        let mut last_token_count = 0;
        while token != Token::End {
            let node = self.nodes.get(&token).expect("Node not found for token!");
            if last_token_val == token.to_string() {
                last_token_count += 1;
            } else {
                last_token_count = 0;
            }
            if last_token_count < 2 {
                value.push_str(&token.to_string());
            }
            last_token_val = token.to_string();
            token = node.choose().expect("No token chosen by node!");
        }
        value
    }

    // gives the average number of neighbours for all tokens.
    // this is kind of a measure of how good the model is; if its average is ~28,
    // then it's probably shithouse because any token can give any token.
    // Test it out by loading one single language, analyzing, then loading another and doing it again.
    pub fn analyze(&self) -> Option<f32> {
        assert!(self.is_calculated);
        let mut total = 0;
        for (token, node) in self.nodes.iter() {
            if *token == Token::Start {
                continue;
            }
            total += node.weights.as_ref()?
                .iter()
                .filter(|(token, _)| *token != Token::End)
                .count();
        }
        let real_length = self.nodes.len() - 2;
        Some((total as f32) / (real_length as f32))
    }

    pub fn reduce(&mut self, a: &Token, b:&Token, amount: i32) -> Result<(), Error> {
        assert!(self.is_calculated);
        let node_a = self.nodes.get_mut(a).ok_or(Error::TokenNotFound(a.clone()))?;
        node_a.unlink(b, amount)?;
        // TODO better way of handling calculation? it's pretty incoherent rn
        self.calculate();
        Ok(())
    }

    // gives the weight for a transition between chars
    // TODO all this query stuff should use tokens, not chars. knobhead
    pub fn query_between(&self, a: char, b: char) -> Option<f32> {
        assert!(self.is_calculated);
        let node_a = self.nodes.get(&Token::Char(a))?;
        let node_b = self.nodes.get(&Token::Char(b))?;
        Some(0.0)
    }

    // returns a vec of all weights and their tokens from a single char
    pub fn query_single(&self, a: &Token) -> Option<Vec<(Token, f32)>> {
        assert!(self.is_calculated);
        let node_a = self.nodes.get(&a)?;
        // ugh! get the tuples, then explicitly clone their inner values. This is so we don't
        // have ownership things happen. Man this is ugly :(
        node_a.weights_map
            .as_ref()
            .map(|map| map.iter().map(|(k, v)| (k.clone(), *v)).collect())
    }

    pub fn is_calculated(&self) -> bool {
        self.is_calculated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    #[test]
    fn nodes_link_correctly() {
        let mut node = Node::new(Token::Start);
        node.link(Token::End);
        assert!(node.total == 1);
        let count = node.links.get(&Token::End).unwrap();
        assert!(*count == 1);
    }

    #[test]
    fn nodes_calculate_correctly() {
        let mut node = Node::new(Token::Start);
        node.link(Token::Char('a'));
        node.link(Token::Char('a'));
        node.link(Token::Char('a'));
        node.link(Token::Char('b'));
        node.calculate();

        let weights = node.weights.expect("Node should have weights");
        assert!(weights.len() == 2);
        assert!(weights.get(0).unwrap().0 == Token::Char('b'));
        assert!(weights.get(1).unwrap().0 == Token::Char('a'));
        assert!(float_eq(weights.get(0).unwrap().1, 0.25));
        assert!(float_eq(weights.get(1).unwrap().1, 1.0));
    }

    #[test]
    fn query_on_node_gives_correct_value() {
        let mut node = Node::new(Token::Start);
        node.link(Token::Char('a'));
        node.link(Token::Char('a'));
        node.link(Token::Char('a'));
        node.link(Token::Char('b'));
        node.calculate();

        let weight_a = node.query_on('a').unwrap();
        let weight_b = node.query_on('b').unwrap();
        assert!(float_eq(weight_a, 0.75));
        assert!(float_eq(weight_b, 0.25));
    }
}