use std::{fs::File, io::Read};
fn readFromFile(filename: &str) -> [u8; 256]{
    let mut memory = [0; 256];
    let mut counter = 0;
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    for i in 0..buffer.len() {
        if buffer[i].split_whitespace().count() == 1 {
            let pos = buffer[i].split_whitespace().next().unwrap().parse::<u8>().unwrap();
            for j in counter..pos {
                memory[j] = 0;
                
            }
            counter = pos;
        }else{
            memory[i] = buffer[i]
        }
        }
    memory
}

fn main() {
    // use 8 bit address space and make it safe for multi-threading
    let mut addr = [0; 256];
    addr[128] = 1;
    addr[129] = 1;
    addr[130] = 0;
    addr[131] = 0;
    addr[0] = 0x31;
    addr[1] = 128;
    addr[2] = 129;
    addr[3] = 130;
    addr[4] = 0x7;
    addr[5] = 131;
    
    let mut pc = 0;
    println!("pc: {}, addr: {:?}\n", pc, addr);
    loop {
        // fetch
        let opcode = addr[pc];
        match opcode {
            0x1 => {
                // add
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] + addr[b];
                pc += 4;
            }
            0x2 => {
                // sub
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] - addr[b];
                pc += 4;
            }
            0x3 => {
                // mul
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] * addr[b];
                pc += 4;
            }
            0x31 => {
                // fibonaci
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] + addr[b];
                addr[a] = addr[b];
                addr[b] = addr[c];
                pc += 4;

            }
            0x4 => {
                // div
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] / addr[b];
                pc += 4;
            }
            0x5 => {
                // mod
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                let c = addr[pc + 3];
                addr[c] = addr[a] % addr[b];
                pc += 4;
            }
            0x6 => {
                // mov
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                addr[b] = addr[a];
                pc += 3;
            }
            0x7 => {
                // jmp
                let a = addr[pc + 1];
                pc = addr[a];
            }
            0x8 => {
                // jz (jump if zero)
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                if addr[a] == 0 {
                    pc = addr[b];
                } else {
                    pc += 3;
                }
            }
            0x9 => {
                // jnz (jump if not zero)
                let a = addr[pc + 1];
                let b = addr[pc + 2];
                if addr[a] != 0 {
                    pc = addr[b];
                } else {
                    pc += 3;
                }
            }
            _ => {
                println!("unknown opcode: {}", opcode);
                println!("pc = {}", pc);
                break;
        }
    }
        println!("pc: {}, addr: {:?}\n", pc, addr);
        // wait for 500 ms
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
