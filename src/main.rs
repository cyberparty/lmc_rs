mod lmc;
mod assembler;

fn main() {
    println!("LMC");
    let mut lmc: lmc::LMC = lmc::LMC::create();
    let instructions: Vec<u16> = assembler::assemble("instructions.txt").unwrap();
    lmc.load_instructions(instructions);
    loop {
        if !lmc.cycle() {
          break;
        }
    }
}
