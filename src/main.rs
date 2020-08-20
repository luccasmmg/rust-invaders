use std::io;

mod condition_codes;
mod cpu;
mod helpers;
mod op_arithmetic;
mod op_branch;
mod op_data_transfer;
mod op_logical;
mod op_special_io;
mod op_stack;

fn main() -> io::Result<()> {
    Ok(())
}
