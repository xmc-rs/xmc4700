#[doc = "Register `RXIPV6_HEADER_ERROR_OCTETS` reader"]
pub type R = crate::R<Rxipv6HeaderErrorOctetsSpec>;
#[doc = "Field `RXIPV6HDRERROCT` reader - This field indicates the number of bytes received in IPv6 datagrams with header errors (length or version mismatch). The value in the IPv6 headers Length field is used to update this counter."]
pub type Rxipv6hdrerroctR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of bytes received in IPv6 datagrams with header errors (length or version mismatch). The value in the IPv6 headers Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6hdrerroct(&self) -> Rxipv6hdrerroctR {
        Rxipv6hdrerroctR::new(self.bits)
    }
}
#[doc = "Receive IPV6 Header Error Octet Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxipv6_header_error_octets::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxipv6HeaderErrorOctetsSpec;
impl crate::RegisterSpec for Rxipv6HeaderErrorOctetsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxipv6_header_error_octets::R`](R) reader structure"]
impl crate::Readable for Rxipv6HeaderErrorOctetsSpec {}
#[doc = "`reset()` method sets RXIPV6_HEADER_ERROR_OCTETS to value 0"]
impl crate::Resettable for Rxipv6HeaderErrorOctetsSpec {
    const RESET_VALUE: u32 = 0;
}
