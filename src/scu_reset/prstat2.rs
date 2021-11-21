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
#[doc = "WDT Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `WDTRS` reader - WDT Reset Status"]
pub struct WDTRS_R(crate::FieldReader<bool, WDTRS_A>);
impl WDTRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == WDTRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WDTRS_A::VALUE2
    }
}
impl core::ops::Deref for WDTRS_R {
    type Target = crate::FieldReader<bool, WDTRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ETH0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `ETH0RS` reader - ETH0 Reset Status"]
pub struct ETH0RS_R(crate::FieldReader<bool, ETH0RS_A>);
impl ETH0RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETH0RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == ETH0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ETH0RS_A::VALUE2
    }
}
impl core::ops::Deref for ETH0RS_R {
    type Target = crate::FieldReader<bool, ETH0RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DMA0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA0RS` reader - DMA0 Reset Status"]
pub struct DMA0RS_R(crate::FieldReader<bool, DMA0RS_A>);
impl DMA0RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA0RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DMA0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DMA0RS_A::VALUE2
    }
}
impl core::ops::Deref for DMA0RS_R {
    type Target = crate::FieldReader<bool, DMA0RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DMA1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `DMA1RS` reader - DMA1 Reset Status"]
pub struct DMA1RS_R(crate::FieldReader<bool, DMA1RS_A>);
impl DMA1RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == DMA1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DMA1RS_A::VALUE2
    }
}
impl core::ops::Deref for DMA1RS_R {
    type Target = crate::FieldReader<bool, DMA1RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FCE Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FCERS` reader - FCE Reset Status"]
pub struct FCERS_R(crate::FieldReader<bool, FCERS_A>);
impl FCERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCERS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == FCERS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FCERS_A::VALUE2
    }
}
impl core::ops::Deref for FCERS_R {
    type Target = crate::FieldReader<bool, FCERS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USB Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `USBRS` reader - USB Reset Status"]
pub struct USBRS_R(crate::FieldReader<bool, USBRS_A>);
impl USBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == USBRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == USBRS_A::VALUE2
    }
}
impl core::ops::Deref for USBRS_R {
    type Target = crate::FieldReader<bool, USBRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - WDT Reset Status"]
    #[inline(always)]
    pub fn wdtrs(&self) -> WDTRS_R {
        WDTRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ETH0 Reset Status"]
    #[inline(always)]
    pub fn eth0rs(&self) -> ETH0RS_R {
        ETH0RS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA0 Reset Status"]
    #[inline(always)]
    pub fn dma0rs(&self) -> DMA0RS_R {
        DMA0RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA1 Reset Status"]
    #[inline(always)]
    pub fn dma1rs(&self) -> DMA1RS_R {
        DMA1RS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FCE Reset Status"]
    #[inline(always)]
    pub fn fcers(&self) -> FCERS_R {
        FCERS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USB Reset Status"]
    #[inline(always)]
    pub fn usbrs(&self) -> USBRS_R {
        USBRS_R::new(((self.bits >> 7) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04f6
    }
}
