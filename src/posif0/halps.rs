#[doc = "Register `HALPS` reader"]
pub type R = crate::R<HALPS_SPEC>;
#[doc = "Register `HALPS` writer"]
pub type W = crate::W<HALPS_SPEC>;
#[doc = "Field `HCPS` reader - Shadow Hall Current Pattern"]
pub type HCPS_R = crate::FieldReader;
#[doc = "Field `HCPS` writer - Shadow Hall Current Pattern"]
pub type HCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HEPS` reader - Shadow Hall expected Pattern"]
pub type HEPS_R = crate::FieldReader;
#[doc = "Field `HEPS` writer - Shadow Hall expected Pattern"]
pub type HEPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    pub fn hcps(&self) -> HCPS_R {
        HCPS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    pub fn heps(&self) -> HEPS_R {
        HEPS_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shadow Hall Current Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn hcps(&mut self) -> HCPS_W<HALPS_SPEC> {
        HCPS_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Shadow Hall expected Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn heps(&mut self) -> HEPS_W<HALPS_SPEC> {
        HEPS_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hall Sensor Shadow Patterns\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`halps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`halps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HALPS_SPEC;
impl crate::RegisterSpec for HALPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`halps::R`](R) reader structure"]
impl crate::Readable for HALPS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`halps::W`](W) writer structure"]
impl crate::Writable for HALPS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HALPS to value 0"]
impl crate::Resettable for HALPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
