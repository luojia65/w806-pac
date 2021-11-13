#[doc = "Register `reset_state` reader"]
pub struct R(crate::R<RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reset_state` writer"]
pub struct W(crate::W<RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATE_SPEC>;
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
impl From<crate::W<RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write 1 to clear CPU soft reset state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_AW {
    #[doc = "1: Write 1 to clear reset state."]
    CLEAR = 1,
}
impl From<CPU_AW> for bool {
    #[inline(always)]
    fn from(variant: CPU_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cpu` writer - Write 1 to clear CPU soft reset state."]
pub struct CPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write 1 to clear reset state."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CPU_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Write 1 to clear watchdog soft reset state."]
pub type WATCHDOG_AW = CPU_AW;
#[doc = "Field `watchdog` writer - Write 1 to clear watchdog soft reset state."]
pub struct WATCHDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WATCHDOG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write 1 to clear reset state."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WATCHDOG_AW::CLEAR)
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
#[doc = "Reads if cpu has been software reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_A {
    #[doc = "1: Reset has procceeded."]
    HAS_RESET = 1,
    #[doc = "0: No reset have been procceeded."]
    NO_RESET = 0,
}
impl From<CPU_A> for bool {
    #[inline(always)]
    fn from(variant: CPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cpu` reader - Reads if cpu has been software reset."]
pub struct CPU_R(crate::FieldReader<bool, CPU_A>);
impl CPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU_A {
        match self.bits {
            true => CPU_A::HAS_RESET,
            false => CPU_A::NO_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `HAS_RESET`"]
    #[inline(always)]
    pub fn is_has_reset(&self) -> bool {
        **self == CPU_A::HAS_RESET
    }
    #[doc = "Checks if the value of the field is `NO_RESET`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        **self == CPU_A::NO_RESET
    }
}
impl core::ops::Deref for CPU_R {
    type Target = crate::FieldReader<bool, CPU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Reads if watchdog has been software reset."]
pub type WATCHDOG_A = CPU_A;
#[doc = "Field `watchdog` reader - Reads if watchdog has been software reset."]
pub type WATCHDOG_R = CPU_R;
impl R {
    #[doc = "Bit 1 - Reads if cpu has been software reset."]
    #[inline(always)]
    pub fn cpu(&self) -> CPU_R {
        CPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Reads if watchdog has been software reset."]
    #[inline(always)]
    pub fn watchdog(&self) -> WATCHDOG_R {
        WATCHDOG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Write 1 to clear CPU soft reset state."]
    #[inline(always)]
    pub fn cpu(&mut self) -> CPU_W {
        CPU_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 to clear watchdog soft reset state."]
    #[inline(always)]
    pub fn watchdog(&mut self) -> WATCHDOG_W {
        WATCHDOG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU and watchdog reset state register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_state::W](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets reset_state to value 0"]
impl crate::Resettable for RESET_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
