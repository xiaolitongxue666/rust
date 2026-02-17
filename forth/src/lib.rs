//! # Forth - 简易 Forth 解释器
//!
//! 栈式语言：数字入栈，+ - * / 二元运算，DUP DROP SWAP OVER 栈操作。
//! 自定义词 `: word def ;`，定义时存储 Op 序列，执行时按 id 调用，避免定义时展开（防止 alloc_attack）。
//!
//! ## 考点
//! - 栈、词法解析
//! - HashMap 存储自定义词
//! - 惰性求值（Call by id 而非展开）

use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Id>,
    definitions: Vec<Vec<Op>>,
}

type Id = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Push(Value),
    Call(Id),
}

impl Forth {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        #[derive(Debug, PartialEq, Eq)]
        enum Mode {
            Execution,
            Word,
            Definition(String, Vec<Op>),
        }
        let mut mode = Mode::Execution;

        for token in input.to_uppercase().split_whitespace() {
            match mode {
                Mode::Execution => {
                    if token == ":" {
                        mode = Mode::Word;
                    } else {
                        eval_op(
                            parse_op(token, &self.words)?,
                            &mut self.stack,
                            &self.definitions,
                        )?;
                    }
                }
                Mode::Word => {
                    if token.parse::<Value>().is_ok() {
                        return Err(Error::InvalidWord);
                    }
                    mode = Mode::Definition(token.into(), Vec::new());
                }
                Mode::Definition(_, ref mut definition) => {
                    if token == ";" {
                        if let Mode::Definition(word, definition) =
                            std::mem::replace(&mut mode, Mode::Execution)
                        {
                            self.definitions.push(definition);
                            self.words.insert(word, self.definitions.len() - 1);
                        } else {
                            unreachable!();
                        }
                    } else {
                        definition.push(parse_op(token, &self.words)?);
                    }
                }
            }
        }

        (mode == Mode::Execution)
            .then_some(())
            .ok_or(Error::InvalidWord)
    }
}

fn parse_op(token: &str, words: &HashMap<String, Id>) -> std::result::Result<Op, Error> {
    Ok(if let Some(&id) = words.get(token) {
        Op::Call(id)
    } else {
        match token {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "DUP" => Op::Dup,
            "DROP" => Op::Drop,
            "SWAP" => Op::Swap,
            "OVER" => Op::Over,
            _ => Op::Push(token.parse::<Value>().map_err(|_| Error::UnknownWord)?),
        }
    })
}

fn eval_op(
    op: Op,
    stack: &mut Vec<Value>,
    definitions: &[Vec<Op>],
) -> std::result::Result<(), Error> {
    let mut pop = || stack.pop().ok_or(Error::StackUnderflow);
    match op {
        Op::Add => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs + rhs);
        }
        Op::Sub => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs - rhs);
        }
        Op::Mul => {
            let (rhs, lhs) = (pop()?, pop()?);
            stack.push(lhs * rhs);
        }
        Op::Div => {
            let (rhs, lhs) = (pop()?, pop()?);
            let quotient = lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?;
            stack.push(quotient);
        }
        Op::Dup => {
            let top = *stack.last().ok_or(Error::StackUnderflow)?;
            stack.push(top);
        }
        Op::Drop => {
            pop()?;
        }
        Op::Swap => {
            let (top, below) = (pop()?, pop()?);
            stack.push(top);
            stack.push(below);
        }
        Op::Over => {
            let below = *stack.iter().nth_back(1).ok_or(Error::StackUnderflow)?;
            stack.push(below);
        }
        Op::Push(num) => {
            stack.push(num);
        }
        Op::Call(fn_id) => {
            for op in &definitions[fn_id] {
                eval_op(*op, stack, definitions)?;
            }
        }
    }
    Ok(())
}
