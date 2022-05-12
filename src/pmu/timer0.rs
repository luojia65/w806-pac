#[doc = "Register `timer0` reader"]
pub struct R(crate::R<TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `timer0` writer"]
pub struct W(crate::W<TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_SPEC>;
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
impl From<crate::W<TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PMU timer0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disable timer0"]
    DISABLE = 0,
    #[doc = "1: Enable timer0"]
    ENABLE = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `enable` reader - PMU timer0 enable"]
pub struct ENABLE_R(crate::FieldReader<bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLE,
            true => ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable` writer - PMU timer0 enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable timer0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLE)
    }
    #[doc = "Enable timer0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `count_of_seconds` reader - PMU timer0 initial count of seconds register"]
pub struct COUNT_OF_SECONDS_R(crate::FieldReader<u16>);
impl COUNT_OF_SECONDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_OF_SECONDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_OF_SECONDS_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `count_of_seconds` writer - PMU timer0 initial count of seconds register"]
pub struct COUNT_OF_SECONDS_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_OF_SECONDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - PMU timer0 enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:15 - PMU timer0 initial count of seconds register"]
    #[inline(always)]
    pub fn count_of_seconds(&self) -> COUNT_OF_SECONDS_R {
        COUNT_OF_SECONDS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - PMU timer0 enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 0:15 - PMU timer0 initial count of seconds register"]
    #[inline(always)]
    pub fn count_of_seconds(&mut self) -> COUNT_OF_SECONDS_W {
        COUNT_OF_SECONDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0](index.html) module"]
pub struct TIMER0_SPEC;
impl crate::RegisterSpec for TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0::R](R) reader structure"]
impl crate::Readable for TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0::W](W) writer structure"]
impl crate::Writable for TIMER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets timer0 to value 0"]
impl crate::Resettable for TIMER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
