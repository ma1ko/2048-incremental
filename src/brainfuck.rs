fn main() {
    let program = "++>+++++[->+<]".to_string();
    let hello = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let x = include_str!("bitwidth.b");
    let mut commands: Vec<Command> = x.chars().map(From::from).filter(|x| *x != Noop).collect();

    let mut refs = Vec::new();
    (0..commands.len()).for_each(|i| {
        let cmd = &commands[i];
        if *cmd == JumpFwd(0) {
            refs.push(i);
        }
        if *cmd == JumpBwd(0) {
            let fwd = refs.pop().unwrap();
            commands[i] = JumpBwd(fwd);
            commands[fwd] = JumpFwd(i);
        }
    });
    // println!("{:?}", commands);

    let mut int = Interpreter::new(commands);
    int.interpret();
}

struct Interpreter {
    data: [u8; 1000],
    ptr: usize,
    program: Vec<Command>,
    cmd: usize,
}
impl Interpreter {
    fn new(program: Vec<Command>) -> Self {
        Interpreter {
            program,
            ptr: 0,
            cmd: 0,
            data: [0; 1000],
        }
    }
    fn interpret(&mut self) {
        loop {
            if self.cmd >= self.program.len() { return}
            let cmd = self.program[self.cmd];
            // println!("Running {:?}", cmd);

            let data = &mut self.data[self.ptr];
            match cmd {
                Inc => self.ptr += 1,
                Dec => self.ptr -= 1,
                Plus => *data = data.wrapping_add(1),
                Minus => *data = data.wrapping_sub(1),
                Input => std::io::stdin()
                    .read_exact(&mut self.data[self.ptr..self.ptr + 1])
                    .unwrap(),
                Output => print!("{}", self.data[self.ptr] as char),
                JumpFwd(ptr) => if self.data[self.ptr] == 0 { self.cmd = ptr},
                JumpBwd(ptr) => if self.data[self.ptr] != 0 { self.cmd = ptr},
                Noop => panic!("Shouldn't be here anymore")
            }
            self.cmd += 1;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Command {
    Inc,
    Dec,
    Plus,
    Minus,
    Input,
    Output,
    JumpFwd(usize),
    JumpBwd(usize),
    Noop,
}
use std::io::Read;

use Command::*;
impl From<char> for Command {
    fn from(c: char) -> Self {
        match c {
            '>' => Inc,
            '<' => Dec,
            '+' => Plus,
            '-' => Minus,
            '.' => Output,
            ',' => Input,
            '[' => JumpFwd(0),
            ']' => JumpBwd(0),
            _ => Noop
        }
    }
}
