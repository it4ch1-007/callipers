

use std::io::{Read, Write};
use std::os::windows::io::{AsRawHandle, FromRawHandle};
use std::ptr;
use std::{thread,time};
use std::io;
use winapi::shared::ntdef::NULL;
use winapi::um::memoryapi::{VirtualProtectEx, WriteProcessMemory};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use winapi::um::processthreadsapi::{OpenProcess,ResumeThread};
use std::os::windows::process::CommandExt;
use winapi::um::winbase::{PIPE_ACCESS_DUPLEX, PIPE_TYPE_BYTE, PIPE_READMODE_BYTE, PIPE_WAIT};
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE,PROCESS_ALL_ACCESS, PROCESS_VM_READ, PROCESS_VM_WRITE};
use winapi::um::winnt::{ MEM_COMMIT, PAGE_EXECUTE_READWRITE};

use crate::disassembler::disass::disassemble;

pub fn dbg_without_options(filename:&String){
    //spawn the process
    let mut output = Command::new(filename)
    .creation_flags(winapi::um::winbase::CREATE_SUSPENDED) //this spawns the child process in suspended state
    .spawn()
    .expect("Failed to execute");

    let pid = output.id();
    println!("PID of the process = {}",pid);

    let mut input = String::new();
    // println!("Enter the option!!");
    // io::stdin().read_line(&mut input)
    //     .expect("Failed to get the input");
   let hProcess = unsafe{OpenProcess(PROCESS_ALL_ACCESS | PROCESS_VM_READ| PROCESS_VM_WRITE, 0, output.id() as u32)};
    if hProcess!=NULL {
        println!("{}","Successfully opened the handle for the child process");
    }
    else{
        println!("Error encountered!!!");
    }
    // thread::sleep(time::Duration::from_millis(1000));
    // println!("Done sleeping");
    disassemble(hProcess);
    // unsafe{
    //     ResumeThread(hProcess);
    // }
    // let breakpoint_address = 0x0;
    // let mut old_protect: u32 = 0;
    // unsafe {
    //     VirtualProtectEx(hProcess, breakpoint_address as _, 1, PAGE_EXECUTE_READWRITE, &mut old_protect);
    //     WriteProcessMemory(hProcess, breakpoint_address as _, &0xcc, 1, ptr::null_mut());
    // }
    // // Set the child process into debug mode
    // if unsafe { DebugActiveProcess(child_pid) } == FALSE {
    //     println!("Failed to debug process. Error code: {}", unsafe { GetLastError() });
    //     return;
    // }
    // // Set a breakpoint at the specified address in the child process
    // let bp_address: LPVOID = 0x0 as LPVOID;
    // if unsafe { DebugBreakProcess(h_child_process) } == FALSE {
    //     println!("Failed to set breakpoint. Error code: {}", unsafe { GetLastError() });
    //     return;
    // }
    // // Wait for a debug event
    // let mut dbg_event = winapi::um::winnt::DEBUG_EVENT::default();
    // if unsafe { WaitForDebugEvent(&mut dbg_event, INFINITE) } == FALSE {
    //     println!("Failed to wait for debug event. Error code: {}", unsafe { GetLastError() });
    //     return;
    // }
    // thread::sleep(time::Duration::from_millis(1000));
    // // Continue the debug event
    // let continue_status = 1;
    // if unsafe { ContinueDebugEvent(dbg_event.dwProcessId, dbg_event.dwThreadId, continue_status) } == FALSE {
    //     println!("Failed to continue debug event. Error code: {}", unsafe { GetLastError() });
    //     return;
    // }
    
    // // Debugging cleanup
    // unsafe { DebugActiveProcessStop(child_pid) };

    
}