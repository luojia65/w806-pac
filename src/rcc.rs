#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software configure clock gate enable"]
    pub clock_enable: crate::Reg<clock_enable::CLOCK_ENABLE_SPEC>,
    #[doc = "0x04 - Software clock mask"]
    pub clock_mask: crate::Reg<clock_mask::CLOCK_MASK_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Software module reset control"]
    pub reset_control: crate::Reg<reset_control::RESET_CONTROL_SPEC>,
    #[doc = "0x10 - ??"]
    pub clock_div: crate::Reg<clock_div::CLOCK_DIV_SPEC>,
    #[doc = "0x14 - ??"]
    pub clock_select: crate::Reg<clock_select::CLOCK_SELECT_SPEC>,
    #[doc = "0x18 - ??"]
    pub i2s_clock: crate::Reg<i2s_clock::I2S_CLOCK_SPEC>,
    #[doc = "0x1c - ??"]
    pub reset_status: crate::Reg<reset_status::RESET_STATUS_SPEC>,
}
#[doc = "clock_enable register accessor: an alias for `Reg<CLOCK_ENABLE_SPEC>`"]
pub type CLOCK_ENABLE = crate::Reg<clock_enable::CLOCK_ENABLE_SPEC>;
#[doc = "Software configure clock gate enable"]
pub mod clock_enable;
#[doc = "clock_mask register accessor: an alias for `Reg<CLOCK_MASK_SPEC>`"]
pub type CLOCK_MASK = crate::Reg<clock_mask::CLOCK_MASK_SPEC>;
#[doc = "Software clock mask"]
pub mod clock_mask;
#[doc = "reset_control register accessor: an alias for `Reg<RESET_CONTROL_SPEC>`"]
pub type RESET_CONTROL = crate::Reg<reset_control::RESET_CONTROL_SPEC>;
#[doc = "Software module reset control"]
pub mod reset_control;
#[doc = "clock_div register accessor: an alias for `Reg<CLOCK_DIV_SPEC>`"]
pub type CLOCK_DIV = crate::Reg<clock_div::CLOCK_DIV_SPEC>;
#[doc = "??"]
pub mod clock_div;
#[doc = "clock_select register accessor: an alias for `Reg<CLOCK_SELECT_SPEC>`"]
pub type CLOCK_SELECT = crate::Reg<clock_select::CLOCK_SELECT_SPEC>;
#[doc = "??"]
pub mod clock_select;
#[doc = "i2s_clock register accessor: an alias for `Reg<I2S_CLOCK_SPEC>`"]
pub type I2S_CLOCK = crate::Reg<i2s_clock::I2S_CLOCK_SPEC>;
#[doc = "??"]
pub mod i2s_clock;
#[doc = "reset_status register accessor: an alias for `Reg<RESET_STATUS_SPEC>`"]
pub type RESET_STATUS = crate::Reg<reset_status::RESET_STATUS_SPEC>;
#[doc = "??"]
pub mod reset_status;
