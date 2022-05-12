#[doc = "Register `control_register` reader"]
pub struct R(crate::R<CONTROL_REGISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_REGISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_REGISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `control_register` writer"]
pub struct W(crate::W<CONTROL_REGISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_REGISTER_SPEC>;
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
impl From<crate::W<CONTROL_REGISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_REGISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IO wake-up polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUP_LEVEL_A {
    #[doc = "0: Low level interrupts"]
    LOW = 0,
    #[doc = "1: High level interrupt"]
    HIGH = 1,
}
impl From<WAKEUP_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `wakeup_level` reader - IO wake-up polarity selection"]
pub struct WAKEUP_LEVEL_R(crate::FieldReader<bool>);
impl WAKEUP_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_LEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUP_LEVEL_A {
        match self.bits {
            false => WAKEUP_LEVEL_A::LOW,
            true => WAKEUP_LEVEL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == WAKEUP_LEVEL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == WAKEUP_LEVEL_A::HIGH
    }
}
impl core::ops::Deref for WAKEUP_LEVEL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wakeup_level` writer - IO wake-up polarity selection"]
pub struct WAKEUP_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_LEVEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUP_LEVEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Low level interrupts"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WAKEUP_LEVEL_A::LOW)
    }
    #[doc = "High level interrupt"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WAKEUP_LEVEL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `wakeup_count` reader - IO wake-up Minimum hold count of 128ms"]
pub struct WAKEUP_COUNT_R(crate::FieldReader<u8>);
impl WAKEUP_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAKEUP_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_COUNT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wakeup_count` writer - IO wake-up Minimum hold count of 128ms"]
pub struct WAKEUP_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Bypass 32k oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_32K_A {
    #[doc = "0: Disable bypass 32k oscillator"]
    DISABLE = 0,
    #[doc = "1: Enable bypass 32k oscillator"]
    ENABLE = 1,
}
impl From<BYPASS_32K_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_32K_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bypass_32k` reader - Bypass 32k oscillator"]
pub struct BYPASS_32K_R(crate::FieldReader<bool>);
impl BYPASS_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_32K_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_32K_A {
        match self.bits {
            false => BYPASS_32K_A::DISABLE,
            true => BYPASS_32K_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BYPASS_32K_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BYPASS_32K_A::ENABLE
    }
}
impl core::ops::Deref for BYPASS_32K_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bypass_32k` writer - Bypass 32k oscillator"]
pub struct BYPASS_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_32K_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable bypass 32k oscillator"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASS_32K_A::DISABLE)
    }
    #[doc = "Enable bypass 32k oscillator"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASS_32K_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Calibrate 32k oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATE_32K_A {
    #[doc = "0: Stop calibrate 32k oscillator"]
    OFF = 0,
    #[doc = "1: Start up calibrate 32k oscillator"]
    ON = 1,
}
impl From<CALIBRATE_32K_A> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATE_32K_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `calibrate_32k` reader - Calibrate 32k oscillator"]
pub struct CALIBRATE_32K_R(crate::FieldReader<bool>);
impl CALIBRATE_32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CALIBRATE_32K_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIBRATE_32K_A {
        match self.bits {
            false => CALIBRATE_32K_A::OFF,
            true => CALIBRATE_32K_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CALIBRATE_32K_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CALIBRATE_32K_A::ON
    }
}
impl core::ops::Deref for CALIBRATE_32K_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `calibrate_32k` writer - Calibrate 32k oscillator"]
pub struct CALIBRATE_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIBRATE_32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALIBRATE_32K_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop calibrate 32k oscillator"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CALIBRATE_32K_A::OFF)
    }
    #[doc = "Start up calibrate 32k oscillator"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CALIBRATE_32K_A::ON)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Sleep enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Disable sleep"]
    DISABLE = 0,
    #[doc = "1: Enable sleep"]
    ENABLE = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sleep` reader - Sleep enable"]
pub struct SLEEP_R(crate::FieldReader<bool>);
impl SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::DISABLE,
            true => SLEEP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SLEEP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SLEEP_A::ENABLE
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sleep` writer - Sleep enable"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable sleep"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SLEEP_A::DISABLE)
    }
    #[doc = "Enable sleep"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SLEEP_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Standby enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STANDBY_A {
    #[doc = "0: Disable standby"]
    DISABLE = 0,
    #[doc = "1: Enable standby"]
    ENABLE = 1,
}
impl From<STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `standby` reader - Standby enable"]
pub struct STANDBY_R(crate::FieldReader<bool>);
impl STANDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STANDBY_A {
        match self.bits {
            false => STANDBY_A::DISABLE,
            true => STANDBY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == STANDBY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == STANDBY_A::ENABLE
    }
}
impl core::ops::Deref for STANDBY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `standby` writer - Standby enable"]
pub struct STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STANDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable standby"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STANDBY_A::DISABLE)
    }
    #[doc = "Enable standby"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(STANDBY_A::ENABLE)
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - IO wake-up polarity selection"]
    #[inline(always)]
    pub fn wakeup_level(&self) -> WAKEUP_LEVEL_R {
        WAKEUP_LEVEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 6:9 - IO wake-up Minimum hold count of 128ms"]
    #[inline(always)]
    pub fn wakeup_count(&self) -> WAKEUP_COUNT_R {
        WAKEUP_COUNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Bypass 32k oscillator"]
    #[inline(always)]
    pub fn bypass_32k(&self) -> BYPASS_32K_R {
        BYPASS_32K_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Calibrate 32k oscillator"]
    #[inline(always)]
    pub fn calibrate_32k(&self) -> CALIBRATE_32K_R {
        CALIBRATE_32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep enable"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Standby enable"]
    #[inline(always)]
    pub fn standby(&self) -> STANDBY_R {
        STANDBY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - IO wake-up polarity selection"]
    #[inline(always)]
    pub fn wakeup_level(&mut self) -> WAKEUP_LEVEL_W {
        WAKEUP_LEVEL_W { w: self }
    }
    #[doc = "Bits 6:9 - IO wake-up Minimum hold count of 128ms"]
    #[inline(always)]
    pub fn wakeup_count(&mut self) -> WAKEUP_COUNT_W {
        WAKEUP_COUNT_W { w: self }
    }
    #[doc = "Bit 4 - Bypass 32k oscillator"]
    #[inline(always)]
    pub fn bypass_32k(&mut self) -> BYPASS_32K_W {
        BYPASS_32K_W { w: self }
    }
    #[doc = "Bit 3 - Calibrate 32k oscillator"]
    #[inline(always)]
    pub fn calibrate_32k(&mut self) -> CALIBRATE_32K_W {
        CALIBRATE_32K_W { w: self }
    }
    #[doc = "Bit 1 - Sleep enable"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 0 - Standby enable"]
    #[inline(always)]
    pub fn standby(&mut self) -> STANDBY_W {
        STANDBY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control_register](index.html) module"]
pub struct CONTROL_REGISTER_SPEC;
impl crate::RegisterSpec for CONTROL_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control_register::R](R) reader structure"]
impl crate::Readable for CONTROL_REGISTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control_register::W](W) writer structure"]
impl crate::Writable for CONTROL_REGISTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets control_register to value 0x02"]
impl crate::Resettable for CONTROL_REGISTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
