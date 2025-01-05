use std::fs;
fn main() {
    println!("{}", analyze("input.txt"));
}
fn analyze(file: &str) -> String {
    let file = fs::read_to_string(file).expect("Could not read file?");
    let data: Vec<&str> = file.trim().split("\n").collect();

    let mut registers: [usize; 3] = [0; 3];
    let mut code: [u8; 16] = [255; 16];
    let mut stage = 0;
    let mut reg_counter = 0;
    for i in data {
        if i.is_empty() {
            stage += 1;
            continue;
        }
        if stage == 0 {
            let s: Vec<&str> = i.split(": ").collect();
            let v: usize = s.last().unwrap().parse().unwrap();
            registers[reg_counter] = v;
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
            }
        }
    }

    let mut ptr = 0;
    let mut output: Vec<usize> = vec![];

    loop {
        if ptr > code.len() - 2 {
            break;
        }
        let opcode = code[ptr];
        if opcode == 255 {
            break;
        }
        let operand = code[ptr + 1];

        if let Some(new_ptr) =
            process_instruction(ptr, opcode, operand, &mut registers, &mut output)
        {
            ptr = new_ptr;
        } else {
            break;
        }
    }

    let out: Vec<String> = output.iter().map(|x| format!("{x}")).collect();
    out.join(",")
}

fn process_instruction(
    ptr: usize,
    opcode: u8,
    operand: u8,
    registers: &mut [usize],
    output: &mut Vec<usize>,
) -> Option<usize> {
    let mut ptr_advance = None;
    match opcode {
        0 => {
            let a = registers[0];
            let b: usize = 2_usize.pow(get_combo(operand, registers) as u32);
            registers[0] = a / b;
        }
        1 => {
            registers[1] ^= operand as usize;
        }
        2 => {
            registers[1] = get_combo(operand, registers) % 8;
        }
        3 => {
            if registers[0] != 0 {
                ptr_advance = Some(operand as usize);
            }
        }
        4 => {
            registers[1] ^= registers[2];
        }
        5 => output.push(get_combo(operand, registers) % 8_usize),
        6 => {
            let a = registers[0];
            let b: usize = 2_usize.pow(get_combo(operand, registers) as u32);
            registers[1] = a / b;
        }
        7 => {
            let a = registers[0];
            let b: usize = 2_usize.pow(get_combo(operand, registers) as u32);
            registers[2] = a / b;
        }
        _ => {}
    }

    if let Some(ptr_advance) = ptr_advance {
        Some(ptr_advance)
    } else {
        Some(ptr + 2)
    }
}

fn get_combo(operand: u8, registers: &[usize]) -> usize {
    match operand {
        0..=3 => operand as usize,
        4..=6 => registers[operand as usize - 4],
        _ => panic!("Invalid combo operand"),
    }
}

#[test]
fn test_1() {
    assert_eq!(analyze("test.txt"), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(analyze("test2.txt"), "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(analyze("input.txt"), "1,3,7,4,6,4,2,3,5");
}
