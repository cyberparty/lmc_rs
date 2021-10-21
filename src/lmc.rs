use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


pub struct Instruction {
    opcode: u8,
    operand: usize
}

pub struct LMC {
    program_counter: u8,
    accumulator: u8,
    neg_flag: bool,
    mailbox: Vec<u8>,

    data_reg: u8,

    current_instruction: Instruction
}

// LMC instructions
impl LMC {

    pub fn create(self) -> LMC
    {
        LMC {
            program_counter: 0,
            accumulator: 0,
            neg_flag: false,
            mailbox: self.load_instructions("instructions"),
    
            data_reg: 0,

            current_instruction: Instruction{opcode: 0, operand: 0}
        }
    }

    pub fn load_instructions(&mut self, path: &str) -> Vec<u8>
    {
        let i_file = File::open(path).expect("Couldn't find instruction file.");
        let i_read = BufReader::new(i_file);

        let instructions: Vec<u8> = i_read
        .lines()
        .map(|i| i.unwrap().parse::<u8>().unwrap())
        .collect();

        return instructions;

    }

    fn fetch(&mut self, addr: usize)
    {
        self.data_reg = self.mailbox[addr];
    }

    fn decode(&mut self)
    {
        //i found out this division and modulus technique from tomc1998. much kudos to them.
        self.current_instruction.opcode = self.data_reg / 100;
        self.current_instruction.operand = (self.data_reg % 100) as usize;
    }

    fn execute(&mut self)
    {
        match self.current_instruction.opcode {
            1 => self.add(),
            2 => self.sub(),
            3 => self.sta(),
            5 => self.lda(),
            6 => self.bra(),
            7 => self.brz(),
            8 => self.brp(),
            9 => {
                if self.current_instruction.operand == 1 {
                    self.inp()
                }
                else if self.current_instruction.operand == 2 {
                    self.out()
                }
            }
            _ => {}

        }
    }

    //LMC spec is vague on what actually happens with regards to negative values in 
    //these cases, if a potential underflow or overflow happens, the value remains unchanged
    //and neg_flag is set to true until the next 'successful' arithmetic operation

    fn add(&mut self) {
        self.fetch(self.current_instruction.operand);
        if self.accumulator+self.data_reg < 999
        {
            self.neg_flag = true;
        } else {
            self.accumulator += self.data_reg;
            self.neg_flag = false;
        }
    }

    fn sub(&mut self) {
        self.fetch(self.current_instruction.operand);
        if self.accumulator < self.data_reg {
            self.neg_flag = true; 
        } else {
            self.accumulator -= self.data_reg;
            self.neg_flag = false;
        }
    }

    fn sta(&mut self) {
        self.mailbox[self.current_instruction.operand] = self.accumulator;
    }

    fn lda(&mut self)
    {
        self.accumulator = self.mailbox[self.current_instruction.operand];
    }

    fn bra(&mut self)
    {
        self.program_counter = self.mailbox[self.current_instruction.operand];
    }

    fn brz(&mut self) {
        if self.program_counter == 0 {
            self.program_counter = self.mailbox[self.current_instruction.operand];
        }
    }

    fn brp(&mut self) {
        if !self.neg_flag {
            self.program_counter = self.mailbox[self.current_instruction.operand];
        }
    }
    
    fn inp(&mut self) {
        let mut read_input = String::new();
        io::stdin().read_line(&mut read_input).unwrap();
        let input: u8 = read_input.trim().parse().unwrap();
        self.accumulator = input;
    }

    fn out(&self) {
        println!("{}", self.accumulator);
    }

}
