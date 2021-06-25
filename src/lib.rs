pub mod cargo_bot_parse {
    use std::fmt::Display;
    use std::mem;
    use std::{cmp::min, fmt::Debug};

    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(FromPrimitive)]
    pub enum IfColor {
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
                IfColor::Always => write!(f, "Always"),
                IfColor::Blue => write!(f, "Blue"),
                IfColor::Green => write!(f, "Green"),
                IfColor::Red => write!(f, "Red"),
                IfColor::Yellow => write!(f, "Yellow"),
                IfColor::Any => write!(f, "Any"),
                IfColor::None => write!(f, "None"),
            }
        }
    }

    #[derive(FromPrimitive)]
    pub enum OpCode {
        Right = 0,
        Left = 1,
        Down = 2,
        Goto1 = 3,
        Goto2 = 4,
        Goto3 = 5,
        Goto4 = 6,
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
    pub struct CbInterpret {
        pub data: Vec<[Box; 6]>,
        finish_state: Vec<[Box; 6]>,
        instructions: [Vec<u8>; 4],
        ip: (u8, u8), //band, position
        stack: Vec<(u8, u8)>,
        dp: u8,
        crane: Box,
    }

    // impl Debug for CbInterpret {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "ip: {:?}, dp: {}\n{}", self.ip, self.dp, self)
    //     }
    // }

    // impl Display for CbInterpret {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         let size = 20;
    //         let mut ret = String::new();
    //         ret += &format!("({},{:0>5})", self.ip.0, self.ip.1);
    //         for i in max(self.ip, size) - size..min(self.ip + size, 30_000) {
    //             match OpCode::from_u8(self.data[i as usize]) {
    //                 Some(inst) => {
    //                     if i == self.ip {
    //                         ret += &format!(
    //                             "{}{}{}",
    //                             color::Fg(color::Red),
    //                             &inst,
    //                             color::Fg(color::Reset)
    //                         );
    //                     } else {
    //                         ret += &format!("{}", &inst);
    //                     }
    //                 }
    //                 None => {
    //                     ret += "";
    //                 }
    //             }
    //         }
    //         //ret += &format!("\n{:0>6}", self.dp);
    //         ret += &format!("\n");
    //         for i in max(self.dp, size) - size..min(self.dp + size, 30_000) {
    //             if i == self.dp {
    //                 ret += &format!("{}", color::Fg(color::Blue));
    //             }
    //             if false {
    //                 match <OpCode as FromPrimitive>::from_u8(self.data[i as usize]) {
    //                     Some(inst) => {
    //                         ret += &format!("{}", &inst);
    //                     }
    //                     None => {
    //                         ret += &format!("{}", self.data[i as usize]);
    //                     }
    //                 }
    //             } else {
    //                 ret += &format!("{:0>3},", self.data[i as usize]);
    //             }
    //             if i == self.dp {
    //                 ret += &format!("{}", color::Fg(color::Reset));
    //             }
    //         }
    //         write!(f, "{}", ret)
    //     }
    // }

    impl CbInterpret {
        pub fn new<S1, S2, S3>(
            instructions_enc: S1,
            data_enc: S2,
            finish_data_enc: S3,
        ) -> Result<CbInterpret, String>
        where
            S1: Into<String>,
            S2: Into<String>,
            S3: Into<String>,
        {
            let mut instructions: [Vec<u8>; 4] = Default::default();
            let mut ip = (0, 0);
            for rows in instructions_enc.into().split(',') {
                println!("parsing {}", rows);
                for chars in rows.chars().collect::<Vec<char>>().chunks(2) {
                    instructions[ip.0].push(
                        FromPrimitive::from_u8(
                            (match chars[1] {
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
                                } as u8),
                        )
                        .unwrap(),
                    );
                }
                ip.0 += 1;
            }
            let mut data = Vec::new();
            for stacks in data_enc.into().split(',') {
                let stack: Vec<Box> = stacks
                    .chars()
                    .map(|char| match char {
                        'r' => Box::Red,
                        'g' => Box::Green,
                        'b' => Box::Blue,
                        'y' => Box::Yellow,
                        'n' => Box::None,
                        _ => Box::None,
                    })
                    .collect();
                let mut stack_b = [Box::default(); 6];
                //stack_b[..].clone_from_slice(&stack[..]);
                for i in 0..min(6, stack.len()) {
                    stack_b[i] = stack[i];
                }
                data.push(stack_b);
            }
            let mut finish_state = Vec::new();
            for stacks in finish_data_enc.into().split(',') {
                let stack: Vec<Box> = stacks
                    .chars()
                    .map(|char| match char {
                        'r' => Box::Red,
                        'g' => Box::Green,
                        'b' => Box::Blue,
                        'y' => Box::Yellow,
                        'n' => Box::None,
                        _ => Box::None,
                    })
                    .collect();
                let mut stack_b = [Box::default(); 6];
                //stack_b[..].clone_from_slice(&stack[..]);
                for i in 0..min(6, stack.len()) {
                    stack_b[i] = stack[i];
                }
                finish_state.push(stack_b);
            }
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

        pub fn run_all(&mut self) {
            while self.step() {}
        }

        fn get_top(stack: &[Box; 6]) -> usize {
            for i in 0..6 {
                let i = 5 - i;
                if stack[i] != Box::None {
                    println!("Box found at: {}", i);
                    return i;
                }
            }
            0
        }

        pub fn step(&mut self) -> bool {
            let col = self.instructions[self.ip.0 as usize][self.ip.1 as usize] & 56;
            if col == self.crane as u8
                || col == IfColor::Always as u8
                || (col == IfColor::Any as u8 && self.crane != Box::None)
            {
                match FromPrimitive::from_u8(
                    self.instructions[self.ip.0 as usize][self.ip.1 as usize] & 7,
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
                        println!("going down");
                        self.ip.1 += 1;
                        let top = CbInterpret::get_top(&self.data[self.dp as usize]);
                        mem::swap(&mut self.crane, &mut self.data[self.dp as usize][top]);
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
                }
            }
            // test if finished by comparing each element of data to finish_state
            if self
                .data
                .iter_mut()
                .zip(self.finish_state.iter_mut())
                .all(|(d, f)| d.iter_mut().zip(f).all(|(d, f)| d == f))
            {
                false
            } else if self.ip.1 >= self.instructions[self.ip.0 as usize].len() as u8 {
                if !self.stack.is_empty() {
                    self.ip = self.stack.pop().unwrap();
                    true
                } else {
                    false
                }
            } else {
                self.dp < self.data.len() as u8
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
                    data = format!("{}|{}", data, self.data[x][y]);
                }
                data = format!("{}|\n", data);
            }
            data
        }
    }

    impl Iterator for CbInterpret {
        type Item = bool;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            match self.step() {
                true => Some(true),
                false => None,
            }
        }
    }
}

pub use cargo_bot_parse::CbInterpret;
