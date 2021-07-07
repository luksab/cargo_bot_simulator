pub mod cargo_bot_parse {
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::io::{stdout, Write};
    use std::ops::Shl;

    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(FromPrimitive, PartialEq)]
    pub enum IfColor {
        Nop = 0,
        Always = 8,
        Blue = 16,
        Green = 24,
        Red = 32,
        Yellow = 40,
        Any = 48,
        None = 56,
    }

    impl Display for IfColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IfColor::Always => write!(f, "q"),
                IfColor::Blue => write!(f, "b"),
                IfColor::Green => write!(f, "g"),
                IfColor::Red => write!(f, "r"),
                IfColor::Yellow => write!(f, "y"),
                IfColor::Any => write!(f, "a"),
                IfColor::None => write!(f, "n"),
                IfColor::Nop => write!(f, "q"),
            }
        }
    }

    impl PartialEq<Box> for IfColor {
        fn eq(&self, other: &Box) -> bool {
            match self {
                IfColor::Nop => true,
                IfColor::Always => true,
                IfColor::Blue => other == &Box::Blue,
                IfColor::Green => other == &Box::Green,
                IfColor::Red => other == &Box::Red,
                IfColor::Yellow => other == &Box::Yellow,
                IfColor::Any => other != &Box::None,
                IfColor::None => other == &Box::None,
            }
        }
    }

    #[derive(FromPrimitive, PartialEq)]
    pub enum OpCode {
        Nop = 0,
        Right = 1,
        Left = 2,
        Down = 3,
        Goto1 = 4,
        Goto2 = 5,
        Goto3 = 6,
        Goto4 = 7,
    }

    impl Display for OpCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                OpCode::Right => write!(f, ">"),
                OpCode::Left => write!(f, "<"),
                OpCode::Down => write!(f, "."),
                OpCode::Goto1 => write!(f, "1"),
                OpCode::Goto2 => write!(f, "2"),
                OpCode::Goto3 => write!(f, "3"),
                OpCode::Goto4 => write!(f, "4"),
                OpCode::Nop => write!(f, " "),
            }
        }
    }

    #[derive(FromPrimitive, PartialEq, Clone, Copy)]
    pub enum Box {
        None = 0, // important to be 0 for initialization
        Blue = 1,
        Green = 2,
        Red = 3,
        Yellow = 4,
    }

    impl Default for Box {
        fn default() -> Self {
            Box::None
        }
    }

    impl Debug for Box {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Box::None => write!(f, "n"),
                Box::Blue => write!(f, "b"),
                Box::Green => write!(f, "g"),
                Box::Red => write!(f, "r"),
                Box::Yellow => write!(f, "y"),
            }
        }
    }

    impl Display for Box {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Box::None => write!(f, " "),
                Box::Blue => write!(f, "b"),
                Box::Green => write!(f, "g"),
                Box::Red => write!(f, "r"),
                Box::Yellow => write!(f, "y"),
            }
        }
    }

    #[derive(Debug)]
    pub struct CbInterpret<const WIDTH: usize> {
        pub data: [u32; WIDTH],
        finish_state: [u32; WIDTH],
        instructions: [u64; 4], //Encoded as 6 bits per Instruction
        ip: (u8, u8),           //band, position
        stack: Vec<(u8, u8)>,
        dp: u8,
        crane: Box,
    }

    #[derive(PartialEq)]
    pub enum StepState {
        Normal,
        Crashed,
        Finished,
    }

    #[derive(PartialEq)]
    pub enum FinishState {
        Crashed(u16),
        Finished(u16),
        Limited(u16),
    }

    impl<const WIDTH: usize> CbInterpret<WIDTH> {
        pub fn new<S1, S2, S3>(
            instructions_enc: S1,
            data_enc: S2,
            finish_data_enc: S3,
        ) -> Result<CbInterpret<WIDTH>, String>
        where
            S1: Into<String>,
            S2: Into<String>,
            S3: Into<String>,
        {
            let mut instructions: [u64; 4] = Default::default();
            let mut ip = (0, 0);
            for (op, rows) in instructions_enc.into().split(',').enumerate() {
                // println!("parsing {}", rows);
                for chars in rows.chars().collect::<Vec<char>>().chunks(2) {
                    instructions[op] |= (((match chars[1] {
                        '>' => OpCode::Right,
                        '<' => OpCode::Left,
                        '.' => OpCode::Down,
                        '1' => OpCode::Goto1,
                        '2' => OpCode::Goto2,
                        '3' => OpCode::Goto3,
                        '4' => OpCode::Goto4,
                        _ => return Err("unknown opCode".to_string()),
                    } as u8)
                        | (match chars[0] {
                            'q' => IfColor::Always,
                            'a' => IfColor::Any,
                            'b' => IfColor::Blue,
                            'g' => IfColor::Green,
                            'y' => IfColor::Yellow,
                            'r' => IfColor::Red,
                            'n' => IfColor::None,
                            _ => return Err("unknown color".to_string()),
                        } as u8)) as u64)
                        .shl(((ip.1) * 6) as u64); // << doesn't work for some reason...

                    ip.1 += 1;
                }
            }
            let mut data = [0; WIDTH];
            let d: Vec<_> = data_enc
                .into()
                .split(',')
                .map(|stack| {
                    stack
                        .chars()
                        .map(|char| match char {
                            'r' => Box::Red,
                            'g' => Box::Green,
                            'b' => Box::Blue,
                            'y' => Box::Yellow,
                            'n' => Box::None,
                            _ => Box::None,
                        } as u8)
                        .enumerate()
                        .fold(0u32, |last, (i, code)| last | (code as u32).shl(i * 3))
                })
                .collect();
            data[..WIDTH].clone_from_slice(&d[..WIDTH]);

            let mut finish_state = [0; WIDTH];
            let d: Vec<_> = finish_data_enc
                .into()
                .split(',')
                .map(|stack| {
                    stack
                        .chars()
                        .map(|char| match char {
                            'r' => Box::Red,
                            'g' => Box::Green,
                            'b' => Box::Blue,
                            'y' => Box::Yellow,
                            'n' => Box::None,
                            _ => Box::None,
                        } as u8)
                        .enumerate()
                        .fold(0u32, |last, (i, code)| last | (code as u32).shl(i * 3))
                })
                .collect();

            finish_state[..WIDTH].clone_from_slice(&d[..WIDTH]);

            // let mut finish_state = Vec::new();
            // for stacks in finish_data_enc.into().split(',') {
            //     let stack: Vec<Box> = stacks
            //         .chars()
            //         .map(|char| match char {
            //             'r' => Box::Red,
            //             'g' => Box::Green,
            //             'b' => Box::Blue,
            //             'y' => Box::Yellow,
            //             'n' => Box::None,
            //             _ => Box::None,
            //         })
            //         .collect();
            //     let mut stack_b = [Box::default(); 6];
            //     //stack_b[..].clone_from_slice(&stack[..]);
            //     for i in 0..min(6, stack.len()) {
            //         stack_b[i] = stack[i];
            //     }
            //     finish_state.push(stack_b);
            // }
            Ok(CbInterpret {
                data,
                finish_state,
                ip: (0, 0),
                stack: Default::default(),
                dp: 0,
                instructions,
                crane: Box::None,
            })
        }

        pub fn brute_force(&mut self) {
            // let inst = self.instructions[0];
            let data = self.data;
            println!("data: {:?}", data);
            println!("self: {:?}", self);
            let mut stdout = stdout();
            // let op_code_list = [OpCode::Down, OpCode::Left, OpCode::Right, OpCode::Goto1];
            for i in 0..u64::MAX << 4 {
                if i % 1_000_000 == 0 {
                    print!("\r{} registers", ((64 - i.leading_zeros()) / 6));
                    stdout.flush().unwrap();
                }
                self.instructions[0] = i;
                // if i == 1000 {
                //     println!("def");
                //     self.instructions[0] = inst;
                //     println!("data: {:?}", data);
                //     println!("self: {:?}", self);
                // }
                self.dp = 0;
                self.ip = (0, 0);
                self.data = data;
                self.stack = Default::default();
                self.crane = Box::None;
                match self.run_all() {
                    FinishState::Crashed(_) => {}
                    FinishState::Finished(_) => return,
                    FinishState::Limited(_) => {}
                }
            }
        }

        pub fn run_all(&mut self) -> FinishState {
            let mut num_rounds = 0;
            loop {
                num_rounds += 1;
                if num_rounds == 100 {
                    return FinishState::Limited(num_rounds);
                }
                match self.step() {
                    StepState::Normal => (),
                    StepState::Crashed => break FinishState::Crashed(num_rounds),
                    StepState::Finished => break FinishState::Finished(num_rounds),
                };
            }
        }

        fn get_top(stack: u32) -> i8 {
            // println!("stack: {:0b}", stack);
            // println!("leading zeros: {}", stack.leading_zeros());
            // let top = (5 - ((stack.leading_zeros() - 14) / 3)) as usize;
            // match 5u32.checked_sub((stack.leading_zeros() - 14) / 3) {
            //     Some(i) => Some(i as usize),
            //     None => None,
            // }
            5u32.wrapping_sub((stack.leading_zeros() - 14) / 3) as i8
        }

        pub fn step(&mut self) -> StepState {
            let col = IfColor::from_u8(
                ((self.instructions[(self.ip.0) as usize] >> ((self.ip.1) * 6)) & 56) as u8,
            )
            .unwrap();
            if col == self.crane || (col == IfColor::Any && self.crane != Box::None) {
                match FromPrimitive::from_u8(
                    ((self.instructions[(self.ip.0) as usize] >> ((self.ip.1) * 6)) & 7) as u8,
                )
                .expect("unknown Instruction")
                {
                    OpCode::Right => {
                        self.ip.1 += 1;
                        self.dp += 1;
                    }
                    OpCode::Left => {
                        self.ip.1 += 1;
                        self.dp -= 1;
                    }
                    OpCode::Down => {
                        // println!("going down");
                        self.ip.1 += 1;

                        let top = CbInterpret::<WIDTH>::get_top(self.data[self.dp as usize]);
                        if self.crane == Box::None {
                            self.crane =
                                Box::from_u32((self.data[self.dp as usize] >> (top * 3)) & 0b111)
                                    .unwrap();
                            self.data[self.dp as usize] &= u32::MAX ^ (0b111 << (top * 3));
                        // TODO: make one constant?
                        } else {
                            self.data[self.dp as usize] |= (self.crane as u32) << ((top + 1) * 3);
                            self.crane = Box::None;
                            if top > 6 {
                                // TODO: check how many boxes can fit on a stack
                                return StepState::Crashed;
                            }
                        }
                    }
                    OpCode::Goto1 => {
                        self.stack.push(self.ip);
                        self.ip = (0, 0)
                    }
                    OpCode::Goto2 => {
                        self.stack.push(self.ip);
                        self.ip = (1, 0)
                    }
                    OpCode::Goto3 => {
                        self.stack.push(self.ip);
                        self.ip = (2, 0)
                    }
                    OpCode::Goto4 => {
                        self.stack.push(self.ip);
                        self.ip = (3, 0)
                    }
                    OpCode::Nop => {
                        if !self.stack.is_empty() {
                            self.ip = self.stack.pop().unwrap();
                        } else {
                            return StepState::Crashed;
                        }
                    }
                }
            } else {
                self.ip.1 += 1;
            }
            // test if finished by comparing each element of data to finish_state
            if self.data == self.finish_state {
                StepState::Finished
            } else if self.dp < WIDTH as u8 {
                StepState::Normal
            } else {
                StepState::Crashed
            }
        }

        pub fn print_crane(&self) -> String {
            let mut data: String = String::from(" ");
            for _ in 0..self.dp {
                data = format!("{}  ", data);
            }
            data = format!("{}{:?}", data, self.crane);
            data
        }
        pub fn print_inst(&self) -> String {
            let mut data: String = String::from(" ");
            for row in 0..4 {
                for i in 0..8 {
                    let instr = (self.instructions[row] >> (i * 6)) & 0b111111;
                    let color = IfColor::from_u64(instr & 56).unwrap();
                    let op_code = OpCode::from_u64(instr & 7).unwrap();
                    if op_code == OpCode::Nop {
                        break;
                    }
                    data = format!("{}{}{}", data, color, op_code);
                }
                data = format!("{}, ", data);
            }
            data
        }
        pub fn print_data(&mut self) -> String {
            let mut data: String = Default::default();
            for y in (0..6).rev() {
                for x in 0..self.data.len() {
                    data = format!(
                        "{}|{}",
                        data,
                        Box::from_u32(self.data[x] >> (y * 3) & 0b111).unwrap()
                    );
                }
                data = format!("{}|\n", data);
            }
            data
        }
    }

    impl<const WIDTH: usize> Iterator for CbInterpret<WIDTH> {
        type Item = bool;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self.step() {
                StepState::Normal => Some(true),
                _ => None,
            }
        }
    }
}

pub use cargo_bot_parse::CbInterpret;
pub use cargo_bot_parse::FinishState;
pub use cargo_bot_parse::StepState;
