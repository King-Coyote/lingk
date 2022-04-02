use std::collections::HashMap;
use rand::prelude::*;
use crate::token::Token;

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
            .or_insert(1)
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

    pub fn choose(&mut self) -> Option<Token> {
        let n: f32 = rand::thread_rng().gen();
        if self.weights.is_none() {
            self.calculate();
        }
        self.weights.as_ref().unwrap()
            .iter()
            .find(|(_, w)| *w < n)
            .map(|(t, _)| t.clone())
    }
}

#[derive(Default)]
pub struct Chain {
    nodes: HashMap<Token, Node>,
}

impl Chain {
    pub fn feed(&mut self, data: Vec<Token>) {
        let mut prev: Option<&mut Node> = None;
        for token in data.iter() {
            let node = self.nodes
                .entry(token.clone())
                .or_insert_with(|| Node::new(token.clone()));
            prev = Some(node);
        }
    }
}

