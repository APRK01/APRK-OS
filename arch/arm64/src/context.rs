extern "C" {
    pub fn context_switch(prev_sp: *mut u64, next_sp: u64);
    pub fn enter_user_mode(entry: u64, stack: u64) -> !;
}
