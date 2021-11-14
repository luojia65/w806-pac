#[doc = "Register `clock_divide` reader"]
pub struct R(crate::R<CLOCK_DIVIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_DIVIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_DIVIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_DIVIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clock_divide` writer"]
pub struct W(crate::W<CLOCK_DIVIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_DIVIDE_SPEC>;
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
impl From<crate::W<CLOCK_DIVIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_DIVIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Divide frequency enable\n\n When you need to reconfigure cpu_clk_divider, wlan_clk_divider, bus2_syncdn_factor, sdadc_fdiv, set this register, the hardware will automatically update the above four parameters to the divider, and then clear this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQUENCY_A {
    #[doc = "0: Frequency division factor has taken effect."]
    IN_EFFECT = 0,
    #[doc = "1: Request hardware to update frequency division parameters."]
    UPDATE = 1,
}
impl From<FREQUENCY_A> for bool {
    #[inline(always)]
    fn from(variant: FREQUENCY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `frequency` reader - Divide frequency enable\n\n When you need to reconfigure cpu_clk_divider, wlan_clk_divider, bus2_syncdn_factor, sdadc_fdiv, set this register, the hardware will automatically update the above four parameters to the divider, and then clear this register."]
pub struct FREQUENCY_R(crate::FieldReader<bool, FREQUENCY_A>);
impl FREQUENCY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREQUENCY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQUENCY_A {
        match self.bits {
            false => FREQUENCY_A::IN_EFFECT,
            true => FREQUENCY_A::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `IN_EFFECT`"]
    #[inline(always)]
    pub fn is_in_effect(&self) -> bool {
        **self == FREQUENCY_A::IN_EFFECT
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        **self == FREQUENCY_A::UPDATE
    }
}
impl core::ops::Deref for FREQUENCY_R {
    type Target = crate::FieldReader<bool, FREQUENCY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `frequency` writer - Divide frequency enable\n\n When you need to reconfigure cpu_clk_divider, wlan_clk_divider, bus2_syncdn_factor, sdadc_fdiv, set this register, the hardware will automatically update the above four parameters to the divider, and then clear this register."]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQUENCY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frequency division factor has taken effect."]
    #[inline(always)]
    pub fn in_effect(self) -> &'a mut W {
        self.variant(FREQUENCY_A::IN_EFFECT)
    }
    #[doc = "Request hardware to update frequency division parameters."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(FREQUENCY_A::UPDATE)
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
#[doc = "Field `peripheral` reader - 160-MHz clock divide factor"]
pub struct PERIPHERAL_R(crate::FieldReader<u8, u8>);
impl PERIPHERAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PERIPHERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPHERAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `peripheral` writer - 160-MHz clock divide factor"]
pub struct PERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `bus2_sync` reader - Ratio between bus1 and bus2 clock frequency"]
pub struct BUS2_SYNC_R(crate::FieldReader<u8, u8>);
impl BUS2_SYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUS2_SYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS2_SYNC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bus2_sync` writer - Ratio between bus1 and bus2 clock frequency"]
pub struct BUS2_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS2_SYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `wlan` reader - PLL to WLAN system divide factor\n\n After dividing the frequency of the clock from the PLL, it is sent to the wlan system. This register is the frequency division factor, the factor should be >= 2.\n\n"]
pub struct WLAN_R(crate::FieldReader<u8, u8>);
impl WLAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wlan` writer - PLL to WLAN system divide factor\n\n After dividing the frequency of the clock from the PLL, it is sent to the wlan system. This register is the frequency division factor, the factor should be >= 2.\n\n"]
pub struct WLAN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLAN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `cpu` reader - PLL to CPU clock divide factor"]
pub struct CPU_R(crate::FieldReader<u8, u8>);
impl CPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu` writer - PLL to CPU clock divide factor"]
pub struct CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Divide frequency enable When you need to reconfigure cpu_clk_divider, wlan_clk_divider, bus2_syncdn_factor, sdadc_fdiv, set this register, the hardware will automatically update the above four parameters to the divider, and then clear this register."]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - 160-MHz clock divide factor"]
    #[inline(always)]
    pub fn peripheral(&self) -> PERIPHERAL_R {
        PERIPHERAL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Ratio between bus1 and bus2 clock frequency"]
    #[inline(always)]
    pub fn bus2_sync(&self) -> BUS2_SYNC_R {
        BUS2_SYNC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PLL to WLAN system divide factor After dividing the frequency of the clock from the PLL, it is sent to the wlan system. This register is the frequency division factor, the factor should be >= 2."]
    #[inline(always)]
    pub fn wlan(&self) -> WLAN_R {
        WLAN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PLL to CPU clock divide factor"]
    #[inline(always)]
    pub fn cpu(&self) -> CPU_R {
        CPU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Divide frequency enable When you need to reconfigure cpu_clk_divider, wlan_clk_divider, bus2_syncdn_factor, sdadc_fdiv, set this register, the hardware will automatically update the above four parameters to the divider, and then clear this register."]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
    #[doc = "Bits 24:27 - 160-MHz clock divide factor"]
    #[inline(always)]
    pub fn peripheral(&mut self) -> PERIPHERAL_W {
        PERIPHERAL_W { w: self }
    }
    #[doc = "Bits 16:23 - Ratio between bus1 and bus2 clock frequency"]
    #[inline(always)]
    pub fn bus2_sync(&mut self) -> BUS2_SYNC_W {
        BUS2_SYNC_W { w: self }
    }
    #[doc = "Bits 8:15 - PLL to WLAN system divide factor After dividing the frequency of the clock from the PLL, it is sent to the wlan system. This register is the frequency division factor, the factor should be >= 2."]
    #[inline(always)]
    pub fn wlan(&mut self) -> WLAN_W {
        WLAN_W { w: self }
    }
    #[doc = "Bits 8:15 - PLL to CPU clock divide factor"]
    #[inline(always)]
    pub fn cpu(&mut self) -> CPU_W {
        CPU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software clock division configuration\n\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_divide](index.html) module"]
pub struct CLOCK_DIVIDE_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_divide::R](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_divide::W](W) writer structure"]
impl crate::Writable for CLOCK_DIVIDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clock_divide to value 0x0302_0306"]
impl crate::Resettable for CLOCK_DIVIDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0302_0306
    }
}
