mod log;
mod sys;
use anyhow::Context;

static CORE_HANDLE: std::sync::Mutex<Option<libloading::os::windows::Library>> =
    std::sync::Mutex::new(None);
static LOG_HANDLE: std::sync::Mutex<Option<crate::log::LogHandle>> = std::sync::Mutex::new(None);

fn get_root_dir() -> std::path::PathBuf {
    process_path::get_dylib_path()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn try_initialize_core_handle() -> anyhow::Result<()> {
    let mut core_handle_lock = CORE_HANDLE.lock().unwrap();
    if core_handle_lock.is_none() {
        let core_path = get_root_dir().join("SourceMonitor.aux2.dll");
        if !core_path.exists() {
            anyhow::bail!("Core library not found: {}", core_path.to_string_lossy());
        }
        *core_handle_lock = Some(unsafe {
            libloading::os::windows::Library::load_with_flags(
                core_path,
                libloading::os::windows::LOAD_WITH_ALTERED_SEARCH_PATH,
            )
            .context("Failed to load core library: SourceMonitor.aux2.dll")?
        });
    }
    Ok(())
}

fn initialize_core_handle() -> anyhow::Result<()> {
    let res = try_initialize_core_handle();
    match res {
        Ok(_) => anyhow::Ok(()),
        Err(e) => {
            native_dialog::DialogBuilder::message()
                .set_title("aviutl2-sourcemonitor-bootstrapper.aux2")
                .set_text(format!(
                    "Failed to initialize core library: {e}\n\nPlease try reinstalling the plugin."
                ))
                .set_level(native_dialog::MessageLevel::Error)
                .alert()
                .show()
                .unwrap();
            Err(e)
        }
    }
}

macro_rules! call_function {
    ($identifier:ident? ($($args:expr),*)) => {
        unsafe {
            CORE_HANDLE
                .lock()
                .unwrap()
                .as_ref()
                .expect("Core library not loaded")
                .get::<sys::$identifier>(std::ffi::CString::new(stringify!($identifier)).unwrap())
                .map(|f| f($($args),*))
        }
    };
    ($identifier:ident ($($args:expr),*)) => {
        unsafe {
            CORE_HANDLE
                .lock()
                .unwrap()
                .as_ref()
                .expect("Core library not loaded")
                .get::<sys::$identifier>(std::ffi::CString::new(stringify!($identifier)).unwrap())
                .expect(concat!("Failed to get ", stringify!($identifier), " function from core library"))($($args),*)
        }
    }
}

fn bootstrapper_version() -> String {
    let aviutl2_toml = include_str!("../aviutl2.toml");
    let version: toml::Value = toml::from_str(aviutl2_toml).expect("Failed to parse aviutl2.toml");
    let version_str = version
        .get("project")
        .and_then(|pkg| pkg.get("version"))
        .and_then(|v| v.as_str())
        .expect("Failed to get version from aviutl2.toml");
    version_str.to_string()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn InitializePlugin(version: u32) -> bool {
    log_info(&format!(
        "aviutl2-sourcemonitor-bootstrapper.aux2: initializing; version={}",
        bootstrapper_version()
    ));
    call_function!(InitializePlugin?(version)).unwrap_or(true)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn GetCommonPluginTable() -> *mut aviutl2_sys::plugin2::COMMON_PLUGIN_TABLE {
    call_function!(GetCommonPluginTable())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn RequiredVersion() -> u32 {
    if initialize_core_handle().is_err() {
        return u32::MAX;
    }
    call_function!(RequiredVersion?()).unwrap_or(0)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn UninitializePlugin() {
    let _ = call_function!(UninitializePlugin?());

    if let Some(core_handle) = CORE_HANDLE.lock().unwrap().take() {
        drop(core_handle);
    }
}
#[unsafe(no_mangle)]
unsafe extern "C" fn RegisterPlugin(host: *mut aviutl2_sys::plugin2::HOST_APP_TABLE) {
    call_function!(RegisterPlugin(host))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn InitializeLogger(logger: *mut aviutl2_sys::logger2::LOG_HANDLE) {
    LOG_HANDLE
        .lock()
        .unwrap()
        .replace(crate::log::LogHandle::new(logger));
    let _ = call_function!(InitializeLogger?(logger));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn InitializeConfig(config: *mut aviutl2_sys::config2::CONFIG_HANDLE) {
    let _ = call_function!(InitializeConfig?(config));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn InitializeCache(cache: *mut aviutl2_sys::cache2::CACHE_HANDLE) {
    let _ = call_function!(InitializeCache?(cache));
}

fn log_info(message: &str) {
    if let Some(log_handle) = LOG_HANDLE.lock().unwrap().as_ref() {
        log_handle.info(message);
    }
}
