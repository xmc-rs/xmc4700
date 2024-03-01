#[doc = "Register `PEFLAG` reader"]
pub type R = crate::R<PeflagSpec>;
#[doc = "Register `PEFLAG` writer"]
pub type W = crate::W<PeflagSpec>;
#[doc = "Parity Error Flag for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefps {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefps> for bool {
    #[inline(always)]
    fn from(variant: Pefps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPS` reader - Parity Error Flag for PSRAM"]
pub type PefpsR = crate::BitReader<Pefps>;
impl PefpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefps {
        match self.bits {
            false => Pefps::Value1,
            true => Pefps::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefps::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefps::Value2
    }
}
#[doc = "Field `PEFPS` writer - Parity Error Flag for PSRAM"]
pub type PefpsW<'a, REG> = crate::BitWriter<'a, REG, Pefps>;
impl<'a, REG> PefpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefps::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefps::Value2)
    }
}
#[doc = "Parity Error Flag for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefds1 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefds1> for bool {
    #[inline(always)]
    fn from(variant: Pefds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFDS1` reader - Parity Error Flag for DSRAM1"]
pub type Pefds1R = crate::BitReader<Pefds1>;
impl Pefds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefds1 {
        match self.bits {
            false => Pefds1::Value1,
            true => Pefds1::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefds1::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefds1::Value2
    }
}
#[doc = "Field `PEFDS1` writer - Parity Error Flag for DSRAM1"]
pub type Pefds1W<'a, REG> = crate::BitWriter<'a, REG, Pefds1>;
impl<'a, REG> Pefds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefds1::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefds1::Value2)
    }
}
#[doc = "Parity Error Flag for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefds2 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefds2> for bool {
    #[inline(always)]
    fn from(variant: Pefds2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFDS2` reader - Parity Error Flag for DSRAM2"]
pub type Pefds2R = crate::BitReader<Pefds2>;
impl Pefds2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefds2 {
        match self.bits {
            false => Pefds2::Value1,
            true => Pefds2::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefds2::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefds2::Value2
    }
}
#[doc = "Field `PEFDS2` writer - Parity Error Flag for DSRAM2"]
pub type Pefds2W<'a, REG> = crate::BitWriter<'a, REG, Pefds2>;
impl<'a, REG> Pefds2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefds2::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefds2::Value2)
    }
}
#[doc = "Parity Error Flag for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefu0 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefu0> for bool {
    #[inline(always)]
    fn from(variant: Pefu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU0` reader - Parity Error Flag for USIC0 Memory"]
pub type Pefu0R = crate::BitReader<Pefu0>;
impl Pefu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefu0 {
        match self.bits {
            false => Pefu0::Value1,
            true => Pefu0::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefu0::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefu0::Value2
    }
}
#[doc = "Field `PEFU0` writer - Parity Error Flag for USIC0 Memory"]
pub type Pefu0W<'a, REG> = crate::BitWriter<'a, REG, Pefu0>;
impl<'a, REG> Pefu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu0::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu0::Value2)
    }
}
#[doc = "Parity Error Flag for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefu1 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefu1> for bool {
    #[inline(always)]
    fn from(variant: Pefu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU1` reader - Parity Error Flag for USIC1 Memory"]
pub type Pefu1R = crate::BitReader<Pefu1>;
impl Pefu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefu1 {
        match self.bits {
            false => Pefu1::Value1,
            true => Pefu1::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefu1::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefu1::Value2
    }
}
#[doc = "Field `PEFU1` writer - Parity Error Flag for USIC1 Memory"]
pub type Pefu1W<'a, REG> = crate::BitWriter<'a, REG, Pefu1>;
impl<'a, REG> Pefu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu1::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu1::Value2)
    }
}
#[doc = "Parity Error Flag for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefu2 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefu2> for bool {
    #[inline(always)]
    fn from(variant: Pefu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU2` reader - Parity Error Flag for USIC2 Memory"]
pub type Pefu2R = crate::BitReader<Pefu2>;
impl Pefu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefu2 {
        match self.bits {
            false => Pefu2::Value1,
            true => Pefu2::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefu2::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefu2::Value2
    }
}
#[doc = "Field `PEFU2` writer - Parity Error Flag for USIC2 Memory"]
pub type Pefu2W<'a, REG> = crate::BitWriter<'a, REG, Pefu2>;
impl<'a, REG> Pefu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu2::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefu2::Value2)
    }
}
#[doc = "Parity Error Flag for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefmc {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefmc> for bool {
    #[inline(always)]
    fn from(variant: Pefmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFMC` reader - Parity Error Flag for MultiCAN Memory"]
pub type PefmcR = crate::BitReader<Pefmc>;
impl PefmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefmc {
        match self.bits {
            false => Pefmc::Value1,
            true => Pefmc::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefmc::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefmc::Value2
    }
}
#[doc = "Field `PEFMC` writer - Parity Error Flag for MultiCAN Memory"]
pub type PefmcW<'a, REG> = crate::BitWriter<'a, REG, Pefmc>;
impl<'a, REG> PefmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefmc::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefmc::Value2)
    }
}
#[doc = "Parity Error Flag for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pefpprf {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pefpprf> for bool {
    #[inline(always)]
    fn from(variant: Pefpprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPPRF` reader - Parity Error Flag for PMU Prefetch Memory"]
pub type PefpprfR = crate::BitReader<Pefpprf>;
impl PefpprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pefpprf {
        match self.bits {
            false => Pefpprf::Value1,
            true => Pefpprf::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pefpprf::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pefpprf::Value2
    }
}
#[doc = "Field `PEFPPRF` writer - Parity Error Flag for PMU Prefetch Memory"]
pub type PefpprfW<'a, REG> = crate::BitWriter<'a, REG, Pefpprf>;
impl<'a, REG> PefpprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pefpprf::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pefpprf::Value2)
    }
}
#[doc = "Parity Error Flag for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peusb {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Peusb> for bool {
    #[inline(always)]
    fn from(variant: Peusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEUSB` reader - Parity Error Flag for USB Memory"]
pub type PeusbR = crate::BitReader<Peusb>;
impl PeusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peusb {
        match self.bits {
            false => Peusb::Value1,
            true => Peusb::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peusb::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peusb::Value2
    }
}
#[doc = "Field `PEUSB` writer - Parity Error Flag for USB Memory"]
pub type PeusbW<'a, REG> = crate::BitWriter<'a, REG, Peusb>;
impl<'a, REG> PeusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peusb::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peusb::Value2)
    }
}
#[doc = "Parity Error Flag for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peeth0tx {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Peeth0tx> for bool {
    #[inline(always)]
    fn from(variant: Peeth0tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEETH0TX` reader - Parity Error Flag for ETH TX Memory"]
pub type Peeth0txR = crate::BitReader<Peeth0tx>;
impl Peeth0txR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peeth0tx {
        match self.bits {
            false => Peeth0tx::Value1,
            true => Peeth0tx::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peeth0tx::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peeth0tx::Value2
    }
}
#[doc = "Field `PEETH0TX` writer - Parity Error Flag for ETH TX Memory"]
pub type Peeth0txW<'a, REG> = crate::BitWriter<'a, REG, Peeth0tx>;
impl<'a, REG> Peeth0txW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peeth0tx::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peeth0tx::Value2)
    }
}
#[doc = "Parity Error Flag for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peeth0rx {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Peeth0rx> for bool {
    #[inline(always)]
    fn from(variant: Peeth0rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEETH0RX` reader - Parity Error Flag for ETH RX Memory"]
pub type Peeth0rxR = crate::BitReader<Peeth0rx>;
impl Peeth0rxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peeth0rx {
        match self.bits {
            false => Peeth0rx::Value1,
            true => Peeth0rx::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peeth0rx::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peeth0rx::Value2
    }
}
#[doc = "Field `PEETH0RX` writer - Parity Error Flag for ETH RX Memory"]
pub type Peeth0rxW<'a, REG> = crate::BitWriter<'a, REG, Peeth0rx>;
impl<'a, REG> Peeth0rxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peeth0rx::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peeth0rx::Value2)
    }
}
#[doc = "Parity Error Flag for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pesd0 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pesd0> for bool {
    #[inline(always)]
    fn from(variant: Pesd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESD0` reader - Parity Error Flag for SDMMC Memory 0"]
pub type Pesd0R = crate::BitReader<Pesd0>;
impl Pesd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pesd0 {
        match self.bits {
            false => Pesd0::Value1,
            true => Pesd0::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pesd0::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pesd0::Value2
    }
}
#[doc = "Field `PESD0` writer - Parity Error Flag for SDMMC Memory 0"]
pub type Pesd0W<'a, REG> = crate::BitWriter<'a, REG, Pesd0>;
impl<'a, REG> Pesd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pesd0::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pesd0::Value2)
    }
}
#[doc = "Parity Error Flag for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pesd1 {
    #[doc = "0: No parity error detected"]
    Value1 = 0,
    #[doc = "1: Parity error detected"]
    Value2 = 1,
}
impl From<Pesd1> for bool {
    #[inline(always)]
    fn from(variant: Pesd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PESD1` reader - Parity Error Flag for SDMMC Memory 1"]
pub type Pesd1R = crate::BitReader<Pesd1>;
impl Pesd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pesd1 {
        match self.bits {
            false => Pesd1::Value1,
            true => Pesd1::Value2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pesd1::Value1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pesd1::Value2
    }
}
#[doc = "Field `PESD1` writer - Parity Error Flag for SDMMC Memory 1"]
pub type Pesd1W<'a, REG> = crate::BitWriter<'a, REG, Pesd1>;
impl<'a, REG> Pesd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pesd1::Value1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pesd1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&self) -> PefpsR {
        PefpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&self) -> Pefds1R {
        Pefds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Flag for DSRAM2"]
    #[inline(always)]
    pub fn pefds2(&self) -> Pefds2R {
        Pefds2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&self) -> Pefu0R {
        Pefu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&self) -> Pefu1R {
        Pefu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Error Flag for USIC2 Memory"]
    #[inline(always)]
    pub fn pefu2(&self) -> Pefu2R {
        Pefu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&self) -> PefmcR {
        PefmcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&self) -> PefpprfR {
        PefpprfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&self) -> PeusbR {
        PeusbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    pub fn peeth0tx(&self) -> Peeth0txR {
        Peeth0txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    pub fn peeth0rx(&self) -> Peeth0rxR {
        Peeth0rxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    pub fn pesd0(&self) -> Pesd0R {
        Pesd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    pub fn pesd1(&self) -> Pesd1R {
        Pesd1R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn pefps(&mut self) -> PefpsW<PeflagSpec> {
        PefpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn pefds1(&mut self) -> Pefds1W<PeflagSpec> {
        Pefds1W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error Flag for DSRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn pefds2(&mut self) -> Pefds2W<PeflagSpec> {
        Pefds2W::new(self, 2)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefu0(&mut self) -> Pefu0W<PeflagSpec> {
        Pefu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefu1(&mut self) -> Pefu1W<PeflagSpec> {
        Pefu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity Error Flag for USIC2 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefu2(&mut self) -> Pefu2W<PeflagSpec> {
        Pefu2W::new(self, 10)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefmc(&mut self) -> PefmcW<PeflagSpec> {
        PefmcW::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefpprf(&mut self) -> PefpprfW<PeflagSpec> {
        PefpprfW::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peusb(&mut self) -> PeusbW<PeflagSpec> {
        PeusbW::new(self, 16)
    }
    #[doc = "Bit 17 - Parity Error Flag for ETH TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeth0tx(&mut self) -> Peeth0txW<PeflagSpec> {
        Peeth0txW::new(self, 17)
    }
    #[doc = "Bit 18 - Parity Error Flag for ETH RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peeth0rx(&mut self) -> Peeth0rxW<PeflagSpec> {
        Peeth0rxW::new(self, 18)
    }
    #[doc = "Bit 19 - Parity Error Flag for SDMMC Memory 0"]
    #[inline(always)]
    #[must_use]
    pub fn pesd0(&mut self) -> Pesd0W<PeflagSpec> {
        Pesd0W::new(self, 19)
    }
    #[doc = "Bit 20 - Parity Error Flag for SDMMC Memory 1"]
    #[inline(always)]
    #[must_use]
    pub fn pesd1(&mut self) -> Pesd1W<PeflagSpec> {
        Pesd1W::new(self, 20)
    }
}
#[doc = "Parity Error Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeflagSpec;
impl crate::RegisterSpec for PeflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peflag::R`](R) reader structure"]
impl crate::Readable for PeflagSpec {}
#[doc = "`write(|w| ..)` method takes [`peflag::W`](W) writer structure"]
impl crate::Writable for PeflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEFLAG to value 0"]
impl crate::Resettable for PeflagSpec {
    const RESET_VALUE: u32 = 0;
}
