use std::fs::File;
use std::io;
use std::io::Read;
use std::collections::HashMap;

pub struct AsmToken<'a> 
{
    pub opcode: Option<&'a str>,
    pub operand: Option<&'a str>,
    pub label: Option<&'a str>,
    pub index: u16
}

pub struct Ops<'a>
{
    operations: [&'a str; 10],
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
    operations: ["ADD", "SUB", "STA", "LDA", "BRA", "BRP", "BRZ", "INP", "OUT", "HLT"],
};


fn load_instructions(path: &str) -> Result<String, io::Error> {
    let mut r_buf = String::new();
    File::open(path)?.read_to_string(&mut r_buf)?;
    Ok(r_buf)
}

fn gen_asmtoken_line(line: &str, index: u16) -> Result<Option<AsmToken>, &str>
{
    let comment_split: Vec<&str> = line.split("//").collect();
    let asm: Vec<&str> = comment_split[0].split_whitespace().collect();

    if asm.len() == 0
    {
        return Ok(None);
    } else if asm.len() > 3 {
        return Err("Invalid syntax! Unexpected characters before line end.");
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
    }
    else
    { 
        //label first
        if asm.len() == 1 
        {
            //just label, error
            return Err("Invalid syntax! Expected instructions after label, found none.");
        }
    
        if !(OPS.is_valid_op(asm[1])) && asm[1] != "DAT"
        {
            //not op and not dat, error
            return Err("Invalid syntax! Unknown operation.");
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
    
    return Ok(Some(asmtoken));

}

fn gen_asmtoken_vec(src: &str) -> Result<Vec<AsmToken>, &str>
{
    let mut asm_token_vec: Vec<AsmToken> = Vec::new();
    let mut line_asm: Result<Option<AsmToken>, &str>;
    let mut index: u16 = 0;
    for l in src.lines()
    {
        line_asm = gen_asmtoken_line(l, index);
        match line_asm
        {
            Err(e) => { println!("{}", e) }
            Ok(r) =>
            {
                if r.is_some()
                {
                    asm_token_vec.push(r.unwrap());
                }
            }
        }
        index += 1;
    }
    return Ok(asm_token_vec);
}

fn gen_label_map(token_vec: &Vec<AsmToken>) -> HashMap<String, u16>
{
    let mut label_map: HashMap<String, u16> = HashMap::new();
    for asm_token in token_vec
    {
        if asm_token.label.is_some()
        {
            label_map.entry(asm_token.label.unwrap().to_string()).or_insert(asm_token.index);
        }
    }
    return label_map;

}
pub fn assemble(filename: &str) -> Result<Vec<u16>, &str>
{

    let instructions = match load_instructions(filename)
    {
        Ok(src) => src,
        Err(e) => panic!("Error in reading file. Full error: \n{}", e)
    };

    let asmtoken_vec: Vec<AsmToken> = gen_asmtoken_vec(instructions.as_str()).unwrap();
    let mut label_map = gen_label_map(&asmtoken_vec);
    let mut machine_code: Vec<u16> = Vec::new();
    let mut current_opcode: Option<u16>;
    let mut current_operand: Option<u16>;
    for asmtoken in asmtoken_vec
    {
        current_operand = None;
        if asmtoken.opcode.is_some()
        {
            current_opcode = match asmtoken.opcode.unwrap()
            {
                "ADD" => Some(100),
                "SUB" => Some(200),
                "STA" => Some(300),
                "LDA" => Some(500),
                "BRA" => Some(600),
                "BRP" => Some(700),
                "BRZ" => Some(800),
                "INP" => 
                {
                    current_operand = Some(1);
                    Some(900)
                },
                "OUT" => 
                {
                    current_operand = Some(2);
                    Some(900)
                },
                "DAT" =>
                {
                    if asmtoken.operand.is_some()
                    {
                        let temp_operand = asmtoken.operand.unwrap().parse::<u16>();
                        if temp_operand.is_ok()
                        {
                            current_operand = Some(temp_operand.unwrap())
                        }
                    }
                    None
                },
                "HLT" => 
                {
                    current_operand = Some(0);
                    Some(000)
                }
                _ => return Err("shidded and farded")
            };

            if current_operand.is_none()
            {
                if asmtoken.operand.is_none()
                {
                    return Err("No operand supplied");
                }
                
                for (label, index) in label_map.iter_mut()
                {
                    if label.as_str() == asmtoken.operand.unwrap()
                    {
                        current_operand = Some(*index);
                    }
                }
                if current_operand.is_none()
                {
                    return Err("Label not defined.")
                }
            }
            let res = current_opcode.unwrap_or(0) + current_operand.unwrap_or(0);
            machine_code.push(res);
        }
        
    }
    return Ok(machine_code);

}