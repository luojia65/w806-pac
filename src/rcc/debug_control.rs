#[doc = "Register `debug_control` reader"]
pub struct R(crate::R<DEBUG_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `debug_control` writer"]
pub struct W(crate::W<DEBUG_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_CONTROL_SPEC>;
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
impl From<crate::W<DEBUG_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "JTAG function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_ENABLE_A {
    #[doc = "0: Disable JTAG debug function."]
    DISABLE = 0,
    #[doc = "1: Enable JTAG debug function."]
    ENABLE = 1,
}
impl From<JTAG_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `jtag_enable` reader - JTAG function enable"]
pub struct JTAG_ENABLE_R(crate::FieldReader<bool>);
impl JTAG_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_ENABLE_A {
        match self.bits {
            false => JTAG_ENABLE_A::DISABLE,
            true => JTAG_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == JTAG_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == JTAG_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for JTAG_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag_enable` writer - JTAG function enable"]
pub struct JTAG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAG_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable JTAG debug function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JTAG_ENABLE_A::DISABLE)
    }
    #[doc = "Enable JTAG debug function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JTAG_ENABLE_A::ENABLE)
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
#[doc = "Field `adc_divide` reader - Sigma-Delta Analog-digital Converter frequency division factor\n\n After writing to this field, set bit `update` of register `clock_divide` to take effect."]
pub struct ADC_DIVIDE_R(crate::FieldReader<u8>);
impl ADC_DIVIDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_DIVIDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DIVIDE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_divide` writer - Sigma-Delta Analog-digital Converter frequency division factor\n\n After writing to this field, set bit `update` of register `clock_divide` to take effect."]
pub struct ADC_DIVIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DIVIDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "QSPI Flash clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QFLASH_SELECT_A {
    #[doc = "1: Use 80 MHz."]
    MHZ_80 = 1,
    #[doc = "0: Use 40 MHz."]
    MHZ_40 = 0,
}
impl From<QFLASH_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: QFLASH_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `qflash_select` reader - QSPI Flash clock source selection"]
pub struct QFLASH_SELECT_R(crate::FieldReader<bool>);
impl QFLASH_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QFLASH_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QFLASH_SELECT_A {
        match self.bits {
            true => QFLASH_SELECT_A::MHZ_80,
            false => QFLASH_SELECT_A::MHZ_40,
        }
    }
    #[doc = "Checks if the value of the field is `MHZ_80`"]
    #[inline(always)]
    pub fn is_mhz_80(&self) -> bool {
        **self == QFLASH_SELECT_A::MHZ_80
    }
    #[doc = "Checks if the value of the field is `MHZ_40`"]
    #[inline(always)]
    pub fn is_mhz_40(&self) -> bool {
        **self == QFLASH_SELECT_A::MHZ_40
    }
}
impl core::ops::Deref for QFLASH_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `qflash_select` writer - QSPI Flash clock source selection"]
pub struct QFLASH_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> QFLASH_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QFLASH_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 80 MHz."]
    #[inline(always)]
    pub fn mhz_80(self) -> &'a mut W {
        self.variant(QFLASH_SELECT_A::MHZ_80)
    }
    #[doc = "Use 40 MHz."]
    #[inline(always)]
    pub fn mhz_40(self) -> &'a mut W {
        self.variant(QFLASH_SELECT_A::MHZ_40)
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "GPSEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPSEC_SELECT_A {
    #[doc = "1: Use 160 MHz."]
    MHZ_160 = 1,
    #[doc = "0: Use 80 MHz."]
    MHZ_80 = 0,
}
impl From<GPSEC_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: GPSEC_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gpsec_select` reader - GPSEC clock source selection"]
pub struct GPSEC_SELECT_R(crate::FieldReader<bool>);
impl GPSEC_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPSEC_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPSEC_SELECT_A {
        match self.bits {
            true => GPSEC_SELECT_A::MHZ_160,
            false => GPSEC_SELECT_A::MHZ_80,
        }
    }
    #[doc = "Checks if the value of the field is `MHZ_160`"]
    #[inline(always)]
    pub fn is_mhz_160(&self) -> bool {
        **self == GPSEC_SELECT_A::MHZ_160
    }
    #[doc = "Checks if the value of the field is `MHZ_80`"]
    #[inline(always)]
    pub fn is_mhz_80(&self) -> bool {
        **self == GPSEC_SELECT_A::MHZ_80
    }
}
impl core::ops::Deref for GPSEC_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpsec_select` writer - GPSEC clock source selection"]
pub struct GPSEC_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSEC_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPSEC_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 160 MHz."]
    #[inline(always)]
    pub fn mhz_160(self) -> &'a mut W {
        self.variant(GPSEC_SELECT_A::MHZ_160)
    }
    #[doc = "Use 80 MHz."]
    #[inline(always)]
    pub fn mhz_80(self) -> &'a mut W {
        self.variant(GPSEC_SELECT_A::MHZ_80)
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "RSA clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSA_SELECT_A {
    #[doc = "1: Use 160 MHz."]
    MHZ_160 = 1,
    #[doc = "0: Use 80 MHz."]
    MHZ_80 = 0,
}
impl From<RSA_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RSA_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rsa_select` reader - RSA clock source selection"]
pub struct RSA_SELECT_R(crate::FieldReader<bool>);
impl RSA_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSA_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSA_SELECT_A {
        match self.bits {
            true => RSA_SELECT_A::MHZ_160,
            false => RSA_SELECT_A::MHZ_80,
        }
    }
    #[doc = "Checks if the value of the field is `MHZ_160`"]
    #[inline(always)]
    pub fn is_mhz_160(&self) -> bool {
        **self == RSA_SELECT_A::MHZ_160
    }
    #[doc = "Checks if the value of the field is `MHZ_80`"]
    #[inline(always)]
    pub fn is_mhz_80(&self) -> bool {
        **self == RSA_SELECT_A::MHZ_80
    }
}
impl core::ops::Deref for RSA_SELECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsa_select` writer - RSA clock source selection"]
pub struct RSA_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSA_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 160 MHz."]
    #[inline(always)]
    pub fn mhz_160(self) -> &'a mut W {
        self.variant(RSA_SELECT_A::MHZ_160)
    }
    #[doc = "Use 80 MHz."]
    #[inline(always)]
    pub fn mhz_80(self) -> &'a mut W {
        self.variant(RSA_SELECT_A::MHZ_80)
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
impl R {
    #[doc = "Bit 16 - JTAG function enable"]
    #[inline(always)]
    pub fn jtag_enable(&self) -> JTAG_ENABLE_R {
        JTAG_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Sigma-Delta Analog-digital Converter frequency division factor\n\n After writing to this field, set bit `update` of register `clock_divide` to take effect."]
    #[inline(always)]
    pub fn adc_divide(&self) -> ADC_DIVIDE_R {
        ADC_DIVIDE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 6 - QSPI Flash clock source selection"]
    #[inline(always)]
    pub fn qflash_select(&self) -> QFLASH_SELECT_R {
        QFLASH_SELECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - GPSEC clock source selection"]
    #[inline(always)]
    pub fn gpsec_select(&self) -> GPSEC_SELECT_R {
        GPSEC_SELECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RSA clock source selection"]
    #[inline(always)]
    pub fn rsa_select(&self) -> RSA_SELECT_R {
        RSA_SELECT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - JTAG function enable"]
    #[inline(always)]
    pub fn jtag_enable(&mut self) -> JTAG_ENABLE_W {
        JTAG_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - Sigma-Delta Analog-digital Converter frequency division factor\n\n After writing to this field, set bit `update` of register `clock_divide` to take effect."]
    #[inline(always)]
    pub fn adc_divide(&mut self) -> ADC_DIVIDE_W {
        ADC_DIVIDE_W { w: self }
    }
    #[doc = "Bit 6 - QSPI Flash clock source selection"]
    #[inline(always)]
    pub fn qflash_select(&mut self) -> QFLASH_SELECT_W {
        QFLASH_SELECT_W { w: self }
    }
    #[doc = "Bit 5 - GPSEC clock source selection"]
    #[inline(always)]
    pub fn gpsec_select(&mut self) -> GPSEC_SELECT_W {
        GPSEC_SELECT_W { w: self }
    }
    #[doc = "Bit 4 - RSA clock source selection"]
    #[inline(always)]
    pub fn rsa_select(&mut self) -> RSA_SELECT_W {
        RSA_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug and additional clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_control](index.html) module"]
pub struct DEBUG_CONTROL_SPEC;
impl crate::RegisterSpec for DEBUG_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_control::R](R) reader structure"]
impl crate::Readable for DEBUG_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_control::W](W) writer structure"]
impl crate::Writable for DEBUG_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets debug_control to value 0x0a00"]
impl crate::Resettable for DEBUG_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00
    }
}
