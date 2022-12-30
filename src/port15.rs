#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 15 Output Register"]
    pub out: OUT,
    #[doc = "0x04 - Port 15 Output Modification Register"]
    pub omr: OMR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Port 15 Input/Output Control Register 0"]
    pub iocr0: IOCR0,
    #[doc = "0x14 - Port 15 Input/Output Control Register 4"]
    pub iocr4: IOCR4,
    #[doc = "0x18 - Port 15 Input/Output Control Register 8"]
    pub iocr8: IOCR8,
    #[doc = "0x1c - Port 15 Input/Output Control Register 12"]
    pub iocr12: IOCR12,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - Port 15 Input Register"]
    pub in_: IN,
    _reserved7: [u8; 0x38],
    #[doc = "0x60 - Port 15 Pin Function Decision Control Register"]
    pub pdisc: PDISC,
    _reserved8: [u8; 0x0c],
    #[doc = "0x70 - Port 15 Pin Power Save Register"]
    pub pps: PPS,
    #[doc = "0x74 - Port 15 Pin Hardware Select Register"]
    pub hwsel: HWSEL,
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Port 15 Output Register"]
pub mod out;
#[doc = "OMR (w) register accessor: an alias for `Reg<OMR_SPEC>`"]
pub type OMR = crate::Reg<omr::OMR_SPEC>;
#[doc = "Port 15 Output Modification Register"]
pub mod omr;
#[doc = "IOCR0 (rw) register accessor: an alias for `Reg<IOCR0_SPEC>`"]
pub type IOCR0 = crate::Reg<iocr0::IOCR0_SPEC>;
#[doc = "Port 15 Input/Output Control Register 0"]
pub mod iocr0;
#[doc = "IOCR4 (rw) register accessor: an alias for `Reg<IOCR4_SPEC>`"]
pub type IOCR4 = crate::Reg<iocr4::IOCR4_SPEC>;
#[doc = "Port 15 Input/Output Control Register 4"]
pub mod iocr4;
#[doc = "IOCR8 (rw) register accessor: an alias for `Reg<IOCR8_SPEC>`"]
pub type IOCR8 = crate::Reg<iocr8::IOCR8_SPEC>;
#[doc = "Port 15 Input/Output Control Register 8"]
pub mod iocr8;
#[doc = "IOCR12 (rw) register accessor: an alias for `Reg<IOCR12_SPEC>`"]
pub type IOCR12 = crate::Reg<iocr12::IOCR12_SPEC>;
#[doc = "Port 15 Input/Output Control Register 12"]
pub mod iocr12;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Port 15 Input Register"]
pub mod in_;
#[doc = "PDISC (rw) register accessor: an alias for `Reg<PDISC_SPEC>`"]
pub type PDISC = crate::Reg<pdisc::PDISC_SPEC>;
#[doc = "Port 15 Pin Function Decision Control Register"]
pub mod pdisc;
#[doc = "PPS (rw) register accessor: an alias for `Reg<PPS_SPEC>`"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Port 15 Pin Power Save Register"]
pub mod pps;
#[doc = "HWSEL (rw) register accessor: an alias for `Reg<HWSEL_SPEC>`"]
pub type HWSEL = crate::Reg<hwsel::HWSEL_SPEC>;
#[doc = "Port 15 Pin Hardware Select Register"]
pub mod hwsel;
