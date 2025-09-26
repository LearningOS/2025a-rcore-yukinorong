//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, get_current_task_id, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

const MAX_COUNT_TRACE: usize = 5;

#[repr(C)]
#[derive(Copy, Clone)]
struct CountTrace {
    id: usize,
    num: isize,
}

static mut COUNT_TRACE0: [CountTrace; MAX_COUNT_TRACE] = [
    CountTrace { id: SYSCALL_WRITE, num: 0 },
    CountTrace { id: SYSCALL_EXIT, num: 0 },
    CountTrace { id: SYSCALL_YIELD, num: 0 },
    CountTrace { id: SYSCALL_GET_TIME, num: 0 },
    CountTrace { id: SYSCALL_TRACE, num: 0 },
];
static mut COUNT_TRACE1: [CountTrace; MAX_COUNT_TRACE] = [
    CountTrace { id: SYSCALL_WRITE, num: 0 },
    CountTrace { id: SYSCALL_EXIT, num: 0 },
    CountTrace { id: SYSCALL_YIELD, num: 0 },
    CountTrace { id: SYSCALL_GET_TIME, num: 0 },
    CountTrace { id: SYSCALL_TRACE, num: 0 },
];
static mut COUNT_TRACE2: [CountTrace; MAX_COUNT_TRACE] = [
    CountTrace { id: SYSCALL_WRITE, num: 0 },
    CountTrace { id: SYSCALL_EXIT, num: 0 },
    CountTrace { id: SYSCALL_YIELD, num: 0 },
    CountTrace { id: SYSCALL_GET_TIME, num: 0 },
    CountTrace { id: SYSCALL_TRACE, num: 0 },
];

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

pub fn add_trace_count(id: usize) {
    info!("request id {} num = {} task id = {}", id, get_trace_count(id), get_current_task_id());
    let current_task_id = get_current_task_id();
    let count_trace_ptr: *mut [CountTrace; MAX_COUNT_TRACE] = unsafe {
        match current_task_id {
            1 => core::ptr::addr_of_mut!(COUNT_TRACE1),
            2 => core::ptr::addr_of_mut!(COUNT_TRACE2),
            _ => core::ptr::addr_of_mut!(COUNT_TRACE0),
        }
    };
    let count_trace = unsafe { &mut *count_trace_ptr };
    
    if let Some(ct) = count_trace.iter_mut().find(|ct| ct.id == id) {
        ct.num += 1;
    }

}

pub fn get_trace_count(id: usize) -> isize {
    let current_task_id = get_current_task_id();
    let count_trace = match current_task_id {
        1 => unsafe { COUNT_TRACE1 },
        2 => unsafe { COUNT_TRACE2 },
        _ => unsafe { COUNT_TRACE0 },
    };

    for ct in count_trace {
        if ct.id == id {
            return ct.num;
        }
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let addr = _id as *const u8;            
            unsafe { core::ptr::read_volatile(addr) as isize }
        },
        1 => {
            let addr = _id as *mut u8;
            unsafe {core::ptr::write_volatile(addr, _data as u8);}
            return 0;
        },
        2 => {
            get_trace_count(_id)
        }
        _ => -1
    }    
}
