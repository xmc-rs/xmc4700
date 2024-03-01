#[doc = "Register `RSTCLR` writer"]
pub type W = crate::W<RstclrSpec>;
#[doc = "Clear Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsclr {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clears field RSTSTAT.RSTSTAT"]
    Value2 = 1,
}
impl From<Rsclr> for bool {
    #[inline(always)]
    fn from(variant: Rsclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSCLR` writer - Clear Reset Status"]
pub type RsclrW<'a, REG> = crate::BitWriter<'a, REG, Rsclr>;
impl<'a, REG> RsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsclr::Value1)
    }
    #[doc = "Clears field RSTSTAT.RSTSTAT"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rsclr::Value2)
    }
}
#[doc = "Clear Hibernate Wake-up Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibwk {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset status bit"]
    Value2 = 1,
}
impl From<Hibwk> for bool {
    #[inline(always)]
    fn from(variant: Hibwk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBWK` writer - Clear Hibernate Wake-up Reset Status"]
pub type HibwkW<'a, REG> = crate::BitWriter<'a, REG, Hibwk>;
impl<'a, REG> HibwkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibwk::Value1)
    }
    #[doc = "De-assert reset status bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibwk::Value2)
    }
}
#[doc = "Clear Hibernate Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibrs {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: De-assert reset"]
    Value2 = 1,
}
impl From<Hibrs> for bool {
    #[inline(always)]
    fn from(variant: Hibrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBRS` writer - Clear Hibernate Reset"]
pub type HibrsW<'a, REG> = crate::BitWriter<'a, REG, Hibrs>;
impl<'a, REG> HibrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibrs::Value1)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibrs::Value2)
    }
}
#[doc = "Enable Lockup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcken {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable reset when Lockup gets asserted"]
    Value2 = 1,
}
impl From<Lcken> for bool {
    #[inline(always)]
    fn from(variant: Lcken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCKEN` writer - Enable Lockup Reset"]
pub type LckenW<'a, REG> = crate::BitWriter<'a, REG, Lcken>;
impl<'a, REG> LckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcken::Value1)
    }
    #[doc = "Disable reset when Lockup gets asserted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lcken::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn rsclr(&mut self) -> RsclrW<RstclrSpec> {
        RsclrW::new(self, 0)
    }
    #[doc = "Bit 8 - Clear Hibernate Wake-up Reset Status"]
    #[inline(always)]
    #[must_use]
    pub fn hibwk(&mut self) -> HibwkW<RstclrSpec> {
        HibwkW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Hibernate Reset"]
    #[inline(always)]
    #[must_use]
    pub fn hibrs(&mut self) -> HibrsW<RstclrSpec> {
        HibrsW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Lockup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lcken(&mut self) -> LckenW<RstclrSpec> {
        LckenW::new(self, 10)
    }
}
#[doc = "RCU Reset Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstclrSpec;
impl crate::RegisterSpec for RstclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstclr::W`](W) writer structure"]
impl crate::Writable for RstclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCLR to value 0"]
impl crate::Resettable for RstclrSpec {
    const RESET_VALUE: u32 = 0;
}
