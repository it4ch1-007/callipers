use std::env;

use std::process;
use execute::Execute;
use dbg_filename::dbg::dbg_without_options;
use dbg_options::dbg_opt::dbg_with_options;


mod disassembler;
mod dbg_filename;
mod dbg_options;

fn main() {
    //getting the filename to attach
   let args: Vec<String> = env::args().collect();
//    if args.len()<2{
//     println!("Usage: ./callipers.exe <options> <filename> OR ./callipers -h {}","for help");
//     std::process::exit(0);
//    }
   // if args.len()>=3{
   //  dbg_with_options();
   // }
   if args.len() == 2{
    dbg_without_options(&args[0]);
   }

}
