#[doc = "Register `MLINKCLKCR` reader"]
pub type R = crate::R<MLINKCLKCR_SPEC>;
#[doc = "Register `MLINKCLKCR` writer"]
pub type W = crate::W<MLINKCLKCR_SPEC>;
#[doc = "Field `SYSDIV` reader - System Clock Division Value"]
pub type SYSDIV_R = crate::FieldReader;
#[doc = "Field `SYSDIV` writer - System Clock Division Value"]
pub type SYSDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFIclock"]
    VALUE1 = 0,
    #[doc = "1: fPLLclock"]
    VALUE2 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::VALUE1,
            true => SYSSEL_A::VALUE2,
        }
    }
    #[doc = "fOFIclock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSSEL_A::VALUE1
    }
    #[doc = "fPLLclock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYSSEL_A::VALUE2
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SYSSEL_A>;
impl<'a, REG> SYSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFIclock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE1)
    }
    #[doc = "fPLLclock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE2)
    }
}
#[doc = "CPU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPUDIV_A {
    #[doc = "0: fCPU=fSYS"]
    VALUE1 = 0,
    #[doc = "1: fCPU=fSYS/ 2"]
    VALUE2 = 1,
}
impl From<CPUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPUDIV` reader - CPU Clock Divider Enable"]
pub type CPUDIV_R = crate::BitReader<CPUDIV_A>;
impl CPUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPUDIV_A {
        match self.bits {
            false => CPUDIV_A::VALUE1,
            true => CPUDIV_A::VALUE2,
        }
    }
    #[doc = "fCPU=fSYS"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CPUDIV_A::VALUE1
    }
    #[doc = "fCPU=fSYS/ 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CPUDIV_A::VALUE2
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Divider Enable"]
pub type CPUDIV_W<'a, REG> = crate::BitWriter<'a, REG, CPUDIV_A>;
impl<'a, REG> CPUDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCPU=fSYS"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIV_A::VALUE1)
    }
    #[doc = "fCPU=fSYS/ 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CPUDIV_A::VALUE2)
    }
}
#[doc = "PB Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PBDIV_A {
    #[doc = "0: fPERIPH=fCPU"]
    VALUE1 = 0,
    #[doc = "1: fPERIPH=fCPU/ 2"]
    VALUE2 = 1,
}
impl From<PBDIV_A> for bool {
    #[inline(always)]
    fn from(variant: PBDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBDIV` reader - PB Clock Divider Enable"]
pub type PBDIV_R = crate::BitReader<PBDIV_A>;
impl PBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PBDIV_A {
        match self.bits {
            false => PBDIV_A::VALUE1,
            true => PBDIV_A::VALUE2,
        }
    }
    #[doc = "fPERIPH=fCPU"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PBDIV_A::VALUE1
    }
    #[doc = "fPERIPH=fCPU/ 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PBDIV_A::VALUE2
    }
}
#[doc = "Field `PBDIV` writer - PB Clock Divider Enable"]
pub type PBDIV_W<'a, REG> = crate::BitWriter<'a, REG, PBDIV_A>;
impl<'a, REG> PBDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fPERIPH=fCPU"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PBDIV_A::VALUE1)
    }
    #[doc = "fPERIPH=fCPU/ 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PBDIV_A::VALUE2)
    }
}
#[doc = "CCU Clock Divider Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUDIV_A {
    #[doc = "0: fCCU=fSYS"]
    VALUE1 = 0,
    #[doc = "1: fCCU=fSYS/ 2"]
    VALUE2 = 1,
}
impl From<CCUDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CCUDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUDIV` reader - CCU Clock Divider Enable"]
pub type CCUDIV_R = crate::BitReader<CCUDIV_A>;
impl CCUDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUDIV_A {
        match self.bits {
            false => CCUDIV_A::VALUE1,
            true => CCUDIV_A::VALUE2,
        }
    }
    #[doc = "fCCU=fSYS"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCUDIV_A::VALUE1
    }
    #[doc = "fCCU=fSYS/ 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCUDIV_A::VALUE2
    }
}
#[doc = "Field `CCUDIV` writer - CCU Clock Divider Enable"]
pub type CCUDIV_W<'a, REG> = crate::BitWriter<'a, REG, CCUDIV_A>;
impl<'a, REG> CCUDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fCCU=fSYS"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUDIV_A::VALUE1)
    }
    #[doc = "fCCU=fSYS/ 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUDIV_A::VALUE2)
    }
}
#[doc = "Field `WDTDIV` reader - WDT Clock Divider Value"]
pub type WDTDIV_R = crate::FieldReader;
#[doc = "Field `WDTDIV` writer - WDT Clock Divider Value"]
pub type WDTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "WDT Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSEL_A {
    #[doc = "0: fOFIclock"]
    VALUE1 = 0,
    #[doc = "1: fSTDBYclock"]
    VALUE2 = 1,
    #[doc = "2: fPLLclock"]
    VALUE3 = 2,
}
impl From<WDTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDTSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for WDTSEL_A {}
#[doc = "Field `WDTSEL` reader - WDT Clock Selection Value"]
pub type WDTSEL_R = crate::FieldReader<WDTSEL_A>;
impl WDTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDTSEL_A> {
        match self.bits {
            0 => Some(WDTSEL_A::VALUE1),
            1 => Some(WDTSEL_A::VALUE2),
            2 => Some(WDTSEL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "fOFIclock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTSEL_A::VALUE1
    }
    #[doc = "fSTDBYclock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTSEL_A::VALUE2
    }
    #[doc = "fPLLclock"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WDTSEL_A::VALUE3
    }
}
#[doc = "Field `WDTSEL` writer - WDT Clock Selection Value"]
pub type WDTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WDTSEL_A>;
impl<'a, REG> WDTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fOFIclock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE1)
    }
    #[doc = "fSTDBYclock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE2)
    }
    #[doc = "fPLLclock"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(WDTSEL_A::VALUE3)
    }
}
#[doc = "Field `EBUDIV` reader - EBU Clock Divider Value"]
pub type EBUDIV_R = crate::FieldReader;
#[doc = "Field `EBUDIV` writer - EBU Clock Divider Value"]
pub type EBUDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&self) -> SYSDIV_R {
        SYSDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&self) -> CCUDIV_R {
        CCUDIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&self) -> WDTDIV_R {
        WDTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&self) -> WDTSEL_R {
        WDTSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&self) -> EBUDIV_R {
        EBUDIV_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System Clock Division Value"]
    #[inline(always)]
    pub fn sysdiv(&mut self) -> SYSDIV_W<MLINKCLKCR_SPEC> {
        SYSDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W<MLINKCLKCR_SPEC> {
        SYSSEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - CPU Clock Divider Enable"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W<MLINKCLKCR_SPEC> {
        CPUDIV_W::new(self, 10)
    }
    #[doc = "Bit 12 - PB Clock Divider Enable"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W<MLINKCLKCR_SPEC> {
        PBDIV_W::new(self, 12)
    }
    #[doc = "Bit 14 - CCU Clock Divider Enable"]
    #[inline(always)]
    pub fn ccudiv(&mut self) -> CCUDIV_W<MLINKCLKCR_SPEC> {
        CCUDIV_W::new(self, 14)
    }
    #[doc = "Bits 16:23 - WDT Clock Divider Value"]
    #[inline(always)]
    pub fn wdtdiv(&mut self) -> WDTDIV_W<MLINKCLKCR_SPEC> {
        WDTDIV_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - WDT Clock Selection Value"]
    #[inline(always)]
    pub fn wdtsel(&mut self) -> WDTSEL_W<MLINKCLKCR_SPEC> {
        WDTSEL_W::new(self, 24)
    }
    #[doc = "Bits 26:31 - EBU Clock Divider Value"]
    #[inline(always)]
    pub fn ebudiv(&mut self) -> EBUDIV_W<MLINKCLKCR_SPEC> {
        EBUDIV_W::new(self, 26)
    }
}
#[doc = "Multi-Link Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mlinkclkcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mlinkclkcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MLINKCLKCR_SPEC;
impl crate::RegisterSpec for MLINKCLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlinkclkcr::R`](R) reader structure"]
impl crate::Readable for MLINKCLKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mlinkclkcr::W`](W) writer structure"]
impl crate::Writable for MLINKCLKCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MLINKCLKCR to value 0"]
impl crate::Resettable for MLINKCLKCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
