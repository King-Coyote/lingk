use std::collections::HashMap;
use rand::prelude::*;
use crate::token::Token;
use crate::util::*;

struct Node {
    data: Token,
    links: HashMap<Token, u32>,
    weights: Option<Vec<(Token, f32)>>,
    total: u32,
}

impl Node {
    pub fn new(data: Token) -> Self {
        Node {
            data,
            links: HashMap::default(),
            weights: None,
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

    pub fn calculate(&mut self) {
        let mut counts: Vec<(Token, u32)> = self.links.iter()
            .map(|(t, c)| (t.clone(), *c))
            .collect::<Vec<(Token, u32)>>();
        counts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut acc = 0;
        let weights = counts
            .into_iter()
            .map(|(token, count)| {
                acc += count;
                (token, acc as f32 / self.total as f32)
            })
            .collect();
        self.weights = Some(weights);
    }

    pub fn choose(&self) -> Option<Token> {
        let n: f32 = rand::thread_rng().gen();
        self.weights.as_ref().expect("Weights requested before calculation")
            .iter()
            .find(|(_, w)| n <= *w)
            .map(|(t, _)| t.clone())
    }
}

#[derive(Default)]
pub struct Chain {
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
        assert!(float_eq(weights.get(0).unwrap().1, 0.25));
        assert!(float_eq(weights.get(1).unwrap().1, 1.0));
    }
}