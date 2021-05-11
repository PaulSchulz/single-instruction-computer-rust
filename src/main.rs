static DEBUG: bool = true;

fn introduction() {
    println!("Welcome to CISC");
    println!("Single Instruction Set Computing");
    println!("--------------------------------");
}

// Do one processor step
fn process(memory: &mut [u16; 0x10000], pc: u16) -> u16 {
    let ptr_a: u16 = memory[pc as usize];
    let ptr_b: u16 = memory[pc as usize + 1];
    let ptr_c: u16 = memory[pc as usize + 2];

    let a: u16 = memory[ptr_a as usize];
    let b: u16 = memory[ptr_b as usize];

    let ret_pc: u16;

    if DEBUG {
        println!(
            "In:  PC:{:04X?} A:{:04X?}({:04X?}) B:{:04X?}({:04X?}) C:{:04X?}",
            pc, ptr_a, a, ptr_b, b, ptr_c
        );
    }

    let diff: u16;
    diff = b.wrapping_sub(a);

    // Write result
    memory[ptr_b as usize] = diff;
    // Is the MSB clear (negative number)?
    if (diff == 0) || (diff & 0b1000_0000) != 0 {
        // Branch
        ret_pc = ptr_c;
    } else {
        // Next Instuction
        ret_pc = pc + 3;
    }

    if DEBUG {
        println!(
            "Out: PC:{:04X?} A:{:04X?}({:04X?}) B:{:04X?}({:04X?}) C:{:04X?}",
            ret_pc, ptr_a, a, ptr_b, diff, ptr_c
        );
    }

    return ret_pc;
}

fn list_page(memory: [u16; 0x10000], pc: u16, page: u16) {
    for i in (page..page + 21).step_by(3) {
        if (page <= pc) && (pc < page + 3) {
            print!("> ");
        } else {
            print!("  ");
        }

        println!(
            "{:04X?}: {:04X?} {:04X?} {:04X?}",
            i,
            memory[i as usize],
            memory[i as usize + 1],
            memory[i as usize + 2]
        );
    }
}

//////////////////////////////////////////////////////////////////////////////
fn main() {
    // Processor data
    let mut pc: u16 = 0x0000;
    let mut memory: [u16; 0x10000] = [0u16; 0x10000];

    println!("DEBUG: Clearing memeory to 0x0");
    for i in 0..memory.len() {
        memory[i] = 0x0000;
    }
    introduction();

    // Testing - Iterating Loop
    // L:0 1 ?
    // A B L
    // ; Data
    // A:1 B:0 0

    memory[0x0000] = 0x0000;
    memory[0x0001] = 0x0001;
    memory[0x0002] = 0x0003;

    memory[0x0003] = 0x0006;
    memory[0x0004] = 0x0007;
    memory[0x0005] = 0x0000;
    // Data
    memory[0x0006] = 0x0001;
    memory[0x0007] = 0x0000;
    memory[0x0008] = 0x0000;

    list_page(memory, pc, 0x0000);

    // Processing loop
    for _i in 0..6 {
        pc = process(&mut memory, pc);
    }

    list_page(memory, pc, 0x0000);
}
