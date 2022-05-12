#[doc = "Register `i2s_clock` reader"]
pub struct R(crate::R<I2S_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_clock` writer"]
pub struct W(crate::W<I2S_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CLOCK_SPEC>;
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
impl From<crate::W<I2S_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bclk_divide` reader - BCLK clock divide factor"]
pub struct BCLK_DIVIDE_R(crate::FieldReader<u16>);
impl BCLK_DIVIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BCLK_DIVIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCLK_DIVIDE_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bclk_divide` writer - BCLK clock divide factor"]
pub struct BCLK_DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCLK_DIVIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 8)) | ((value as u32 & 0x03ff) << 8);
        self.w
    }
}
#[doc = "Field `mclk_divide` reader - MCLK clock divide factor"]
pub struct MCLK_DIVIDE_R(crate::FieldReader<u8>);
impl MCLK_DIVIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCLK_DIVIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLK_DIVIDE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mclk_divide` writer - MCLK clock divide factor"]
pub struct MCLK_DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_DIVIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "MCLK clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_ENABLE_A {
    #[doc = "0: MCLK is disabled."]
    DISABLE = 0,
    #[doc = "1: MCLK is enabled."]
    ENABLE = 1,
}
impl From<MCLK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mclk_enable` reader - MCLK clock enable"]
pub struct MCLK_ENABLE_R(crate::FieldReader<bool>);
impl MCLK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCLK_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_ENABLE_A {
        match self.bits {
            false => MCLK_ENABLE_A::DISABLE,
            true => MCLK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MCLK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MCLK_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for MCLK_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mclk_enable` writer - MCLK clock enable"]
pub struct MCLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MCLK is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MCLK_ENABLE_A::DISABLE)
    }
    #[doc = "MCLK is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MCLK_ENABLE_A::ENABLE)
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
#[doc = "Use internal I2S clock or external XTAL clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLOCK_SOURCE_A {
    #[doc = "0: Use internal clock."]
    INTERNAL = 0,
    #[doc = "1: Use external XTAL clock.\n\n External clock frequency should be 2 * N * 256 fs, where fs is sample frequency and N must be an integer."]
    EXTERNAL = 1,
}
impl From<CLOCK_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: CLOCK_SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clock_source` reader - Use internal I2S clock or external XTAL clock"]
pub struct CLOCK_SOURCE_R(crate::FieldReader<bool>);
impl CLOCK_SOURCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLOCK_SOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLOCK_SOURCE_A {
        match self.bits {
            false => CLOCK_SOURCE_A::INTERNAL,
            true => CLOCK_SOURCE_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        **self == CLOCK_SOURCE_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        **self == CLOCK_SOURCE_A::EXTERNAL
    }
}
impl core::ops::Deref for CLOCK_SOURCE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clock_source` writer - Use internal I2S clock or external XTAL clock"]
pub struct CLOCK_SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCK_SOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use internal clock."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(CLOCK_SOURCE_A::INTERNAL)
    }
    #[doc = "Use external XTAL clock.\n\n External clock frequency should be 2 * N * 256 fs, where fs is sample frequency and N must be an integer."]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(CLOCK_SOURCE_A::EXTERNAL)
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
    #[doc = "Bits 8:17 - BCLK clock divide factor"]
    #[inline(always)]
    pub fn bclk_divide(&self) -> BCLK_DIVIDE_R {
        BCLK_DIVIDE_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 2:7 - MCLK clock divide factor"]
    #[inline(always)]
    pub fn mclk_divide(&self) -> MCLK_DIVIDE_R {
        MCLK_DIVIDE_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - MCLK clock enable"]
    #[inline(always)]
    pub fn mclk_enable(&self) -> MCLK_ENABLE_R {
        MCLK_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Use internal I2S clock or external XTAL clock"]
    #[inline(always)]
    pub fn clock_source(&self) -> CLOCK_SOURCE_R {
        CLOCK_SOURCE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:17 - BCLK clock divide factor"]
    #[inline(always)]
    pub fn bclk_divide(&mut self) -> BCLK_DIVIDE_W {
        BCLK_DIVIDE_W { w: self }
    }
    #[doc = "Bits 2:7 - MCLK clock divide factor"]
    #[inline(always)]
    pub fn mclk_divide(&mut self) -> MCLK_DIVIDE_W {
        MCLK_DIVIDE_W { w: self }
    }
    #[doc = "Bit 1 - MCLK clock enable"]
    #[inline(always)]
    pub fn mclk_enable(&mut self) -> MCLK_ENABLE_W {
        MCLK_ENABLE_W { w: self }
    }
    #[doc = "Bit 0 - Use internal I2S clock or external XTAL clock"]
    #[inline(always)]
    pub fn clock_source(&mut self) -> CLOCK_SOURCE_W {
        CLOCK_SOURCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter-Integrated Sound clock configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_clock](index.html) module"]
pub struct I2S_CLOCK_SPEC;
impl crate::RegisterSpec for I2S_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_clock::R](R) reader structure"]
impl crate::Readable for I2S_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_clock::W](W) writer structure"]
impl crate::Writable for I2S_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_clock to value 0"]
impl crate::Resettable for I2S_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
