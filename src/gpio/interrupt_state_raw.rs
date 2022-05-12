#[doc = "Register `interrupt_state_raw` reader"]
pub struct R(crate::R<INTERRUPT_STATE_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_STATE_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_STATE_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_STATE_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `interrupt_state_raw` writer"]
pub struct W(crate::W<INTERRUPT_STATE_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_STATE_RAW_SPEC>;
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
impl From<crate::W<INTERRUPT_STATE_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_STATE_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO pin raw interrupt state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: No GPIO pin interrupt is generated."]
    NO_INTERRUPT = 0,
    #[doc = "1: GPIO pin interrupt is generated."]
    HAS_INTERRUPT = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `pin(0-15)` reader - GPIO pin raw interrupt state"]
pub struct PIN_R(crate::FieldReader<bool>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::NO_INTERRUPT,
            true => PIN_A::HAS_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        **self == PIN_A::NO_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `HAS_INTERRUPT`"]
    #[inline(always)]
    pub fn is_has_interrupt(&self) -> bool {
        **self == PIN_A::HAS_INTERRUPT
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `pin(0-15)` const generic writer - GPIO pin raw interrupt state"]
pub struct PIN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PIN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No GPIO pin interrupt is generated."]
    #[inline(always)]
    pub fn no_interrupt(self) -> &'a mut W {
        self.variant(PIN_A::NO_INTERRUPT)
    }
    #[doc = "GPIO pin interrupt is generated."]
    #[inline(always)]
    pub fn has_interrupt(self) -> &'a mut W {
        self.variant(PIN_A::HAS_INTERRUPT)
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
impl R {
    #[doc = "GPIO pin raw interrupt state"]
    #[inline(always)]
    pub unsafe fn pin(&self, n: usize) -> PIN_R {
        PIN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN_R {
        PIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "GPIO pin raw interrupt state"]
    #[inline(always)]
    pub unsafe fn pin<const O: usize>(&mut self) -> PIN_W<O> {
        PIN_W { w: self }
    }
    #[doc = "Bit 0 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN_W<0> {
        PIN_W { w: self }
    }
    #[doc = "Bit 1 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN_W<1> {
        PIN_W { w: self }
    }
    #[doc = "Bit 2 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN_W<2> {
        PIN_W { w: self }
    }
    #[doc = "Bit 3 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN_W<3> {
        PIN_W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN_W<4> {
        PIN_W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN_W<5> {
        PIN_W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN_W<6> {
        PIN_W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN_W<7> {
        PIN_W { w: self }
    }
    #[doc = "Bit 8 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN_W<8> {
        PIN_W { w: self }
    }
    #[doc = "Bit 9 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN_W<9> {
        PIN_W { w: self }
    }
    #[doc = "Bit 10 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN_W<10> {
        PIN_W { w: self }
    }
    #[doc = "Bit 11 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN_W<11> {
        PIN_W { w: self }
    }
    #[doc = "Bit 12 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN_W<12> {
        PIN_W { w: self }
    }
    #[doc = "Bit 13 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN_W<13> {
        PIN_W { w: self }
    }
    #[doc = "Bit 14 - GPIO pin raw interrupt state"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN_W<14> {
        PIN_W { w: self }
    }
    #[doc = "Bit 15 - GPIO pin raw interrupt state"]
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
#[doc = "Raw interrupt state register\n\n GPIO bare interrupt status before masking.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_state_raw](index.html) module"]
pub struct INTERRUPT_STATE_RAW_SPEC;
impl crate::RegisterSpec for INTERRUPT_STATE_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_state_raw::R](R) reader structure"]
impl crate::Readable for INTERRUPT_STATE_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_state_raw::W](W) writer structure"]
impl crate::Writable for INTERRUPT_STATE_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets interrupt_state_raw to value 0"]
impl crate::Resettable for INTERRUPT_STATE_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
