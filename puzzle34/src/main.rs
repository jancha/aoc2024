use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}
fn analyze(file: &str) -> u64 {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").collect();

    let mut registers: [u64; 3] = [0; 3];
    let mut code: [u8; 16] = [255; 16];
    let mut stage = 0;
    let mut reg_counter: u64 = 0;
    let mut code_len = 0;

    for i in data {
        if i.is_empty() {
            stage += 1;
            continue;
        }
        if stage == 0 {
            let s: Vec<&str> = i.split(": ").collect();
            let v: u64 = s.last().unwrap().parse().unwrap();
            registers[reg_counter as usize] = v;
            reg_counter += 1;
        } else {
            let s: Vec<&str> = i.split("Program: ").collect();
            let v_code: Vec<u8> = s
                .last()
                .unwrap()
                .split(",")
                .map(|x| {
                    let n: u8 = x.parse().unwrap();
                    n
                })
                .collect();

            for (index, val) in v_code.iter().enumerate() {
                code[index] = *val;
                code_len += 1;
            }
        }
    }
    let mut a = 0;
    for i in 0..code_len {
        let mut registers: Vec<u64> = vec![a, 0, 0];
        loop {
            let output = process_code(&code, &mut registers);

            if matches(&output, &code, &code_len, i + 1) {
                if i < code_len - 1 {
                    a <<= 3;
                }
                break;
            } else {
                a += 1;
                registers = vec![a, 0, 0];
            }
        }
    }
    a
}

fn matches(output: &[u64], code: &[u8], code_len: &usize, compare_len: usize) -> bool {
    if output.len() != compare_len {
        return false;
    }
    let offset = code_len - output.len();
    for i in 0..output.len() {
        if output[i] != code[offset + i] as u64 {
            return false;
        }
    }
    true
}
fn process_code(code: &[u8; 16], registers: &mut [u64]) -> Vec<u64> {
    let mut ptr = 0;
    let mut output: Vec<u64> = vec![];
    loop {
        if ptr > code.len() - 2 {
            break;
        }
        let opcode = code[ptr];
        if opcode == 255 {
            break;
        }
        let operand = code[ptr + 1];

        if let Some(new_ptr) = process_instruction_u64(ptr, opcode, operand, registers, &mut output)
        {
            ptr = new_ptr;
        } else {
            break;
        }
    }
    output
}

fn process_instruction_u64(
    ptr: usize,
    opcode: u8,
    operand: u8,
    registers: &mut [u64],
    output: &mut Vec<u64>,
) -> Option<usize> {
    let mut ptr_advance = None;

    match opcode {
        0 => {
            let a = registers[0];
            let b = get_combo_u64(operand, registers);
            let c = a >> b;
            registers[0] = c;
        }
        1 => {
            let b = registers[1] ^ operand as u64;
            registers[1] = b;
        }
        2 => {
            let a = get_combo_u64(operand, registers);
            let b = a & 7;
            registers[1] = b;
        }
        3 => {
            if registers[0] != 0 {
                ptr_advance = Some(operand as usize);
            }
        }
        4 => {
            let c = registers[1] ^ registers[2];
            registers[1] = c;
        }
        5 => {
            let a = get_combo_u64(operand, registers);
            let b = a & 7;
            output.push(b);
        }
        6 => {
            let a = registers[0];
            let b = get_combo_u64(operand, registers);
            let c = a >> b;
            registers[1] = c;
        }
        7 => {
            let a = registers[0];
            let b = get_combo_u64(operand, registers);
            let c = a >> b;
            registers[2] = c;
        }
        _ => {}
    }

    if let Some(ptr_advance) = ptr_advance {
        Some(ptr_advance)
    } else {
        Some(ptr + 2)
    }
}

fn get_combo_u64(operand: u8, registers: &[u64]) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4..=6 => registers[operand as usize - 4],
        _ => panic!("Invalid combo operand"),
    }
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), 117440);
    assert_eq!(analyze("input.txt"), 202367025818154);
}
