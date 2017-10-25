# ! [ cfg_attr ( feature = "rt" , feature ( global_asm ) ) ] # ! [ cfg_attr ( feature = "rt" , feature ( macro_reexport ) ) ] # ! [ cfg_attr ( feature = "rt" , feature ( used ) ) ] # ! [ doc = "Peripheral access API for XMC4700 microcontrollers (generated using svd2rust v0.11.4)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.11.4/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ feature ( const_fn ) ] # ! [ no_std ]extern crate cortex_m ;
#[macro_reexport(default_handler, exception)]
#[cfg(feature = "rt")]
extern crate cortex_m_rt ;
extern crate bare_metal ;
extern crate vcell ;
use core::ops::Deref;
use bare_metal::Peripheral;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 6;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::FPB;
pub use cortex_m::peripheral::FPU;
pub use cortex_m::peripheral::ITM;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
pub use cortex_m::peripheral::TPIU;
#[doc = "Cortex-M4 Private Peripheral Block"]
pub const PPB: Peripheral<PPB> = unsafe { Peripheral::new(3758153728) };
#[doc = "Cortex-M4 Private Peripheral Block"]
pub mod ppb;
#[doc = "Cortex-M4 Private Peripheral Block"]
pub struct PPB {
    register_block: ppb::RegisterBlock,
}
impl Deref for PPB {
    type Target = ppb::RegisterBlock;
    fn deref(&self) -> &ppb::RegisterBlock {
        &self.register_block
    }
}
#[doc = "DMA Line Router"]
pub const DLR: Peripheral<DLR> = unsafe { Peripheral::new(1342195968) };
#[doc = "DMA Line Router"]
pub mod dlr;
#[doc = "DMA Line Router"]
pub struct DLR {
    register_block: dlr::RegisterBlock,
}
impl Deref for DLR {
    type Target = dlr::RegisterBlock;
    fn deref(&self) -> &dlr::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Event Request Unit 0"]
pub const ERU0: Peripheral<ERU0> = unsafe { Peripheral::new(1342195712) };
#[doc = "Event Request Unit 0"]
pub mod eru0;
#[doc = "Event Request Unit 0"]
pub struct ERU0 {
    register_block: eru0::RegisterBlock,
}
impl Deref for ERU0 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &eru0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Event Request Unit 1"]
pub const ERU1: Peripheral<ERU1> = unsafe { Peripheral::new(1074020352) };
#[doc = r" Register block"]
pub struct ERU1 {
    register_block: eru0::RegisterBlock,
}
impl Deref for ERU1 {
    type Target = eru0::RegisterBlock;
    fn deref(&self) -> &eru0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0: Peripheral<GPDMA0> = unsafe { Peripheral::new(1342259904) };
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0 {
    register_block: gpdma0::RegisterBlock,
}
impl Deref for GPDMA0 {
    type Target = gpdma0::RegisterBlock;
    fn deref(&self) -> &gpdma0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH0: Peripheral<GPDMA0_CH0> = unsafe { Peripheral::new(1342259200) };
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch0;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH0 {
    register_block: gpdma0_ch0::RegisterBlock,
}
impl Deref for GPDMA0_CH0 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH1: Peripheral<GPDMA0_CH1> = unsafe { Peripheral::new(1342259288) };
#[doc = r" Register block"]
pub struct GPDMA0_CH1 {
    register_block: gpdma0_ch0::RegisterBlock,
}
impl Deref for GPDMA0_CH1 {
    type Target = gpdma0_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH2: Peripheral<GPDMA0_CH2> = unsafe { Peripheral::new(1342259376) };
#[doc = "General Purpose DMA Unit 0"]
pub mod gpdma0_ch2;
#[doc = "General Purpose DMA Unit 0"]
pub struct GPDMA0_CH2 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH2 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH3: Peripheral<GPDMA0_CH3> = unsafe { Peripheral::new(1342259464) };
#[doc = r" Register block"]
pub struct GPDMA0_CH3 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH3 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH4: Peripheral<GPDMA0_CH4> = unsafe { Peripheral::new(1342259552) };
#[doc = r" Register block"]
pub struct GPDMA0_CH4 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH4 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH5: Peripheral<GPDMA0_CH5> = unsafe { Peripheral::new(1342259640) };
#[doc = r" Register block"]
pub struct GPDMA0_CH5 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH5 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH6: Peripheral<GPDMA0_CH6> = unsafe { Peripheral::new(1342259728) };
#[doc = r" Register block"]
pub struct GPDMA0_CH6 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH6 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 0"]
pub const GPDMA0_CH7: Peripheral<GPDMA0_CH7> = unsafe { Peripheral::new(1342259816) };
#[doc = r" Register block"]
pub struct GPDMA0_CH7 {
    register_block: gpdma0_ch2::RegisterBlock,
}
impl Deref for GPDMA0_CH7 {
    type Target = gpdma0_ch2::RegisterBlock;
    fn deref(&self) -> &gpdma0_ch2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 1"]
pub const GPDMA1: Peripheral<GPDMA1> = unsafe { Peripheral::new(1342276288) };
#[doc = "General Purpose DMA Unit 1"]
pub mod gpdma1;
#[doc = "General Purpose DMA Unit 1"]
pub struct GPDMA1 {
    register_block: gpdma1::RegisterBlock,
}
impl Deref for GPDMA1 {
    type Target = gpdma1::RegisterBlock;
    fn deref(&self) -> &gpdma1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 1"]
pub const GPDMA1_CH0: Peripheral<GPDMA1_CH0> = unsafe { Peripheral::new(1342275584) };
#[doc = "General Purpose DMA Unit 1"]
pub mod gpdma1_ch0;
#[doc = "General Purpose DMA Unit 1"]
pub struct GPDMA1_CH0 {
    register_block: gpdma1_ch0::RegisterBlock,
}
impl Deref for GPDMA1_CH0 {
    type Target = gpdma1_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma1_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 1"]
pub const GPDMA1_CH1: Peripheral<GPDMA1_CH1> = unsafe { Peripheral::new(1342275672) };
#[doc = r" Register block"]
pub struct GPDMA1_CH1 {
    register_block: gpdma1_ch0::RegisterBlock,
}
impl Deref for GPDMA1_CH1 {
    type Target = gpdma1_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma1_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 1"]
pub const GPDMA1_CH2: Peripheral<GPDMA1_CH2> = unsafe { Peripheral::new(1342275760) };
#[doc = r" Register block"]
pub struct GPDMA1_CH2 {
    register_block: gpdma1_ch0::RegisterBlock,
}
impl Deref for GPDMA1_CH2 {
    type Target = gpdma1_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma1_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "General Purpose DMA Unit 1"]
pub const GPDMA1_CH3: Peripheral<GPDMA1_CH3> = unsafe { Peripheral::new(1342275848) };
#[doc = r" Register block"]
pub struct GPDMA1_CH3 {
    register_block: gpdma1_ch0::RegisterBlock,
}
impl Deref for GPDMA1_CH3 {
    type Target = gpdma1_ch0::RegisterBlock;
    fn deref(&self) -> &gpdma1_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible CRC Engine"]
pub const FCE: Peripheral<FCE> = unsafe { Peripheral::new(1342308352) };
#[doc = "Flexible CRC Engine"]
pub mod fce;
#[doc = "Flexible CRC Engine"]
pub struct FCE {
    register_block: fce::RegisterBlock,
}
impl Deref for FCE {
    type Target = fce::RegisterBlock;
    fn deref(&self) -> &fce::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible CRC Engine"]
pub const FCE_KE0: Peripheral<FCE_KE0> = unsafe { Peripheral::new(1342308384) };
#[doc = "Flexible CRC Engine"]
pub mod fce_ke0;
#[doc = "Flexible CRC Engine"]
pub struct FCE_KE0 {
    register_block: fce_ke0::RegisterBlock,
}
impl Deref for FCE_KE0 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible CRC Engine"]
pub const FCE_KE1: Peripheral<FCE_KE1> = unsafe { Peripheral::new(1342308416) };
#[doc = r" Register block"]
pub struct FCE_KE1 {
    register_block: fce_ke0::RegisterBlock,
}
impl Deref for FCE_KE1 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible CRC Engine"]
pub const FCE_KE2: Peripheral<FCE_KE2> = unsafe { Peripheral::new(1342308448) };
#[doc = r" Register block"]
pub struct FCE_KE2 {
    register_block: fce_ke0::RegisterBlock,
}
impl Deref for FCE_KE2 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flexible CRC Engine"]
pub const FCE_KE3: Peripheral<FCE_KE3> = unsafe { Peripheral::new(1342308480) };
#[doc = r" Register block"]
pub struct FCE_KE3 {
    register_block: fce_ke0::RegisterBlock,
}
impl Deref for FCE_KE3 {
    type Target = fce_ke0::RegisterBlock;
    fn deref(&self) -> &fce_ke0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Peripheral Bridge AHB 0"]
pub const PBA0: Peripheral<PBA0> = unsafe { Peripheral::new(1073741824) };
#[doc = "Peripheral Bridge AHB 0"]
pub mod pba0;
#[doc = "Peripheral Bridge AHB 0"]
pub struct PBA0 {
    register_block: pba0::RegisterBlock,
}
impl Deref for PBA0 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &pba0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Peripheral Bridge AHB 1"]
pub const PBA1: Peripheral<PBA1> = unsafe { Peripheral::new(1207959552) };
#[doc = r" Register block"]
pub struct PBA1 {
    register_block: pba0::RegisterBlock,
}
impl Deref for PBA1 {
    type Target = pba0::RegisterBlock;
    fn deref(&self) -> &pba0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Flash Memory Controller"]
pub const FLASH0: Peripheral<FLASH0> = unsafe { Peripheral::new(1476399104) };
#[doc = "Flash Memory Controller"]
pub mod flash0;
#[doc = "Flash Memory Controller"]
pub struct FLASH0 {
    register_block: flash0::RegisterBlock,
}
impl Deref for FLASH0 {
    type Target = flash0::RegisterBlock;
    fn deref(&self) -> &flash0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Prefetch Unit"]
pub const PREF: Peripheral<PREF> = unsafe { Peripheral::new(1476411392) };
#[doc = "Prefetch Unit"]
pub mod pref;
#[doc = "Prefetch Unit"]
pub struct PREF {
    register_block: pref::RegisterBlock,
}
impl Deref for PREF {
    type Target = pref::RegisterBlock;
    fn deref(&self) -> &pref::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Program Management Unit"]
pub const PMU0: Peripheral<PMU0> = unsafe { Peripheral::new(1476396296) };
#[doc = "Program Management Unit"]
pub mod pmu0;
#[doc = "Program Management Unit"]
pub struct PMU0 {
    register_block: pmu0::RegisterBlock,
}
impl Deref for PMU0 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Watch Dog Timer"]
pub const WDT: Peripheral<WDT> = unsafe { Peripheral::new(1342210048) };
#[doc = "Watch Dog Timer"]
pub mod wdt;
#[doc = "Watch Dog Timer"]
pub struct WDT {
    register_block: wdt::RegisterBlock,
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Real Time Clock"]
pub const RTC: Peripheral<RTC> = unsafe { Peripheral::new(1342196224) };
#[doc = "Real Time Clock"]
pub mod rtc;
#[doc = "Real Time Clock"]
pub struct RTC {
    register_block: rtc::RegisterBlock,
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_CLK: Peripheral<SCU_CLK> = unsafe { Peripheral::new(1342195200) };
#[doc = "System Control Unit"]
pub mod scu_clk;
#[doc = "System Control Unit"]
pub struct SCU_CLK {
    register_block: scu_clk::RegisterBlock,
}
impl Deref for SCU_CLK {
    type Target = scu_clk::RegisterBlock;
    fn deref(&self) -> &scu_clk::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_OSC: Peripheral<SCU_OSC> = unsafe { Peripheral::new(1342195456) };
#[doc = "System Control Unit"]
pub mod scu_osc;
#[doc = "System Control Unit"]
pub struct SCU_OSC {
    register_block: scu_osc::RegisterBlock,
}
impl Deref for SCU_OSC {
    type Target = scu_osc::RegisterBlock;
    fn deref(&self) -> &scu_osc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_PLL: Peripheral<SCU_PLL> = unsafe { Peripheral::new(1342195472) };
#[doc = "System Control Unit"]
pub mod scu_pll;
#[doc = "System Control Unit"]
pub struct SCU_PLL {
    register_block: scu_pll::RegisterBlock,
}
impl Deref for SCU_PLL {
    type Target = scu_pll::RegisterBlock;
    fn deref(&self) -> &scu_pll::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_GENERAL: Peripheral<SCU_GENERAL> = unsafe { Peripheral::new(1342193664) };
#[doc = "System Control Unit"]
pub mod scu_general;
#[doc = "System Control Unit"]
pub struct SCU_GENERAL {
    register_block: scu_general::RegisterBlock,
}
impl Deref for SCU_GENERAL {
    type Target = scu_general::RegisterBlock;
    fn deref(&self) -> &scu_general::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_INTERRUPT: Peripheral<SCU_INTERRUPT> = unsafe { Peripheral::new(1342193780) };
#[doc = "System Control Unit"]
pub mod scu_interrupt;
#[doc = "System Control Unit"]
pub struct SCU_INTERRUPT {
    register_block: scu_interrupt::RegisterBlock,
}
impl Deref for SCU_INTERRUPT {
    type Target = scu_interrupt::RegisterBlock;
    fn deref(&self) -> &scu_interrupt::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_PARITY: Peripheral<SCU_PARITY> = unsafe { Peripheral::new(1342193980) };
#[doc = "System Control Unit"]
pub mod scu_parity;
#[doc = "System Control Unit"]
pub struct SCU_PARITY {
    register_block: scu_parity::RegisterBlock,
}
impl Deref for SCU_PARITY {
    type Target = scu_parity::RegisterBlock;
    fn deref(&self) -> &scu_parity::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_TRAP: Peripheral<SCU_TRAP> = unsafe { Peripheral::new(1342194016) };
#[doc = "System Control Unit"]
pub mod scu_trap;
#[doc = "System Control Unit"]
pub struct SCU_TRAP {
    register_block: scu_trap::RegisterBlock,
}
impl Deref for SCU_TRAP {
    type Target = scu_trap::RegisterBlock;
    fn deref(&self) -> &scu_trap::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_HIBERNATE: Peripheral<SCU_HIBERNATE> = unsafe { Peripheral::new(1342194432) };
#[doc = "System Control Unit"]
pub mod scu_hibernate;
#[doc = "System Control Unit"]
pub struct SCU_HIBERNATE {
    register_block: scu_hibernate::RegisterBlock,
}
impl Deref for SCU_HIBERNATE {
    type Target = scu_hibernate::RegisterBlock;
    fn deref(&self) -> &scu_hibernate::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_POWER: Peripheral<SCU_POWER> = unsafe { Peripheral::new(1342194176) };
#[doc = "System Control Unit"]
pub mod scu_power;
#[doc = "System Control Unit"]
pub struct SCU_POWER {
    register_block: scu_power::RegisterBlock,
}
impl Deref for SCU_POWER {
    type Target = scu_power::RegisterBlock;
    fn deref(&self) -> &scu_power::RegisterBlock {
        &self.register_block
    }
}
#[doc = "System Control Unit"]
pub const SCU_RESET: Peripheral<SCU_RESET> = unsafe { Peripheral::new(1342194688) };
#[doc = "System Control Unit"]
pub mod scu_reset;
#[doc = "System Control Unit"]
pub struct SCU_RESET {
    register_block: scu_reset::RegisterBlock,
}
impl Deref for SCU_RESET {
    type Target = scu_reset::RegisterBlock;
    fn deref(&self) -> &scu_reset::RegisterBlock {
        &self.register_block
    }
}
#[doc = "LED and Touch Sense Unit 0"]
pub const LEDTS0: Peripheral<LEDTS0> = unsafe { Peripheral::new(1208025088) };
#[doc = "LED and Touch Sense Unit 0"]
pub mod ledts0;
#[doc = "LED and Touch Sense Unit 0"]
pub struct LEDTS0 {
    register_block: ledts0::RegisterBlock,
}
impl Deref for LEDTS0 {
    type Target = ledts0::RegisterBlock;
    fn deref(&self) -> &ledts0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SD and Multimediacard Control Register"]
pub const SDMMC_CON: Peripheral<SDMMC_CON> = unsafe { Peripheral::new(1342193844) };
#[doc = "SD and Multimediacard Control Register"]
pub mod sdmmc_con;
#[doc = "SD and Multimediacard Control Register"]
pub struct SDMMC_CON {
    register_block: sdmmc_con::RegisterBlock,
}
impl Deref for SDMMC_CON {
    type Target = sdmmc_con::RegisterBlock;
    fn deref(&self) -> &sdmmc_con::RegisterBlock {
        &self.register_block
    }
}
#[doc = "SD and Multimediacard Interface"]
pub const SDMMC: Peripheral<SDMMC> = unsafe { Peripheral::new(1208074240) };
#[doc = "SD and Multimediacard Interface"]
pub mod sdmmc;
#[doc = "SD and Multimediacard Interface"]
pub struct SDMMC {
    register_block: sdmmc::RegisterBlock,
}
impl Deref for SDMMC {
    type Target = sdmmc::RegisterBlock;
    fn deref(&self) -> &sdmmc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "External Bus Unit"]
pub const EBU: Peripheral<EBU> = unsafe { Peripheral::new(1476427776) };
#[doc = "External Bus Unit"]
pub mod ebu;
#[doc = "External Bus Unit"]
pub struct EBU {
    register_block: ebu::RegisterBlock,
}
impl Deref for EBU {
    type Target = ebu::RegisterBlock;
    fn deref(&self) -> &ebu::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet Control Register"]
pub const ETH0_CON: Peripheral<ETH0_CON> = unsafe { Peripheral::new(1342193728) };
#[doc = "Ethernet Control Register"]
pub mod eth0_con;
#[doc = "Ethernet Control Register"]
pub struct ETH0_CON {
    register_block: eth0_con::RegisterBlock,
}
impl Deref for ETH0_CON {
    type Target = eth0_con::RegisterBlock;
    fn deref(&self) -> &eth0_con::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Ethernet Unit 0"]
pub const ETH0: Peripheral<ETH0> = unsafe { Peripheral::new(1342226432) };
#[doc = "Ethernet Unit 0"]
pub mod eth0;
#[doc = "Ethernet Unit 0"]
pub struct ETH0 {
    register_block: eth0::RegisterBlock,
}
impl Deref for ETH0 {
    type Target = eth0::RegisterBlock;
    fn deref(&self) -> &eth0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0: Peripheral<USB0> = unsafe { Peripheral::new(1342439424) };
#[doc = "Universal Serial Bus"]
pub mod usb0;
#[doc = "Universal Serial Bus"]
pub struct USB0 {
    register_block: usb0::RegisterBlock,
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &usb0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP0: Peripheral<USB0_EP0> = unsafe { Peripheral::new(1342441728) };
#[doc = "Universal Serial Bus"]
pub mod usb0_ep0;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP0 {
    register_block: usb0_ep0::RegisterBlock,
}
impl Deref for USB0_EP0 {
    type Target = usb0_ep0::RegisterBlock;
    fn deref(&self) -> &usb0_ep0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP1: Peripheral<USB0_EP1> = unsafe { Peripheral::new(1342441760) };
#[doc = "Universal Serial Bus"]
pub mod usb0_ep1;
#[doc = "Universal Serial Bus"]
pub struct USB0_EP1 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP1 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP2: Peripheral<USB0_EP2> = unsafe { Peripheral::new(1342441792) };
#[doc = r" Register block"]
pub struct USB0_EP2 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP2 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP3: Peripheral<USB0_EP3> = unsafe { Peripheral::new(1342441824) };
#[doc = r" Register block"]
pub struct USB0_EP3 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP3 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP4: Peripheral<USB0_EP4> = unsafe { Peripheral::new(1342441856) };
#[doc = r" Register block"]
pub struct USB0_EP4 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP4 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP5: Peripheral<USB0_EP5> = unsafe { Peripheral::new(1342441888) };
#[doc = r" Register block"]
pub struct USB0_EP5 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP5 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_EP6: Peripheral<USB0_EP6> = unsafe { Peripheral::new(1342441920) };
#[doc = r" Register block"]
pub struct USB0_EP6 {
    register_block: usb0_ep1::RegisterBlock,
}
impl Deref for USB0_EP6 {
    type Target = usb0_ep1::RegisterBlock;
    fn deref(&self) -> &usb0_ep1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH0: Peripheral<USB0_CH0> = unsafe { Peripheral::new(1342440704) };
#[doc = "Universal Serial Bus"]
pub mod usb0_ch0;
#[doc = "Universal Serial Bus"]
pub struct USB0_CH0 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH0 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH1: Peripheral<USB0_CH1> = unsafe { Peripheral::new(1342440736) };
#[doc = r" Register block"]
pub struct USB0_CH1 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH1 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH2: Peripheral<USB0_CH2> = unsafe { Peripheral::new(1342440768) };
#[doc = r" Register block"]
pub struct USB0_CH2 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH2 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH3: Peripheral<USB0_CH3> = unsafe { Peripheral::new(1342440800) };
#[doc = r" Register block"]
pub struct USB0_CH3 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH3 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH4: Peripheral<USB0_CH4> = unsafe { Peripheral::new(1342440832) };
#[doc = r" Register block"]
pub struct USB0_CH4 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH4 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH5: Peripheral<USB0_CH5> = unsafe { Peripheral::new(1342440864) };
#[doc = r" Register block"]
pub struct USB0_CH5 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH5 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH6: Peripheral<USB0_CH6> = unsafe { Peripheral::new(1342440896) };
#[doc = r" Register block"]
pub struct USB0_CH6 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH6 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH7: Peripheral<USB0_CH7> = unsafe { Peripheral::new(1342440928) };
#[doc = r" Register block"]
pub struct USB0_CH7 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH7 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH8: Peripheral<USB0_CH8> = unsafe { Peripheral::new(1342440960) };
#[doc = r" Register block"]
pub struct USB0_CH8 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH8 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH9: Peripheral<USB0_CH9> = unsafe { Peripheral::new(1342440992) };
#[doc = r" Register block"]
pub struct USB0_CH9 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH9 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH10: Peripheral<USB0_CH10> = unsafe { Peripheral::new(1342441024) };
#[doc = r" Register block"]
pub struct USB0_CH10 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH10 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH11: Peripheral<USB0_CH11> = unsafe { Peripheral::new(1342441056) };
#[doc = r" Register block"]
pub struct USB0_CH11 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH11 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH12: Peripheral<USB0_CH12> = unsafe { Peripheral::new(1342441088) };
#[doc = r" Register block"]
pub struct USB0_CH12 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH12 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Bus"]
pub const USB0_CH13: Peripheral<USB0_CH13> = unsafe { Peripheral::new(1342441120) };
#[doc = r" Register block"]
pub struct USB0_CH13 {
    register_block: usb0_ch0::RegisterBlock,
}
impl Deref for USB0_CH13 {
    type Target = usb0_ch0::RegisterBlock;
    fn deref(&self) -> &usb0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC0: Peripheral<USIC0> = unsafe { Peripheral::new(1073938440) };
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0 {
    register_block: usic0::RegisterBlock,
}
impl Deref for USIC0 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 1"]
pub const USIC1: Peripheral<USIC1> = unsafe { Peripheral::new(1208090632) };
#[doc = r" Register block"]
pub struct USIC1 {
    register_block: usic0::RegisterBlock,
}
impl Deref for USIC1 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 2"]
pub const USIC2: Peripheral<USIC2> = unsafe { Peripheral::new(1208107016) };
#[doc = r" Register block"]
pub struct USIC2 {
    register_block: usic0::RegisterBlock,
}
impl Deref for USIC2 {
    type Target = usic0::RegisterBlock;
    fn deref(&self) -> &usic0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC0_CH0: Peripheral<USIC0_CH0> = unsafe { Peripheral::new(1073938432) };
#[doc = "Universal Serial Interface Controller 0"]
pub mod usic0_ch0;
#[doc = "Universal Serial Interface Controller 0"]
pub struct USIC0_CH0 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC0_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC0_CH1: Peripheral<USIC0_CH1> = unsafe { Peripheral::new(1073938944) };
#[doc = r" Register block"]
pub struct USIC0_CH1 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC0_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC1_CH0: Peripheral<USIC1_CH0> = unsafe { Peripheral::new(1208090624) };
#[doc = r" Register block"]
pub struct USIC1_CH0 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC1_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC1_CH1: Peripheral<USIC1_CH1> = unsafe { Peripheral::new(1208091136) };
#[doc = r" Register block"]
pub struct USIC1_CH1 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC1_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC2_CH0: Peripheral<USIC2_CH0> = unsafe { Peripheral::new(1208107008) };
#[doc = r" Register block"]
pub struct USIC2_CH0 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC2_CH0 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Universal Serial Interface Controller 0"]
pub const USIC2_CH1: Peripheral<USIC2_CH1> = unsafe { Peripheral::new(1208107520) };
#[doc = r" Register block"]
pub struct USIC2_CH1 {
    register_block: usic0_ch0::RegisterBlock,
}
impl Deref for USIC2_CH1 {
    type Target = usic0_ch0::RegisterBlock;
    fn deref(&self) -> &usic0_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN: Peripheral<CAN> = unsafe { Peripheral::new(1208041472) };
#[doc = "Controller Area Networks"]
pub mod can;
#[doc = "Controller Area Networks"]
pub struct CAN {
    register_block: can::RegisterBlock,
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE0: Peripheral<CAN_NODE0> = unsafe { Peripheral::new(1208041984) };
#[doc = "Controller Area Networks"]
pub mod can_node0;
#[doc = "Controller Area Networks"]
pub struct CAN_NODE0 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE0 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE1: Peripheral<CAN_NODE1> = unsafe { Peripheral::new(1208042240) };
#[doc = r" Register block"]
pub struct CAN_NODE1 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE1 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE2: Peripheral<CAN_NODE2> = unsafe { Peripheral::new(1208042496) };
#[doc = r" Register block"]
pub struct CAN_NODE2 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE2 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE3: Peripheral<CAN_NODE3> = unsafe { Peripheral::new(1208042752) };
#[doc = r" Register block"]
pub struct CAN_NODE3 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE3 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE4: Peripheral<CAN_NODE4> = unsafe { Peripheral::new(1208043008) };
#[doc = r" Register block"]
pub struct CAN_NODE4 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE4 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Controller Area Networks"]
pub const CAN_NODE5: Peripheral<CAN_NODE5> = unsafe { Peripheral::new(1208043264) };
#[doc = r" Register block"]
pub struct CAN_NODE5 {
    register_block: can_node0::RegisterBlock,
}
impl Deref for CAN_NODE5 {
    type Target = can_node0::RegisterBlock;
    fn deref(&self) -> &can_node0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog to Digital Converter"]
pub const VADC: Peripheral<VADC> = unsafe { Peripheral::new(1073758208) };
#[doc = "Analog to Digital Converter"]
pub mod vadc;
#[doc = "Analog to Digital Converter"]
pub struct VADC {
    register_block: vadc::RegisterBlock,
}
impl Deref for VADC {
    type Target = vadc::RegisterBlock;
    fn deref(&self) -> &vadc::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog to Digital Converter"]
pub const VADC_G0: Peripheral<VADC_G0> = unsafe { Peripheral::new(1073759232) };
#[doc = "Analog to Digital Converter"]
pub mod vadc_g0;
#[doc = "Analog to Digital Converter"]
pub struct VADC_G0 {
    register_block: vadc_g0::RegisterBlock,
}
impl Deref for VADC_G0 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog to Digital Converter"]
pub const VADC_G1: Peripheral<VADC_G1> = unsafe { Peripheral::new(1073760256) };
#[doc = r" Register block"]
pub struct VADC_G1 {
    register_block: vadc_g0::RegisterBlock,
}
impl Deref for VADC_G1 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog to Digital Converter"]
pub const VADC_G2: Peripheral<VADC_G2> = unsafe { Peripheral::new(1073761280) };
#[doc = r" Register block"]
pub struct VADC_G2 {
    register_block: vadc_g0::RegisterBlock,
}
impl Deref for VADC_G2 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Analog to Digital Converter"]
pub const VADC_G3: Peripheral<VADC_G3> = unsafe { Peripheral::new(1073762304) };
#[doc = r" Register block"]
pub struct VADC_G3 {
    register_block: vadc_g0::RegisterBlock,
}
impl Deref for VADC_G3 {
    type Target = vadc_g0::RegisterBlock;
    fn deref(&self) -> &vadc_g0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Delta Sigma Demodulator"]
pub const DSD: Peripheral<DSD> = unsafe { Peripheral::new(1073774592) };
#[doc = "Delta Sigma Demodulator"]
pub mod dsd;
#[doc = "Delta Sigma Demodulator"]
pub struct DSD {
    register_block: dsd::RegisterBlock,
}
impl Deref for DSD {
    type Target = dsd::RegisterBlock;
    fn deref(&self) -> &dsd::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Delta Sigma Demodulator"]
pub const DSD_CH0: Peripheral<DSD_CH0> = unsafe { Peripheral::new(1073774848) };
#[doc = "Delta Sigma Demodulator"]
pub mod dsd_ch0;
#[doc = "Delta Sigma Demodulator"]
pub struct DSD_CH0 {
    register_block: dsd_ch0::RegisterBlock,
}
impl Deref for DSD_CH0 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Delta Sigma Demodulator"]
pub const DSD_CH1: Peripheral<DSD_CH1> = unsafe { Peripheral::new(1073775104) };
#[doc = r" Register block"]
pub struct DSD_CH1 {
    register_block: dsd_ch0::RegisterBlock,
}
impl Deref for DSD_CH1 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Delta Sigma Demodulator"]
pub const DSD_CH2: Peripheral<DSD_CH2> = unsafe { Peripheral::new(1073775360) };
#[doc = r" Register block"]
pub struct DSD_CH2 {
    register_block: dsd_ch0::RegisterBlock,
}
impl Deref for DSD_CH2 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Delta Sigma Demodulator"]
pub const DSD_CH3: Peripheral<DSD_CH3> = unsafe { Peripheral::new(1073775616) };
#[doc = r" Register block"]
pub struct DSD_CH3 {
    register_block: dsd_ch0::RegisterBlock,
}
impl Deref for DSD_CH3 {
    type Target = dsd_ch0::RegisterBlock;
    fn deref(&self) -> &dsd_ch0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Digital to Analog Converter"]
pub const DAC: Peripheral<DAC> = unsafe { Peripheral::new(1208057856) };
#[doc = "Digital to Analog Converter"]
pub mod dac;
#[doc = "Digital to Analog Converter"]
pub struct DAC {
    register_block: dac::RegisterBlock,
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub const CCU40: Peripheral<CCU40> = unsafe { Peripheral::new(1073790976) };
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40 {
    register_block: ccu40::RegisterBlock,
}
impl Deref for CCU40 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub const CCU41: Peripheral<CCU41> = unsafe { Peripheral::new(1073807360) };
#[doc = r" Register block"]
pub struct CCU41 {
    register_block: ccu40::RegisterBlock,
}
impl Deref for CCU41 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub const CCU42: Peripheral<CCU42> = unsafe { Peripheral::new(1073823744) };
#[doc = r" Register block"]
pub struct CCU42 {
    register_block: ccu40::RegisterBlock,
}
impl Deref for CCU42 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub const CCU43: Peripheral<CCU43> = unsafe { Peripheral::new(1207975936) };
#[doc = r" Register block"]
pub struct CCU43 {
    register_block: ccu40::RegisterBlock,
}
impl Deref for CCU43 {
    type Target = ccu40::RegisterBlock;
    fn deref(&self) -> &ccu40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub const CCU40_CC40: Peripheral<CCU40_CC40> = unsafe { Peripheral::new(1073791232) };
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub mod ccu40_cc40;
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub struct CCU40_CC40 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU40_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub const CCU40_CC41: Peripheral<CCU40_CC41> = unsafe { Peripheral::new(1073791488) };
#[doc = r" Register block"]
pub struct CCU40_CC41 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU40_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub const CCU40_CC42: Peripheral<CCU40_CC42> = unsafe { Peripheral::new(1073791744) };
#[doc = r" Register block"]
pub struct CCU40_CC42 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU40_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 0"]
pub const CCU40_CC43: Peripheral<CCU40_CC43> = unsafe { Peripheral::new(1073792000) };
#[doc = r" Register block"]
pub struct CCU40_CC43 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU40_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub const CCU41_CC40: Peripheral<CCU41_CC40> = unsafe { Peripheral::new(1073807616) };
#[doc = r" Register block"]
pub struct CCU41_CC40 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU41_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub const CCU41_CC41: Peripheral<CCU41_CC41> = unsafe { Peripheral::new(1073807872) };
#[doc = r" Register block"]
pub struct CCU41_CC41 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU41_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub const CCU41_CC42: Peripheral<CCU41_CC42> = unsafe { Peripheral::new(1073808128) };
#[doc = r" Register block"]
pub struct CCU41_CC42 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU41_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 1"]
pub const CCU41_CC43: Peripheral<CCU41_CC43> = unsafe { Peripheral::new(1073808384) };
#[doc = r" Register block"]
pub struct CCU41_CC43 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU41_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub const CCU42_CC40: Peripheral<CCU42_CC40> = unsafe { Peripheral::new(1073824000) };
#[doc = r" Register block"]
pub struct CCU42_CC40 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU42_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub const CCU42_CC41: Peripheral<CCU42_CC41> = unsafe { Peripheral::new(1073824256) };
#[doc = r" Register block"]
pub struct CCU42_CC41 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU42_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub const CCU42_CC42: Peripheral<CCU42_CC42> = unsafe { Peripheral::new(1073824512) };
#[doc = r" Register block"]
pub struct CCU42_CC42 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU42_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 2"]
pub const CCU42_CC43: Peripheral<CCU42_CC43> = unsafe { Peripheral::new(1073824768) };
#[doc = r" Register block"]
pub struct CCU42_CC43 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU42_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub const CCU43_CC40: Peripheral<CCU43_CC40> = unsafe { Peripheral::new(1207976192) };
#[doc = r" Register block"]
pub struct CCU43_CC40 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU43_CC40 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub const CCU43_CC41: Peripheral<CCU43_CC41> = unsafe { Peripheral::new(1207976448) };
#[doc = r" Register block"]
pub struct CCU43_CC41 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU43_CC41 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub const CCU43_CC42: Peripheral<CCU43_CC42> = unsafe { Peripheral::new(1207976704) };
#[doc = r" Register block"]
pub struct CCU43_CC42 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU43_CC42 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 4 - Unit 3"]
pub const CCU43_CC43: Peripheral<CCU43_CC43> = unsafe { Peripheral::new(1207976960) };
#[doc = r" Register block"]
pub struct CCU43_CC43 {
    register_block: ccu40_cc40::RegisterBlock,
}
impl Deref for CCU43_CC43 {
    type Target = ccu40_cc40::RegisterBlock;
    fn deref(&self) -> &ccu40_cc40::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub const CCU80: Peripheral<CCU80> = unsafe { Peripheral::new(1073872896) };
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80 {
    register_block: ccu80::RegisterBlock,
}
impl Deref for CCU80 {
    type Target = ccu80::RegisterBlock;
    fn deref(&self) -> &ccu80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub const CCU81: Peripheral<CCU81> = unsafe { Peripheral::new(1073889280) };
#[doc = r" Register block"]
pub struct CCU81 {
    register_block: ccu80::RegisterBlock,
}
impl Deref for CCU81 {
    type Target = ccu80::RegisterBlock;
    fn deref(&self) -> &ccu80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub const CCU80_CC80: Peripheral<CCU80_CC80> = unsafe { Peripheral::new(1073873152) };
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub mod ccu80_cc80;
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub struct CCU80_CC80 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU80_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub const CCU80_CC81: Peripheral<CCU80_CC81> = unsafe { Peripheral::new(1073873408) };
#[doc = r" Register block"]
pub struct CCU80_CC81 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU80_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub const CCU80_CC82: Peripheral<CCU80_CC82> = unsafe { Peripheral::new(1073873664) };
#[doc = r" Register block"]
pub struct CCU80_CC82 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU80_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 0"]
pub const CCU80_CC83: Peripheral<CCU80_CC83> = unsafe { Peripheral::new(1073873920) };
#[doc = r" Register block"]
pub struct CCU80_CC83 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU80_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub const CCU81_CC80: Peripheral<CCU81_CC80> = unsafe { Peripheral::new(1073889536) };
#[doc = r" Register block"]
pub struct CCU81_CC80 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU81_CC80 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub const CCU81_CC81: Peripheral<CCU81_CC81> = unsafe { Peripheral::new(1073889792) };
#[doc = r" Register block"]
pub struct CCU81_CC81 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU81_CC81 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub const CCU81_CC82: Peripheral<CCU81_CC82> = unsafe { Peripheral::new(1073890048) };
#[doc = r" Register block"]
pub struct CCU81_CC82 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU81_CC82 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Capture Compare Unit 8 - Unit 1"]
pub const CCU81_CC83: Peripheral<CCU81_CC83> = unsafe { Peripheral::new(1073890304) };
#[doc = r" Register block"]
pub struct CCU81_CC83 {
    register_block: ccu80_cc80::RegisterBlock,
}
impl Deref for CCU81_CC83 {
    type Target = ccu80_cc80::RegisterBlock;
    fn deref(&self) -> &ccu80_cc80::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Position Interface 0"]
pub const POSIF0: Peripheral<POSIF0> = unsafe { Peripheral::new(1073905664) };
#[doc = "Position Interface 0"]
pub mod posif0;
#[doc = "Position Interface 0"]
pub struct POSIF0 {
    register_block: posif0::RegisterBlock,
}
impl Deref for POSIF0 {
    type Target = posif0::RegisterBlock;
    fn deref(&self) -> &posif0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Position Interface 1"]
pub const POSIF1: Peripheral<POSIF1> = unsafe { Peripheral::new(1073922048) };
#[doc = r" Register block"]
pub struct POSIF1 {
    register_block: posif0::RegisterBlock,
}
impl Deref for POSIF1 {
    type Target = posif0::RegisterBlock;
    fn deref(&self) -> &posif0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 0"]
pub const PORT0: Peripheral<PORT0> = unsafe { Peripheral::new(1208123392) };
#[doc = "Port 0"]
pub mod port0;
#[doc = "Port 0"]
pub struct PORT0 {
    register_block: port0::RegisterBlock,
}
impl Deref for PORT0 {
    type Target = port0::RegisterBlock;
    fn deref(&self) -> &port0::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 1"]
pub const PORT1: Peripheral<PORT1> = unsafe { Peripheral::new(1208123648) };
#[doc = "Port 1"]
pub mod port1;
#[doc = "Port 1"]
pub struct PORT1 {
    register_block: port1::RegisterBlock,
}
impl Deref for PORT1 {
    type Target = port1::RegisterBlock;
    fn deref(&self) -> &port1::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 2"]
pub const PORT2: Peripheral<PORT2> = unsafe { Peripheral::new(1208123904) };
#[doc = "Port 2"]
pub mod port2;
#[doc = "Port 2"]
pub struct PORT2 {
    register_block: port2::RegisterBlock,
}
impl Deref for PORT2 {
    type Target = port2::RegisterBlock;
    fn deref(&self) -> &port2::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 3"]
pub const PORT3: Peripheral<PORT3> = unsafe { Peripheral::new(1208124160) };
#[doc = "Port 3"]
pub mod port3;
#[doc = "Port 3"]
pub struct PORT3 {
    register_block: port3::RegisterBlock,
}
impl Deref for PORT3 {
    type Target = port3::RegisterBlock;
    fn deref(&self) -> &port3::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 4"]
pub const PORT4: Peripheral<PORT4> = unsafe { Peripheral::new(1208124416) };
#[doc = "Port 4"]
pub mod port4;
#[doc = "Port 4"]
pub struct PORT4 {
    register_block: port4::RegisterBlock,
}
impl Deref for PORT4 {
    type Target = port4::RegisterBlock;
    fn deref(&self) -> &port4::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 5"]
pub const PORT5: Peripheral<PORT5> = unsafe { Peripheral::new(1208124672) };
#[doc = "Port 5"]
pub mod port5;
#[doc = "Port 5"]
pub struct PORT5 {
    register_block: port5::RegisterBlock,
}
impl Deref for PORT5 {
    type Target = port5::RegisterBlock;
    fn deref(&self) -> &port5::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 6"]
pub const PORT6: Peripheral<PORT6> = unsafe { Peripheral::new(1208124928) };
#[doc = "Port 6"]
pub mod port6;
#[doc = "Port 6"]
pub struct PORT6 {
    register_block: port6::RegisterBlock,
}
impl Deref for PORT6 {
    type Target = port6::RegisterBlock;
    fn deref(&self) -> &port6::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 7"]
pub const PORT7: Peripheral<PORT7> = unsafe { Peripheral::new(1208125184) };
#[doc = "Port 7"]
pub mod port7;
#[doc = "Port 7"]
pub struct PORT7 {
    register_block: port7::RegisterBlock,
}
impl Deref for PORT7 {
    type Target = port7::RegisterBlock;
    fn deref(&self) -> &port7::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 8"]
pub const PORT8: Peripheral<PORT8> = unsafe { Peripheral::new(1208125440) };
#[doc = "Port 8"]
pub mod port8;
#[doc = "Port 8"]
pub struct PORT8 {
    register_block: port8::RegisterBlock,
}
impl Deref for PORT8 {
    type Target = port8::RegisterBlock;
    fn deref(&self) -> &port8::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 9"]
pub const PORT9: Peripheral<PORT9> = unsafe { Peripheral::new(1208125696) };
#[doc = "Port 9"]
pub mod port9;
#[doc = "Port 9"]
pub struct PORT9 {
    register_block: port9::RegisterBlock,
}
impl Deref for PORT9 {
    type Target = port9::RegisterBlock;
    fn deref(&self) -> &port9::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 14"]
pub const PORT14: Peripheral<PORT14> = unsafe { Peripheral::new(1208126976) };
#[doc = "Port 14"]
pub mod port14;
#[doc = "Port 14"]
pub struct PORT14 {
    register_block: port14::RegisterBlock,
}
impl Deref for PORT14 {
    type Target = port14::RegisterBlock;
    fn deref(&self) -> &port14::RegisterBlock {
        &self.register_block
    }
}
#[doc = "Port 15"]
pub const PORT15: Peripheral<PORT15> = unsafe { Peripheral::new(1208127232) };
#[doc = "Port 15"]
pub mod port15;
#[doc = "Port 15"]
pub struct PORT15 {
    register_block: port15::RegisterBlock,
}
impl Deref for PORT15 {
    type Target = port15::RegisterBlock;
    fn deref(&self) -> &port15::RegisterBlock {
        &self.register_block
    }
}
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals<'a> {
    #[doc = "CPUID"]
    pub CPUID: &'a CPUID,
    #[doc = "DCB"]
    pub DCB: &'a DCB,
    #[doc = "DWT"]
    pub DWT: &'a DWT,
    #[doc = "FPB"]
    pub FPB: &'a FPB,
    #[doc = "FPU"]
    pub FPU: &'a FPU,
    #[doc = "ITM"]
    pub ITM: &'a ITM,
    #[doc = "MPU"]
    pub MPU: &'a MPU,
    #[doc = "NVIC"]
    pub NVIC: &'a NVIC,
    #[doc = "SCB"]
    pub SCB: &'a SCB,
    #[doc = "SYST"]
    pub SYST: &'a SYST,
    #[doc = "TPIU"]
    pub TPIU: &'a TPIU,
    #[doc = "PPB"]
    pub PPB: &'a PPB,
    #[doc = "DLR"]
    pub DLR: &'a DLR,
    #[doc = "ERU0"]
    pub ERU0: &'a ERU0,
    #[doc = "ERU1"]
    pub ERU1: &'a ERU1,
    #[doc = "GPDMA0"]
    pub GPDMA0: &'a GPDMA0,
    #[doc = "GPDMA0_CH0"]
    pub GPDMA0_CH0: &'a GPDMA0_CH0,
    #[doc = "GPDMA0_CH1"]
    pub GPDMA0_CH1: &'a GPDMA0_CH1,
    #[doc = "GPDMA0_CH2"]
    pub GPDMA0_CH2: &'a GPDMA0_CH2,
    #[doc = "GPDMA0_CH3"]
    pub GPDMA0_CH3: &'a GPDMA0_CH3,
    #[doc = "GPDMA0_CH4"]
    pub GPDMA0_CH4: &'a GPDMA0_CH4,
    #[doc = "GPDMA0_CH5"]
    pub GPDMA0_CH5: &'a GPDMA0_CH5,
    #[doc = "GPDMA0_CH6"]
    pub GPDMA0_CH6: &'a GPDMA0_CH6,
    #[doc = "GPDMA0_CH7"]
    pub GPDMA0_CH7: &'a GPDMA0_CH7,
    #[doc = "GPDMA1"]
    pub GPDMA1: &'a GPDMA1,
    #[doc = "GPDMA1_CH0"]
    pub GPDMA1_CH0: &'a GPDMA1_CH0,
    #[doc = "GPDMA1_CH1"]
    pub GPDMA1_CH1: &'a GPDMA1_CH1,
    #[doc = "GPDMA1_CH2"]
    pub GPDMA1_CH2: &'a GPDMA1_CH2,
    #[doc = "GPDMA1_CH3"]
    pub GPDMA1_CH3: &'a GPDMA1_CH3,
    #[doc = "FCE"]
    pub FCE: &'a FCE,
    #[doc = "FCE_KE0"]
    pub FCE_KE0: &'a FCE_KE0,
    #[doc = "FCE_KE1"]
    pub FCE_KE1: &'a FCE_KE1,
    #[doc = "FCE_KE2"]
    pub FCE_KE2: &'a FCE_KE2,
    #[doc = "FCE_KE3"]
    pub FCE_KE3: &'a FCE_KE3,
    #[doc = "PBA0"]
    pub PBA0: &'a PBA0,
    #[doc = "PBA1"]
    pub PBA1: &'a PBA1,
    #[doc = "FLASH0"]
    pub FLASH0: &'a FLASH0,
    #[doc = "PREF"]
    pub PREF: &'a PREF,
    #[doc = "PMU0"]
    pub PMU0: &'a PMU0,
    #[doc = "WDT"]
    pub WDT: &'a WDT,
    #[doc = "RTC"]
    pub RTC: &'a RTC,
    #[doc = "SCU_CLK"]
    pub SCU_CLK: &'a SCU_CLK,
    #[doc = "SCU_OSC"]
    pub SCU_OSC: &'a SCU_OSC,
    #[doc = "SCU_PLL"]
    pub SCU_PLL: &'a SCU_PLL,
    #[doc = "SCU_GENERAL"]
    pub SCU_GENERAL: &'a SCU_GENERAL,
    #[doc = "SCU_INTERRUPT"]
    pub SCU_INTERRUPT: &'a SCU_INTERRUPT,
    #[doc = "SCU_PARITY"]
    pub SCU_PARITY: &'a SCU_PARITY,
    #[doc = "SCU_TRAP"]
    pub SCU_TRAP: &'a SCU_TRAP,
    #[doc = "SCU_HIBERNATE"]
    pub SCU_HIBERNATE: &'a SCU_HIBERNATE,
    #[doc = "SCU_POWER"]
    pub SCU_POWER: &'a SCU_POWER,
    #[doc = "SCU_RESET"]
    pub SCU_RESET: &'a SCU_RESET,
    #[doc = "LEDTS0"]
    pub LEDTS0: &'a LEDTS0,
    #[doc = "SDMMC_CON"]
    pub SDMMC_CON: &'a SDMMC_CON,
    #[doc = "SDMMC"]
    pub SDMMC: &'a SDMMC,
    #[doc = "EBU"]
    pub EBU: &'a EBU,
    #[doc = "ETH0_CON"]
    pub ETH0_CON: &'a ETH0_CON,
    #[doc = "ETH0"]
    pub ETH0: &'a ETH0,
    #[doc = "USB0"]
    pub USB0: &'a USB0,
    #[doc = "USB0_EP0"]
    pub USB0_EP0: &'a USB0_EP0,
    #[doc = "USB0_EP1"]
    pub USB0_EP1: &'a USB0_EP1,
    #[doc = "USB0_EP2"]
    pub USB0_EP2: &'a USB0_EP2,
    #[doc = "USB0_EP3"]
    pub USB0_EP3: &'a USB0_EP3,
    #[doc = "USB0_EP4"]
    pub USB0_EP4: &'a USB0_EP4,
    #[doc = "USB0_EP5"]
    pub USB0_EP5: &'a USB0_EP5,
    #[doc = "USB0_EP6"]
    pub USB0_EP6: &'a USB0_EP6,
    #[doc = "USB0_CH0"]
    pub USB0_CH0: &'a USB0_CH0,
    #[doc = "USB0_CH1"]
    pub USB0_CH1: &'a USB0_CH1,
    #[doc = "USB0_CH2"]
    pub USB0_CH2: &'a USB0_CH2,
    #[doc = "USB0_CH3"]
    pub USB0_CH3: &'a USB0_CH3,
    #[doc = "USB0_CH4"]
    pub USB0_CH4: &'a USB0_CH4,
    #[doc = "USB0_CH5"]
    pub USB0_CH5: &'a USB0_CH5,
    #[doc = "USB0_CH6"]
    pub USB0_CH6: &'a USB0_CH6,
    #[doc = "USB0_CH7"]
    pub USB0_CH7: &'a USB0_CH7,
    #[doc = "USB0_CH8"]
    pub USB0_CH8: &'a USB0_CH8,
    #[doc = "USB0_CH9"]
    pub USB0_CH9: &'a USB0_CH9,
    #[doc = "USB0_CH10"]
    pub USB0_CH10: &'a USB0_CH10,
    #[doc = "USB0_CH11"]
    pub USB0_CH11: &'a USB0_CH11,
    #[doc = "USB0_CH12"]
    pub USB0_CH12: &'a USB0_CH12,
    #[doc = "USB0_CH13"]
    pub USB0_CH13: &'a USB0_CH13,
    #[doc = "USIC0"]
    pub USIC0: &'a USIC0,
    #[doc = "USIC1"]
    pub USIC1: &'a USIC1,
    #[doc = "USIC2"]
    pub USIC2: &'a USIC2,
    #[doc = "USIC0_CH0"]
    pub USIC0_CH0: &'a USIC0_CH0,
    #[doc = "USIC0_CH1"]
    pub USIC0_CH1: &'a USIC0_CH1,
    #[doc = "USIC1_CH0"]
    pub USIC1_CH0: &'a USIC1_CH0,
    #[doc = "USIC1_CH1"]
    pub USIC1_CH1: &'a USIC1_CH1,
    #[doc = "USIC2_CH0"]
    pub USIC2_CH0: &'a USIC2_CH0,
    #[doc = "USIC2_CH1"]
    pub USIC2_CH1: &'a USIC2_CH1,
    #[doc = "CAN"]
    pub CAN: &'a CAN,
    #[doc = "CAN_NODE0"]
    pub CAN_NODE0: &'a CAN_NODE0,
    #[doc = "CAN_NODE1"]
    pub CAN_NODE1: &'a CAN_NODE1,
    #[doc = "CAN_NODE2"]
    pub CAN_NODE2: &'a CAN_NODE2,
    #[doc = "CAN_NODE3"]
    pub CAN_NODE3: &'a CAN_NODE3,
    #[doc = "CAN_NODE4"]
    pub CAN_NODE4: &'a CAN_NODE4,
    #[doc = "CAN_NODE5"]
    pub CAN_NODE5: &'a CAN_NODE5,
    #[doc = "VADC"]
    pub VADC: &'a VADC,
    #[doc = "VADC_G0"]
    pub VADC_G0: &'a VADC_G0,
    #[doc = "VADC_G1"]
    pub VADC_G1: &'a VADC_G1,
    #[doc = "VADC_G2"]
    pub VADC_G2: &'a VADC_G2,
    #[doc = "VADC_G3"]
    pub VADC_G3: &'a VADC_G3,
    #[doc = "DSD"]
    pub DSD: &'a DSD,
    #[doc = "DSD_CH0"]
    pub DSD_CH0: &'a DSD_CH0,
    #[doc = "DSD_CH1"]
    pub DSD_CH1: &'a DSD_CH1,
    #[doc = "DSD_CH2"]
    pub DSD_CH2: &'a DSD_CH2,
    #[doc = "DSD_CH3"]
    pub DSD_CH3: &'a DSD_CH3,
    #[doc = "DAC"]
    pub DAC: &'a DAC,
    #[doc = "CCU40"]
    pub CCU40: &'a CCU40,
    #[doc = "CCU41"]
    pub CCU41: &'a CCU41,
    #[doc = "CCU42"]
    pub CCU42: &'a CCU42,
    #[doc = "CCU43"]
    pub CCU43: &'a CCU43,
    #[doc = "CCU40_CC40"]
    pub CCU40_CC40: &'a CCU40_CC40,
    #[doc = "CCU40_CC41"]
    pub CCU40_CC41: &'a CCU40_CC41,
    #[doc = "CCU40_CC42"]
    pub CCU40_CC42: &'a CCU40_CC42,
    #[doc = "CCU40_CC43"]
    pub CCU40_CC43: &'a CCU40_CC43,
    #[doc = "CCU41_CC40"]
    pub CCU41_CC40: &'a CCU41_CC40,
    #[doc = "CCU41_CC41"]
    pub CCU41_CC41: &'a CCU41_CC41,
    #[doc = "CCU41_CC42"]
    pub CCU41_CC42: &'a CCU41_CC42,
    #[doc = "CCU41_CC43"]
    pub CCU41_CC43: &'a CCU41_CC43,
    #[doc = "CCU42_CC40"]
    pub CCU42_CC40: &'a CCU42_CC40,
    #[doc = "CCU42_CC41"]
    pub CCU42_CC41: &'a CCU42_CC41,
    #[doc = "CCU42_CC42"]
    pub CCU42_CC42: &'a CCU42_CC42,
    #[doc = "CCU42_CC43"]
    pub CCU42_CC43: &'a CCU42_CC43,
    #[doc = "CCU43_CC40"]
    pub CCU43_CC40: &'a CCU43_CC40,
    #[doc = "CCU43_CC41"]
    pub CCU43_CC41: &'a CCU43_CC41,
    #[doc = "CCU43_CC42"]
    pub CCU43_CC42: &'a CCU43_CC42,
    #[doc = "CCU43_CC43"]
    pub CCU43_CC43: &'a CCU43_CC43,
    #[doc = "CCU80"]
    pub CCU80: &'a CCU80,
    #[doc = "CCU81"]
    pub CCU81: &'a CCU81,
    #[doc = "CCU80_CC80"]
    pub CCU80_CC80: &'a CCU80_CC80,
    #[doc = "CCU80_CC81"]
    pub CCU80_CC81: &'a CCU80_CC81,
    #[doc = "CCU80_CC82"]
    pub CCU80_CC82: &'a CCU80_CC82,
    #[doc = "CCU80_CC83"]
    pub CCU80_CC83: &'a CCU80_CC83,
    #[doc = "CCU81_CC80"]
    pub CCU81_CC80: &'a CCU81_CC80,
    #[doc = "CCU81_CC81"]
    pub CCU81_CC81: &'a CCU81_CC81,
    #[doc = "CCU81_CC82"]
    pub CCU81_CC82: &'a CCU81_CC82,
    #[doc = "CCU81_CC83"]
    pub CCU81_CC83: &'a CCU81_CC83,
    #[doc = "POSIF0"]
    pub POSIF0: &'a POSIF0,
    #[doc = "POSIF1"]
    pub POSIF1: &'a POSIF1,
    #[doc = "PORT0"]
    pub PORT0: &'a PORT0,
    #[doc = "PORT1"]
    pub PORT1: &'a PORT1,
    #[doc = "PORT2"]
    pub PORT2: &'a PORT2,
    #[doc = "PORT3"]
    pub PORT3: &'a PORT3,
    #[doc = "PORT4"]
    pub PORT4: &'a PORT4,
    #[doc = "PORT5"]
    pub PORT5: &'a PORT5,
    #[doc = "PORT6"]
    pub PORT6: &'a PORT6,
    #[doc = "PORT7"]
    pub PORT7: &'a PORT7,
    #[doc = "PORT8"]
    pub PORT8: &'a PORT8,
    #[doc = "PORT9"]
    pub PORT9: &'a PORT9,
    #[doc = "PORT14"]
    pub PORT14: &'a PORT14,
    #[doc = "PORT15"]
    pub PORT15: &'a PORT15,
}
impl<'a> Peripherals<'a> {
    #[doc = r" Grants access to all the peripherals"]
    pub unsafe fn all() -> Self {
        Peripherals {
            CPUID: &*CPUID.get(),
            DCB: &*DCB.get(),
            DWT: &*DWT.get(),
            FPB: &*FPB.get(),
            FPU: &*FPU.get(),
            ITM: &*ITM.get(),
            MPU: &*MPU.get(),
            NVIC: &*NVIC.get(),
            SCB: &*SCB.get(),
            SYST: &*SYST.get(),
            TPIU: &*TPIU.get(),
            PPB: &*PPB.get(),
            DLR: &*DLR.get(),
            ERU0: &*ERU0.get(),
            ERU1: &*ERU1.get(),
            GPDMA0: &*GPDMA0.get(),
            GPDMA0_CH0: &*GPDMA0_CH0.get(),
            GPDMA0_CH1: &*GPDMA0_CH1.get(),
            GPDMA0_CH2: &*GPDMA0_CH2.get(),
            GPDMA0_CH3: &*GPDMA0_CH3.get(),
            GPDMA0_CH4: &*GPDMA0_CH4.get(),
            GPDMA0_CH5: &*GPDMA0_CH5.get(),
            GPDMA0_CH6: &*GPDMA0_CH6.get(),
            GPDMA0_CH7: &*GPDMA0_CH7.get(),
            GPDMA1: &*GPDMA1.get(),
            GPDMA1_CH0: &*GPDMA1_CH0.get(),
            GPDMA1_CH1: &*GPDMA1_CH1.get(),
            GPDMA1_CH2: &*GPDMA1_CH2.get(),
            GPDMA1_CH3: &*GPDMA1_CH3.get(),
            FCE: &*FCE.get(),
            FCE_KE0: &*FCE_KE0.get(),
            FCE_KE1: &*FCE_KE1.get(),
            FCE_KE2: &*FCE_KE2.get(),
            FCE_KE3: &*FCE_KE3.get(),
            PBA0: &*PBA0.get(),
            PBA1: &*PBA1.get(),
            FLASH0: &*FLASH0.get(),
            PREF: &*PREF.get(),
            PMU0: &*PMU0.get(),
            WDT: &*WDT.get(),
            RTC: &*RTC.get(),
            SCU_CLK: &*SCU_CLK.get(),
            SCU_OSC: &*SCU_OSC.get(),
            SCU_PLL: &*SCU_PLL.get(),
            SCU_GENERAL: &*SCU_GENERAL.get(),
            SCU_INTERRUPT: &*SCU_INTERRUPT.get(),
            SCU_PARITY: &*SCU_PARITY.get(),
            SCU_TRAP: &*SCU_TRAP.get(),
            SCU_HIBERNATE: &*SCU_HIBERNATE.get(),
            SCU_POWER: &*SCU_POWER.get(),
            SCU_RESET: &*SCU_RESET.get(),
            LEDTS0: &*LEDTS0.get(),
            SDMMC_CON: &*SDMMC_CON.get(),
            SDMMC: &*SDMMC.get(),
            EBU: &*EBU.get(),
            ETH0_CON: &*ETH0_CON.get(),
            ETH0: &*ETH0.get(),
            USB0: &*USB0.get(),
            USB0_EP0: &*USB0_EP0.get(),
            USB0_EP1: &*USB0_EP1.get(),
            USB0_EP2: &*USB0_EP2.get(),
            USB0_EP3: &*USB0_EP3.get(),
            USB0_EP4: &*USB0_EP4.get(),
            USB0_EP5: &*USB0_EP5.get(),
            USB0_EP6: &*USB0_EP6.get(),
            USB0_CH0: &*USB0_CH0.get(),
            USB0_CH1: &*USB0_CH1.get(),
            USB0_CH2: &*USB0_CH2.get(),
            USB0_CH3: &*USB0_CH3.get(),
            USB0_CH4: &*USB0_CH4.get(),
            USB0_CH5: &*USB0_CH5.get(),
            USB0_CH6: &*USB0_CH6.get(),
            USB0_CH7: &*USB0_CH7.get(),
            USB0_CH8: &*USB0_CH8.get(),
            USB0_CH9: &*USB0_CH9.get(),
            USB0_CH10: &*USB0_CH10.get(),
            USB0_CH11: &*USB0_CH11.get(),
            USB0_CH12: &*USB0_CH12.get(),
            USB0_CH13: &*USB0_CH13.get(),
            USIC0: &*USIC0.get(),
            USIC1: &*USIC1.get(),
            USIC2: &*USIC2.get(),
            USIC0_CH0: &*USIC0_CH0.get(),
            USIC0_CH1: &*USIC0_CH1.get(),
            USIC1_CH0: &*USIC1_CH0.get(),
            USIC1_CH1: &*USIC1_CH1.get(),
            USIC2_CH0: &*USIC2_CH0.get(),
            USIC2_CH1: &*USIC2_CH1.get(),
            CAN: &*CAN.get(),
            CAN_NODE0: &*CAN_NODE0.get(),
            CAN_NODE1: &*CAN_NODE1.get(),
            CAN_NODE2: &*CAN_NODE2.get(),
            CAN_NODE3: &*CAN_NODE3.get(),
            CAN_NODE4: &*CAN_NODE4.get(),
            CAN_NODE5: &*CAN_NODE5.get(),
            VADC: &*VADC.get(),
            VADC_G0: &*VADC_G0.get(),
            VADC_G1: &*VADC_G1.get(),
            VADC_G2: &*VADC_G2.get(),
            VADC_G3: &*VADC_G3.get(),
            DSD: &*DSD.get(),
            DSD_CH0: &*DSD_CH0.get(),
            DSD_CH1: &*DSD_CH1.get(),
            DSD_CH2: &*DSD_CH2.get(),
            DSD_CH3: &*DSD_CH3.get(),
            DAC: &*DAC.get(),
            CCU40: &*CCU40.get(),
            CCU41: &*CCU41.get(),
            CCU42: &*CCU42.get(),
            CCU43: &*CCU43.get(),
            CCU40_CC40: &*CCU40_CC40.get(),
            CCU40_CC41: &*CCU40_CC41.get(),
            CCU40_CC42: &*CCU40_CC42.get(),
            CCU40_CC43: &*CCU40_CC43.get(),
            CCU41_CC40: &*CCU41_CC40.get(),
            CCU41_CC41: &*CCU41_CC41.get(),
            CCU41_CC42: &*CCU41_CC42.get(),
            CCU41_CC43: &*CCU41_CC43.get(),
            CCU42_CC40: &*CCU42_CC40.get(),
            CCU42_CC41: &*CCU42_CC41.get(),
            CCU42_CC42: &*CCU42_CC42.get(),
            CCU42_CC43: &*CCU42_CC43.get(),
            CCU43_CC40: &*CCU43_CC40.get(),
            CCU43_CC41: &*CCU43_CC41.get(),
            CCU43_CC42: &*CCU43_CC42.get(),
            CCU43_CC43: &*CCU43_CC43.get(),
            CCU80: &*CCU80.get(),
            CCU81: &*CCU81.get(),
            CCU80_CC80: &*CCU80_CC80.get(),
            CCU80_CC81: &*CCU80_CC81.get(),
            CCU80_CC82: &*CCU80_CC82.get(),
            CCU80_CC83: &*CCU80_CC83.get(),
            CCU81_CC80: &*CCU81_CC80.get(),
            CCU81_CC81: &*CCU81_CC81.get(),
            CCU81_CC82: &*CCU81_CC82.get(),
            CCU81_CC83: &*CCU81_CC83.get(),
            POSIF0: &*POSIF0.get(),
            POSIF1: &*POSIF1.get(),
            PORT0: &*PORT0.get(),
            PORT1: &*PORT1.get(),
            PORT2: &*PORT2.get(),
            PORT3: &*PORT3.get(),
            PORT4: &*PORT4.get(),
            PORT5: &*PORT5.get(),
            PORT6: &*PORT6.get(),
            PORT7: &*PORT7.get(),
            PORT8: &*PORT8.get(),
            PORT9: &*PORT9.get(),
            PORT14: &*PORT14.get(),
            PORT15: &*PORT15.get(),
        }
    }
}
