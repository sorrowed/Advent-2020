use crate::common;

struct Machine {
    pc: usize,
    acc: i64,
}

impl Machine {
    pub fn new() -> Machine {
        Machine { pc: 0, acc: 0 }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    pub fn print(&self) {
        print!("\tpc:{} acc:{}\n", self.pc, self.acc);
    }
}

#[derive(Clone)]
struct Instr {
    command: String,
    operand: i64,
}

impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.command, self.operand)
    }
}

impl Instr {
    pub fn new(source: &str) -> Option<Instr> {
        let s = source.split_ascii_whitespace().collect::<Vec<_>>();
        if s.len() >= 2 {
            Some(Instr {
                command: s[0].to_string(),
                operand: s[1].parse::<i64>().expect("Invalid operand"),
            })
        } else {
            None
        }
    }

    pub fn execute(&self, machine: &mut Machine) {
        match self.command.as_str() {
            "nop" => {
                machine.pc += 1;
            }
            "acc" => {
                machine.acc += self.operand;
                machine.pc += 1;
            }
            "jmp" => {
                let pc = machine.pc as i64 + self.operand;
                if pc >= 0 {
                    machine.pc = pc as usize;
                } else {
                    panic!("Invalid jump");
                }
            }
            _ => panic!("Unsupprted instruction"),
        }
    }

    pub fn print(&self) {
        print!("cmd:{} op:{} ", self.command, self.operand);
    }
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instr>,
    trace: Vec<(Instr, usize)>,
}

impl Program {
    fn new(source: &[&str]) -> Program {
        Program {
            instructions: source
                .iter()
                .map(|token| Instr::new(token).expect("Parse errorr"))
                .collect(),
            trace: vec![],
        }
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }
    fn is_valid(&self, machine: &Machine) -> bool {
        machine.pc < self.instructions.len()
    }

    fn execute(&mut self, machine: &mut Machine) -> bool {
        let mut result = true;

        while result && self.is_valid(machine) {
            let instr = &self.instructions[machine.pc];

            if self.trace.iter().any(|(_, pc)| pc == &machine.pc) {
                result = false
            } else {
                self.trace.push((instr.clone(), machine.pc));

                instr.execute(machine);
            }
        }

        result
    }
}

pub fn test() {
    let input = [
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    let mut machine = Machine::new();
    let mut program = Program::new(&input);
    program.execute(&mut machine);
}

pub fn part1() {
    let input = common::import("day8/input.txt");

    let mut machine = Machine::new();
    let mut program = Program::new(&input.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
    program.execute(&mut machine);

    println!("Accumulator before loop : {}", machine.acc);
}

pub fn part2() {
    let input = common::import("day8/input2.txt");

    let mut machine = Machine::new();
    let template = Program::new(&input.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

    let mut pc = 0;
    let mut program = template.clone();
    while !program.execute(&mut machine) {
        machine.reset();
        program = template.clone();
        pc += 1;

        while pc < program.len() && program.instructions[pc].command == "acc" {
            pc += 1;
        }

        //println!("Changing instruction @{}: {}", pc, program.instructions[pc]);
        match program.instructions[pc].command.as_str() {
            "nop" => program.instructions[pc].command = "jmp".to_string(),
            "jmp" => program.instructions[pc].command = "nop".to_string(),
            _ => panic!(),
        }
    }

    println!("Final accumulator value : {}", machine.acc);
}
