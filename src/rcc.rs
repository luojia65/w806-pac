#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software configure clock gate enable\n\n By configuring this clock gating enable register, you can control the clock to shutdown a specified function, so as to achieve the purpose of shutting down a certain module.\n\n In order to provide the firmware with flexibility to control the power consumption of the system, the clock and reset module provides the clock gating function of each module in the system. When the clock of the corresponding module is turned off, the digital logic and clock tree of the module will stop working, which can reduce the dynamic power consumption of the system."]
    pub clock_enable: crate::Reg<clock_enable::CLOCK_ENABLE_SPEC>,
    #[doc = "0x04 - Adaptive clock configuration mask\n\n The chip adaptively turns off the clock of some functional modules according to the transition of some internal states.\n\n Please do not change the configuration, otherwise it may cause system abnormality when configuring PMU function."]
    pub adapt_mask: crate::Reg<adapt_mask::ADAPT_MASK_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Software module reset control\n\n The chip provides the soft reset function of each subsystem, and the subsystem reset can be achieved by setting the corresponding bit of this register to 0. However, the reset state will not be automatically cleared. To restore normal operation, the corresponding bit of this register must be set to 1.\n\n The soft reset function does not reset the CPU and watchdog. In this register, the reset operation of APB, BUS1 and BUS2 (corresponding to APB bus, system bus and data bus) is not recommended, which will cause system access device abnormality."]
    pub reset_control: crate::Reg<reset_control::RESET_CONTROL_SPEC>,
    #[doc = "0x10 - Software clock division configuration\n\n"]
    pub clock_divide: crate::Reg<clock_divide::CLOCK_DIVIDE_SPEC>,
    #[doc = "0x14 - Debug and additional clock control"]
    pub debug_control: crate::Reg<debug_control::DEBUG_CONTROL_SPEC>,
    #[doc = "0x18 - Inter-Integrated Sound clock configuration"]
    pub i2s_clock: crate::Reg<i2s_clock::I2S_CLOCK_SPEC>,
    #[doc = "0x1c - CPU and watchdog reset state register"]
    pub reset_state: crate::Reg<reset_state::RESET_STATE_SPEC>,
}
#[doc = "clock_enable register accessor: an alias for `Reg<CLOCK_ENABLE_SPEC>`"]
pub type CLOCK_ENABLE = crate::Reg<clock_enable::CLOCK_ENABLE_SPEC>;
#[doc = "Software configure clock gate enable\n\n By configuring this clock gating enable register, you can control the clock to shutdown a specified function, so as to achieve the purpose of shutting down a certain module.\n\n In order to provide the firmware with flexibility to control the power consumption of the system, the clock and reset module provides the clock gating function of each module in the system. When the clock of the corresponding module is turned off, the digital logic and clock tree of the module will stop working, which can reduce the dynamic power consumption of the system."]
pub mod clock_enable;
#[doc = "adapt_mask register accessor: an alias for `Reg<ADAPT_MASK_SPEC>`"]
pub type ADAPT_MASK = crate::Reg<adapt_mask::ADAPT_MASK_SPEC>;
#[doc = "Adaptive clock configuration mask\n\n The chip adaptively turns off the clock of some functional modules according to the transition of some internal states.\n\n Please do not change the configuration, otherwise it may cause system abnormality when configuring PMU function."]
pub mod adapt_mask;
#[doc = "reset_control register accessor: an alias for `Reg<RESET_CONTROL_SPEC>`"]
pub type RESET_CONTROL = crate::Reg<reset_control::RESET_CONTROL_SPEC>;
#[doc = "Software module reset control\n\n The chip provides the soft reset function of each subsystem, and the subsystem reset can be achieved by setting the corresponding bit of this register to 0. However, the reset state will not be automatically cleared. To restore normal operation, the corresponding bit of this register must be set to 1.\n\n The soft reset function does not reset the CPU and watchdog. In this register, the reset operation of APB, BUS1 and BUS2 (corresponding to APB bus, system bus and data bus) is not recommended, which will cause system access device abnormality."]
pub mod reset_control;
#[doc = "clock_divide register accessor: an alias for `Reg<CLOCK_DIVIDE_SPEC>`"]
pub type CLOCK_DIVIDE = crate::Reg<clock_divide::CLOCK_DIVIDE_SPEC>;
#[doc = "Software clock division configuration\n\n"]
pub mod clock_divide;
#[doc = "debug_control register accessor: an alias for `Reg<DEBUG_CONTROL_SPEC>`"]
pub type DEBUG_CONTROL = crate::Reg<debug_control::DEBUG_CONTROL_SPEC>;
#[doc = "Debug and additional clock control"]
pub mod debug_control;
#[doc = "i2s_clock register accessor: an alias for `Reg<I2S_CLOCK_SPEC>`"]
pub type I2S_CLOCK = crate::Reg<i2s_clock::I2S_CLOCK_SPEC>;
#[doc = "Inter-Integrated Sound clock configuration"]
pub mod i2s_clock;
#[doc = "reset_state register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "CPU and watchdog reset state register"]
pub mod reset_state;
