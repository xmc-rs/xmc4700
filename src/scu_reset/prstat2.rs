#[doc = "Register `PRSTAT2` reader"]
pub type R = crate::R<Prstat2Spec>;
#[doc = "WDT Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtrs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Wdtrs> for bool {
    #[inline(always)]
    fn from(variant: Wdtrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTRS` reader - WDT Reset Status"]
pub type WdtrsR = crate::BitReader<Wdtrs>;
impl WdtrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdtrs {
        match self.bits {
            false => Wdtrs::Value1,
            true => Wdtrs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wdtrs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wdtrs::Value2
    }
}
#[doc = "ETH0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eth0rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Eth0rs> for bool {
    #[inline(always)]
    fn from(variant: Eth0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0RS` reader - ETH0 Reset Status"]
pub type Eth0rsR = crate::BitReader<Eth0rs>;
impl Eth0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eth0rs {
        match self.bits {
            false => Eth0rs::Value1,
            true => Eth0rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eth0rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eth0rs::Value2
    }
}
#[doc = "DMA0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma0rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Dma0rs> for bool {
    #[inline(always)]
    fn from(variant: Dma0rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0RS` reader - DMA0 Reset Status"]
pub type Dma0rsR = crate::BitReader<Dma0rs>;
impl Dma0rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma0rs {
        match self.bits {
            false => Dma0rs::Value1,
            true => Dma0rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dma0rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dma0rs::Value2
    }
}
#[doc = "DMA1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma1rs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Dma1rs> for bool {
    #[inline(always)]
    fn from(variant: Dma1rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1RS` reader - DMA1 Reset Status"]
pub type Dma1rsR = crate::BitReader<Dma1rs>;
impl Dma1rsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma1rs {
        match self.bits {
            false => Dma1rs::Value1,
            true => Dma1rs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dma1rs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dma1rs::Value2
    }
}
#[doc = "FCE Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcers {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Fcers> for bool {
    #[inline(always)]
    fn from(variant: Fcers) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCERS` reader - FCE Reset Status"]
pub type FcersR = crate::BitReader<Fcers>;
impl FcersR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcers {
        match self.bits {
            false => Fcers::Value1,
            true => Fcers::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fcers::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fcers::Value2
    }
}
#[doc = "USB Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrs {
    #[doc = "0: Reset de-asserted"]
    Value1 = 0,
    #[doc = "1: Reset asserted"]
    Value2 = 1,
}
impl From<Usbrs> for bool {
    #[inline(always)]
    fn from(variant: Usbrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRS` reader - USB Reset Status"]
pub type UsbrsR = crate::BitReader<Usbrs>;
impl UsbrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrs {
        match self.bits {
            false => Usbrs::Value1,
            true => Usbrs::Value2,
        }
    }
    #[doc = "Reset de-asserted"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usbrs::Value1
    }
    #[doc = "Reset asserted"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usbrs::Value2
    }
}
impl R {
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WdtrsR {
        WdtrsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline(always)]
    pub fn eth0rs(&self) -> Eth0rsR {
        Eth0rsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline(always)]
    pub fn dma0rs(&self) -> Dma0rsR {
        Dma0rsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA1 Reset Status"]
    #[inline(always)]
    pub fn dma1rs(&self) -> Dma1rsR {
        Dma1rsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline(always)]
    pub fn fcers(&self) -> FcersR {
        FcersR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline(always)]
    pub fn usbrs(&self) -> UsbrsR {
        UsbrsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 2 Reset Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prstat2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstat2Spec;
impl crate::RegisterSpec for Prstat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prstat2::R`](R) reader structure"]
impl crate::Readable for Prstat2Spec {}
#[doc = "`reset()` method sets PRSTAT2 to value 0x04f6"]
impl crate::Resettable for Prstat2Spec {
    const RESET_VALUE: u32 = 0x04f6;
}
