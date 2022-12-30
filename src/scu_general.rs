#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    pub id: ID,
    #[doc = "0x04 - Chip ID Register"]
    pub idchip: IDCHIP,
    #[doc = "0x08 - Manufactory ID Register"]
    pub idmanuf: IDMANUF,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Startup Configuration Register"]
    pub stcon: STCON,
    _reserved4: [u8; 0x18],
    #[doc = "0x2c - General Purpose Register 0"]
    pub gpr0: GPR0,
    #[doc = "0x30 - General Purpose Register 1"]
    pub gpr1: GPR1,
    _reserved6: [u8; 0x18],
    #[doc = "0x4c - CCU Control Register"]
    pub ccucon: CCUCON,
    _reserved7: [u8; 0x3c],
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    pub dtscon: DTSCON,
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    pub dtsstat: DTSSTAT,
    _reserved9: [u8; 0x08],
    #[doc = "0x9c - SD-MMC Delay Control Register"]
    pub sdmmcdel: SDMMCDEL,
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    pub g0orcen: G0ORCEN,
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    pub g1orcen: G1ORCEN,
    _reserved12: [u8; 0x1c],
    #[doc = "0xc4 - Mirror Write Status Register"]
    pub mirrsts: MIRRSTS,
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    pub rmacr: RMACR,
    #[doc = "0xcc - Retention Memory Access Data Register"]
    pub rmdata: RMDATA,
}
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "IDCHIP (r) register accessor: an alias for `Reg<IDCHIP_SPEC>`"]
pub type IDCHIP = crate::Reg<idchip::IDCHIP_SPEC>;
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "IDMANUF (r) register accessor: an alias for `Reg<IDMANUF_SPEC>`"]
pub type IDMANUF = crate::Reg<idmanuf::IDMANUF_SPEC>;
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "STCON (rw) register accessor: an alias for `Reg<STCON_SPEC>`"]
pub type STCON = crate::Reg<stcon::STCON_SPEC>;
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "GPR0 (rw) register accessor: an alias for `Reg<GPR0_SPEC>`"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: an alias for `Reg<GPR1_SPEC>`"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCUCON (rw) register accessor: an alias for `Reg<CCUCON_SPEC>`"]
pub type CCUCON = crate::Reg<ccucon::CCUCON_SPEC>;
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "DTSCON (rw) register accessor: an alias for `Reg<DTSCON_SPEC>`"]
pub type DTSCON = crate::Reg<dtscon::DTSCON_SPEC>;
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "DTSSTAT (r) register accessor: an alias for `Reg<DTSSTAT_SPEC>`"]
pub type DTSSTAT = crate::Reg<dtsstat::DTSSTAT_SPEC>;
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "SDMMCDEL (rw) register accessor: an alias for `Reg<SDMMCDEL_SPEC>`"]
pub type SDMMCDEL = crate::Reg<sdmmcdel::SDMMCDEL_SPEC>;
#[doc = "SD-MMC Delay Control Register"]
pub mod sdmmcdel;
#[doc = "G0ORCEN (rw) register accessor: an alias for `Reg<G0ORCEN_SPEC>`"]
pub type G0ORCEN = crate::Reg<g0orcen::G0ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "G1ORCEN (rw) register accessor: an alias for `Reg<G1ORCEN_SPEC>`"]
pub type G1ORCEN = crate::Reg<g1orcen::G1ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "MIRRSTS (r) register accessor: an alias for `Reg<MIRRSTS_SPEC>`"]
pub type MIRRSTS = crate::Reg<mirrsts::MIRRSTS_SPEC>;
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "RMACR (rw) register accessor: an alias for `Reg<RMACR_SPEC>`"]
pub type RMACR = crate::Reg<rmacr::RMACR_SPEC>;
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "RMDATA (rw) register accessor: an alias for `Reg<RMDATA_SPEC>`"]
pub type RMDATA = crate::Reg<rmdata::RMDATA_SPEC>;
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
