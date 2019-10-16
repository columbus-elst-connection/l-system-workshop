use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct RenderConfig {
    pub step: u16,
    pub angle: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RendererInstruction {
    Forward,

    RotateLeft,
    RotateRight,

    Push,
    Pop,

    NoOp,
}

pub trait Symbol: Display + Debug + Eq + Hash + Copy {
    fn to_rendering_instruction(&self) -> RendererInstruction;
}

impl Symbol for char {
    fn to_rendering_instruction(&self) -> RendererInstruction {
        match *self {
            'F' => RendererInstruction::Forward,
            '+' => RendererInstruction::RotateRight,
            '-' => RendererInstruction::RotateLeft,

            '[' => RendererInstruction::Push,
            ']' => RendererInstruction::Pop,

            _ => RendererInstruction::NoOp,
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct Rule<T: Symbol> {
    pub match_input: T,
    pub productions: Vec<T>
}

impl<T: Symbol> Rule<T> {
    pub fn new(match_input: T, productions: Vec<T>) -> Rule<T> {
        Rule { match_input, productions }
    }
}

#[derive(Debug, PartialEq)]
pub struct LSystemRules<T: Symbol> {
    rules: HashMap<T, Vec<T>>,
}

impl <T: Symbol> LSystemRules<T> {

    pub fn new() -> LSystemRules<T> {
        LSystemRules {
            rules: HashMap::with_capacity(4),
        }
    }

    pub fn from_rules(rules: Vec<Rule<T>>) -> LSystemRules<T> {
        let mut system = LSystemRules::new();
        for rule in rules {
            system = system.add_rule(rule);
        }
        system
    }

    pub fn add_rule(self, rule: Rule<T>) -> Self {
        let Rule {match_input, productions} = rule;
        self.add(match_input, productions)
    }

    pub fn add(mut self, match_input: T, productions: Vec<T>) -> Self {
        self.rules.insert(match_input, productions);
        self
    }

    pub fn apply(&self, symbol: T) -> Vec<T> {
        self.rules.get(&symbol).cloned().unwrap_or_else(|| {
            vec![symbol]
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct LSystem<T: Symbol> {
    pub render_config: RenderConfig,
    pub axiom: Vec<T>,
    pub rules: LSystemRules<T>,
}
