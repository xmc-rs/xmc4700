#[doc = "Register `RX_VLAN_FRAMES_GOOD_BAD` reader"]
pub struct R(crate::R<RX_VLAN_FRAMES_GOOD_BAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_VLAN_FRAMES_GOOD_BAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_VLAN_FRAMES_GOOD_BAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_VLAN_FRAMES_GOOD_BAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXVLANFRGB` reader - This field indicates the number of received good and bad VLAN frames."]
pub type RXVLANFRGB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of received good and bad VLAN frames."]
    #[inline(always)]
    pub fn rxvlanfrgb(&self) -> RXVLANFRGB_R {
        RXVLANFRGB_R::new(self.bits)
    }
}
#[doc = "Receive Frame Count for Good and Bad VLAN Frames\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_vlan_frames_good_bad](index.html) module"]
pub struct RX_VLAN_FRAMES_GOOD_BAD_SPEC;
impl crate::RegisterSpec for RX_VLAN_FRAMES_GOOD_BAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_vlan_frames_good_bad::R](R) reader structure"]
impl crate::Readable for RX_VLAN_FRAMES_GOOD_BAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_VLAN_FRAMES_GOOD_BAD to value 0"]
impl crate::Resettable for RX_VLAN_FRAMES_GOOD_BAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
