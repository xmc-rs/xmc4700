#[doc = "Register `TX_VLAN_FRAMES_GOOD` reader"]
pub type R = crate::R<TxVlanFramesGoodSpec>;
#[doc = "Field `TXVLANG` reader - This register maintains the number of transmitted good VLAN frames, exclusive of retried frames."]
pub type TxvlangR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register maintains the number of transmitted good VLAN frames, exclusive of retried frames."]
    #[inline(always)]
    pub fn txvlang(&self) -> TxvlangR {
        TxvlangR::new(self.bits)
    }
}
#[doc = "Transmit Frame Count for Good VLAN Frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_vlan_frames_good::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxVlanFramesGoodSpec;
impl crate::RegisterSpec for TxVlanFramesGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_vlan_frames_good::R`](R) reader structure"]
impl crate::Readable for TxVlanFramesGoodSpec {}
#[doc = "`reset()` method sets TX_VLAN_FRAMES_GOOD to value 0"]
impl crate::Resettable for TxVlanFramesGoodSpec {
    const RESET_VALUE: u32 = 0;
}
