#[doc = "Register `TRAPSET` writer"]
pub type W = crate::W<TrapsetSpec>;
#[doc = "OSC_HP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soscwdgt {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Soscwdgt> for bool {
    #[inline(always)]
    fn from(variant: Soscwdgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` writer - OSC_HP Oscillator Watchdog Trap Set"]
pub type SoscwdgtW<'a, REG> = crate::BitWriter<'a, REG, Soscwdgt>;
impl<'a, REG> SoscwdgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Soscwdgt::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Soscwdgt::Value2)
    }
}
#[doc = "System VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcolckt {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Svcolckt> for bool {
    #[inline(always)]
    fn from(variant: Svcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Set"]
pub type SvcolcktW<'a, REG> = crate::BitWriter<'a, REG, Svcolckt>;
impl<'a, REG> SvcolcktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Svcolckt::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Svcolckt::Value2)
    }
}
#[doc = "USB VCO Lock Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uvcolckt {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Uvcolckt> for bool {
    #[inline(always)]
    fn from(variant: Uvcolckt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Set"]
pub type UvcolcktW<'a, REG> = crate::BitWriter<'a, REG, Uvcolckt>;
impl<'a, REG> UvcolcktW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Uvcolckt::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Uvcolckt::Value2)
    }
}
#[doc = "Parity Error Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pet {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Pet> for bool {
    #[inline(always)]
    fn from(variant: Pet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Set"]
pub type PetW<'a, REG> = crate::BitWriter<'a, REG, Pet>;
impl<'a, REG> PetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pet::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pet::Value2)
    }
}
#[doc = "Brown Out Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brwnt {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Brwnt> for bool {
    #[inline(always)]
    fn from(variant: Brwnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Set"]
pub type BrwntW<'a, REG> = crate::BitWriter<'a, REG, Brwnt>;
impl<'a, REG> BrwntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Brwnt::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Brwnt::Value2)
    }
}
#[doc = "OSC_ULP Oscillator Watchdog Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdt {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Ulpwdt> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDT` writer - OSC_ULP Oscillator Watchdog Trap Set"]
pub type UlpwdtW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdt>;
impl<'a, REG> UlpwdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdt::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdt::Value2)
    }
}
#[doc = "Peripheral Bridge 0 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr0t {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Bwerr0t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr0t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Set"]
pub type Bwerr0tW<'a, REG> = crate::BitWriter<'a, REG, Bwerr0t>;
impl<'a, REG> Bwerr0tW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr0t::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr0t::Value2)
    }
}
#[doc = "Peripheral Bridge 1 Trap Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwerr1t {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Set trap request"]
    Value2 = 1,
}
impl From<Bwerr1t> for bool {
    #[inline(always)]
    fn from(variant: Bwerr1t) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Set"]
pub type Bwerr1tW<'a, REG> = crate::BitWriter<'a, REG, Bwerr1t>;
impl<'a, REG> Bwerr1tW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr1t::Value1)
    }
    #[doc = "Set trap request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bwerr1t::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - OSC_HP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn soscwdgt(&mut self) -> SoscwdgtW<TrapsetSpec> {
        SoscwdgtW::new(self, 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn svcolckt(&mut self) -> SvcolcktW<TrapsetSpec> {
        SvcolcktW::new(self, 2)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn uvcolckt(&mut self) -> UvcolcktW<TrapsetSpec> {
        UvcolcktW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity Error Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn pet(&mut self) -> PetW<TrapsetSpec> {
        PetW::new(self, 4)
    }
    #[doc = "Bit 5 - Brown Out Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn brwnt(&mut self) -> BrwntW<TrapsetSpec> {
        BrwntW::new(self, 5)
    }
    #[doc = "Bit 6 - OSC_ULP Oscillator Watchdog Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdt(&mut self) -> UlpwdtW<TrapsetSpec> {
        UlpwdtW::new(self, 6)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr0t(&mut self) -> Bwerr0tW<TrapsetSpec> {
        Bwerr0tW::new(self, 7)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Set"]
    #[inline(always)]
    #[must_use]
    pub fn bwerr1t(&mut self) -> Bwerr1tW<TrapsetSpec> {
        Bwerr1tW::new(self, 8)
    }
}
#[doc = "Trap Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trapset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrapsetSpec;
impl crate::RegisterSpec for TrapsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trapset::W`](W) writer structure"]
impl crate::Writable for TrapsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRAPSET to value 0"]
impl crate::Resettable for TrapsetSpec {
    const RESET_VALUE: u32 = 0;
}
