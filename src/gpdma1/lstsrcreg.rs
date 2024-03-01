#[doc = "Register `LSTSRCREG` reader"]
pub type R = crate::R<LstsrcregSpec>;
#[doc = "Register `LSTSRCREG` writer"]
pub type W = crate::W<LstsrcregSpec>;
#[doc = "Source last request for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Source last request for channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Value1,
            true => Ch0::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch0::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch0::Value2
    }
}
#[doc = "Field `CH0` writer - Source last request for channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Value2)
    }
}
#[doc = "Source last request for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Source last request for channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Value1,
            true => Ch1::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch1::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch1::Value2
    }
}
#[doc = "Field `CH1` writer - Source last request for channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Value2)
    }
}
#[doc = "Source last request for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Source last request for channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Value1,
            true => Ch2::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch2::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch2::Value2
    }
}
#[doc = "Field `CH2` writer - Source last request for channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Value2)
    }
}
#[doc = "Source last request for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Not last transaction in current block"]
    Value1 = 0,
    #[doc = "1: Last transaction in current block"]
    Value2 = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Source last request for channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Value1,
            true => Ch3::Value2,
        }
    }
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ch3::Value1
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ch3::Value2
    }
}
#[doc = "Field `CH3` writer - Source last request for channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last transaction in current block"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value1)
    }
    #[doc = "Last transaction in current block"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Value2)
    }
}
#[doc = "Source last transaction request write enable for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh0 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh0> for bool {
    #[inline(always)]
    fn from(variant: WeCh0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH0` writer - Source last transaction request write enable for channel 0"]
pub type WeCh0W<'a, REG> = crate::BitWriter<'a, REG, WeCh0>;
impl<'a, REG> WeCh0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh0::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh0::Value2)
    }
}
#[doc = "Source last transaction request write enable for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh1 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh1> for bool {
    #[inline(always)]
    fn from(variant: WeCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH1` writer - Source last transaction request write enable for channel 1"]
pub type WeCh1W<'a, REG> = crate::BitWriter<'a, REG, WeCh1>;
impl<'a, REG> WeCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh1::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh1::Value2)
    }
}
#[doc = "Source last transaction request write enable for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh2 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh2> for bool {
    #[inline(always)]
    fn from(variant: WeCh2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH2` writer - Source last transaction request write enable for channel 2"]
pub type WeCh2W<'a, REG> = crate::BitWriter<'a, REG, WeCh2>;
impl<'a, REG> WeCh2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh2::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh2::Value2)
    }
}
#[doc = "Source last transaction request write enable for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeCh3 {
    #[doc = "0: write disabled"]
    Value1 = 0,
    #[doc = "1: write enabled"]
    Value2 = 1,
}
impl From<WeCh3> for bool {
    #[inline(always)]
    fn from(variant: WeCh3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE_CH3` writer - Source last transaction request write enable for channel 3"]
pub type WeCh3W<'a, REG> = crate::BitWriter<'a, REG, WeCh3>;
impl<'a, REG> WeCh3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh3::Value1)
    }
    #[doc = "write enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WeCh3::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source last request for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<LstsrcregSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Source last request for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<LstsrcregSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Source last request for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<LstsrcregSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Source last request for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<LstsrcregSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 8 - Source last transaction request write enable for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch0(&mut self) -> WeCh0W<LstsrcregSpec> {
        WeCh0W::new(self, 8)
    }
    #[doc = "Bit 9 - Source last transaction request write enable for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch1(&mut self) -> WeCh1W<LstsrcregSpec> {
        WeCh1W::new(self, 9)
    }
    #[doc = "Bit 10 - Source last transaction request write enable for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch2(&mut self) -> WeCh2W<LstsrcregSpec> {
        WeCh2W::new(self, 10)
    }
    #[doc = "Bit 11 - Source last transaction request write enable for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn we_ch3(&mut self) -> WeCh3W<LstsrcregSpec> {
        WeCh3W::new(self, 11)
    }
}
#[doc = "Last Source Transaction Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstsrcreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstsrcreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LstsrcregSpec;
impl crate::RegisterSpec for LstsrcregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lstsrcreg::R`](R) reader structure"]
impl crate::Readable for LstsrcregSpec {}
#[doc = "`write(|w| ..)` method takes [`lstsrcreg::W`](W) writer structure"]
impl crate::Writable for LstsrcregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LSTSRCREG to value 0"]
impl crate::Resettable for LstsrcregSpec {
    const RESET_VALUE: u32 = 0;
}
