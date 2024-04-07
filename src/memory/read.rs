use core::ffi::c_void;
use windows_sys::{Win32::Foundation,Win32::System::Diagnostics::Debug::*};

struct LiveMemorySource{
    hprocess: Foundation::HANDLE,
}
pub trait MemorySource{
    fn read_memory(&self,address:u64,len:usize) -> Result<Vec<Option<u8>>,&'static str>;

    fn read_raw_memory(&self,address:u64,len:usize) -> Vec<u8>;
}

//The thing is to make the attached process memory live to be debugged at runtime and not just read thje memory using the windows APIs

//This fn will make the child process memory live or we can say dynamically allocated so that we can easily read or write on it.
pub fn make_live_memory_source(hProcess: Foundation::HANDLE) -> Box<dyn MemorySource>{
    Box::new(LiveMemorySource{hProcess})    
    //We are allocating the memory of the attached process to the dynamic memory so that we are able to allocate it or else make it dynamic or readable at runtime

}
pub fn read_memory_array<T:Sized + Default>(
    source: &dyn MemorySource,
    address: u64,
    max_count:usize,
) -> Result<>{

}


impl MemorySource for LiveMemorySource{

}

pub fn read_raw_memory(&self,address:u64,len:usize) -> Vec<u8>{
    let mut buffer: Vec<u8> = vec![0;len]; //initializing a vector of bytes
    let mut bytes_read: usize = 0;

    let resulting_bytes = unsafe{
        ReadProcessMemory(
            self.hProcess, //Here self acts as a pointer to not the self class but the same module
            address as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            len,
            &mut bytes_read as *mut usize,
        )
    };

    if result == 0{
        bytes_read = 0; //The ReadProcessMemory function failed
        eprintln!("{}","Failed to read the memory bytes");
    }

    buffer.truncate(bytes_read); //This will give us the desired number of bytes from a desired address inside the module

    buffer

}