#[doc = "Register `DAC1PATH` reader"]
pub type R = crate::R<Dac1pathSpec>;
#[doc = "Register `DAC1PATH` writer"]
pub type W = crate::W<Dac1pathSpec>;
#[doc = "Field `PAT6` reader - Pattern Number 6 for PATGEN of DAC1"]
pub type Pat6R = crate::FieldReader;
#[doc = "Field `PAT6` writer - Pattern Number 6 for PATGEN of DAC1"]
pub type Pat6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT7` reader - Pattern Number 7 for PATGEN of DAC1"]
pub type Pat7R = crate::FieldReader;
#[doc = "Field `PAT7` writer - Pattern Number 7 for PATGEN of DAC1"]
pub type Pat7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAT8` reader - Pattern Number 8 for PATGEN of DAC1"]
pub type Pat8R = crate::FieldReader;
#[doc = "Field `PAT8` writer - Pattern Number 8 for PATGEN of DAC1"]
pub type Pat8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat6(&self) -> Pat6R {
        Pat6R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat7(&self) -> Pat7R {
        Pat7R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    pub fn pat8(&self) -> Pat8R {
        Pat8R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pattern Number 6 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat6(&mut self) -> Pat6W<Dac1pathSpec> {
        Pat6W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Pattern Number 7 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat7(&mut self) -> Pat7W<Dac1pathSpec> {
        Pat7W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Pattern Number 8 for PATGEN of DAC1"]
    #[inline(always)]
    #[must_use]
    pub fn pat8(&mut self) -> Pat8W<Dac1pathSpec> {
        Pat8W::new(self, 10)
    }
}
#[doc = "DAC1 Higher Pattern Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac1path::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac1path::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1pathSpec;
impl crate::RegisterSpec for Dac1pathSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1path::R`](R) reader structure"]
impl crate::Readable for Dac1pathSpec {}
#[doc = "`write(|w| ..)` method takes [`dac1path::W`](W) writer structure"]
impl crate::Writable for Dac1pathSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1PATH to value 0x7fdd"]
impl crate::Resettable for Dac1pathSpec {
    const RESET_VALUE: u32 = 0x7fdd;
}
