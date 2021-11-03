mod lmc;

fn main() {
    println!("LMC");
    let mut lmc: lmc::LMC = lmc::LMC::create();
    loop 
    {
        if !lmc.cycle()
        {
            break;
        }
    }
}
