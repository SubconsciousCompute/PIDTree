#![allow(non_snake_case)]

mod process;

use std::collections::HashMap;

use crate::process::Win32_Process;

pub use wmi::COMLibrary;
pub use wmi::{Variant, WMIConnection, WMIDateTime, WMIDuration};

pub type Query = Vec<HashMap<String, Variant>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::with_namespace_path(r"Root\CIMV2", com_con)?;

    //  Represents a sequence of events on a computer system running Windows
    let results: Vec<Win32_Process> = wmi_con.raw_query("SELECT * FROM Win32_Process")?;

    dbg!(&results);

    /*
    let mut tree = ProcessTree::new();

    for process in results {
        tree.add_process(process);
    }

    tree.print();
     */

    //let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_Thread")?;

    //dbg!(&results[0]);

    Ok(())
}
