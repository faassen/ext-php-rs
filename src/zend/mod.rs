//! Types used to interact with the Zend engine.

mod _type;
pub mod ce;
mod class;
mod ex;
mod function;
mod globals;
mod handlers;
mod module;

use crate::{
    error::Result,
    ffi::{php_printf, sapi_module},
};
use std::ffi::CString;

pub use _type::ZendType;
pub use class::ClassEntry;
pub use ex::ExecuteData;
pub use function::FunctionEntry;
pub use globals::ExecutorGlobals;
pub use handlers::ZendObjectHandlers;
pub use module::ModuleEntry;

// Used as the format string for `php_printf`.
const FORMAT_STR: &[u8] = b"%s\0";

/// Prints to stdout using the `php_printf` function.
///
/// Also see the [`php_print`] and [`php_println`] macros.
///
/// # Arguments
///
/// * message - The message to print to stdout.
///
/// # Returns
///
/// Nothing on success, error if the message could not be converted to a
/// [`CString`].
pub fn printf(message: &str) -> Result<()> {
    let message = CString::new(message)?;
    unsafe {
        php_printf(FORMAT_STR.as_ptr().cast(), message.as_ptr());
    };
    Ok(())
}

/// Get the name of the SAPI module.
pub fn php_sapi_name() -> String {
    let c_str = unsafe { std::ffi::CStr::from_ptr(sapi_module.name) };
    c_str.to_str().expect("Unable to parse CStr").to_string()
}
