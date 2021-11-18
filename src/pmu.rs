#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMU control register"]
    pub control_register: crate::Reg<control_register::CONTROL_REGISTER_SPEC>,
    #[doc = "0x04 - Timer0 register"]
    pub timer0: crate::Reg<timer0::TIMER0_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - RTC register 0"]
    pub rtc_register_0: crate::Reg<rtc_register_0::RTC_REGISTER_0_SPEC>,
    #[doc = "0x10 - RTC register 1"]
    pub rtc_register_1: crate::Reg<rtc_register_1::RTC_REGISTER_1_SPEC>,
    #[doc = "0x14 - PMU interrupt source register"]
    pub interrupt_source: crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>,
}
#[doc = "control_register register accessor: an alias for `Reg<CONTROL_REGISTER_SPEC>`"]
pub type CONTROL_REGISTER = crate::Reg<control_register::CONTROL_REGISTER_SPEC>;
#[doc = "PMU control register"]
pub mod control_register;
#[doc = "timer0 register accessor: an alias for `Reg<TIMER0_SPEC>`"]
pub type TIMER0 = crate::Reg<timer0::TIMER0_SPEC>;
#[doc = "Timer0 register"]
pub mod timer0;
#[doc = "rtc_register_0 register accessor: an alias for `Reg<RTC_REGISTER_0_SPEC>`"]
pub type RTC_REGISTER_0 = crate::Reg<rtc_register_0::RTC_REGISTER_0_SPEC>;
#[doc = "RTC register 0"]
pub mod rtc_register_0;
#[doc = "rtc_register_1 register accessor: an alias for `Reg<RTC_REGISTER_1_SPEC>`"]
pub type RTC_REGISTER_1 = crate::Reg<rtc_register_1::RTC_REGISTER_1_SPEC>;
#[doc = "RTC register 1"]
pub mod rtc_register_1;
#[doc = "interrupt_source register accessor: an alias for `Reg<INTERRUPT_SOURCE_SPEC>`"]
pub type INTERRUPT_SOURCE = crate::Reg<interrupt_source::INTERRUPT_SOURCE_SPEC>;
#[doc = "PMU interrupt source register"]
pub mod interrupt_source;
