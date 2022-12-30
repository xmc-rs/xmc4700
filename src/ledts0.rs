#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Identification Register"]
    pub id: ID,
    #[doc = "0x04 - Global Control Register"]
    pub globctl: GLOBCTL,
    #[doc = "0x08 - Function Control Register"]
    pub fnctl: FNCTL,
    #[doc = "0x0c - Event Flag Register"]
    pub evfr: EVFR,
    #[doc = "0x10 - Touch-sense TS-Counter Value"]
    pub tsval: TSVAL,
    #[doc = "0x14 - Line Pattern Register 0"]
    pub line0: LINE0,
    #[doc = "0x18 - Line Pattern Register 1"]
    pub line1: LINE1,
    #[doc = "0x1c - LED Compare Register 0"]
    pub ldcmp0: LDCMP0,
    #[doc = "0x20 - LED Compare Register 1"]
    pub ldcmp1: LDCMP1,
    #[doc = "0x24 - Touch-sense Compare Register 0"]
    pub tscmp0: TSCMP0,
    #[doc = "0x28 - Touch-sense Compare Register 1"]
    pub tscmp1: TSCMP1,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Module Identification Register"]
pub mod id;
#[doc = "GLOBCTL (rw) register accessor: an alias for `Reg<GLOBCTL_SPEC>`"]
pub type GLOBCTL = crate::Reg<globctl::GLOBCTL_SPEC>;
#[doc = "Global Control Register"]
pub mod globctl;
#[doc = "FNCTL (rw) register accessor: an alias for `Reg<FNCTL_SPEC>`"]
pub type FNCTL = crate::Reg<fnctl::FNCTL_SPEC>;
#[doc = "Function Control Register"]
pub mod fnctl;
#[doc = "EVFR (rw) register accessor: an alias for `Reg<EVFR_SPEC>`"]
pub type EVFR = crate::Reg<evfr::EVFR_SPEC>;
#[doc = "Event Flag Register"]
pub mod evfr;
#[doc = "TSVAL (rw) register accessor: an alias for `Reg<TSVAL_SPEC>`"]
pub type TSVAL = crate::Reg<tsval::TSVAL_SPEC>;
#[doc = "Touch-sense TS-Counter Value"]
pub mod tsval;
#[doc = "LINE0 (rw) register accessor: an alias for `Reg<LINE0_SPEC>`"]
pub type LINE0 = crate::Reg<line0::LINE0_SPEC>;
#[doc = "Line Pattern Register 0"]
pub mod line0;
#[doc = "LINE1 (rw) register accessor: an alias for `Reg<LINE1_SPEC>`"]
pub type LINE1 = crate::Reg<line1::LINE1_SPEC>;
#[doc = "Line Pattern Register 1"]
pub mod line1;
#[doc = "LDCMP0 (rw) register accessor: an alias for `Reg<LDCMP0_SPEC>`"]
pub type LDCMP0 = crate::Reg<ldcmp0::LDCMP0_SPEC>;
#[doc = "LED Compare Register 0"]
pub mod ldcmp0;
#[doc = "LDCMP1 (rw) register accessor: an alias for `Reg<LDCMP1_SPEC>`"]
pub type LDCMP1 = crate::Reg<ldcmp1::LDCMP1_SPEC>;
#[doc = "LED Compare Register 1"]
pub mod ldcmp1;
#[doc = "TSCMP0 (rw) register accessor: an alias for `Reg<TSCMP0_SPEC>`"]
pub type TSCMP0 = crate::Reg<tscmp0::TSCMP0_SPEC>;
#[doc = "Touch-sense Compare Register 0"]
pub mod tscmp0;
#[doc = "TSCMP1 (rw) register accessor: an alias for `Reg<TSCMP1_SPEC>`"]
pub type TSCMP1 = crate::Reg<tscmp1::TSCMP1_SPEC>;
#[doc = "Touch-sense Compare Register 1"]
pub mod tscmp1;
