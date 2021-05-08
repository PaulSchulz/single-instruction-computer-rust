static DEBUG: bool = true;

fn introduction() {
    println!("Welcome to CISC");
    println!("Single Instruction Set Computing");
    println!("--------------------------------");
}

// Do one processor step
// FIXME: Does not alter memory. Need to pass a reference to elements somehow,
// to make the elements mutable.
// See: https://stackoverflow.com/questions/24831828/how-do-i-pass-an-array-to-a-function-in-rust-and-change-its-content

// FIXME: Check for negative 'diff' as this causes rust code to abort due to integer overflow.
fn process(memory: &mut [u16; 0xffff], pc: u16) -> u16 {
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
    let diff = b - a;

    // Write result
    memory[ptr_b as usize] = diff;
    if diff as isize > 0 {
        ret_pc = pc + 3;
    } else {
        ret_pc = ptr_c;
    }

    if DEBUG {
        println!(
            "Out: PC:{:04X?} A:{:04X?}({:04X?}) B:{:04X?}({:04X?}) C:{:04X?}",
            ret_pc, ptr_a, a, ptr_b, diff, ptr_c
        );
    }
    return ret_pc;
}

fn list_page(memory: [u16; 0xffff], pc: u16, page: u16) {
    for i in (page..page + 10).step_by(3) {
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
    let mut memory: [u16; 0xffff] = [0u16; 65535];

    println!("DEBUG: Clearing memeory to 0");
    for i in 0..memory.len() {
        memory[i] = 0x0000;
    }
    introduction();

    // Testing
    memory[0x0000] = 0x0000;
    memory[0x0001] = 0x0001;
    memory[0x0002] = 0x0003;

    list_page(memory, pc, 0x0000);

    pc = process(&mut memory, pc);
    pc = process(&mut memory, pc);

    list_page(memory, pc, 0x0000);
}
