#[doc = "Register `rtc_register_1` reader"]
pub struct R(crate::R<RTC_REGISTER_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_REGISTER_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_REGISTER_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_REGISTER_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rtc_register_1` writer"]
pub struct W(crate::W<RTC_REGISTER_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_REGISTER_1_SPEC>;
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
impl From<crate::W<RTC_REGISTER_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_REGISTER_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RTC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ENABLE_A {
    #[doc = "0: RTC disable"]
    DISABLE = 0,
    #[doc = "1: RTC enable"]
    ENABLE = 1,
}
impl From<RTC_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rtc_enable` reader - RTC enable"]
pub struct RTC_ENABLE_R(crate::FieldReader<bool, RTC_ENABLE_A>);
impl RTC_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ENABLE_A {
        match self.bits {
            false => RTC_ENABLE_A::DISABLE,
            true => RTC_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RTC_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RTC_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for RTC_ENABLE_R {
    type Target = crate::FieldReader<bool, RTC_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_enable` writer - RTC enable"]
pub struct RTC_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RTC disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_ENABLE_A::DISABLE)
    }
    #[doc = "RTC enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_ENABLE_A::ENABLE)
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
#[doc = "Field `year` reader - year"]
pub struct YEAR_R(crate::FieldReader<u8, u8>);
impl YEAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `year` writer - year"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `month` reader - month"]
pub struct MONTH_R(crate::FieldReader<u8, u8>);
impl MONTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `month` writer - month"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - RTC enable"]
    #[inline(always)]
    pub fn rtc_enable(&self) -> RTC_ENABLE_R {
        RTC_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:3 - month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - RTC enable"]
    #[inline(always)]
    pub fn rtc_enable(&mut self) -> RTC_ENABLE_W {
        RTC_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:14 - year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
    #[doc = "Bits 0:3 - month"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_register_1](index.html) module"]
pub struct RTC_REGISTER_1_SPEC;
impl crate::RegisterSpec for RTC_REGISTER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_register_1::R](R) reader structure"]
impl crate::Readable for RTC_REGISTER_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_register_1::W](W) writer structure"]
impl crate::Writable for RTC_REGISTER_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rtc_register_1 to value 0"]
impl crate::Resettable for RTC_REGISTER_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
