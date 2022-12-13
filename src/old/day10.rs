use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

pub fn solve() {
    solve_part_two(PUZZLE);
}

fn solve_part_one(input: &str) -> isize {
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
        let cycle = machine.get_ticks() as isize;
        let number = machine.read_register();

        // Sampling time!
        if cycle == next_sample {
            let local_signal_strength = cycle * number;
            signal_strength += local_signal_strength;
            next_sample += 40;
        }
    }

    signal_strength
}

fn solve_part_two(input: &str) {
    let program = parse_instructions(input);
    let machine = VirtualMachine::new(program);

    let mut screen = Screen::new(machine);
    screen.refresh();

    println!("{}", screen);
}

/// The problem's input is well formatted. Every line contains one instruction.
fn parse_instructions(input: &str) -> VecDeque<Instruction> {
    let mut instructions = VecDeque::new();

    for line in input.split('\n') {
        instructions.push_back(Instruction::from(line));
    }

    instructions
}

/// This machine has a myriad of options: add with one operand or do nothing.
///
/// NB this is implicitly coupled to the machine's implementation of scheduling.
/// A `Noop` takes a single CPU cycle to complete, but `Addx` takes two. Neither
/// is captured here.
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

/// A virtual machine executes a sequence of `Instruction`s (i.e. a program). It
/// maintains the value of a single register. Since some instructions take
/// longer to execute, it separates the program instructions from those
/// in-flight.
#[derive(Debug)]
struct VirtualMachine {
    /// The program is a sequence of instructions that will be executed
    /// sequentially.
    program: VecDeque<Instruction>,

    /// An in-flight instruction is currently executing
    in_flight: Option<Instruction>,

    /// The single register used in this VM. It is initially `1`.
    register: isize,

    /// Stores how many cycles this VM has executed. It is initially `0` and
    /// increases by one every time the CPU cycles (i.e. `cycle()` is called).
    ticks: usize,
}

impl VirtualMachine {
    fn new(program: VecDeque<Instruction>) -> Self {
        let in_flight = None;

        // Start at tick one then increment after completing a cycle.
        //
        // TODO This problem begs for property-directed testing!
        let ticks = 1;

        VirtualMachine {
            program,
            in_flight,
            ticks,
            register: 1, // Initially `1` by specification
        }
    }

    /// Returns `false` when the program has finished executing (i.e. all
    /// instructions) have completed.
    fn is_executing(&self) -> bool {
        !self.program.is_empty() || !self.in_flight.is_none()
    }

    /// Cycles the CPU by executing the next instruction. This will increase
    /// the cycle counter and possibly the register (depending on the
    /// instruction).
    ///
    /// NB the cycle counter (i.e. `ticks`) is incremented only after the cycle
    /// is complete.
    fn cycle(&mut self) {
        if self.in_flight.is_none() {
            self.schedule();
        } else {
            self.execute();
        }

        self.ticks += 1;
    }

    /// An instruction is currently executing. In this architecture, that means
    /// an `addx` instruction was scheduled on the previous cycle. Since `addx`
    /// takes two cycles it can be completed on this cycle.
    ///
    /// If a `Noop` was scheduled, ignore it. The VM sets a `Noop` instruction
    /// as in-flight when starting so `Addx` doesn't execute too fast.
    ///
    /// This pattern would require refactoring if more instructions with varying
    /// execution lengths are added.
    fn execute(&mut self) {
        let instruction = self.in_flight.unwrap();

        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(number) => self.register += number,
        }

        self.in_flight = None;
    }

    /// No instructions are currently executing. Pull the next one from the
    /// program and execute or schedule it depending on the type.
    fn schedule(&mut self) {
        let instruction = self.program.pop_front();
        let instruction = instruction.unwrap();

        // `Noop` instructions take a single cycle to execute and have no side
        // effects. Adding takes two cycles, so the instruction is scheduled to
        // complete on the next cycle.
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(_) => self.in_flight = Some(instruction),
        }
    }

    /// Return the value currently stored in the register. When instructions
    /// that modify this value (e.g. `Addx`) execute, the value is only updated
    /// after the instruction completes, at the end of the CPU cycle.
    fn read_register(&self) -> isize {
        self.register
    }

    /// Returns the number of cycles performed by the CPU
    fn get_ticks(&self) -> usize {
        self.ticks
    }
}

/// A single pixel on the screen. It can only be lit or dark.
#[derive(Debug, Clone, Copy)]
enum Pixel {
    Lit,
    Dark,
}

impl Into<char> for Pixel {
    /// Transform the pixel into the character it should display on the screen
    fn into(self) -> char {
        match self {
            Pixel::Lit => '#',
            Pixel::Dark => '.',
        }
    }
}

/// Defined by the specification
const SCREEN_WIDTH: usize = 40;

/// Defined by the specification
const SCREEN_HEIGHT: usize = 6;

/// A screen is a visual output controlled by an underlying machine. In this
/// case, the underlying machine executes a program which instructs the screen
/// when and where to light pixels.
struct Screen {
    machine: VirtualMachine,
    sprite_middle: isize,
    pixels: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    /// Creates a new screen controlled by the given VM and its program
    fn new(machine: VirtualMachine) -> Self {
        // Initially, every pixel is dark
        let pixels = [Pixel::Dark; SCREEN_WIDTH * SCREEN_HEIGHT];

        // A sprite is three pixels wide. Its middle is the easiest way to track
        // its position. Initially, the middle is at index `1` so one pixel to
        // the left (index `0`) and one to the right (index `2`) are also
        // displayed.
        let middle = 1;

        Screen {
            machine,
            pixels,
            sprite_middle: middle,
        }
    }

    /// Refresh the screen so it is ready to be displayed. Underneath, this
    /// cycles the VM to determine if a pixel should be lit or not.
    fn refresh(&mut self) {
        while self.machine.is_executing() {
            self.light();
            self.machine.cycle();
            self.sprite_middle = self.machine.read_register();
        }
    }

    /// Lights a pixel if the VM signals for it
    fn light(&mut self) {
        // The screen updates pixels according to the program executing in the
        // underlying VM. It cycles the VM 240 times -- once for each pixel on
        // the screen. At each cycle, the index for the pixel is the machine's
        // tick (or cycle count). Since it's an index, subtract one.
        let screen_index = self.machine.get_ticks() - 1;
        let middle = self.sprite_middle;

        if screen_index >= SCREEN_HEIGHT * SCREEN_WIDTH {
            // TODO this is required for the sample but not the puzzle input
            return;
        }

        // The screen index maps into a flat array, but the screen is vertical.
        // The row index is found using the screen width.
        let row_index = screen_index % SCREEN_WIDTH;
        let row_index = row_index as isize;

        // The sprite is three pixels wide and tracked by its center position.
        // Light the pixel when the underlying program's register value (given
        // `screen_index` and `row_index`) aligns with the sprite's position on
        // the row.
        let should_light =
            row_index == middle || row_index == middle - 1 || row_index == middle + 1;

        let index = screen_index as usize;

        if should_light {
            self.pixels[index] = Pixel::Lit;
        }
    }
}

impl Display for Screen {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut column = 0;

        for pixel in self.pixels {
            if column == 40 {
                writeln!(formatter).unwrap();
                column = 0;
            }

            let pixel: char = pixel.into();
            write!(formatter, "{}", pixel).unwrap();

            column += 1;
        }

        writeln!(formatter)
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::day10::{PUZZLE, SAMPLE};

    use super::{solve_part_one, Instruction, VirtualMachine};

    #[test]
    fn start_with_noop() {
        let mut program = VecDeque::new();
        program.push_back(Instruction::Noop);
        program.push_back(Instruction::Addx(3));
        program.push_back(Instruction::Addx(-5));

        let mut vm = VirtualMachine::new(program);

        assert_eq!(vm.get_ticks(), 1, "Wrong tick before starting");
        assert_eq!(
            vm.read_register(),
            1,
            "Wrong register value before starting"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 2, "Wrong tick after noop");
        assert_eq!(vm.read_register(), 1, "Wrong register value after noop");

        vm.cycle();

        assert_eq!(vm.get_ticks(), 3, "Wrong tick after starting `Addx 3`");
        assert_eq!(
            vm.read_register(),
            1,
            "Wrong register value after starting `Addx 3`"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 4, "Wrong tick after finishing `Addx 3`");
        assert_eq!(
            vm.read_register(),
            4,
            "Wrong register value after finishing `Addx 3`"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 5, "Wrong tick after starting `Addx -5`");
        assert_eq!(
            vm.read_register(),
            4,
            "Wrong register value after starting `Addx -5`"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 6, "Wrong tick after finishing `Addx -5`");
        assert_eq!(
            vm.read_register(),
            -1,
            "Wrong register value after finishing `Addx -5`"
        );
    }

    #[test]
    fn start_with_add() {
        let mut program = VecDeque::new();
        program.push_back(Instruction::Addx(15));
        program.push_back(Instruction::Addx(-11));
        program.push_back(Instruction::Addx(6));

        let mut vm = VirtualMachine::new(program);

        assert_eq!(vm.get_ticks(), 1, "Wrong tick before starting");
        assert_eq!(
            vm.read_register(),
            1,
            "Wrong register value before starting"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 2, "Wrong tick after starting `Addx 15`");
        assert_eq!(
            vm.read_register(),
            1,
            "Wrong register value after starting `Addx 15`"
        );

        vm.cycle();

        assert_eq!(vm.get_ticks(), 3, "Wrong tick after finishing `Addx 15`");
        assert_eq!(
            vm.read_register(),
            16,
            "Wrong register value after finishing `Addx 15`"
        );
    }

    #[test]
    fn part_one_sample() {
        let signal_strength = solve_part_one(SAMPLE);
        assert_eq!(signal_strength, 13140);
    }

    #[test]
    fn part_one_contest() {
        let signal_strength = solve_part_one(PUZZLE);
        assert_eq!(signal_strength, 12980);
    }
}

const SAMPLE: &str = r"addx 15
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

const PUZZLE: &str = r"noop
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
