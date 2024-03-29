#[doc = "Register `RAWDSTTRAN` reader"]
pub type R = crate::R<RawdsttranSpec>;
#[doc = "Register `RAWDSTTRAN` writer"]
pub type W = crate::W<RawdsttranSpec>;
#[doc = "Field `CH0` reader - Raw Interrupt Status for channel 0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH0` writer - Raw Interrupt Status for channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - Raw Interrupt Status for channel 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH1` writer - Raw Interrupt Status for channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - Raw Interrupt Status for channel 2"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH2` writer - Raw Interrupt Status for channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` reader - Raw Interrupt Status for channel 3"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH3` writer - Raw Interrupt Status for channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw Interrupt Status for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<RawdsttranSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Raw Interrupt Status for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<RawdsttranSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Raw Interrupt Status for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<RawdsttranSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Raw Interrupt Status for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<RawdsttranSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "Raw IntBlock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rawdsttran::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rawdsttran::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawdsttranSpec;
impl crate::RegisterSpec for RawdsttranSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawdsttran::R`](R) reader structure"]
impl crate::Readable for RawdsttranSpec {}
#[doc = "`write(|w| ..)` method takes [`rawdsttran::W`](W) writer structure"]
impl crate::Writable for RawdsttranSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAWDSTTRAN to value 0"]
impl crate::Resettable for RawdsttranSpec {
    const RESET_VALUE: u32 = 0;
}
