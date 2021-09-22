use std::io;

pub struct LMC {
    program_counter: u8,
    accumulator: u8,
    mailbox: [u8; 99],
}

// LMC instructions
impl LMC {
    fn add(&mut self, addr: usize) {
        self.accumulator += self.mailbox[addr];
    }

    fn sub(&mut self, addr: usize) {
        self.accumulator -= self.mailbox[addr];
    }

    fn sta(&mut self, addr: usize) {
        self.mailbox[addr] = self.accumulator;
    }

    fn lda(&mut self, addr: usize)
    {
        self.accumulator = self.mailbox[addr];
    }

    fn bra(&mut self, addr: usize)
    {
        self.program_counter = self.mailbox[addr];
    }

    fn brz(&mut self, addr: usize) {
        if self.program_counter == 0 {
            self.program_counter = self.mailbox[addr];
        }
    }

    fn brp(&mut self, addr: usize) {
        if self.program_counter >= 0 { // todo: remove redundant check and replace common functionality with macro?
            self.program_counter = self.mailbox[addr];
        }
    }

    fn inp(&mut self) {
        let mut read_input = String::new();
        io::stdin().read_line(&mut read_input).unwrap();
        let input: u8 = read_input.trim().parse().expect("Error: Not an integer!")
        self.accumulator = input;
    }

    fn out(&self) {
        println!("{}", self.accumulator);
    }

    //likely not necessary in future; essentially just a shortcut to STA in next free memory address
    fn dat(&self, addr: u8)
    {
        let mut empty_addr = self.mailbox.iter().position(|&x| x == 0).unwrap();
        self.mailbox[empty_addr] = addr;
    }

}
