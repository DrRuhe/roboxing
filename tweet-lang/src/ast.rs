use std::iter::repeat;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action {
    Jump(Direction),
    Walk(Direction),
    Punch(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Repeat {
    action: Action,
    count: usize,
}

impl Iterator for Repeat {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count != 0 {
            self.count -= 1;
            return Some(self.action);
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Instruction {
    Action(Action),
    Repetition(Box<Instruction>, usize),
    List(Vec<Instruction>),
}

impl Into<Vec<Action>> for Instruction {
    fn into(self) -> Vec<Action> {
        match self {
            Instruction::Action(action) => vec![action],
            Instruction::Repetition(instruction, n) => {
                let actions: Vec<Action> = (*instruction).into();
                repeat(actions.into_iter()).take(n).flatten().collect()
            }
            Instruction::List(instructions) => instructions
                .into_iter()
                .flat_map(|i| {
                    let actions: Vec<Action> = i.into();
                    actions.into_iter()
                })
                .collect(),
        }
    }
}
