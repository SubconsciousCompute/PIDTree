use serde::Deserialize;

use std::collections::HashMap;
use std::hash::Hasher;
use wmi::WMIDateTime;

/// The Win32_Process WMI class represents a process on an operating system <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process>
#[derive(Deserialize, Debug, Clone)]
pub struct Win32_Process {
    CreationClassName: Option<String>,
    Caption: Option<String>,
    CommandLine: Option<String>,
    CreationDate: Option<WMIDateTime>,
    CSCreationClassName: Option<String>,
    CSName: Option<String>,
    Description: Option<String>,
    ExecutablePath: Option<String>,
    ExecutionState: Option<u16>,
    Handle: Option<String>,
    HandleCount: Option<u32>,
    InstallDate: Option<WMIDateTime>,
    KernelModeTime: Option<u64>,
    MaximumWorkingSetSize: Option<u32>,
    MinimumWorkingSetSize: Option<u32>,
    Name: Option<String>,
    OSCreationClassName: Option<String>,
    OSName: Option<String>,
    OtherOperationCount: Option<u64>,
    OtherTransferCount: Option<u64>,
    PageFaults: Option<u32>,
    PageFileUsage: Option<u32>,
    ParentProcessId: Option<u32>,
    PeakPageFileUsage: Option<u32>,
    PeakVirtualSize: Option<u64>,
    PeakWorkingSetSize: Option<u32>,
    Priority: Option<u32>,
    PrivatePageCount: Option<u64>,
    ProcessId: Option<u32>,
    QuotaNonPagedPoolUsage: Option<u32>,
    QuotaPagedPoolUsage: Option<u32>,
    QuotaPeakNonPagedPoolUsage: Option<u32>,
    QuotaPeakPagedPoolUsage: Option<u32>,
    ReadOperationCount: Option<u64>,
    ReadTransferCount: Option<u64>,
    SessionId: Option<u32>,
    // is always null
    Status: Option<String>,
    TerminationDate: Option<WMIDateTime>,
    ThreadCount: Option<u32>,
    UserModeTime: Option<u64>,
    VirtualSize: Option<u64>,
    WindowsVersion: Option<String>,
    WorkingSetSize: Option<u64>,
    WriteOperationCount: Option<u64>,
    WriteTransferCount: Option<u64>,
}

pub struct ProcessNode {
    process: Win32_Process,
    children: Vec<ProcessNode>,
}

pub struct ProcessTree {
    nodes: HashMap<u32, ProcessNode>,
}

impl ProcessTree {
    pub fn new() -> ProcessTree {
        ProcessTree {
            nodes: HashMap::new(),
        }
    }

    pub(crate) fn add_process(&mut self, process: Win32_Process) {
        let node = ProcessNode {
            process: process.clone(),
            children: Vec::new(),
        };

        if let Some(pid) = process.ParentProcessId {
            if let Some(parent_node) = self.nodes.get_mut(&pid) {
                parent_node.children.push(node);
                return;
            }
        }

        self.nodes.insert(process.ProcessId.unwrap(), node);
    }

    pub fn path_to_root(&self, pid: u32) -> Option<Vec<u32>> {
        if let Some(node) = self.nodes.get(&pid) {
            let mut path = Vec::new();

            let mut current_node = node;
            while let Some(parent_pid) = current_node.process.ParentProcessId {
                path.push(parent_pid);

                if let Some(parent_node) = self.nodes.get(&parent_pid) {
                    current_node = parent_node;
                } else {
                    break;
                }
            }

            path.reverse();
            path.push(pid);

            Some(path)
        } else {
            None
        }
    }

    pub(crate) fn print(&self) {
        for node in self.nodes.values() {
            self.print_node(node, 0);
        }
    }

    fn print_node(&self, node: &ProcessNode, level: u32) {
        println!(
            "{:indent$}{:?} {:?}",
            "",
            node.process.ParentProcessId,
            node.process.ProcessId,
            indent = (level * 2) as usize
        );

        for child in &node.children {
            self.print_node(child, level + 1);
        }
    }
}
