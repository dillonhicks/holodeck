#[macro_use]
pub mod panic {


    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[inline(always)]
    fn thread_id() -> u64 {
        unsafe { std::mem::transmute::<_, u64>(std::thread::current().id()) }
    }

    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
    #[inline(always)]
    fn thread_id() -> std::thread::ThreadId {
        std::thread::current().id()
    }


    #[cold]
    pub fn crashing<T>(
        file: &str,
        line: u32,
        message: Option<&str>,
    ) -> T {
        let message = message.unwrap_or("unspecified reason");
        let error_msg = format!(
            r#"!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

CRASHING[{}-{:?}]: {}L#{}: {}

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"#,
            std::process::id(),
            thread_id(),
            file,
            line,
            message
        );

        panic!("{}", error_msg);
    }


    #[macro_export]
    macro_rules! crash {
       () => {{
            $crate::macros::panic::crashing(file!(), line!(), None)
        }};
        ($msg:expr) => {{
            $crate::macros::panic::crashing(file!(), line!(), Some(stringify!($msg)))
        }};
        ($msg:expr,) => {{
            $crate::macros::panic::crashing(file!(), line!(), Some(stringify!($msg)))
        }};
        ($fmt:expr, $($arg:tt)*) => {{
            $crate::macros::panic::crashing(file!(), line!(), Some(&format!("{}", format_args!($fmt, $($arg)*))))
        }};
    }

    /// A conventional way to unwrap_or_else-ing `Option<T>` without the manual closure building.
    ///
    /// Uses the same convention as `crash!()` to create a closure else handler with the default
    /// conventions and formatting to provide the context info about the site of the crash.
    ///
    /// ```ignore
    /// 
    /// let count_of_a = count_for_kind.get(Kind::A)
    ///     .unwrap_or_else(crash_on_none!());
    /// ```
    ///
    /// This can be easier to reason about and refactor code that mixes unwrapping `Result<T,E>`
    /// which requires a closure that takes an `E` parameter an the `Option<T>` which requires a
    /// a parameter-less closure.
    #[macro_export]
    macro_rules! crash_on_none {
        ($fmt:expr, $($arg:tt)*) => {{
            || crash!($fmt, $($arg)*)
        }};
        ($msg:expr,) => {{
         || crash!($msg)
        }};
        ($msg:expr) => {{
            crash_on_none!($msg,)
        }};
        () => {{
            crash_on_none!("a required value to continue was missing (None)")
        }};
    }

    /// A conventional way to unwrap_or_else-ing `Result<T, E>` without the manual closure building.
    ///
    /// Uses the same convention as `crash!()` to create a closure else handler with the default
    /// conventions and formatting to provide the context info about the site of the crash. This
    /// macro automatically includes the error in the `crash!()` message.
    ///
    /// ```ignore
    /// let parsed_value = u32::try_from(value)
    ///     .unwrap_or_else(crash_on_err!("could not parse {} into a u32", value));
    /// ```
    ///
    /// This can be easier to reason about and refactor code that mixes unwrapping `Result<T,E>`
    /// which requires a closure that takes an `E` parameter an the `Option<T>` which requires a
    /// a parameter-less closure.
    #[macro_export]
    macro_rules! crash_on_err {
        ($fmt:expr, $($arg:tt)*) => {{
          |err| crash!("{} - error: {:?}", format_args!($fmt, $($arg)*), err)
        }};
        ($msg:expr,) => {{
         |err| crash!("{} - error: {:?}", $msg, err)
        }};
        ($msg:expr) => {{
            crash_on_err!($msg,)
        }};
        () => {{
            crash_on_err!("unrecoverable error encountered")
        }};
    }

    /// A `crash!()` formatted alternative to `assert!(expr)`
    #[macro_export]
    macro_rules! crash_if_not {
        ($cond:expr) => {{
            if $crate::utils::branch_optimization::unlikely(!$cond) {
                crash!("crash_if_not assertion failed! ({})", stringify!($cond))
            }
        }};
        ($cond:expr,) => {{
            crash_if_not!($cond )
        }};
        ($cond:expr, $($arg:tt)+) => {{
            if $crate::utils::branch_optimization::unlikely(!$cond) {
                crash!(r#"crash_if_not assertion failed! `({})` {}"#, stringify!($cond), format_args!($($arg)*))
            }
      }};
    }

    /// A `crash!()` formatted alternative to `assert!(!expr)`
    #[macro_export]
    macro_rules! crash_if {
        ($cond:expr) => {{
            if $crate::utils::branch_optimization::unlikely($cond) {
                crash!("assertion failed! `crash_if({})", stringify!($cond))
            }
        }};
        ($cond:expr,) => {{
            crash_if!($cond )

        }};
        ($cond:expr, $($arg:tt)+) => {{
                    if $crate::utils::branch_optimization::unlikely($cond) {

            crash!(r#"assertion failed! `crash_if({})` {}"#, stringify!($cond), format_args!($($arg)*))
        }
      }};
    }

    /// A `crash!()` formatted alternative to `assert_eq!(a, b)`
    /// Note: *style taken from the stdlib asserts*
    #[macro_export]
    macro_rules! crash_if_not_equal {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if crate::utils::branch_optimization::unlikely(!(*left_val == *right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    crash!(r#"assertion failed! `crash_if_not_equal({}, {})`
  left: `{:?}`,
 right: `{:?}`"#, stringify!($left), stringify!($right), &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        crash_if_not_equal!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if crate::utils::branch_optimization::unlikely(!(*left_val == *right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    crash!(r#"assertion failed! `crash_if_not_equal({}, {})`
  left: `{:?}`,
 right: `{:?}`: {}"#,stringify!($left), stringify!($right), &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
    }

    /// A `crash!()` formatted alternative to `assert_eq!(a, b)`
    /// Note: *style taken from the stdlib asserts*
    #[macro_export]
    macro_rules! crash_if_equal {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if crate::utils::branch_optimization::unlikely((*left_val == *right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    crash!(r#"assertion failed! `crash_if_equal({}, {})`
  left: `{:?}`,
 right: `{:?}`"#, stringify!($left), stringify!($right), &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        crash_if_equal!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if crate::utils::branch_optimization::unlikely((*left_val == *right_val)) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    crash!(r#"assertion failed! `crash_if_equal({}, {})`
  left: `{:?}`,
 right: `{:?}`: {}"#,stringify!($left), stringify!($right), &*left_val, &*right_val,
                           format_args!($($arg)+))
                }
            }
        }
    });
    }


    /// A `crash!()` formatted alternative to `assert!(expr)`
    #[macro_export]
    macro_rules! dbg_crash_if_not {
        ($cond:expr) => {{
            if cfg!(debug_assertions) && $crate::utils::branch_optimization::unlikely(!$cond) {
                crash!("dbg_crash_if_not assertion failed! ({})", stringify!($cond))
            }
        }};
        ($cond:expr,) => {{
            dbg_crash_if_not!($cond )
        }};
        ($cond:expr, $($arg:tt)+) => {{
            if cfg!(debug_assertions) && $crate::utils::branch_optimization::unlikely(!$cond) {
                crash!(r#"dbg_crash_if_not assertion failed! `({})` {}"#, stringify!($cond), format_args!($($arg)*))
            }
      }};
    }

    /// A `crash!()` formatted alternative to `assert!(!expr)`
    #[macro_export]
    macro_rules! dbg_crash_if {
        ($cond:expr) => {{
            if cfg!(debug_assertions) && $crate::utils::branch_optimization::unlikely($cond) {
                crash!("assertion failed! `dbg_crash_if({})", stringify!($cond))
            }
        }};
        ($cond:expr,) => {{
            dbg_crash_if!($cond )

        }};
        ($cond:expr, $($arg:tt)+) => {{
           if cfg!(debug_assertions) && $crate::utils::branch_optimization::unlikely($cond) {

            crash!(r#"assertion failed! `dbg_crash_if({})` {}"#, stringify!($cond), format_args!($($arg)*))
        }
      }};
    }


    /// A `crash!()` formatted alternative to `assert!(expr)`
    #[macro_export]
    macro_rules! dbg_crash_if_not_equal {
        ($($arg:tt)+) => {{
            if cfg!(debug_assertions)  {
                crash_if_not_equal!($($arg)*)
            }
        }};
    }

    /// A `crash!()` formatted alternative to `assert!(expr)`
    #[macro_export]
    macro_rules! dbg_crash_if_not_eq {
        ($($arg:tt)+) => {{
            dbg_crash_if_not_equal!($($arg)*)
        }};
    }

    /// A `crash!()` formatted alternative to `assert!(!expr)`
    #[macro_export]
    macro_rules! dbg_crash_if_equal {
        ($($arg:tt)+) => {{
            if cfg!(debug_assertions)  {
                crash_if_equal!($($arg)*)
            }
        }};
    }

    /// A `crash!()` formatted alternative to `assert!(expr)`
    #[macro_export]
    macro_rules! dbg_crash_if_eq {
        ($($arg:tt)+) => {{
            dbg_crash_if_equal!($($arg)*)
        }};
    }
}

#[macro_use]
pub mod warnings {

    #[macro_export]
    macro_rules! warn_on_err {
        ($fmt:expr, $($arg:tt)*) => {{
          |err| { $crate::deps::log::warn!("{}: error={:?}", format_args!($fmt, $($arg)*), err) }
        }};
        ($msg:expr,) => {{
         |err| { $crate::deps::log::warn!("{}: error={:?}", $msg, err) }
        }};
        ($msg:expr) => {{
            warn_on_err!($msg,)
        }};
        () => {{
            warn_on_err!("potentially recoverable error encountered")
        }};
    }

    #[macro_export]
    macro_rules! peek_warn {
        ($fmt:expr, $($arg:tt)*) => {{
          |err| { $crate::deps::log::warn!("{} error={:?}", ::std::format_args!($fmt, $($arg)*), err); err}
        }};
        ($msg:expr,) => {{
         |err| { $crate::deps::log::warn!("{} error={:?}", $msg, err); err}
        }};
        ($msg:expr) => {{
            peek_warn!($msg,)
        }};
        () => {{
            peek_warn!("potentially recoverable error encountered")
        }};
    }
}
