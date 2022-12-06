use crate::Machine;

pub fn print(machine: &mut Machine) {
    let x = machine.int_stack.pop().unwrap();
    println!("{}", x);
}

pub fn bye(_: &mut Machine) {
    std::process::exit(0);
}

pub fn inspect(machine: &mut Machine) {
    println!("Stack: {:?}", machine.int_stack);
}

pub fn plus(machine: &mut Machine) {
    if machine.int_stack.len() >= 2 {
        let x = machine.int_stack.pop().unwrap();
        let y = machine.int_stack.pop().unwrap();
        machine.int_stack.push(x.saturating_add(y));
    } else {
        eprintln!(" <err> stack smaller than 2 elements")
    }
}

pub fn double(machine: &mut Machine) {
    let x = machine.int_stack.pop().unwrap();
    machine.int_stack.push(x.saturating_add(x));
}
