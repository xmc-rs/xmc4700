#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCU Reset Status"]
    pub rststat: RSTSTAT,
    #[doc = "0x04 - RCU Reset Set Register"]
    pub rstset: RSTSET,
    #[doc = "0x08 - RCU Reset Clear Register"]
    pub rstclr: RSTCLR,
    #[doc = "0x0c - RCU Peripheral 0 Reset Status"]
    pub prstat0: PRSTAT0,
    #[doc = "0x10 - RCU Peripheral 0 Reset Set"]
    pub prset0: PRSET0,
    #[doc = "0x14 - RCU Peripheral 0 Reset Clear"]
    pub prclr0: PRCLR0,
    #[doc = "0x18 - RCU Peripheral 1 Reset Status"]
    pub prstat1: PRSTAT1,
    #[doc = "0x1c - RCU Peripheral 1 Reset Set"]
    pub prset1: PRSET1,
    #[doc = "0x20 - RCU Peripheral 1 Reset Clear"]
    pub prclr1: PRCLR1,
    #[doc = "0x24 - RCU Peripheral 2 Reset Status"]
    pub prstat2: PRSTAT2,
    #[doc = "0x28 - RCU Peripheral 2 Reset Set"]
    pub prset2: PRSET2,
    #[doc = "0x2c - RCU Peripheral 2 Reset Clear"]
    pub prclr2: PRCLR2,
    #[doc = "0x30 - RCU Peripheral 3 Reset Status"]
    pub prstat3: PRSTAT3,
    #[doc = "0x34 - RCU Peripheral 3 Reset Set"]
    pub prset3: PRSET3,
    #[doc = "0x38 - RCU Peripheral 3 Reset Clear"]
    pub prclr3: PRCLR3,
}
#[doc = "RSTSTAT (r) register accessor: an alias for `Reg<RSTSTAT_SPEC>`"]
pub type RSTSTAT = crate::Reg<rststat::RSTSTAT_SPEC>;
#[doc = "RCU Reset Status"]
pub mod rststat;
#[doc = "RSTSET (w) register accessor: an alias for `Reg<RSTSET_SPEC>`"]
pub type RSTSET = crate::Reg<rstset::RSTSET_SPEC>;
#[doc = "RCU Reset Set Register"]
pub mod rstset;
#[doc = "RSTCLR (w) register accessor: an alias for `Reg<RSTCLR_SPEC>`"]
pub type RSTCLR = crate::Reg<rstclr::RSTCLR_SPEC>;
#[doc = "RCU Reset Clear Register"]
pub mod rstclr;
#[doc = "PRSTAT0 (r) register accessor: an alias for `Reg<PRSTAT0_SPEC>`"]
pub type PRSTAT0 = crate::Reg<prstat0::PRSTAT0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Status"]
pub mod prstat0;
#[doc = "PRSET0 (w) register accessor: an alias for `Reg<PRSET0_SPEC>`"]
pub type PRSET0 = crate::Reg<prset0::PRSET0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Set"]
pub mod prset0;
#[doc = "PRCLR0 (w) register accessor: an alias for `Reg<PRCLR0_SPEC>`"]
pub type PRCLR0 = crate::Reg<prclr0::PRCLR0_SPEC>;
#[doc = "RCU Peripheral 0 Reset Clear"]
pub mod prclr0;
#[doc = "PRSTAT1 (r) register accessor: an alias for `Reg<PRSTAT1_SPEC>`"]
pub type PRSTAT1 = crate::Reg<prstat1::PRSTAT1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Status"]
pub mod prstat1;
#[doc = "PRSET1 (w) register accessor: an alias for `Reg<PRSET1_SPEC>`"]
pub type PRSET1 = crate::Reg<prset1::PRSET1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Set"]
pub mod prset1;
#[doc = "PRCLR1 (w) register accessor: an alias for `Reg<PRCLR1_SPEC>`"]
pub type PRCLR1 = crate::Reg<prclr1::PRCLR1_SPEC>;
#[doc = "RCU Peripheral 1 Reset Clear"]
pub mod prclr1;
#[doc = "PRSTAT2 (r) register accessor: an alias for `Reg<PRSTAT2_SPEC>`"]
pub type PRSTAT2 = crate::Reg<prstat2::PRSTAT2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Status"]
pub mod prstat2;
#[doc = "PRSET2 (w) register accessor: an alias for `Reg<PRSET2_SPEC>`"]
pub type PRSET2 = crate::Reg<prset2::PRSET2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Set"]
pub mod prset2;
#[doc = "PRCLR2 (w) register accessor: an alias for `Reg<PRCLR2_SPEC>`"]
pub type PRCLR2 = crate::Reg<prclr2::PRCLR2_SPEC>;
#[doc = "RCU Peripheral 2 Reset Clear"]
pub mod prclr2;
#[doc = "PRSTAT3 (r) register accessor: an alias for `Reg<PRSTAT3_SPEC>`"]
pub type PRSTAT3 = crate::Reg<prstat3::PRSTAT3_SPEC>;
#[doc = "RCU Peripheral 3 Reset Status"]
pub mod prstat3;
#[doc = "PRSET3 (w) register accessor: an alias for `Reg<PRSET3_SPEC>`"]
pub type PRSET3 = crate::Reg<prset3::PRSET3_SPEC>;
#[doc = "RCU Peripheral 3 Reset Set"]
pub mod prset3;
#[doc = "PRCLR3 (w) register accessor: an alias for `Reg<PRCLR3_SPEC>`"]
pub type PRCLR3 = crate::Reg<prclr3::PRCLR3_SPEC>;
#[doc = "RCU Peripheral 3 Reset Clear"]
pub mod prclr3;
