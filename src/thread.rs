use serde::Deserialize;

use std::collections::HashMap;
use std::hash::Hasher;
use wmi::WMIDateTime;

/// The Win32_Thread WMI class represents a thread on an operating system <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-thread>
#[derive(Deserialize, Debug, Clone)]
pub struct Win32_Thread {
    Caption: Option<String>,
    CreationClassName: Option<String>,
    CSCreationClassName: Option<String>,
    CSName: Option<String>,
    Description: Option<String>,
    ElapsedTime: Option<u64>,
    ExecutionState: Option<u16>,
    Handle: Option<String>,
    InstallDate: Option<WMIDateTime>,
    KernelModeTime: Option<u64>,
    Name: Option<String>,
    OSCreationClassName: Option<String>,
    OSName: Option<String>,
    Priority: Option<u32>,
    PriorityBase: Option<u32>,
    ProcessCreationClassName: Option<String>,
    ProcessHandle: Option<String>,
    StartAddress: Option<u32>,
    Status: Option<String>,
    ThreadState: Option<u32>,
    ThreadWaitReason: Option<u32>,
    UserModeTime: Option<u64>,
}
