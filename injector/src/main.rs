// slightly modified https://github.com/aurazi/Simple-DLL-Injection-Rust

mod needle;
mod path;
mod process;

use anyhow::Result;
use std::thread;
use std::time;
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;
use clap::Parser;

use needle::{InjectionMethod, Needle};
use path::CPath;
use process::{Process, __GetFullPathNameW};

#[cfg(target_arch = "x86_64")]
const INJECTION_METHODS_CFG: [InjectionMethod; 3] = [
    InjectionMethod::CreateRemoteThreadInject,
    InjectionMethod::x64ThreadHijacking,
    InjectionMethod::NtCreateThreadEx,
];

#[cfg(not(target_arch = "x86_64"))]
const INJECTION_METHODS_CFG: [InjectionMethod; 2] = [
    InjectionMethod::CreateRemoteThreadInject,
    InjectionMethod::x86ThreadHijacking,
];

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    procname: String,
    #[clap(short, long, value_parser)]
    lib: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let process = Process::open_from_process_name(PROCESS_ALL_ACCESS, args.procname.trim().to_string())?;

    println!(
        "\nSelected process: {}\nPID: {}\n",
        args.procname.trim(),
        process.pid
    );

    // CreateRemoteThread
    let injection_method = INJECTION_METHODS_CFG[0];
    let dll = args.lib.clone();

    let cpath = CPath::new(__GetFullPathNameW(dll)?);

    let needle = Needle::from_process(process);
    needle.inject(injection_method, Some(cpath))?;

    println!("\nFinished injection and closed handles.");
    thread::sleep(time::Duration::from_secs(1));

    Ok(())
}
