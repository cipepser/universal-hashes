//! POLYVAL backends

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(feature = "force-soft")
))]
pub(crate) mod clmul;

#[cfg_attr(not(target_pointer_width = "64"), path = "backends/soft32.rs")]
#[cfg_attr(target_pointer_width = "64", path = "backends/soft64.rs")]
pub(crate) mod soft;
