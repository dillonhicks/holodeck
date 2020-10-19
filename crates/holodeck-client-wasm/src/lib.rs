pub(crate) mod deps {
    pub(crate) use cfg_if;
    pub(crate) use console_error_panic_hook;
    pub(crate) use holodeck_core::{
        self,
        deps::bincode,
    };
    pub(crate) use holodeck_viewer;
    pub(crate) use js_sys;
    pub(crate) use wasm_bindgen;
    pub(crate) use web_sys;
    #[cfg(feature = "wee_alloc")]
    pub(crate) use wee_alloc;
}


mod wasm;


pub use self::wasm::run;
