#[doc = "Register `clock_mask` reader"]
pub struct R(crate::R<CLOCK_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clock_mask` writer"]
pub struct W(crate::W<CLOCK_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_MASK_SPEC>;
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
impl From<crate::W<CLOCK_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cpu domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM, SRAM) can be turned off adaptively.\n\n When the CPU needs to enter the WFI state, do not set the adaptive shutdown.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_A {
    #[doc = "0: Allow adaptive shutdown and opening."]
    ALLOW = 0,
    #[doc = "1: Disallow adaptive shutdown and opening."]
    DISALLOW = 1,
}
impl From<CPU_A> for bool {
    #[inline(always)]
    fn from(variant: CPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cpu` reader - Cpu domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM, SRAM) can be turned off adaptively.\n\n When the CPU needs to enter the WFI state, do not set the adaptive shutdown."]
pub struct CPU_R(crate::FieldReader<bool, CPU_A>);
impl CPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU_A {
        match self.bits {
            false => CPU_A::ALLOW,
            true => CPU_A::DISALLOW,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW`"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        **self == CPU_A::ALLOW
    }
    #[doc = "Checks if the value of the field is `DISALLOW`"]
    #[inline(always)]
    pub fn is_disallow(&self) -> bool {
        **self == CPU_A::DISALLOW
    }
}
impl core::ops::Deref for CPU_R {
    type Target = crate::FieldReader<bool, CPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu` writer - Cpu domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM, SRAM) can be turned off adaptively.\n\n When the CPU needs to enter the WFI state, do not set the adaptive shutdown."]
pub struct CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(CPU_A::ALLOW)
    }
    #[doc = "Disallow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut W {
        self.variant(CPU_A::DISALLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Sdio-Ahb domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
pub type SDIO_AHB_A = CPU_A;
#[doc = "Field `sdio_ahb` reader - Sdio-Ahb domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
pub type SDIO_AHB_R = CPU_R;
#[doc = "Field `sdio_ahb` writer - Sdio-Ahb domain allow adaptive clock configurations\n\n Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
pub struct SDIO_AHB_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_AHB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_AHB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(SDIO_AHB_A::ALLOW)
    }
    #[doc = "Disallow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut W {
        self.variant(SDIO_AHB_A::DISALLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Allow pmu to shutdown gate unit connected to pll output\n\n After the clock output by PLL, there is a gate control unit. This register configuration is used to indicate whether this unit is allowed to be shut down by the PMU."]
pub type PMU_A = CPU_A;
#[doc = "Field `pmu` reader - Allow pmu to shutdown gate unit connected to pll output\n\n After the clock output by PLL, there is a gate control unit. This register configuration is used to indicate whether this unit is allowed to be shut down by the PMU."]
pub type PMU_R = CPU_R;
#[doc = "Field `pmu` writer - Allow pmu to shutdown gate unit connected to pll output\n\n After the clock output by PLL, there is a gate control unit. This register configuration is used to indicate whether this unit is allowed to be shut down by the PMU."]
pub struct PMU_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(PMU_A::ALLOW)
    }
    #[doc = "Disallow adaptive shutdown and opening."]
    #[inline(always)]
    pub fn disallow(self) -> &'a mut W {
        self.variant(PMU_A::DISALLOW)
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
    #[doc = "Bit 6 - Cpu domain allow adaptive clock configurations Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM, SRAM) can be turned off adaptively. When the CPU needs to enter the WFI state, do not set the adaptive shutdown."]
    #[inline(always)]
    pub fn cpu(&self) -> CPU_R {
        CPU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sdio-Ahb domain allow adaptive clock configurations Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
    #[inline(always)]
    pub fn sdio_ahb(&self) -> SDIO_AHB_R {
        SDIO_AHB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Allow pmu to shutdown gate unit connected to pll output After the clock output by PLL, there is a gate control unit. This register configuration is used to indicate whether this unit is allowed to be shut down by the PMU."]
    #[inline(always)]
    pub fn pmu(&self) -> PMU_R {
        PMU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Cpu domain allow adaptive clock configurations Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM, SRAM) can be turned off adaptively. When the CPU needs to enter the WFI state, do not set the adaptive shutdown."]
    #[inline(always)]
    pub fn cpu(&mut self) -> CPU_W {
        CPU_W { w: self }
    }
    #[doc = "Bit 1 - Sdio-Ahb domain allow adaptive clock configurations Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
    #[inline(always)]
    pub fn sdio_ahb(&mut self) -> SDIO_AHB_W {
        SDIO_AHB_W { w: self }
    }
    #[doc = "Bit 0 - Allow pmu to shutdown gate unit connected to pll output After the clock output by PLL, there is a gate control unit. This register configuration is used to indicate whether this unit is allowed to be shut down by the PMU."]
    #[inline(always)]
    pub fn pmu(&mut self) -> PMU_W {
        PMU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software clock mask\n\n The chip adaptively turns off the clock of some functional modules according to the transition of some internal states.\n\n Please do not change the configuration, otherwise it may cause system abnormality when configuring PMU function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_mask](index.html) module"]
pub struct CLOCK_MASK_SPEC;
impl crate::RegisterSpec for CLOCK_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_mask::R](R) reader structure"]
impl crate::Readable for CLOCK_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_mask::W](W) writer structure"]
impl crate::Writable for CLOCK_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clock_mask to value 0x7e"]
impl crate::Resettable for CLOCK_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7e
    }
}
