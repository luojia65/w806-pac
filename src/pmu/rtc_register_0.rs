#[doc = "Register `rtc_register_0` reader"]
pub struct R(crate::R<RTC_REGISTER_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_REGISTER_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_REGISTER_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_REGISTER_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_register_0` writer"]
pub struct W(crate::W<RTC_REGISTER_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_REGISTER_0_SPEC>;
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
impl From<crate::W<RTC_REGISTER_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_REGISTER_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alarm interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM_INTERRUPT_EN_A {
    #[doc = "0: RTC does not generate Alarm Interrupts"]
    DISABLE = 0,
    #[doc = "1: RTC triggers the Alarm Interrupt"]
    ENABLE = 1,
}
impl From<ALARM_INTERRUPT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_INTERRUPT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `alarm_interrupt_en` reader - Alarm interrupt enable"]
pub struct ALARM_INTERRUPT_EN_R(crate::FieldReader<bool, ALARM_INTERRUPT_EN_A>);
impl ALARM_INTERRUPT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALARM_INTERRUPT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_INTERRUPT_EN_A {
        match self.bits {
            false => ALARM_INTERRUPT_EN_A::DISABLE,
            true => ALARM_INTERRUPT_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ALARM_INTERRUPT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ALARM_INTERRUPT_EN_A::ENABLE
    }
}
impl core::ops::Deref for ALARM_INTERRUPT_EN_R {
    type Target = crate::FieldReader<bool, ALARM_INTERRUPT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `alarm_interrupt_en` writer - Alarm interrupt enable"]
pub struct ALARM_INTERRUPT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_INTERRUPT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM_INTERRUPT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC does not generate Alarm Interrupts"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALARM_INTERRUPT_EN_A::DISABLE)
    }
    #[doc = "RTC triggers the Alarm Interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALARM_INTERRUPT_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `day` reader - day"]
pub struct DAY_R(crate::FieldReader<u8, u8>);
impl DAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `day` writer - day"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `hour` reader - hour"]
pub struct HOUR_R(crate::FieldReader<u8, u8>);
impl HOUR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HOUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOUR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hour` writer - hour"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `minute` reader - minute"]
pub struct MINUTE_R(crate::FieldReader<u8, u8>);
impl MINUTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `minute` writer - minute"]
pub struct MINUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `second` reader - second"]
pub struct SECOND_R(crate::FieldReader<u8, u8>);
impl SECOND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SECOND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECOND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `second` writer - second"]
pub struct SECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> SECOND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alarm_interrupt_en(&self) -> ALARM_INTERRUPT_EN_R {
        ALARM_INTERRUPT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - minute"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - second"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alarm_interrupt_en(&mut self) -> ALARM_INTERRUPT_EN_W {
        ALARM_INTERRUPT_EN_W { w: self }
    }
    #[doc = "Bits 24:28 - day"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bits 16:20 - hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bits 8:13 - minute"]
    #[inline(always)]
    pub fn minute(&mut self) -> MINUTE_W {
        MINUTE_W { w: self }
    }
    #[doc = "Bits 0:5 - second"]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W {
        SECOND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_register_0](index.html) module"]
pub struct RTC_REGISTER_0_SPEC;
impl crate::RegisterSpec for RTC_REGISTER_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_register_0::R](R) reader structure"]
impl crate::Readable for RTC_REGISTER_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_register_0::W](W) writer structure"]
impl crate::Writable for RTC_REGISTER_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtc_register_0 to value 0"]
impl crate::Resettable for RTC_REGISTER_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
