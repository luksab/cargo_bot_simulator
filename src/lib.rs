pub mod cargo_bot_parse {
    use std::fmt::Display;
    use std::mem;
    use std::ops::Shl;
    use std::{cmp::min, fmt::Debug};

    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(FromPrimitive)]
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
                IfColor::Always => write!(f, "a"),
                IfColor::Blue => write!(f, "b"),
                IfColor::Green => write!(f, "g"),
                IfColor::Red => write!(f, "r"),
                IfColor::Yellow => write!(f, "y"),
                IfColor::Any => write!(f, "y"),
                IfColor::None => write!(f, "n"),
                IfColor::Nop => write!(f, "N"),
            }
        }
    }

    #[derive(FromPrimitive)]
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
                OpCode::Nop => write!(f, "\""),
            }
        }
    }

    #[derive(FromPrimitive, PartialEq, Clone, Copy)]
    pub enum Box {
        None = 0,
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
                        'd' => OpCode::Down,
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
                        .fold(0u32, |last, (i, code)| last | (code as u32).shl(i * 6))
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
                        .fold(0u32, |last, (i, code)| last | (code as u32).shl(i * 6))
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

        pub fn run_all(&mut self) -> FinishState {
            let mut num_rounds = 0;
            loop {
                num_rounds += 1;
                if num_rounds == 65535 {
                    return FinishState::Limited(num_rounds);
                }
                match self.step() {
                    StepState::Normal => (),
                    StepState::Crashed => break FinishState::Crashed(num_rounds),
                    StepState::Finished => break FinishState::Finished(num_rounds),
                };
            }
        }

        fn get_top(stack: u32) -> usize {
            (6 - stack.leading_zeros() / 3) as usize
        }

        pub fn step(&mut self) -> StepState {
            let col = ((self.instructions[(self.ip.0) as usize] >> ((self.ip.1) * 6)) & 56) as u8;
            if col == self.crane as u8
                || col == IfColor::Always as u8
                || (col == IfColor::Any as u8 && self.crane != Box::None)
            {
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
                        // TODO: this needs to be made more boosted!
                        let temp = ((self.data[self.dp as usize] >> (top * 3)) & 0b111) as u8;
                        self.data[self.dp as usize] &= u32::MAX ^ 0b111 << (top * 3);
                        self.data[self.dp as usize] |= (self.crane as u32) << (top * 3);
                        self.crane = FromPrimitive::from_u8(temp).unwrap();
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
            }
            // test if finished by comparing each element of data to finish_state
            if self.data == self.finish_state {
                StepState::Finished
            } else if self.dp < self.data.len() as u8 {
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

        pub fn print_data(&mut self) -> String {
            let mut data: String = Default::default();
            for y in (0..6).rev() {
                for x in 0..self.data.len() {
                    // data = format!("{}|{}", data, self.data[x][y]);
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
