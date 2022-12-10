use std::collections::VecDeque;

pub fn solve() {
    let input = PART_1;

    // A program is a sequence of instructions for the machine to execute
    let program = parse_instructions(input);
    let mut machine = VirtualMachine::new(program);

    // Sample the register value at specific intervals. As required, start
    // at cycle 20 then sample every 40 cycles (i.e. at 60, 100, etc.)
    //
    // This could be done with `%` but I found it easier to understand by doing
    // this first.
    let mut next_sample = 20;

    // At sampling points, read the register and calculate a signal strength.
    // Add that value to the overall signal strength: this value.
    let mut signal_strength = 0;

    while machine.is_executing() {
        // Cycle the machine before reading the register
        machine.cycle();

        // The cycle count and number in the register are used to calculate the
        // signal score
        let cycle = machine.cycles as isize;
        let number = machine.read_register();

        // Sampling time!
        if cycle == next_sample {
            let local_signal_strength = cycle * number;
            signal_strength += local_signal_strength;
            next_sample += 40;
        }
    }

    println!("total signal strength: {}", signal_strength)
}

fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    let mut instructions = VecDeque::new();

    for line in input.split('\n') {
        instructions.push_back(Instruction::from(line));
    }

    instructions
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        // The noop instruction is the simplest. Parse it first without bother.
        if s.starts_with("noop") {
            return Instruction::Noop;
        }

        // Add instructions always start with `addx` followed by a space
        // followed by the value (operand). Split at that index and ignore the
        // first portion.
        let (_addx, number) = s.split_at(5);

        let number = number
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("Unable to parse isize from {}", number));

        Instruction::Addx(number)
    }
}

#[derive(Debug)]
struct VirtualMachine {
    /// The program is a sequence of instructions that will be executed
    /// sequentially.
    program: VecDeque<Instruction>,

    /// In-flight instructions are currently executing. Like in real computers,
    /// this one has instructions that vary in how long they take to execute.
    in_flight: Option<Instruction>,

    /// The single register used in this VM. It is initially `1`.
    register: isize,

    /// Stores how many cycles this VM has executed. It is initially `0` and
    /// increases by one every time the CPU cycles (i.e. `cycle()` is called).
    cycles: usize,
}

impl VirtualMachine {
    fn new(program: VecDeque<Instruction>) -> Self {
        let in_flight = Some(Instruction::Noop);

        VirtualMachine {
            program,
            in_flight,
            register: 1, // Initially `1` by specification
            cycles: 0,   // Initially `0` by specification
        }
    }

    fn is_executing(&self) -> bool {
        !self.program.is_empty() || !self.in_flight.is_none()
    }

    fn cycle(&mut self) {
        self.cycles += 1;

        if self.in_flight.is_none() {
            self.schedule();
        } else {
            self.execute();
        }
    }

    /// An instruction is currently executing. In this architecture, that
    /// means an `addx` instruction was scheduled on the previous cycle.
    /// Since `addx` takes two cycles it can be completed on this cycle.
    ///
    /// Obviously this hard-coding is flimsy, but the pattern can generalize.
    fn execute(&mut self) {
        let instruction = self.in_flight.unwrap();

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(number) => self.register += number,
        }

        self.in_flight = None;
    }

    fn schedule(&mut self) {
        // No instructions are currently executing. Pull the next from the
        // program and execute or schedule it.
        let instruction = self.program.pop_front();
        let instruction = instruction.unwrap();

        // Noop instructions take a single cycle to execute and have no
        // side effects. Adding takes two cycles, so the instruction is
        // scheduled to complete on the next cycle.
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(_) => self.in_flight = Some(instruction),
        }
    }

    fn read_register(&self) -> isize {
        self.register
    }
}

const PART_1_SAMPLE_2: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

const PART_1: &str = r"noop
noop
addx 5
noop
noop
addx 6
addx 4
addx -4
addx 4
addx -6
addx 11
addx -1
addx 2
addx 4
addx 3
noop
addx 2
addx -30
addx 2
addx 33
noop
addx -37
noop
noop
noop
addx 3
addx 2
addx 5
addx 20
addx 7
addx -24
addx 2
noop
addx 7
addx -2
addx -6
addx 13
addx 3
addx -2
addx 2
noop
addx -5
addx 10
addx 5
addx -39
addx 1
addx 5
noop
addx 3
noop
addx -5
addx 10
addx -2
addx 2
noop
noop
addx 7
noop
noop
noop
noop
addx 3
noop
addx 3
addx 2
addx 8
addx -1
addx -20
addx 21
addx -38
addx 5
addx 2
noop
noop
noop
addx 8
noop
noop
addx -2
addx 2
addx -7
addx 14
addx 5
noop
noop
noop
addx -16
addx 17
addx 2
addx -12
addx 19
noop
noop
addx -37
noop
noop
noop
addx 3
addx 2
addx 2
addx 5
addx 20
addx -19
addx 2
noop
noop
noop
addx 5
addx 19
addx -12
addx 3
addx -2
addx 2
addx -18
addx 25
addx -14
addx -22
addx 1
noop
noop
noop
addx 3
addx 5
addx -4
addx 7
addx 4
noop
addx 1
noop
noop
addx 2
addx -6
addx 15
addx -1
addx 4
noop
noop
addx 1
addx 4
addx -33
noop
addx 21
noop";
