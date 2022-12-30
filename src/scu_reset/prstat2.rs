#[doc = "Register `PRSTAT2` reader"]
pub struct R(crate::R<PRSTAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDTRS` reader - WDT Reset Status"]
pub type WDTRS_R = crate::BitReader<WDTRS_A>;
#[doc = "WDT Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<WDTRS_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRS_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTRS_A {
        match self.bits {
            false => WDTRS_A::VALUE1,
            true => WDTRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTRS_A::VALUE2
    }
}
#[doc = "Field `ETH0RS` reader - ETH0 Reset Status"]
pub type ETH0RS_R = crate::BitReader<ETH0RS_A>;
#[doc = "ETH0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<ETH0RS_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl ETH0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETH0RS_A {
        match self.bits {
            false => ETH0RS_A::VALUE1,
            true => ETH0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ETH0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ETH0RS_A::VALUE2
    }
}
#[doc = "Field `DMA0RS` reader - DMA0 Reset Status"]
pub type DMA0RS_R = crate::BitReader<DMA0RS_A>;
#[doc = "DMA0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<DMA0RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0RS_A {
        match self.bits {
            false => DMA0RS_A::VALUE1,
            true => DMA0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA0RS_A::VALUE2
    }
}
#[doc = "Field `DMA1RS` reader - DMA1 Reset Status"]
pub type DMA1RS_R = crate::BitReader<DMA1RS_A>;
#[doc = "DMA1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<DMA1RS_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RS_A {
        match self.bits {
            false => DMA1RS_A::VALUE1,
            true => DMA1RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMA1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMA1RS_A::VALUE2
    }
}
#[doc = "Field `FCERS` reader - FCE Reset Status"]
pub type FCERS_R = crate::BitReader<FCERS_A>;
#[doc = "FCE Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCERS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<FCERS_A> for bool {
    #[inline(always)]
    fn from(variant: FCERS_A) -> Self {
        variant as u8 != 0
    }
}
impl FCERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCERS_A {
        match self.bits {
            false => FCERS_A::VALUE1,
            true => FCERS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FCERS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FCERS_A::VALUE2
    }
}
#[doc = "Field `USBRS` reader - USB Reset Status"]
pub type USBRS_R = crate::BitReader<USBRS_A>;
#[doc = "USB Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<USBRS_A> for bool {
    #[inline(always)]
    fn from(variant: USBRS_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRS_A {
        match self.bits {
            false => USBRS_A::VALUE1,
            true => USBRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBRS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WDTRS_R {
        WDTRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline(always)]
    pub fn eth0rs(&self) -> ETH0RS_R {
        ETH0RS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline(always)]
    pub fn dma0rs(&self) -> DMA0RS_R {
        DMA0RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA1 Reset Status"]
    #[inline(always)]
    pub fn dma1rs(&self) -> DMA1RS_R {
        DMA1RS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline(always)]
    pub fn fcers(&self) -> FCERS_R {
        FCERS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline(always)]
    pub fn usbrs(&self) -> USBRS_R {
        USBRS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 2 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat2](index.html) module"]
pub struct PRSTAT2_SPEC;
impl crate::RegisterSpec for PRSTAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstat2::R](R) reader structure"]
impl crate::Readable for PRSTAT2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTAT2 to value 0x04f6"]
impl crate::Resettable for PRSTAT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04f6;
}
