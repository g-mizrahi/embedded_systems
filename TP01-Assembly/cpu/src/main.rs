use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

/*
Le codage des instructions utilise le schema suivant
- Chaque instruction est codee sur 32 bits
- Operation sur 3 bits correspond a
    - operation arithmetique
    - operation arithmetique immediate
    - operation de controle de logique
    - load word
    - save word
- La premiere operande est toujours un registre code sur 6 bits (64 registres numerotes de 0 a 63)
- La deuxieme operande est toujours un registre code sur 6 bits
- La troisieme operande peut etre un registre sur 6 bits ou une constante sur 12 bits
- Function sur 3 bits correspond a l'indication de si l'operation est ADD, SUB, MUL...

On note que le total de bits utilise est 3 + 6 + 6 + 12 + 3 = 30. De plus, grace a l'utilisation de du code Function qui n'est pas utilise lors des operations Load et Save, il est possible d'utiliser ces bits supplementaires pour la constante et ainsi adresser plus de memoire.
En l'occurrence, il est ensisageable d'avoir des constantes avec 5 bits (17 bits au total) de plus et ainsi adresser 2^5 = 32 fois plus de memoire (130ko de memoire).
*/

#[derive(Debug)]
enum Operation {
    OPa, // Arithmetic operations
    OPi, // Immediate arithmetic operations
    OPc, // Control operations
    Lw,  // Load word
    Sw,  // Save word
}

#[derive(Debug)]
enum Function {
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    XOR,
    BEQ,
    BNE,
    BLO,
    BGT,
}

#[derive(Debug)]
enum OperandType {
    REG, // Register
    CST, // Constant
    LBL, // Label
}

#[derive(Debug)]
struct Operand {
    op_type: OperandType,
    value: Option<u32>,
    label: Option<String>,
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    function: Option<Function>,
    op1: Operand,
    op2: Operand,
    op3: Operand,
}

fn tokenize_string(program: String) -> Vec<Vec<String>> {
    return program
        .lines()
        .map(String::from)
        .map(|x| {
            x.split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .collect();
}

fn parse_instruction(line: &Vec<String>) -> Option<Instruction> {
    let op: Operation;
    let func: Option<Function>;
    match line[0].as_str() {
        "add" => {
            op = Operation::OPa;
            func = Some(Function::ADD);
        }
        "addi" => {
            op = Operation::OPi;
            func = Some(Function::ADD);
        }
        "sub" => {
            op = Operation::OPa;
            func = Some(Function::SUB);
        }
        "subi" => {
            op = Operation::OPi;
            func = Some(Function::SUB);
        }
        "mul" => {
            op = Operation::OPa;
            func = Some(Function::MUL);
        }
        "muli" => {
            op = Operation::OPi;
            func = Some(Function::MUL);
        }
        "div" => {
            op = Operation::OPa;
            func = Some(Function::DIV);
        }
        "divi" => {
            op = Operation::OPi;
            func = Some(Function::DIV);
        }
        "and" => {
            op = Operation::OPa;
            func = Some(Function::AND);
        }
        "andi" => {
            op = Operation::OPi;
            func = Some(Function::AND);
        }
        "or" => {
            op = Operation::OPa;
            func = Some(Function::OR);
        }
        "ori" => {
            op = Operation::OPi;
            func = Some(Function::OR);
        }
        "xor" => {
            op = Operation::OPa;
            func = Some(Function::XOR);
        }
        "xori" => {
            op = Operation::OPi;
            func = Some(Function::XOR);
        }
        "beq" => {
            op = Operation::OPc;
            func = Some(Function::BEQ);
        }
        "bne" => {
            op = Operation::OPc;
            func = Some(Function::BNE);
        }
        "blo" => {
            op = Operation::OPc;
            func = Some(Function::BLO);
        }
        "bgt" => {
            op = Operation::OPc;
            func = Some(Function::BGT);
        }
        "lw" => {
            op = Operation::Lw;
            func = None;
        }
        "sw" => {
            op = Operation::Sw;
            func = None;
        }
        _ => {
            return None;
        }
    }

    let mut op1 = line[1].to_string();
    op1.retain(|c| c != '$' && c != ',');
    let mut op2 = line[2].to_string();
    op2.retain(|c| c != '$' && c != ',');
    let mut op3 = line[3].to_string();
    op3.retain(|c| c != '$' && c != ',');

    let instruction: Instruction;
    match op {
        Operation::OPa => {
            instruction = Instruction {
                operation: op,
                function: func,
                op1: Operand {
                    op_type: OperandType::REG,
                    value: Some(op1.parse::<u32>().unwrap()),
                    label: None,
                },
                op2: Operand {
                    op_type: OperandType::REG,
                    value: Some(op2.parse::<u32>().unwrap()),
                    label: None,
                },
                op3: Operand {
                    op_type: OperandType::REG,
                    value: Some(op3.parse::<u32>().unwrap()),
                    label: None,
                },
            };
        }
        Operation::OPi => {
            instruction = Instruction {
                operation: op,
                function: func,
                op1: Operand {
                    op_type: OperandType::REG,
                    value: Some(op1.parse::<u32>().unwrap()),
                    label: None,
                },
                op2: Operand {
                    op_type: OperandType::REG,
                    value: Some(op2.parse::<u32>().unwrap()),
                    label: None,
                },
                op3: Operand {
                    op_type: OperandType::CST,
                    value: Some(op3.parse::<u32>().unwrap()),
                    label: None,
                },
            };
        }
        Operation::OPc => {
            instruction = Instruction {
                operation: op,
                function: func,
                op1: Operand {
                    op_type: OperandType::REG,
                    value: Some(op1.parse::<u32>().unwrap()),
                    label: None,
                },
                op2: Operand {
                    op_type: OperandType::REG,
                    value: Some(op2.parse::<u32>().unwrap()),
                    label: None,
                },
                op3: Operand {
                    op_type: OperandType::LBL,
                    value: None,
                    label: Some(op3),
                },
            };
        }
        Operation::Lw => {
            instruction = Instruction {
                operation: op,
                function: func,
                op1: Operand {
                    op_type: OperandType::REG,
                    value: Some(op1.parse::<u32>().unwrap()),
                    label: None,
                },
                op2: Operand {
                    op_type: OperandType::REG,
                    value: Some(op2.parse::<u32>().unwrap()),
                    label: None,
                },
                op3: Operand {
                    op_type: OperandType::CST,
                    value: Some(op3.parse::<u32>().unwrap()),
                    label: None,
                },
            };
        }
        Operation::Sw => {
            instruction = Instruction {
                operation: op,
                function: func,
                op1: Operand {
                    op_type: OperandType::REG,
                    value: Some(op1.parse::<u32>().unwrap()),
                    label: None,
                },
                op2: Operand {
                    op_type: OperandType::REG,
                    value: Some(op2.parse::<u32>().unwrap()),
                    label: None,
                },
                op3: Operand {
                    op_type: OperandType::CST,
                    value: Some(op3.parse::<u32>().unwrap()),
                    label: None,
                },
            };
        }
    }
    return Some(instruction);
}

fn print_instruction(instruction: &Instruction, labels: &HashMap<String, usize>) -> Option<u32> {
    let mut instruction_code: u32 = 0;

    match instruction.operation {
        Operation::OPa => instruction_code += 1u32 << 29,
        Operation::OPi => instruction_code += 2u32 << 29,
        Operation::OPc => instruction_code += 3u32 << 29,
        Operation::Lw => instruction_code += 4u32 << 29,
        Operation::Sw => instruction_code += 5u32 << 29,
    }
    instruction_code += instruction.op1.value.unwrap_or(0u32) << 23;
    instruction_code += instruction.op2.value.unwrap_or(0u32) << 17;
    match instruction.op3.op_type {
        OperandType::REG => {
            instruction_code += instruction.op3.value.unwrap_or(0u32) << 11
        }
        OperandType::CST => {
            instruction_code += instruction.op3.value.unwrap_or(0u32) << 5
        }
        OperandType::LBL => {
            instruction_code +=
                u32::try_from(labels[instruction.op3.label.to_owned().unwrap().as_str()])
                    .unwrap_or(0u32)
                    << 5
        }
    }
    match instruction.function {
        Some(Function::ADD) | Some(Function::BEQ) => instruction_code += 1u32 << 2,
        Some(Function::SUB) | Some(Function::BNE) => instruction_code += 2u32 << 2,
        Some(Function::MUL) | Some(Function::BLO) => instruction_code += 3u32 << 2,
        Some(Function::DIV) | Some(Function::BGT) => instruction_code += 4u32 << 2,
        Some(Function::AND) => instruction_code += 5u32 << 2,
        Some(Function::OR) => instruction_code += 6u32 << 2,
        Some(Function::XOR) => instruction_code += 7u32 << 2,
        _ => (),
    }
    return Some(instruction_code);
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage : cargo run <src file> <dst file>");
        return Ok(());
    }

    let program = fs::read_to_string(args[1].to_owned()).expect("Unable to read file");
    let tokens = tokenize_string(program);

    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut instructions: Vec<Instruction> = vec![];

    for (index, line) in tokens.iter().enumerate() {
        match line[0].chars().last().unwrap() {
            ':' => {
                labels.insert(
                    line[0].split(':').next().unwrap().to_string(),
                    index - labels.keys().len(),
                );
                ()
            }
            _ => match parse_instruction(line) {
                Some(instruction) => instructions.push(instruction),
                None => (),
            },
        }
    }

    println!("{:?}", &instructions);
    println!("{:?}", &labels);

    let mut file = fs::File::create(args[2].to_owned())?;

    for (index, instruction) in instructions.iter().enumerate() {
        print!("{index} : ");
        println!("{:#?}", instruction);
        let instruction_string = print_instruction(instruction, &labels).unwrap();
        println!("String : {:032b}\n", instruction_string);
        file.write_all(format!("{:032b}\n", instruction_string).as_bytes())?;
    }

    return Ok(());
}
