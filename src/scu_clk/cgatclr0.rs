#[doc = "Register `CGATCLR0` writer"]
pub type W = crate::W<Cgatclr0Spec>;
#[doc = "VADC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vadc {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Vadc> for bool {
    #[inline(always)]
    fn from(variant: Vadc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` writer - VADC Gating Clear"]
pub type VadcW<'a, REG> = crate::BitWriter<'a, REG, Vadc>;
impl<'a, REG> VadcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vadc::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vadc::Value2)
    }
}
#[doc = "DSD Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsd {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Dsd> for bool {
    #[inline(always)]
    fn from(variant: Dsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSD` writer - DSD Gating Clear"]
pub type DsdW<'a, REG> = crate::BitWriter<'a, REG, Dsd>;
impl<'a, REG> DsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsd::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dsd::Value2)
    }
}
#[doc = "CCU40 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu40 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ccu40> for bool {
    #[inline(always)]
    fn from(variant: Ccu40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` writer - CCU40 Gating Clear"]
pub type Ccu40W<'a, REG> = crate::BitWriter<'a, REG, Ccu40>;
impl<'a, REG> Ccu40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu40::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu40::Value2)
    }
}
#[doc = "CCU41 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu41 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ccu41> for bool {
    #[inline(always)]
    fn from(variant: Ccu41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` writer - CCU41 Gating Clear"]
pub type Ccu41W<'a, REG> = crate::BitWriter<'a, REG, Ccu41>;
impl<'a, REG> Ccu41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu41::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu41::Value2)
    }
}
#[doc = "CCU42 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu42 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ccu42> for bool {
    #[inline(always)]
    fn from(variant: Ccu42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU42` writer - CCU42 Gating Clear"]
pub type Ccu42W<'a, REG> = crate::BitWriter<'a, REG, Ccu42>;
impl<'a, REG> Ccu42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu42::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu42::Value2)
    }
}
#[doc = "CCU80 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu80 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ccu80> for bool {
    #[inline(always)]
    fn from(variant: Ccu80) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` writer - CCU80 Gating Clear"]
pub type Ccu80W<'a, REG> = crate::BitWriter<'a, REG, Ccu80>;
impl<'a, REG> Ccu80W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu80::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu80::Value2)
    }
}
#[doc = "CCU81 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccu81 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Ccu81> for bool {
    #[inline(always)]
    fn from(variant: Ccu81) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU81` writer - CCU81 Gating Clear"]
pub type Ccu81W<'a, REG> = crate::BitWriter<'a, REG, Ccu81>;
impl<'a, REG> Ccu81W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu81::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccu81::Value2)
    }
}
#[doc = "POSIF0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Posif0> for bool {
    #[inline(always)]
    fn from(variant: Posif0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` writer - POSIF0 Gating Clear"]
pub type Posif0W<'a, REG> = crate::BitWriter<'a, REG, Posif0>;
impl<'a, REG> Posif0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Posif0::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Posif0::Value2)
    }
}
#[doc = "POSIF1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Posif1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Posif1> for bool {
    #[inline(always)]
    fn from(variant: Posif1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF1` writer - POSIF1 Gating Clear"]
pub type Posif1W<'a, REG> = crate::BitWriter<'a, REG, Posif1>;
impl<'a, REG> Posif1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Posif1::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Posif1::Value2)
    }
}
#[doc = "USIC0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Usic0> for bool {
    #[inline(always)]
    fn from(variant: Usic0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` writer - USIC0 Gating Clear"]
pub type Usic0W<'a, REG> = crate::BitWriter<'a, REG, Usic0>;
impl<'a, REG> Usic0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0::Value2)
    }
}
#[doc = "ERU1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eru1 {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Disable gating"]
    Value2 = 1,
}
impl From<Eru1> for bool {
    #[inline(always)]
    fn from(variant: Eru1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` writer - ERU1 Gating Clear"]
pub type Eru1W<'a, REG> = crate::BitWriter<'a, REG, Eru1>;
impl<'a, REG> Eru1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eru1::Value1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eru1::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vadc(&mut self) -> VadcW<Cgatclr0Spec> {
        VadcW::new(self, 0)
    }
    #[doc = "Bit 1 - DSD Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dsd(&mut self) -> DsdW<Cgatclr0Spec> {
        DsdW::new(self, 1)
    }
    #[doc = "Bit 2 - CCU40 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40(&mut self) -> Ccu40W<Cgatclr0Spec> {
        Ccu40W::new(self, 2)
    }
    #[doc = "Bit 3 - CCU41 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41(&mut self) -> Ccu41W<Cgatclr0Spec> {
        Ccu41W::new(self, 3)
    }
    #[doc = "Bit 4 - CCU42 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu42(&mut self) -> Ccu42W<Cgatclr0Spec> {
        Ccu42W::new(self, 4)
    }
    #[doc = "Bit 7 - CCU80 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80(&mut self) -> Ccu80W<Cgatclr0Spec> {
        Ccu80W::new(self, 7)
    }
    #[doc = "Bit 8 - CCU81 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu81(&mut self) -> Ccu81W<Cgatclr0Spec> {
        Ccu81W::new(self, 8)
    }
    #[doc = "Bit 9 - POSIF0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn posif0(&mut self) -> Posif0W<Cgatclr0Spec> {
        Posif0W::new(self, 9)
    }
    #[doc = "Bit 10 - POSIF1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn posif1(&mut self) -> Posif1W<Cgatclr0Spec> {
        Posif1W::new(self, 10)
    }
    #[doc = "Bit 11 - USIC0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic0(&mut self) -> Usic0W<Cgatclr0Spec> {
        Usic0W::new(self, 11)
    }
    #[doc = "Bit 16 - ERU1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eru1(&mut self) -> Eru1W<Cgatclr0Spec> {
        Eru1W::new(self, 16)
    }
}
#[doc = "Peripheral 0 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cgatclr0Spec;
impl crate::RegisterSpec for Cgatclr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr0::W`](W) writer structure"]
impl crate::Writable for Cgatclr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR0 to value 0"]
impl crate::Resettable for Cgatclr0Spec {
    const RESET_VALUE: u32 = 0;
}
