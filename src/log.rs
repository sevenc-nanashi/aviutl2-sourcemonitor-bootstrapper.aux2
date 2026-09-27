pub struct LogHandle {
    log_handle: *mut aviutl2_sys::logger2::LOG_HANDLE,
}
unsafe impl Send for LogHandle {}
unsafe impl Sync for LogHandle {}

impl LogHandle {
    pub fn new(log_handle: *mut aviutl2_sys::logger2::LOG_HANDLE) -> Self {
        Self { log_handle }
    }

    pub fn info(&self, message: &str) {
        unsafe {
            let u16_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
            let log_handle = &mut *self.log_handle;
            (log_handle.info)(log_handle, u16_message.as_ptr());
        }
    }
}
