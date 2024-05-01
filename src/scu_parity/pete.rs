#[doc = "Register `PETE` reader"]
pub type R = crate::R<PeteSpec>;
#[doc = "Register `PETE` writer"]
pub type W = crate::W<PeteSpec>;
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteps {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteps> for bool {
    #[inline(always)]
    fn from(variant: Peteps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub type PetepsR = crate::BitReader<Peteps>;
impl PetepsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteps {
        match self.bits {
            false => Peteps::Value1,
            true => Peteps::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteps::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteps::Value2
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub type PetepsW<'a, REG> = crate::BitWriter<'a, REG, Peteps>;
impl<'a, REG> PetepsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteps::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteps::Value2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteds1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteds1> for bool {
    #[inline(always)]
    fn from(variant: Peteds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub type Peteds1R = crate::BitReader<Peteds1>;
impl Peteds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteds1 {
        match self.bits {
            false => Peteds1::Value1,
            true => Peteds1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteds1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteds1::Value2
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub type Peteds1W<'a, REG> = crate::BitWriter<'a, REG, Peteds1>;
impl<'a, REG> Peteds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds1::Value2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteds2 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteds2> for bool {
    #[inline(always)]
    fn from(variant: Peteds2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS2` reader - Parity Error Trap Enable for DSRAM2"]
pub type Peteds2R = crate::BitReader<Peteds2>;
impl Peteds2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteds2 {
        match self.bits {
            false => Peteds2::Value1,
            true => Peteds2::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteds2::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteds2::Value2
    }
}
#[doc = "Field `PETEDS2` writer - Parity Error Trap Enable for DSRAM2"]
pub type Peteds2W<'a, REG> = crate::BitWriter<'a, REG, Peteds2>;
impl<'a, REG> Peteds2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds2::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds2::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteu0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteu0> for bool {
    #[inline(always)]
    fn from(variant: Peteu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub type Peteu0R = crate::BitReader<Peteu0>;
impl Peteu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteu0 {
        match self.bits {
            false => Peteu0::Value1,
            true => Peteu0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteu0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteu0::Value2
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub type Peteu0W<'a, REG> = crate::BitWriter<'a, REG, Peteu0>;
impl<'a, REG> Peteu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu0::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteu1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteu1> for bool {
    #[inline(always)]
    fn from(variant: Peteu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub type Peteu1R = crate::BitReader<Peteu1>;
impl Peteu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteu1 {
        match self.bits {
            false => Peteu1::Value1,
            true => Peteu1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteu1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteu1::Value2
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub type Peteu1W<'a, REG> = crate::BitWriter<'a, REG, Peteu1>;
impl<'a, REG> Peteu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu1::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteu2 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteu2> for bool {
    #[inline(always)]
    fn from(variant: Peteu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU2` reader - Parity Error Trap Enable for USIC2 Memory"]
pub type Peteu2R = crate::BitReader<Peteu2>;
impl Peteu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteu2 {
        match self.bits {
            false => Peteu2::Value1,
            true => Peteu2::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteu2::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteu2::Value2
    }
}
#[doc = "Field `PETEU2` writer - Parity Error Trap Enable for USIC2 Memory"]
pub type Peteu2W<'a, REG> = crate::BitWriter<'a, REG, Peteu2>;
impl<'a, REG> Peteu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu2::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu2::Value2)
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petemc {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petemc> for bool {
    #[inline(always)]
    fn from(variant: Petemc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub type PetemcR = crate::BitReader<Petemc>;
impl PetemcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petemc {
        match self.bits {
            false => Petemc::Value1,
            true => Petemc::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petemc::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petemc::Value2
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub type PetemcW<'a, REG> = crate::BitWriter<'a, REG, Petemc>;
impl<'a, REG> PetemcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petemc::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petemc::Value2)
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petepprf {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petepprf> for bool {
    #[inline(always)]
    fn from(variant: Petepprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PetepprfR = crate::BitReader<Petepprf>;
impl PetepprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petepprf {
        match self.bits {
            false => Petepprf::Value1,
            true => Petepprf::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petepprf::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petepprf::Value2
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PetepprfW<'a, REG> = crate::BitWriter<'a, REG, Petepprf>;
impl<'a, REG> PetepprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petepprf::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petepprf::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteusb {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteusb> for bool {
    #[inline(always)]
    fn from(variant: Peteusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub type PeteusbR = crate::BitReader<Peteusb>;
impl PeteusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteusb {
        match self.bits {
            false => Peteusb::Value1,
            true => Peteusb::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteusb::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteusb::Value2
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub type PeteusbW<'a, REG> = crate::BitWriter<'a, REG, Peteusb>;
impl<'a, REG> PeteusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteusb::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteusb::Value2)
    }
}
#[doc = "Parity Error Trap Enable for ETH 0TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteeth0tx {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteeth0tx> for bool {
    #[inline(always)]
    fn from(variant: Peteeth0tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEETH0TX` reader - Parity Error Trap Enable for ETH 0TX Memory"]
pub type Peteeth0txR = crate::BitReader<Peteeth0tx>;
impl Peteeth0txR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteeth0tx {
        match self.bits {
            false => Peteeth0tx::Value1,
            true => Peteeth0tx::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteeth0tx::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteeth0tx::Value2
    }
}
#[doc = "Field `PETEETH0TX` writer - Parity Error Trap Enable for ETH 0TX Memory"]
pub type Peteeth0txW<'a, REG> = crate::BitWriter<'a, REG, Peteeth0tx>;
impl<'a, REG> Peteeth0txW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteeth0tx::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteeth0tx::Value2)
    }
}
#[doc = "Parity Error Trap Enable for ETH0 RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteeth0rx {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteeth0rx> for bool {
    #[inline(always)]
    fn from(variant: Peteeth0rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEETH0RX` reader - Parity Error Trap Enable for ETH0 RX Memory"]
pub type Peteeth0rxR = crate::BitReader<Peteeth0rx>;
impl Peteeth0rxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteeth0rx {
        match self.bits {
            false => Peteeth0rx::Value1,
            true => Peteeth0rx::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteeth0rx::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteeth0rx::Value2
    }
}
#[doc = "Field `PETEETH0RX` writer - Parity Error Trap Enable for ETH0 RX Memory"]
pub type Peteeth0rxW<'a, REG> = crate::BitWriter<'a, REG, Peteeth0rx>;
impl<'a, REG> Peteeth0rxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteeth0rx::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteeth0rx::Value2)
    }
}
#[doc = "Parity Error Trap Enable for SDMMC SRAM 0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petesd0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petesd0> for bool {
    #[inline(always)]
    fn from(variant: Petesd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETESD0` reader - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type Petesd0R = crate::BitReader<Petesd0>;
impl Petesd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petesd0 {
        match self.bits {
            false => Petesd0::Value1,
            true => Petesd0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petesd0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petesd0::Value2
    }
}
#[doc = "Field `PETESD0` writer - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
pub type Petesd0W<'a, REG> = crate::BitWriter<'a, REG, Petesd0>;
impl<'a, REG> Petesd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petesd0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petesd0::Value2)
    }
}
#[doc = "Parity Error Trap Enable for SDMMC SRAM 1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petesd1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petesd1> for bool {
    #[inline(always)]
    fn from(variant: Petesd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETESD1` reader - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type Petesd1R = crate::BitReader<Petesd1>;
impl Petesd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petesd1 {
        match self.bits {
            false => Petesd1::Value1,
            true => Petesd1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petesd1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petesd1::Value2
    }
}
#[doc = "Field `PETESD1` writer - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
pub type Petesd1W<'a, REG> = crate::BitWriter<'a, REG, Petesd1>;
impl<'a, REG> Petesd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petesd1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petesd1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PetepsR {
        PetepsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> Peteds1R {
        Peteds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    pub fn peteds2(&self) -> Peteds2R {
        Peteds2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> Peteu0R {
        Peteu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> Peteu1R {
        Peteu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    pub fn peteu2(&self) -> Peteu2R {
        Peteu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PetemcR {
        PetemcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PetepprfR {
        PetepprfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PeteusbR {
        PeteusbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    pub fn peteeth0tx(&self) -> Peteeth0txR {
        Peteeth0txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    pub fn peteeth0rx(&self) -> Peteeth0rxR {
        Peteeth0rxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    pub fn petesd0(&self) -> Petesd0R {
        Petesd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    pub fn petesd1(&self) -> Petesd1R {
        Petesd1R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peteps(&mut self) -> PetepsW<PeteSpec> {
        PetepsW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peteds1(&mut self) -> Peteds1W<PeteSpec> {
        Peteds1W::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error Trap Enable for DSRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn peteds2(&mut self) -> Peteds2W<PeteSpec> {
        Peteds2W::new(self, 2)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu0(&mut self) -> Peteu0W<PeteSpec> {
        Peteu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu1(&mut self) -> Peteu1W<PeteSpec> {
        Peteu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity Error Trap Enable for USIC2 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu2(&mut self) -> Peteu2W<PeteSpec> {
        Peteu2W::new(self, 10)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petemc(&mut self) -> PetemcW<PeteSpec> {
        PetemcW::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petepprf(&mut self) -> PetepprfW<PeteSpec> {
        PetepprfW::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteusb(&mut self) -> PeteusbW<PeteSpec> {
        PeteusbW::new(self, 16)
    }
    #[doc = "Bit 17 - Parity Error Trap Enable for ETH 0TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteeth0tx(&mut self) -> Peteeth0txW<PeteSpec> {
        Peteeth0txW::new(self, 17)
    }
    #[doc = "Bit 18 - Parity Error Trap Enable for ETH0 RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteeth0rx(&mut self) -> Peteeth0rxW<PeteSpec> {
        Peteeth0rxW::new(self, 18)
    }
    #[doc = "Bit 19 - Parity Error Trap Enable for SDMMC SRAM 0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petesd0(&mut self) -> Petesd0W<PeteSpec> {
        Petesd0W::new(self, 19)
    }
    #[doc = "Bit 20 - Parity Error Trap Enable for SDMMC SRAM 1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petesd1(&mut self) -> Petesd1W<PeteSpec> {
        Petesd1W::new(self, 20)
    }
}
#[doc = "Parity Error Trap Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pete::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pete::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeteSpec;
impl crate::RegisterSpec for PeteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pete::R`](R) reader structure"]
impl crate::Readable for PeteSpec {}
#[doc = "`write(|w| ..)` method takes [`pete::W`](W) writer structure"]
impl crate::Writable for PeteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PeteSpec {
    const RESET_VALUE: u32 = 0;
}
