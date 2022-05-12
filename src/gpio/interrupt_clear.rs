#[doc = "Register `interrupt_clear` writer"]
pub struct W(crate::W<INTERRUPT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_CLEAR_SPEC>;
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
impl From<crate::W<INTERRUPT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO pin masked interrupt state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_AW {
    #[doc = "1: Write 1 to clear GPIO interrupt state."]
    CLEAR = 1,
}
impl From<PIN_AW> for bool {
    #[inline(always)]
    fn from(variant: PIN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `pin(0-15)` const generic writer - GPIO pin masked interrupt state"]
pub struct PIN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PIN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write 1 to clear GPIO interrupt state."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIN_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
impl W {
    #[doc = "GPIO pin masked interrupt state"]
    #[inline(always)]
    pub unsafe fn pin<const O: usize>(&mut self) -> PIN_W<O> {
        PIN_W { w: self }
    }
    #[doc = "Bit 0 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN_W<0> {
        PIN_W { w: self }
    }
    #[doc = "Bit 1 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN_W<1> {
        PIN_W { w: self }
    }
    #[doc = "Bit 2 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN_W<2> {
        PIN_W { w: self }
    }
    #[doc = "Bit 3 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN_W<3> {
        PIN_W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN_W<4> {
        PIN_W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN_W<5> {
        PIN_W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN_W<6> {
        PIN_W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN_W<7> {
        PIN_W { w: self }
    }
    #[doc = "Bit 8 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN_W<8> {
        PIN_W { w: self }
    }
    #[doc = "Bit 9 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN_W<9> {
        PIN_W { w: self }
    }
    #[doc = "Bit 10 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN_W<10> {
        PIN_W { w: self }
    }
    #[doc = "Bit 11 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN_W<11> {
        PIN_W { w: self }
    }
    #[doc = "Bit 12 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN_W<12> {
        PIN_W { w: self }
    }
    #[doc = "Bit 13 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN_W<13> {
        PIN_W { w: self }
    }
    #[doc = "Bit 14 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN_W<14> {
        PIN_W { w: self }
    }
    #[doc = "Bit 15 - GPIO pin masked interrupt state"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN_W<15> {
        PIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_clear](index.html) module"]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [interrupt_clear::W](W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
