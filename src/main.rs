use winapi::um::winreg::{HKEY_LOCAL_MACHINE,RegSetValueExA, RegOpenKeyExA};
use winapi::um::winnt::REG_EXPAND_SZ;
use winapi::shared::minwindef::HKEY;
use std::mem;
use std::ptr;
fn main() {
    unsafe{
        let val=String::from("%SystemRoot%\\System32\\svchost.exe -k LocalServiceNetworkRestricted -p");
        let mut hkey: HKEY = ptr::null_mut();
        RegOpenKeyExA(HKEY_LOCAL_MACHINE,
             String::from("SYSTEM\\CurrentControlSet\\Services\\EventLog").as_ptr() as *const i8, 
             mem::zeroed(), 
             winapi::um::winnt::KEY_SET_VALUE, 
             &mut hkey);
        RegSetValueExA(hkey, 
            String::from("ImagePath").as_ptr() as *const i8, 
            mem::zeroed(), 
            REG_EXPAND_SZ, 
            val.as_ptr() as *const u8, 
            val.len() as u32);
    }

}
