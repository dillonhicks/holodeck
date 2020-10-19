pub mod branch_optimization {
    // Branch prediction hint. This is currently only available on
    // nightly
    #[cfg(feature = "nightly")]
    pub(in crate) use core::intrinsics::{
        likely,
        unlikely,
    };


    #[cold]
    fn cold() {}


    #[cfg(not(feature = "nightly"))]
    #[inline(always)]
    pub fn unlikely(b: bool) -> bool {
        if b {
            cold()
        }
        b
    }


    #[cfg(not(feature = "nightly"))]
    #[inline(always)]
    pub fn likely(b: bool) -> bool {
        if !b {
            cold()
        }
        b
    }
}
