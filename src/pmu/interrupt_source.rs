#[doc = "Register `interrupt_source` reader"]
pub struct R(crate::R<INTERRUPT_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interrupt_source` writer"]
pub struct W(crate::W<INTERRUPT_SOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_SOURCE_SPEC>;
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
impl From<crate::W<INTERRUPT_SOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_SOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Whether power-on from sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: disappear wake up from sleep"]
    DISAPPEAR = 0,
    #[doc = "1: appear wake up from sleep, set clear"]
    APPEAR = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sleep` reader - Whether power-on from sleep"]
pub struct SLEEP_R(crate::FieldReader<bool, SLEEP_A>);
impl SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::DISAPPEAR,
            true => SLEEP_A::APPEAR,
        }
    }
    #[doc = "Checks if the value of the field is `DISAPPEAR`"]
    #[inline(always)]
    pub fn is_disappear(&self) -> bool {
        **self == SLEEP_A::DISAPPEAR
    }
    #[doc = "Checks if the value of the field is `APPEAR`"]
    #[inline(always)]
    pub fn is_appear(&self) -> bool {
        **self == SLEEP_A::APPEAR
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, SLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sleep` writer - Whether power-on from sleep"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disappear wake up from sleep"]
    #[inline(always)]
    pub fn disappear(self) -> &'a mut W {
        self.variant(SLEEP_A::DISAPPEAR)
    }
    #[doc = "appear wake up from sleep, set clear"]
    #[inline(always)]
    pub fn appear(self) -> &'a mut W {
        self.variant(SLEEP_A::APPEAR)
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
#[doc = "Whether power-on from standby\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STANDBY_A {
    #[doc = "0: disappear wake up from standby"]
    DISAPPEAR = 0,
    #[doc = "1: appear wake up from standby, set clear"]
    APPEAR = 1,
}
impl From<STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `standby` reader - Whether power-on from standby"]
pub struct STANDBY_R(crate::FieldReader<bool, STANDBY_A>);
impl STANDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STANDBY_A {
        match self.bits {
            false => STANDBY_A::DISAPPEAR,
            true => STANDBY_A::APPEAR,
        }
    }
    #[doc = "Checks if the value of the field is `DISAPPEAR`"]
    #[inline(always)]
    pub fn is_disappear(&self) -> bool {
        **self == STANDBY_A::DISAPPEAR
    }
    #[doc = "Checks if the value of the field is `APPEAR`"]
    #[inline(always)]
    pub fn is_appear(&self) -> bool {
        **self == STANDBY_A::APPEAR
    }
}
impl core::ops::Deref for STANDBY_R {
    type Target = crate::FieldReader<bool, STANDBY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `standby` writer - Whether power-on from standby"]
pub struct STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STANDBY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disappear wake up from standby"]
    #[inline(always)]
    pub fn disappear(self) -> &'a mut W {
        self.variant(STANDBY_A::DISAPPEAR)
    }
    #[doc = "appear wake up from standby, set clear"]
    #[inline(always)]
    pub fn appear(self) -> &'a mut W {
        self.variant(STANDBY_A::APPEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "RTC interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_INT_A {
    #[doc = "0: No RTC interrupt is generated."]
    NO_INTERRUPT = 0,
    #[doc = "1: RTC interrupt is generated, set clear"]
    HAS_INTERRUPT_SET_CLR = 1,
}
impl From<RTC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rtc_int` reader - RTC interrupt flag"]
pub struct RTC_INT_R(crate::FieldReader<bool, RTC_INT_A>);
impl RTC_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_INT_A {
        match self.bits {
            false => RTC_INT_A::NO_INTERRUPT,
            true => RTC_INT_A::HAS_INTERRUPT_SET_CLR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == RTC_INT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT_SET_CLR`"]
    #[inline(always)]
    pub fn is_has_interrupt_set_clr(&self) -> bool {
        **self == RTC_INT_A::HAS_INTERRUPT_SET_CLR
    }
}
impl core::ops::Deref for RTC_INT_R {
    type Target = crate::FieldReader<bool, RTC_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_int` writer - RTC interrupt flag"]
pub struct RTC_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No RTC interrupt is generated."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(RTC_INT_A::NO_INTERRUPT)
    }
    #[doc = "RTC interrupt is generated, set clear"]
    #[inline(always)]
    pub fn has_interrupt_set_clr(self) -> &'a mut W {
        self.variant(RTC_INT_A::HAS_INTERRUPT_SET_CLR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "IO wake up interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_WAKE_INT_A {
    #[doc = "0: No IO wake up interrupt is generated."]
    NO_INTERRUPT = 0,
    #[doc = "1: IO wake up interrupt is generated, set clear"]
    HAS_INTERRUPT_SET_CLR = 1,
}
impl From<IO_WAKE_INT_A> for bool {
    #[inline(always)]
    fn from(variant: IO_WAKE_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `io_wake_int` reader - IO wake up interrupt flag"]
pub struct IO_WAKE_INT_R(crate::FieldReader<bool, IO_WAKE_INT_A>);
impl IO_WAKE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_WAKE_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_WAKE_INT_A {
        match self.bits {
            false => IO_WAKE_INT_A::NO_INTERRUPT,
            true => IO_WAKE_INT_A::HAS_INTERRUPT_SET_CLR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == IO_WAKE_INT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT_SET_CLR`"]
    #[inline(always)]
    pub fn is_has_interrupt_set_clr(&self) -> bool {
        **self == IO_WAKE_INT_A::HAS_INTERRUPT_SET_CLR
    }
}
impl core::ops::Deref for IO_WAKE_INT_R {
    type Target = crate::FieldReader<bool, IO_WAKE_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_wake_int` writer - IO wake up interrupt flag"]
pub struct IO_WAKE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_WAKE_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_WAKE_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No IO wake up interrupt is generated."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(IO_WAKE_INT_A::NO_INTERRUPT)
    }
    #[doc = "IO wake up interrupt is generated, set clear"]
    #[inline(always)]
    pub fn has_interrupt_set_clr(self) -> &'a mut W {
        self.variant(IO_WAKE_INT_A::HAS_INTERRUPT_SET_CLR)
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
#[doc = "PMU TIMER0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_INT_A {
    #[doc = "0: No PMU TIMER0 interrupt is generated."]
    NO_INTERRUPT = 0,
    #[doc = "1: PMU TIMER0 interrupt is generated, set clear"]
    HAS_INTERRUPT_SET_CLR = 1,
}
impl From<TIMER0_INT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER0_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `timer0_int` reader - PMU TIMER0 interrupt flag"]
pub struct TIMER0_INT_R(crate::FieldReader<bool, TIMER0_INT_A>);
impl TIMER0_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER0_INT_A {
        match self.bits {
            false => TIMER0_INT_A::NO_INTERRUPT,
            true => TIMER0_INT_A::HAS_INTERRUPT_SET_CLR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == TIMER0_INT_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT_SET_CLR`"]
    #[inline(always)]
    pub fn is_has_interrupt_set_clr(&self) -> bool {
        **self == TIMER0_INT_A::HAS_INTERRUPT_SET_CLR
    }
}
impl core::ops::Deref for TIMER0_INT_R {
    type Target = crate::FieldReader<bool, TIMER0_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer0_int` writer - PMU TIMER0 interrupt flag"]
pub struct TIMER0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER0_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No PMU TIMER0 interrupt is generated."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(TIMER0_INT_A::NO_INTERRUPT)
    }
    #[doc = "PMU TIMER0 interrupt is generated, set clear"]
    #[inline(always)]
    pub fn has_interrupt_set_clr(self) -> &'a mut W {
        self.variant(TIMER0_INT_A::HAS_INTERRUPT_SET_CLR)
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
impl R {
    #[doc = "Bit 8 - Whether power-on from sleep"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Whether power-on from standby"]
    #[inline(always)]
    pub fn standby(&self) -> STANDBY_R {
        STANDBY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC interrupt flag"]
    #[inline(always)]
    pub fn rtc_int(&self) -> RTC_INT_R {
        RTC_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO wake up interrupt flag"]
    #[inline(always)]
    pub fn io_wake_int(&self) -> IO_WAKE_INT_R {
        IO_WAKE_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PMU TIMER0 interrupt flag"]
    #[inline(always)]
    pub fn timer0_int(&self) -> TIMER0_INT_R {
        TIMER0_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Whether power-on from sleep"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 7 - Whether power-on from standby"]
    #[inline(always)]
    pub fn standby(&mut self) -> STANDBY_W {
        STANDBY_W { w: self }
    }
    #[doc = "Bit 4 - RTC interrupt flag"]
    #[inline(always)]
    pub fn rtc_int(&mut self) -> RTC_INT_W {
        RTC_INT_W { w: self }
    }
    #[doc = "Bit 2 - IO wake up interrupt flag"]
    #[inline(always)]
    pub fn io_wake_int(&mut self) -> IO_WAKE_INT_W {
        IO_WAKE_INT_W { w: self }
    }
    #[doc = "Bit 0 - PMU TIMER0 interrupt flag"]
    #[inline(always)]
    pub fn timer0_int(&mut self) -> TIMER0_INT_W {
        TIMER0_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMU interrupt source register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_source](index.html) module"]
pub struct INTERRUPT_SOURCE_SPEC;
impl crate::RegisterSpec for INTERRUPT_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_source::R](R) reader structure"]
impl crate::Readable for INTERRUPT_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_source::W](W) writer structure"]
impl crate::Writable for INTERRUPT_SOURCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets interrupt_source to value 0"]
impl crate::Resettable for INTERRUPT_SOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
