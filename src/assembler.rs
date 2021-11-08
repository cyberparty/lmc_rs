use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;



pub struct AsmToken<'a> 
{
    pub opcode: Option<&'a str>,
    pub operand: Option<&'a str>,
    pub label: Option<&'a str>,
    pub index: u16
}

pub struct Ops<'a>
{
    operations: [&'a str; 9]
}
impl Ops<'_>
{
    pub fn is_valid_op(self, op: &str) -> bool
    {
        for i in self.operations.iter()
        {
            if *i == op
            {
                return true;
            }
        }
        return false;
    }
}
const OPS: Ops<'static> = Ops{
    operations: ["ADD", "SUB", "STA", "LDA", "BRA", "BRP", "BRZ", "INP", "OUT"]
};


fn load_instructions(path: &str) -> Vec<u16> {
    let i_file = File::open(path).expect("Couldn't find instruction file.");
    let i_read = BufReader::new(i_file);

    let instructions: Vec<u16> = i_read
    .lines()
    .map(|i| i.unwrap().parse::<u16>().unwrap())
    .collect();
    return instructions;
}

fn gen_asmtoken_line(line: &str, index: u16) -> Result<Option<AsmToken>, &str>
{
    let comment_split: Vec<&str> = line.split("//").collect();
    let asm: Vec<&str> = comment_split[0].split_whitespace().collect();

    if asm.len() == 0
    {
        return Ok(None);
    } else if asm.len() > 3 {
        return Err(format!("Invalid syntax! Unexpected characters before line end. \nLine {}: {} <<", index, comment_split[0]));
    } 

    let mut asmtoken: AsmToken = AsmToken
    {
        opcode: None,
        operand: None,
        label: None,
        index: index
    };

    if OPS.is_valid_op(asm[0])
    {
        //op first, store
        asmtoken.opcode = Some(asm[0]);
        if asm.len() == 2
        {
            //if op is first and split size is only 2, then the 2nd part is always operand
            asmtoken.operand = Some(asm[1]);
        }
        else
        { 
            //label first
            if asm.len() == 1 
            {
                //just label, error
                return Err(format!("Invalid syntax! Expected instructions after label, found none. \nLine {}: {} <<", index, asm[0]));
            }
            if !OPS.is_valid_op(asm[1]) && asm[1] != "DAT"
            {
                //not op and not dat, error
                return Err(format!("Invalid syntax! Unknown operation. \nLine {}: {} <<", index, comment_split[0]));
            }
            //op with label. store
            asmtoken.opcode = Some(asm[1]);
            asmtoken.label = Some(asm[0]);
            
            if asm.len() == 3
            {
                //if op with label and split size is 3, 3rd part is always operand
                asmtoken.operand = Some(asm[2])
            }

        }


    }
    return Ok(Some(asmtoken));

}

//fn gen_asmtoken_vec(src: &str) -> Result<Vec<AsmToken>, &str>
//{
    
//}