#[doc = "Register `PEFLAG` reader"]
pub struct R(crate::R<PEFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEFLAG` writer"]
pub struct W(crate::W<PEFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEFLAG_SPEC>;
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
impl From<crate::W<PEFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Parity Error Flag for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPS_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPS` reader - Parity Error Flag for PSRAM"]
pub struct PEFPS_R(crate::FieldReader<bool, PEFPS_A>);
impl PEFPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFPS_A {
        match self.bits {
            false => PEFPS_A::VALUE1,
            true => PEFPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFPS_A::VALUE2
    }
}
impl core::ops::Deref for PEFPS_R {
    type Target = crate::FieldReader<bool, PEFPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFPS` writer - Parity Error Flag for PSRAM"]
pub struct PEFPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFPS_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFPS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Parity Error Flag for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFDS1_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFDS1` reader - Parity Error Flag for DSRAM1"]
pub struct PEFDS1_R(crate::FieldReader<bool, PEFDS1_A>);
impl PEFDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFDS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFDS1_A {
        match self.bits {
            false => PEFDS1_A::VALUE1,
            true => PEFDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFDS1_A::VALUE2
    }
}
impl core::ops::Deref for PEFDS1_R {
    type Target = crate::FieldReader<bool, PEFDS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFDS1` writer - Parity Error Flag for DSRAM1"]
pub struct PEFDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFDS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFDS1_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFDS1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Parity Error Flag for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFDS2_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFDS2_A> for bool {
    #[inline(always)]
    fn from(variant: PEFDS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFDS2` reader - Parity Error Flag for DSRAM2"]
pub struct PEFDS2_R(crate::FieldReader<bool, PEFDS2_A>);
impl PEFDS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFDS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFDS2_A {
        match self.bits {
            false => PEFDS2_A::VALUE1,
            true => PEFDS2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFDS2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFDS2_A::VALUE2
    }
}
impl core::ops::Deref for PEFDS2_R {
    type Target = crate::FieldReader<bool, PEFDS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFDS2` writer - Parity Error Flag for DSRAM2"]
pub struct PEFDS2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFDS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFDS2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFDS2_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFDS2_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Parity Error Flag for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU0_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU0` reader - Parity Error Flag for USIC0 Memory"]
pub struct PEFU0_R(crate::FieldReader<bool, PEFU0_A>);
impl PEFU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFU0_A {
        match self.bits {
            false => PEFU0_A::VALUE1,
            true => PEFU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFU0_A::VALUE2
    }
}
impl core::ops::Deref for PEFU0_R {
    type Target = crate::FieldReader<bool, PEFU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFU0` writer - Parity Error Flag for USIC0 Memory"]
pub struct PEFU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFU0_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFU0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Parity Error Flag for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU1_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU1` reader - Parity Error Flag for USIC1 Memory"]
pub struct PEFU1_R(crate::FieldReader<bool, PEFU1_A>);
impl PEFU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFU1_A {
        match self.bits {
            false => PEFU1_A::VALUE1,
            true => PEFU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFU1_A::VALUE2
    }
}
impl core::ops::Deref for PEFU1_R {
    type Target = crate::FieldReader<bool, PEFU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFU1` writer - Parity Error Flag for USIC1 Memory"]
pub struct PEFU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFU1_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFU1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Parity Error Flag for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFU2_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFU2_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU2` reader - Parity Error Flag for USIC2 Memory"]
pub struct PEFU2_R(crate::FieldReader<bool, PEFU2_A>);
impl PEFU2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFU2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFU2_A {
        match self.bits {
            false => PEFU2_A::VALUE1,
            true => PEFU2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFU2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFU2_A::VALUE2
    }
}
impl core::ops::Deref for PEFU2_R {
    type Target = crate::FieldReader<bool, PEFU2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFU2` writer - Parity Error Flag for USIC2 Memory"]
pub struct PEFU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFU2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFU2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFU2_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFU2_A::VALUE2)
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
#[doc = "Parity Error Flag for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFMC_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEFMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFMC` reader - Parity Error Flag for MultiCAN Memory"]
pub struct PEFMC_R(crate::FieldReader<bool, PEFMC_A>);
impl PEFMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFMC_A {
        match self.bits {
            false => PEFMC_A::VALUE1,
            true => PEFMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFMC_A::VALUE2
    }
}
impl core::ops::Deref for PEFMC_R {
    type Target = crate::FieldReader<bool, PEFMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFMC` writer - Parity Error Flag for MultiCAN Memory"]
pub struct PEFMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFMC_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFMC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Parity Error Flag for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFPPRF_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPPRF` reader - Parity Error Flag for PMU Prefetch Memory"]
pub struct PEFPPRF_R(crate::FieldReader<bool, PEFPPRF_A>);
impl PEFPPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEFPPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEFPPRF_A {
        match self.bits {
            false => PEFPPRF_A::VALUE1,
            true => PEFPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEFPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEFPPRF_A::VALUE2
    }
}
impl core::ops::Deref for PEFPPRF_R {
    type Target = crate::FieldReader<bool, PEFPPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEFPPRF` writer - Parity Error Flag for PMU Prefetch Memory"]
pub struct PEFPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEFPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEFPPRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEFPPRF_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEFPPRF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Parity Error Flag for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEUSB_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEUSB` reader - Parity Error Flag for USB Memory"]
pub struct PEUSB_R(crate::FieldReader<bool, PEUSB_A>);
impl PEUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEUSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEUSB_A {
        match self.bits {
            false => PEUSB_A::VALUE1,
            true => PEUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEUSB_A::VALUE2
    }
}
impl core::ops::Deref for PEUSB_R {
    type Target = crate::FieldReader<bool, PEUSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEUSB` writer - Parity Error Flag for USB Memory"]
pub struct PEUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PEUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEUSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEUSB_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEUSB_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Parity Error Flag for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0TX_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEETH0TX_A> for bool {
    #[inline(always)]
    fn from(variant: PEETH0TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEETH0TX` reader - Parity Error Flag for ETH TX Memory"]
pub struct PEETH0TX_R(crate::FieldReader<bool, PEETH0TX_A>);
impl PEETH0TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEETH0TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEETH0TX_A {
        match self.bits {
            false => PEETH0TX_A::VALUE1,
            true => PEETH0TX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEETH0TX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEETH0TX_A::VALUE2
    }
}
impl core::ops::Deref for PEETH0TX_R {
    type Target = crate::FieldReader<bool, PEETH0TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEETH0TX` writer - Parity Error Flag for ETH TX Memory"]
pub struct PEETH0TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEETH0TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEETH0TX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEETH0TX_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEETH0TX_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Parity Error Flag for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEETH0RX_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEETH0RX_A> for bool {
    #[inline(always)]
    fn from(variant: PEETH0RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEETH0RX` reader - Parity Error Flag for ETH RX Memory"]
pub struct PEETH0RX_R(crate::FieldReader<bool, PEETH0RX_A>);
impl PEETH0RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEETH0RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEETH0RX_A {
        match self.bits {
            false => PEETH0RX_A::VALUE1,
            true => PEETH0RX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PEETH0RX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PEETH0RX_A::VALUE2
    }
}
impl core::ops::Deref for PEETH0RX_R {
    type Target = crate::FieldReader<bool, PEETH0RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEETH0RX` writer - Parity Error Flag for ETH RX Memory"]
pub struct PEETH0RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PEETH0RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEETH0RX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEETH0RX_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEETH0RX_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Parity Error Flag for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD0_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PESD0_A> for bool {
    #[inline(always)]
    fn from(variant: PESD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESD0` reader - Parity Error Flag for SDMMC Memory 0"]
pub struct PESD0_R(crate::FieldReader<bool, PESD0_A>);
impl PESD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD0_A {
        match self.bits {
            false => PESD0_A::VALUE1,
            true => PESD0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PESD0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PESD0_A::VALUE2
    }
}
impl core::ops::Deref for PESD0_R {
    type Target = crate::FieldReader<bool, PESD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESD0` writer - Parity Error Flag for SDMMC Memory 0"]
pub struct PESD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PESD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PESD0_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PESD0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Parity Error Flag for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PESD1_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PESD1_A> for bool {
    #[inline(always)]
    fn from(variant: PESD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESD1` reader - Parity Error Flag for SDMMC Memory 1"]
pub struct PESD1_R(crate::FieldReader<bool, PESD1_A>);
impl PESD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PESD1_A {
        match self.bits {
            false => PESD1_A::VALUE1,
            true => PESD1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PESD1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PESD1_A::VALUE2
    }
}
impl core::ops::Deref for PESD1_R {
    type Target = crate::FieldReader<bool, PESD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESD1` writer - Parity Error Flag for SDMMC Memory 1"]
pub struct PESD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PESD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PESD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PESD1_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PESD1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&self) -> PEFPS_R {
        PEFPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&self) -> PEFDS1_R {
        PEFDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity Error Flag for DSRAM2"]
    #[inline(always)]
    pub fn pefds2(&self) -> PEFDS2_R {
        PEFDS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&self) -> PEFU0_R {
        PEFU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&self) -> PEFU1_R {
        PEFU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Parity Error Flag for USIC2 Memory"]
    #[inline(always)]
    pub fn pefu2(&self) -> PEFU2_R {
        PEFU2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&self) -> PEFMC_R {
        PEFMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&self) -> PEFPPRF_R {
        PEFPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&self) -> PEUSB_R {
        PEUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    pub fn peeth0tx(&self) -> PEETH0TX_R {
        PEETH0TX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    pub fn peeth0rx(&self) -> PEETH0RX_R {
        PEETH0RX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    pub fn pesd0(&self) -> PESD0_R {
        PESD0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    pub fn pesd1(&self) -> PESD1_R {
        PESD1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&mut self) -> PEFPS_W {
        PEFPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&mut self) -> PEFDS1_W {
        PEFDS1_W { w: self }
    }
    #[doc = "Bit 2 - Parity Error Flag for DSRAM2"]
    #[inline(always)]
    pub fn pefds2(&mut self) -> PEFDS2_W {
        PEFDS2_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&mut self) -> PEFU0_W {
        PEFU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&mut self) -> PEFU1_W {
        PEFU1_W { w: self }
    }
    #[doc = "Bit 10 - Parity Error Flag for USIC2 Memory"]
    #[inline(always)]
    pub fn pefu2(&mut self) -> PEFU2_W {
        PEFU2_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&mut self) -> PEFMC_W {
        PEFMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&mut self) -> PEFPPRF_W {
        PEFPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&mut self) -> PEUSB_W {
        PEUSB_W { w: self }
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    pub fn peeth0tx(&mut self) -> PEETH0TX_W {
        PEETH0TX_W { w: self }
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    pub fn peeth0rx(&mut self) -> PEETH0RX_W {
        PEETH0RX_W { w: self }
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    pub fn pesd0(&mut self) -> PESD0_W {
        PESD0_W { w: self }
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    pub fn pesd1(&mut self) -> PESD1_W {
        PESD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peflag](index.html) module"]
pub struct PEFLAG_SPEC;
impl crate::RegisterSpec for PEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peflag::R](R) reader structure"]
impl crate::Readable for PEFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peflag::W](W) writer structure"]
impl crate::Writable for PEFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEFLAG to value 0"]
impl crate::Resettable for PEFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
