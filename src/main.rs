use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn read_from_file(filename: &str) -> [usize; 256]{
    let mut memory = [0; 256];
    let mut counter = 0;

    let mut opcodes = std::collections::HashMap::new();
    opcodes.insert("add", 0x1);
    opcodes.insert("sub", 0x2);
    opcodes.insert("mul", 0x3);
    opcodes.insert("fib", 0x31);
    opcodes.insert("div", 0x4);
    opcodes.insert("mod", 0x5);
    opcodes.insert("mov", 0x6);
    opcodes.insert("jmp", 0x7);
    opcodes.insert("jz", 0x8);
    opcodes.insert("jnz", 0x9);
    opcodes.insert("end", 0x0);

    let buf_reader = BufReader::new(File::open(filename).unwrap());    

    for line in buf_reader.lines() {
        let tmp = line.unwrap();
        println!("{}", tmp);
        
        if opcodes.contains_key(tmp.split(" ").nth(0).unwrap()) {
            for i in 0..tmp.split(" ").count() {
                if i == 0 {
                    memory[counter] = opcodes[tmp.split(" ").nth(i).unwrap()];
                    counter += 1;
                    continue;
                }
                else {
                    memory[counter] = tmp.split(" ").nth(i).unwrap().parse::<usize>().unwrap();
                    counter += 1;
                }
            }
            continue;
        }
        if tmp.split(" ").nth(0).unwrap() == "start" {
            let pos = tmp.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            for j in counter..=pos {
                memory[j] = 0;
                
            }
            counter = pos;
            continue;
        }
        memory[counter] = tmp.parse::<usize>().unwrap();
        counter += 1;
     
    }
    memory
}

fn main() {
    let mut addr = read_from_file("write.txt");
    // create dictionary for opcodes
    println!("addr: {:?}\n", addr);
    loop {
        // fetch
        let mut pc = addr[1];
        let opcode = addr[pc];
        
        match opcode {
            0x0=> {
                // end
                println!("pc: {}, addr: {:?}\n", pc, addr);
                println!("program finished");
                break;
            }
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
        addr[1] = pc;
        std::thread::sleep(std::time::Duration::from_millis(100));
        
    }
}
