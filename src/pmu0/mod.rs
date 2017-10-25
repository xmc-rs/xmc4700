use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMU0 Identification Register"]
    pub id: ID,
}
#[doc = "PMU0 Identification Register"]
pub struct ID {
    register: VolatileCell<u32>,
}
#[doc = "PMU0 Identification Register"]
pub mod id;
