#[doc = "Register `PDISC` reader"]
pub type R = crate::R<PdiscSpec>;
#[doc = "Register `PDISC` writer"]
pub type W = crate::W<PdiscSpec>;
#[doc = "Pad Disable for Port 14 Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis0 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 analog input 0 selected."]
    Value2 = 1,
}
impl From<Pdis0> for bool {
    #[inline(always)]
    fn from(variant: Pdis0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS0` reader - Pad Disable for Port 14 Pin 0"]
pub type Pdis0R = crate::BitReader<Pdis0>;
impl Pdis0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis0 {
        match self.bits {
            false => Pdis0::Value1,
            true => Pdis0::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis0::Value1
    }
    #[doc = "Pad is disabled, ADC 0 analog input 0 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis0::Value2
    }
}
#[doc = "Field `PDIS0` writer - Pad Disable for Port 14 Pin 0"]
pub type Pdis0W<'a, REG> = crate::BitWriter<'a, REG, Pdis0>;
impl<'a, REG> Pdis0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis0::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 analog input 0 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis0::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis1 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 analog input 1 selected."]
    Value2 = 1,
}
impl From<Pdis1> for bool {
    #[inline(always)]
    fn from(variant: Pdis1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS1` reader - Pad Disable for Port 14 Pin 1"]
pub type Pdis1R = crate::BitReader<Pdis1>;
impl Pdis1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis1 {
        match self.bits {
            false => Pdis1::Value1,
            true => Pdis1::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis1::Value1
    }
    #[doc = "Pad is disabled, ADC 0 analog input 1 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis1::Value2
    }
}
#[doc = "Field `PDIS1` writer - Pad Disable for Port 14 Pin 1"]
pub type Pdis1W<'a, REG> = crate::BitWriter<'a, REG, Pdis1>;
impl<'a, REG> Pdis1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis1::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 analog input 1 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis1::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis2 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 and ADC 1 analog input 2 selected."]
    Value2 = 1,
}
impl From<Pdis2> for bool {
    #[inline(always)]
    fn from(variant: Pdis2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS2` reader - Pad Disable for Port 14 Pin 2"]
pub type Pdis2R = crate::BitReader<Pdis2>;
impl Pdis2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis2 {
        match self.bits {
            false => Pdis2::Value1,
            true => Pdis2::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis2::Value1
    }
    #[doc = "Pad is disabled, ADC 0 and ADC 1 analog input 2 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis2::Value2
    }
}
#[doc = "Field `PDIS2` writer - Pad Disable for Port 14 Pin 2"]
pub type Pdis2W<'a, REG> = crate::BitWriter<'a, REG, Pdis2>;
impl<'a, REG> Pdis2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis2::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 and ADC 1 analog input 2 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis2::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis3 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 and ADC 1 analog input 3 selected."]
    Value2 = 1,
}
impl From<Pdis3> for bool {
    #[inline(always)]
    fn from(variant: Pdis3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS3` reader - Pad Disable for Port 14 Pin 3"]
pub type Pdis3R = crate::BitReader<Pdis3>;
impl Pdis3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis3 {
        match self.bits {
            false => Pdis3::Value1,
            true => Pdis3::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis3::Value1
    }
    #[doc = "Pad is disabled, ADC 0 and ADC 1 analog input 3 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis3::Value2
    }
}
#[doc = "Field `PDIS3` writer - Pad Disable for Port 14 Pin 3"]
pub type Pdis3W<'a, REG> = crate::BitWriter<'a, REG, Pdis3>;
impl<'a, REG> Pdis3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis3::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 and ADC 1 analog input 3 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis3::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis4 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 analog input 4 and ADC 2 analog input 0 and DAC Reference selected."]
    Value2 = 1,
}
impl From<Pdis4> for bool {
    #[inline(always)]
    fn from(variant: Pdis4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS4` reader - Pad Disable for Port 14 Pin 4"]
pub type Pdis4R = crate::BitReader<Pdis4>;
impl Pdis4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis4 {
        match self.bits {
            false => Pdis4::Value1,
            true => Pdis4::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis4::Value1
    }
    #[doc = "Pad is disabled, ADC 0 analog input 4 and ADC 2 analog input 0 and DAC Reference selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis4::Value2
    }
}
#[doc = "Field `PDIS4` writer - Pad Disable for Port 14 Pin 4"]
pub type Pdis4W<'a, REG> = crate::BitWriter<'a, REG, Pdis4>;
impl<'a, REG> Pdis4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis4::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 analog input 4 and ADC 2 analog input 0 and DAC Reference selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis4::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis5 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 analog input 5 and ADC 2 analog input 1 selected."]
    Value2 = 1,
}
impl From<Pdis5> for bool {
    #[inline(always)]
    fn from(variant: Pdis5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS5` reader - Pad Disable for Port 14 Pin 5"]
pub type Pdis5R = crate::BitReader<Pdis5>;
impl Pdis5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis5 {
        match self.bits {
            false => Pdis5::Value1,
            true => Pdis5::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis5::Value1
    }
    #[doc = "Pad is disabled, ADC 0 analog input 5 and ADC 2 analog input 1 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis5::Value2
    }
}
#[doc = "Field `PDIS5` writer - Pad Disable for Port 14 Pin 5"]
pub type Pdis5W<'a, REG> = crate::BitWriter<'a, REG, Pdis5>;
impl<'a, REG> Pdis5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis5::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 analog input 5 and ADC 2 analog input 1 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis5::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis6 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 0 analog input 6 selected."]
    Value2 = 1,
}
impl From<Pdis6> for bool {
    #[inline(always)]
    fn from(variant: Pdis6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS6` reader - Pad Disable for Port 14 Pin 6"]
pub type Pdis6R = crate::BitReader<Pdis6>;
impl Pdis6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis6 {
        match self.bits {
            false => Pdis6::Value1,
            true => Pdis6::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis6::Value1
    }
    #[doc = "Pad is disabled, ADC 0 analog input 6 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis6::Value2
    }
}
#[doc = "Field `PDIS6` writer - Pad Disable for Port 14 Pin 6"]
pub type Pdis6W<'a, REG> = crate::BitWriter<'a, REG, Pdis6>;
impl<'a, REG> Pdis6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis6::Value1)
    }
    #[doc = "Pad is disabled, ADC 0 analog input 6 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis6::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis7 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC0 analog input 7 selected."]
    Value2 = 1,
}
impl From<Pdis7> for bool {
    #[inline(always)]
    fn from(variant: Pdis7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS7` reader - Pad Disable for Port 14 Pin 7"]
pub type Pdis7R = crate::BitReader<Pdis7>;
impl Pdis7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis7 {
        match self.bits {
            false => Pdis7::Value1,
            true => Pdis7::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis7::Value1
    }
    #[doc = "Pad is disabled, ADC0 analog input 7 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis7::Value2
    }
}
#[doc = "Field `PDIS7` writer - Pad Disable for Port 14 Pin 7"]
pub type Pdis7W<'a, REG> = crate::BitWriter<'a, REG, Pdis7>;
impl<'a, REG> Pdis7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis7::Value1)
    }
    #[doc = "Pad is disabled, ADC0 analog input 7 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis7::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis8 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 0 and ADC 2 analog input 4 and DAC output 0 selected."]
    Value2 = 1,
}
impl From<Pdis8> for bool {
    #[inline(always)]
    fn from(variant: Pdis8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS8` reader - Pad Disable for Port 14 Pin 8"]
pub type Pdis8R = crate::BitReader<Pdis8>;
impl Pdis8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis8 {
        match self.bits {
            false => Pdis8::Value1,
            true => Pdis8::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis8::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 0 and ADC 2 analog input 4 and DAC output 0 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis8::Value2
    }
}
#[doc = "Field `PDIS8` writer - Pad Disable for Port 14 Pin 8"]
pub type Pdis8W<'a, REG> = crate::BitWriter<'a, REG, Pdis8>;
impl<'a, REG> Pdis8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis8::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 0 and ADC 2 analog input 4 and DAC output 0 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis8::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis9 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 1 and ADC 2 analog input 5 and DAC output 1 selected."]
    Value2 = 1,
}
impl From<Pdis9> for bool {
    #[inline(always)]
    fn from(variant: Pdis9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS9` reader - Pad Disable for Port 14 Pin 9"]
pub type Pdis9R = crate::BitReader<Pdis9>;
impl Pdis9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis9 {
        match self.bits {
            false => Pdis9::Value1,
            true => Pdis9::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis9::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 1 and ADC 2 analog input 5 and DAC output 1 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis9::Value2
    }
}
#[doc = "Field `PDIS9` writer - Pad Disable for Port 14 Pin 9"]
pub type Pdis9W<'a, REG> = crate::BitWriter<'a, REG, Pdis9>;
impl<'a, REG> Pdis9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis9::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 1 and ADC 2 analog input 5 and DAC output 1 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis9::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis12 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 4 selected."]
    Value2 = 1,
}
impl From<Pdis12> for bool {
    #[inline(always)]
    fn from(variant: Pdis12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS12` reader - Pad Disable for Port 14 Pin 12"]
pub type Pdis12R = crate::BitReader<Pdis12>;
impl Pdis12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis12 {
        match self.bits {
            false => Pdis12::Value1,
            true => Pdis12::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis12::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 4 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis12::Value2
    }
}
#[doc = "Field `PDIS12` writer - Pad Disable for Port 14 Pin 12"]
pub type Pdis12W<'a, REG> = crate::BitWriter<'a, REG, Pdis12>;
impl<'a, REG> Pdis12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis12::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 4 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis12::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis13 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 5 selected."]
    Value2 = 1,
}
impl From<Pdis13> for bool {
    #[inline(always)]
    fn from(variant: Pdis13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS13` reader - Pad Disable for Port 14 Pin 13"]
pub type Pdis13R = crate::BitReader<Pdis13>;
impl Pdis13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis13 {
        match self.bits {
            false => Pdis13::Value1,
            true => Pdis13::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis13::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 5 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis13::Value2
    }
}
#[doc = "Field `PDIS13` writer - Pad Disable for Port 14 Pin 13"]
pub type Pdis13W<'a, REG> = crate::BitWriter<'a, REG, Pdis13>;
impl<'a, REG> Pdis13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis13::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 5 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis13::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis14 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 6 selected."]
    Value2 = 1,
}
impl From<Pdis14> for bool {
    #[inline(always)]
    fn from(variant: Pdis14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS14` reader - Pad Disable for Port 14 Pin 14"]
pub type Pdis14R = crate::BitReader<Pdis14>;
impl Pdis14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis14 {
        match self.bits {
            false => Pdis14::Value1,
            true => Pdis14::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis14::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 6 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis14::Value2
    }
}
#[doc = "Field `PDIS14` writer - Pad Disable for Port 14 Pin 14"]
pub type Pdis14W<'a, REG> = crate::BitWriter<'a, REG, Pdis14>;
impl<'a, REG> Pdis14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis14::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 6 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis14::Value2)
    }
}
#[doc = "Pad Disable for Port 14 Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdis15 {
    #[doc = "0: Pad is enabled, digital input selected."]
    Value1 = 0,
    #[doc = "1: Pad is disabled, ADC 1 analog input 7 selected."]
    Value2 = 1,
}
impl From<Pdis15> for bool {
    #[inline(always)]
    fn from(variant: Pdis15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDIS15` reader - Pad Disable for Port 14 Pin 15"]
pub type Pdis15R = crate::BitReader<Pdis15>;
impl Pdis15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdis15 {
        match self.bits {
            false => Pdis15::Value1,
            true => Pdis15::Value2,
        }
    }
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pdis15::Value1
    }
    #[doc = "Pad is disabled, ADC 1 analog input 7 selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pdis15::Value2
    }
}
#[doc = "Field `PDIS15` writer - Pad Disable for Port 14 Pin 15"]
pub type Pdis15W<'a, REG> = crate::BitWriter<'a, REG, Pdis15>;
impl<'a, REG> Pdis15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pad is enabled, digital input selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis15::Value1)
    }
    #[doc = "Pad is disabled, ADC 1 analog input 7 selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pdis15::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Pad Disable for Port 14 Pin 0"]
    #[inline(always)]
    pub fn pdis0(&self) -> Pdis0R {
        Pdis0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pad Disable for Port 14 Pin 1"]
    #[inline(always)]
    pub fn pdis1(&self) -> Pdis1R {
        Pdis1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pad Disable for Port 14 Pin 2"]
    #[inline(always)]
    pub fn pdis2(&self) -> Pdis2R {
        Pdis2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pad Disable for Port 14 Pin 3"]
    #[inline(always)]
    pub fn pdis3(&self) -> Pdis3R {
        Pdis3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pad Disable for Port 14 Pin 4"]
    #[inline(always)]
    pub fn pdis4(&self) -> Pdis4R {
        Pdis4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pad Disable for Port 14 Pin 5"]
    #[inline(always)]
    pub fn pdis5(&self) -> Pdis5R {
        Pdis5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pad Disable for Port 14 Pin 6"]
    #[inline(always)]
    pub fn pdis6(&self) -> Pdis6R {
        Pdis6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pad Disable for Port 14 Pin 7"]
    #[inline(always)]
    pub fn pdis7(&self) -> Pdis7R {
        Pdis7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pad Disable for Port 14 Pin 8"]
    #[inline(always)]
    pub fn pdis8(&self) -> Pdis8R {
        Pdis8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pad Disable for Port 14 Pin 9"]
    #[inline(always)]
    pub fn pdis9(&self) -> Pdis9R {
        Pdis9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Pad Disable for Port 14 Pin 12"]
    #[inline(always)]
    pub fn pdis12(&self) -> Pdis12R {
        Pdis12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pad Disable for Port 14 Pin 13"]
    #[inline(always)]
    pub fn pdis13(&self) -> Pdis13R {
        Pdis13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pad Disable for Port 14 Pin 14"]
    #[inline(always)]
    pub fn pdis14(&self) -> Pdis14R {
        Pdis14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pad Disable for Port 14 Pin 15"]
    #[inline(always)]
    pub fn pdis15(&self) -> Pdis15R {
        Pdis15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pad Disable for Port 14 Pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pdis0(&mut self) -> Pdis0W<PdiscSpec> {
        Pdis0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pad Disable for Port 14 Pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pdis1(&mut self) -> Pdis1W<PdiscSpec> {
        Pdis1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pad Disable for Port 14 Pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pdis2(&mut self) -> Pdis2W<PdiscSpec> {
        Pdis2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pad Disable for Port 14 Pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdis3(&mut self) -> Pdis3W<PdiscSpec> {
        Pdis3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pad Disable for Port 14 Pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pdis4(&mut self) -> Pdis4W<PdiscSpec> {
        Pdis4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pad Disable for Port 14 Pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pdis5(&mut self) -> Pdis5W<PdiscSpec> {
        Pdis5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pad Disable for Port 14 Pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pdis6(&mut self) -> Pdis6W<PdiscSpec> {
        Pdis6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pad Disable for Port 14 Pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pdis7(&mut self) -> Pdis7W<PdiscSpec> {
        Pdis7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pad Disable for Port 14 Pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pdis8(&mut self) -> Pdis8W<PdiscSpec> {
        Pdis8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pad Disable for Port 14 Pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pdis9(&mut self) -> Pdis9W<PdiscSpec> {
        Pdis9W::new(self, 9)
    }
    #[doc = "Bit 12 - Pad Disable for Port 14 Pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pdis12(&mut self) -> Pdis12W<PdiscSpec> {
        Pdis12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pad Disable for Port 14 Pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pdis13(&mut self) -> Pdis13W<PdiscSpec> {
        Pdis13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pad Disable for Port 14 Pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pdis14(&mut self) -> Pdis14W<PdiscSpec> {
        Pdis14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pad Disable for Port 14 Pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pdis15(&mut self) -> Pdis15W<PdiscSpec> {
        Pdis15W::new(self, 15)
    }
}
#[doc = "Port 14 Pin Function Decision Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdiscSpec;
impl crate::RegisterSpec for PdiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdisc::R`](R) reader structure"]
impl crate::Readable for PdiscSpec {}
#[doc = "`write(|w| ..)` method takes [`pdisc::W`](W) writer structure"]
impl crate::Writable for PdiscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDISC to value 0"]
impl crate::Resettable for PdiscSpec {
    const RESET_VALUE: u32 = 0;
}
