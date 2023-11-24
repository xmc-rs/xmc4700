#[doc = "Register `SAR` reader"]
pub type R = crate::R<SAR_SPEC>;
#[doc = "Register `SAR` writer"]
pub type W = crate::W<SAR_SPEC>;
#[doc = "Field `SAR` reader - Current Source Address of DMA transfer"]
pub type SAR_R = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - Current Source Address of DMA transfer"]
pub type SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    pub fn sar(&self) -> SAR_R {
        SAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Source Address of DMA transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SAR_W<SAR_SPEC> {
        SAR_W::new(self, 0)
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
#[doc = "Source Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SPEC;
impl crate::RegisterSpec for SAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR to value 0"]
impl crate::Resettable for SAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
