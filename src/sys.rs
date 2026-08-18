pub type GetCommonPluginTable =
    unsafe extern "C" fn() -> *mut aviutl2_sys::plugin2::COMMON_PLUGIN_TABLE;
pub type RegisterPlugin = unsafe extern "C" fn(host: *mut aviutl2_sys::plugin2::HOST_APP_TABLE);
pub type RequiredVersion = unsafe extern "C" fn() -> u32;
pub type InitializePlugin = unsafe extern "C" fn(version: u32) -> bool;
pub type UninitializePlugin = unsafe extern "C" fn();
pub type InitializeLogger = unsafe extern "C" fn(logger: *mut aviutl2_sys::logger2::LOG_HANDLE);
pub type InitializeConfig = unsafe extern "C" fn(config: *mut aviutl2_sys::config2::CONFIG_HANDLE);
pub type InitializeCache = unsafe extern "C" fn(cache: *mut aviutl2_sys::cache2::CACHE_HANDLE);
