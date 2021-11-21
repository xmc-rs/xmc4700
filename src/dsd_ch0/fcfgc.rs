#[doc = "Register `FCFGC` reader"]
pub struct R(crate::R<FCFGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFGC` writer"]
pub struct W(crate::W<FCFGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FCFGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFMDF` reader - CIC Filter (Main Chain) Decimation Factor"]
pub struct CFMDF_R(crate::FieldReader<u8, u8>);
impl CFMDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFMDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFMDF` writer - CIC Filter (Main Chain) Decimation Factor"]
pub struct CFMDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "CIC Filter (Main Chain) Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFMC_A {
    #[doc = "0: CIC1"]
    VALUE1 = 0,
    #[doc = "1: CIC2"]
    VALUE2 = 1,
    #[doc = "2: CIC3"]
    VALUE3 = 2,
    #[doc = "3: CICF"]
    VALUE4 = 3,
}
impl From<CFMC_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFMC` reader - CIC Filter (Main Chain) Configuration"]
pub struct CFMC_R(crate::FieldReader<u8, CFMC_A>);
impl CFMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFMC_A {
        match self.bits {
            0 => CFMC_A::VALUE1,
            1 => CFMC_A::VALUE2,
            2 => CFMC_A::VALUE3,
            3 => CFMC_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFMC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CFMC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CFMC_A::VALUE4
    }
}
impl core::ops::Deref for CFMC_R {
    type Target = crate::FieldReader<u8, CFMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFMC` writer - CIC Filter (Main Chain) Configuration"]
pub struct CFMC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFMC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CIC1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE1)
    }
    #[doc = "CIC2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE2)
    }
    #[doc = "CIC3"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE3)
    }
    #[doc = "CICF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CFMC_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "CIC Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEN_A {
    #[doc = "0: CIC filter disabled and bypassed"]
    VALUE1 = 0,
    #[doc = "1: Enable CIC filter"]
    VALUE2 = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEN` reader - CIC Filter Enable"]
pub struct CFEN_R(crate::FieldReader<bool, CFEN_A>);
impl CFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::VALUE1,
            true => CFEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFEN_A::VALUE2
    }
}
impl core::ops::Deref for CFEN_R {
    type Target = crate::FieldReader<bool, CFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEN` writer - CIC Filter Enable"]
pub struct CFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CIC filter disabled and bypassed"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE1)
    }
    #[doc = "Enable CIC filter"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Service Request Generation Main Chain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRGM_A {
    #[doc = "0: Never, service requests disabled"]
    VALUE1 = 0,
    #[doc = "3: Always, for each new result value"]
    VALUE4 = 3,
}
impl From<SRGM_A> for u8 {
    #[inline(always)]
    fn from(variant: SRGM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRGM` reader - Service Request Generation Main Chain"]
pub struct SRGM_R(crate::FieldReader<u8, SRGM_A>);
impl SRGM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRGM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRGM_A> {
        match self.bits {
            0 => Some(SRGM_A::VALUE1),
            3 => Some(SRGM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SRGM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SRGM_A::VALUE4
    }
}
impl core::ops::Deref for SRGM_R {
    type Target = crate::FieldReader<u8, SRGM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRGM` writer - Service Request Generation Main Chain"]
pub struct SRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRGM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Never, service requests disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE1)
    }
    #[doc = "Always, for each new result value"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRGM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `CFMSV` reader - CIC Filter (Main Chain) Start Value"]
pub struct CFMSV_R(crate::FieldReader<u8, u8>);
impl CFMSV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMSV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFMSV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFMSV` writer - CIC Filter (Main Chain) Start Value"]
pub struct CFMSV_W<'a> {
    w: &'a mut W,
}
impl<'a> CFMSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CFMDCNT` reader - CIC Filter (Main Chain) Decimation Counter"]
pub struct CFMDCNT_R(crate::FieldReader<u8, u8>);
impl CFMDCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFMDCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFMDCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&self) -> CFMDF_R {
        CFMDF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&self) -> CFMC_R {
        CFMC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&self) -> SRGM_R {
        SRGM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&self) -> CFMSV_R {
        CFMSV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CIC Filter (Main Chain) Decimation Counter"]
    #[inline(always)]
    pub fn cfmdcnt(&self) -> CFMDCNT_R {
        CFMDCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CIC Filter (Main Chain) Decimation Factor"]
    #[inline(always)]
    pub fn cfmdf(&mut self) -> CFMDF_W {
        CFMDF_W { w: self }
    }
    #[doc = "Bits 8:9 - CIC Filter (Main Chain) Configuration"]
    #[inline(always)]
    pub fn cfmc(&mut self) -> CFMC_W {
        CFMC_W { w: self }
    }
    #[doc = "Bit 10 - CIC Filter Enable"]
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W {
        CFEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Service Request Generation Main Chain"]
    #[inline(always)]
    pub fn srgm(&mut self) -> SRGM_W {
        SRGM_W { w: self }
    }
    #[doc = "Bits 16:23 - CIC Filter (Main Chain) Start Value"]
    #[inline(always)]
    pub fn cfmsv(&mut self) -> CFMSV_W {
        CFMSV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Configuration Register, Main CIC Filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfgc](index.html) module"]
pub struct FCFGC_SPEC;
impl crate::RegisterSpec for FCFGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfgc::R](R) reader structure"]
impl crate::Readable for FCFGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfgc::W](W) writer structure"]
impl crate::Writable for FCFGC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFGC to value 0"]
impl crate::Resettable for FCFGC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
