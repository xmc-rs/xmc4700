#[doc = "Register `TARGET_TIME_NANOSECONDS` reader"]
pub type R = crate::R<TargetTimeNanosecondsSpec>;
#[doc = "Register `TARGET_TIME_NANOSECONDS` writer"]
pub type W = crate::W<TargetTimeNanosecondsSpec>;
#[doc = "Field `TTSLO` reader - Target Timestamp Low Register"]
pub type TtsloR = crate::FieldReader<u32>;
#[doc = "Field `TTSLO` writer - Target Timestamp Low Register"]
pub type TtsloW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `TRGTBUSY` reader - Target Time Register Busy"]
pub type TrgtbusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    pub fn ttslo(&self) -> TtsloR {
        TtsloR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Target Time Register Busy"]
    #[inline(always)]
    pub fn trgtbusy(&self) -> TrgtbusyR {
        TrgtbusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Target Timestamp Low Register"]
    #[inline(always)]
    #[must_use]
    pub fn ttslo(&mut self) -> TtsloW<TargetTimeNanosecondsSpec> {
        TtsloW::new(self, 0)
    }
}
#[doc = "Target Time Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_nanoseconds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_nanoseconds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TargetTimeNanosecondsSpec;
impl crate::RegisterSpec for TargetTimeNanosecondsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target_time_nanoseconds::R`](R) reader structure"]
impl crate::Readable for TargetTimeNanosecondsSpec {}
#[doc = "`write(|w| ..)` method takes [`target_time_nanoseconds::W`](W) writer structure"]
impl crate::Writable for TargetTimeNanosecondsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET_TIME_NANOSECONDS to value 0"]
impl crate::Resettable for TargetTimeNanosecondsSpec {
    const RESET_VALUE: u32 = 0;
}
