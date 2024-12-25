use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            other => panic!("unknown operation: {other}"),
        }
    }
}

impl Operation {
    fn operate(&self, inputs: [bool; 2]) -> bool {
        match self {
            Operation::And => inputs[0] && inputs[1],
            Operation::Or => inputs[0] || inputs[1],
            Operation::Xor => inputs[0] ^ inputs[1],
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Instruction {
    inputs: [String; 2],
    output: String,
    operation: Operation,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.inputs == other.inputs
    }
}

impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.inputs.contains(&self.output) {
            return Some(Ordering::Less);
        }
        if self.inputs.contains(&other.output) {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut words = value.split(' ');
        let input1 = words.next().unwrap();
        let operation = words.next().unwrap();
        let input2 = words.next().unwrap();
        let _arrow = words.next().unwrap();
        let output = words.next().unwrap();

        Self {
            inputs: [String::from(input1), String::from(input2)],
            output: String::from(output),
            operation: Operation::from(operation),
        }
    }
}

#[derive(Debug)]
struct Device {
    memory: HashMap<String, bool>,
    instructions: Vec<Instruction>,
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let (wires, instructions) = value.split_once("\r\n\r\n").unwrap();

        let memory = wires
            .lines()
            .map(|line| match line.split_once(": ") {
                Some((name, "1")) => (String::from(name), true),
                Some((name, "0")) => (String::from(name), false),
                Some(other) => panic!("unknown wire configuration: {other:?}"),
                None => panic!("couldn't parse wire line"),
            })
            .collect();

        let instructions: Vec<Instruction> = instructions.lines().map(Instruction::from).collect();

        Self {
            memory,
            instructions,
        }
    }
}

impl Device {
    fn execute(&mut self) {
        let mut ready_instructions = Vec::new();

        let mut index = 0;
        while index < self.instructions.len() {
            let instruction = &self.instructions[index];
            if instruction
                .inputs
                .iter()
                .all(|input| self.memory.contains_key(input))
            {
                ready_instructions.push(self.instructions.remove(index));
            } else {
                index += 1;
            }
        }

        while let Some(instruction) = ready_instructions.pop() {
            self.execute_instruction(instruction);

            let mut index = 0;
            while index < self.instructions.len() {
                let instruction = &self.instructions[index];
                if instruction
                    .inputs
                    .iter()
                    .all(|input| self.memory.contains_key(input))
                {
                    ready_instructions.push(self.instructions.remove(index));
                } else {
                    index += 1;
                }
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let Instruction {
            inputs,
            output,
            operation,
        } = instruction;

        let inputs = inputs.map(|input| self.memory[&input]);
        self.memory.insert(output, operation.operate(inputs));
    }

    fn get_output(&self) -> usize {
        let mut index = 0;
        let mut result = 0;
        let mut pow = 1;

        loop {
            let key = format!("z{index:0>2}");

            match self.memory.get(&key) {
                Some(&value) => result += pow * usize::from(value),
                None => break,
            }

            pow *= 2;
            index += 1;
        }

        result
    }
}

#[allow(dead_code)]
pub fn part_one() -> usize {
    let input: &str = include_str!("../../input/day24.txt");

    let mut device = Device::from(input);

    device.execute();

    device.get_output()
}

#[allow(dead_code)]
pub fn part_two() -> String {
    // found these by hand, kept changing the values of x and y
    // and tracking down the mistakes and swapping them
    let pairs = [
        ("gfm", "z32"),
        ("cdj", "z08"),
        ("qjd", "dhm"),
        ("z16", "mrb"),
    ];

    let mut wires: Vec<&str> = pairs.iter().flat_map(|pair| [pair.0, pair.1].into_iter()).collect();
    wires.sort_unstable();

    wires.iter().join(",")
}
