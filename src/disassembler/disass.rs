// extern crate capstone;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, TH32CS_SNAPPROCESS};
// use capstone::prelude::*;
use std::{thread,time};
use winapi::shared::minwindef::LPVOID;
use winapi::ctypes::{c_void};
use winapi::um::memoryapi::{VirtualQueryEx,ReadProcessMemory};
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::psapi::EnumProcessModules;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use winapi::um::winnt::MEMORY_BASIC_INFORMATION;
use winapi::um::errhandlingapi::GetLastError;
use std::ptr;

fn bytes_to_byte_string(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|byte| format!("\\x{:02X}", byte))
        .collect::<String>()
}
pub fn disassemble(process_handle:*mut c_void){
    // let cs = Capstone::new()
    //         .x86()
    //         .mode(arch::x86::ArchMode::Mode64)
    //         .build()
    //         .expect("Failed to create Capstone object");
    
    
    
        thread::sleep(time::Duration::from_millis(2000));
        println!("{:?}",memory_info.BaseAddress);
        // Access memory regions information through `memory_info` struct
        // For example, read memory from `memory_info.BaseAddress` to `memory_info.RegionSize`
        let mut buffer = vec![0u8; memory_info.RegionSize as usize];
            let mut bytes_read: usize = 0;
            let status = unsafe {
                ReadProcessMemory(
                    process_handle,
                    memory_info.BaseAddress,
                    buffer.as_mut_ptr() as LPVOID,
                    memory_info.RegionSize as usize,
                    &mut bytes_read,
                )
            };
            if  status == 0
            {
                thread::sleep(time::Duration::from_millis(1000));
                println!("Failed to read memory from process.");
                // Additional error handling if needed
            } 
            else {
                // Print the memory bytes
                thread::sleep(time::Duration::from_millis(1000));
                
                println!("Memory bytes: {:?}", buffer);
                println!("{}",bytes_to_byte_string(&buffer));

            }
        
        
    

    // let mut bu
    // let instructions = cs.disasm_all(&buffer[..bytes_read], 0x1000)
    //     .expect("Failed to disassemble instructions");
    // println!("Found {} instructions", insns.len());
    // for insn in instructions.iter() {
    //     println!("{:?}", insn);
    // }
    

}
}