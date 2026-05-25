use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::Arc;
use vmm::Vmm;
use vmm::resources::VmResources;
use vmm::linux::vstate::VcpuState;
use vmm::linux::vstate::VmState;

static LIVE_VMMS: Lazy<Mutex<HashMap<u32, Arc<Mutex<Vmm>>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

static MEMFD_FOR_CTX: Lazy<Mutex<HashMap<u32, Vec<std::os::fd::RawFd>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

static VMR_FOR_CTX: Lazy<Mutex<HashMap<u32, VmResources>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

static VCPU_STATES: Lazy<Mutex<HashMap<u32, Vec<VcpuState>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static VM_STATES: Lazy<Mutex<HashMap<u32, VmState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn insert_live_vmm(ctx_id: u32, vmm: Arc<Mutex<Vmm>>) {
    LIVE_VMMS.lock().unwrap().insert(ctx_id, vmm);
}
pub fn get_live_vmm(ctx_id: u32) -> Option<Arc<Mutex<Vmm>>> {
    LIVE_VMMS.lock().unwrap().get(&ctx_id).cloned()
}
pub fn store_memfds(ctx_id: u32, memfds: Vec<std::os::fd::RawFd>) {
    MEMFD_FOR_CTX.lock().unwrap().insert(ctx_id, memfds);
}
pub fn get_memfds(ctx_id: u32) -> Option<Vec<std::os::fd::RawFd>> {
    MEMFD_FOR_CTX.lock().unwrap().get(&ctx_id).cloned()
}
pub fn store_vmr(ctx_id: u32, vmr: VmResources) {
    VMR_FOR_CTX.lock().unwrap().insert(ctx_id, vmr);
}
pub fn get_vmr(ctx_id: u32) -> Option<VmResources> {
    VMR_FOR_CTX.lock().unwrap().get(&ctx_id).cloned()
}
pub fn store_vcpu_states(ctx_id: u32, states: Vec<VcpuState>) {
    VCPU_STATES.lock().unwrap().insert(ctx_id, states);
}
pub fn get_vcpu_states(ctx_id: u32) -> Option<Vec<VcpuState>> {
    VCPU_STATES.lock().unwrap().get(&ctx_id).cloned()
}
pub fn store_vm_state(ctx_id: u32, state: VmState) {
    VM_STATES.lock().unwrap().insert(ctx_id, state);
}
pub fn get_vm_state(ctx_id: u32) -> Option<VmState> {
    VM_STATES.lock().unwrap().get(&ctx_id).cloned()
}
