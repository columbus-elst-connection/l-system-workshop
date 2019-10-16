use api::{LSystem, Symbol};
use std::iter::successors;

pub struct Interpreter<T> where T: Symbol {
    lsystem: LSystem<T>
}

impl <T> Interpreter<T> where T: Symbol {
    pub fn new(lsystem: LSystem<T>) -> Self {
        Self { lsystem }
    }

    pub fn level(&self, n : usize) -> Vec<T> {
        let start: Vec<T> = self.lsystem.axiom.iter().cloned().collect();
        successors(Some(start), |word|{
            let next = word.iter().flat_map(|symbol| self.lsystem.rules.apply(*symbol)).collect();
            Some(next)
        }).nth(n).unwrap() // there should always be an production
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use api::{RenderConfig, LSystemRules, Rule, LSystem};
    use std::collections::HashMap;

    #[test]
    fn interpreter_produces_correct_result_for_each_level() {
        let system = lsystem();
        let interpreter = Interpreter::new(system);
        let mut expectations = HashMap::new();
        expectations.insert(0, vec!['A']);
        expectations.insert(1, vec!['B', 'A']);
        expectations.insert(2, vec!['A', 'B', 'A']);

        for (level, expected) in expectations {
            let actual: Vec<char> = interpreter.level(level);

            assert_eq!(actual, expected);
        }
    }

    fn lsystem() -> LSystem<char> {
        let render_config = RenderConfig { step: 100, angle: 45 };
        let axiom = vec!['A'];
        let rules = LSystemRules::from_rules(vec![
            Rule::new('A', vec!['B', 'A']),
            Rule::new('B', vec!['A']),
        ]);
        LSystem {
            render_config,
            axiom,
            rules,
        }
        
    }
}