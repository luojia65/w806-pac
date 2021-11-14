#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO data value read/write register"]
    pub data_value: crate::Reg<data_value::DATA_VALUE_SPEC>,
    #[doc = "0x04 - GPIO data read/write enable register"]
    pub data_enable: crate::Reg<data_enable::DATA_ENABLE_SPEC>,
    #[doc = "0x08 - GPIO direction register"]
    pub direction: crate::Reg<direction::DIRECTION_SPEC>,
    #[doc = "0x0c - Internal pull up mask register"]
    pub pull_up: crate::Reg<pull_up::PULL_UP_SPEC>,
    #[doc = "0x10 - Alternate function enable register"]
    pub function_enable: crate::Reg<function_enable::FUNCTION_ENABLE_SPEC>,
    #[doc = "0x14 - Alternate function select high register"]
    pub function_high: crate::Reg<function_high::FUNCTION_HIGH_SPEC>,
    #[doc = "0x18 - Alternate function select low register"]
    pub function_low: crate::Reg<function_low::FUNCTION_LOW_SPEC>,
    #[doc = "0x1c - Internal pull down register"]
    pub pull_down: crate::Reg<pull_down::PULL_DOWN_SPEC>,
    #[doc = "0x20 - Interrupt trigger mode selection"]
    pub trigger_mode: crate::Reg<trigger_mode::TRIGGER_MODE_SPEC>,
    #[doc = "0x24 - Interrupt trigger on both sides selection"]
    pub trigger_both: crate::Reg<trigger_both::TRIGGER_BOTH_SPEC>,
    #[doc = "0x28 - Interrupt trigger edge or level type selection"]
    pub trigger_edge_level: crate::Reg<trigger_edge_level::TRIGGER_EDGE_LEVEL_SPEC>,
    #[doc = "0x2c - Interrupt enable register"]
    pub interrupt_enable: crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>,
    #[doc = "0x30 - Raw interrupt state register"]
    pub interrupt_state_raw: crate::Reg<interrupt_state_raw::INTERRUPT_STATE_RAW_SPEC>,
    #[doc = "0x34 - Masked interrupt state register"]
    pub interrupt_state: crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>,
    #[doc = "0x38 - Clear interrupt register"]
    pub interrupt_clear: crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>,
}
#[doc = "data_value register accessor: an alias for `Reg<DATA_VALUE_SPEC>`"]
pub type DATA_VALUE = crate::Reg<data_value::DATA_VALUE_SPEC>;
#[doc = "GPIO data value read/write register"]
pub mod data_value;
#[doc = "data_enable register accessor: an alias for `Reg<DATA_ENABLE_SPEC>`"]
pub type DATA_ENABLE = crate::Reg<data_enable::DATA_ENABLE_SPEC>;
#[doc = "GPIO data read/write enable register"]
pub mod data_enable;
#[doc = "direction register accessor: an alias for `Reg<DIRECTION_SPEC>`"]
pub type DIRECTION = crate::Reg<direction::DIRECTION_SPEC>;
#[doc = "GPIO direction register"]
pub mod direction;
#[doc = "pull_up register accessor: an alias for `Reg<PULL_UP_SPEC>`"]
pub type PULL_UP = crate::Reg<pull_up::PULL_UP_SPEC>;
#[doc = "Internal pull up mask register"]
pub mod pull_up;
#[doc = "function_enable register accessor: an alias for `Reg<FUNCTION_ENABLE_SPEC>`"]
pub type FUNCTION_ENABLE = crate::Reg<function_enable::FUNCTION_ENABLE_SPEC>;
#[doc = "Alternate function enable register"]
pub mod function_enable;
#[doc = "function_high register accessor: an alias for `Reg<FUNCTION_HIGH_SPEC>`"]
pub type FUNCTION_HIGH = crate::Reg<function_high::FUNCTION_HIGH_SPEC>;
#[doc = "Alternate function select high register"]
pub mod function_high;
#[doc = "function_low register accessor: an alias for `Reg<FUNCTION_LOW_SPEC>`"]
pub type FUNCTION_LOW = crate::Reg<function_low::FUNCTION_LOW_SPEC>;
#[doc = "Alternate function select low register"]
pub mod function_low;
#[doc = "pull_down register accessor: an alias for `Reg<PULL_DOWN_SPEC>`"]
pub type PULL_DOWN = crate::Reg<pull_down::PULL_DOWN_SPEC>;
#[doc = "Internal pull down register"]
pub mod pull_down;
#[doc = "trigger_mode register accessor: an alias for `Reg<TRIGGER_MODE_SPEC>`"]
pub type TRIGGER_MODE = crate::Reg<trigger_mode::TRIGGER_MODE_SPEC>;
#[doc = "Interrupt trigger mode selection"]
pub mod trigger_mode;
#[doc = "trigger_both register accessor: an alias for `Reg<TRIGGER_BOTH_SPEC>`"]
pub type TRIGGER_BOTH = crate::Reg<trigger_both::TRIGGER_BOTH_SPEC>;
#[doc = "Interrupt trigger on both sides selection"]
pub mod trigger_both;
#[doc = "trigger_edge_level register accessor: an alias for `Reg<TRIGGER_EDGE_LEVEL_SPEC>`"]
pub type TRIGGER_EDGE_LEVEL = crate::Reg<trigger_edge_level::TRIGGER_EDGE_LEVEL_SPEC>;
#[doc = "Interrupt trigger edge or level type selection"]
pub mod trigger_edge_level;
#[doc = "interrupt_enable register accessor: an alias for `Reg<INTERRUPT_ENABLE_SPEC>`"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
#[doc = "interrupt_state_raw register accessor: an alias for `Reg<INTERRUPT_STATE_RAW_SPEC>`"]
pub type INTERRUPT_STATE_RAW = crate::Reg<interrupt_state_raw::INTERRUPT_STATE_RAW_SPEC>;
#[doc = "Raw interrupt state register"]
pub mod interrupt_state_raw;
#[doc = "interrupt_state register accessor: an alias for `Reg<INTERRUPT_STATE_SPEC>`"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Masked interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_clear register accessor: an alias for `Reg<INTERRUPT_CLEAR_SPEC>`"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear interrupt register"]
pub mod interrupt_clear;
