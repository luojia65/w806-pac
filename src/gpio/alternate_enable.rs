#[doc = "Register `alternate_enable` reader"]
pub struct R(crate::R<ALTERNATE_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTERNATE_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTERNATE_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTERNATE_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `alternate_enable` writer"]
pub struct W(crate::W<ALTERNATE_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTERNATE_ENABLE_SPEC>;
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
impl From<crate::W<ALTERNATE_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTERNATE_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alternate function enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Disable alternate function."]
    DISABLE = 0,
    #[doc = "1: Enable alternate function."]
    ENABLE = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `pin(0-15)` reader - Alternate function enable."]
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
            false => PIN_A::DISABLE,
            true => PIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PIN_A::ENABLE
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `pin(0-15)` const generic writer - Alternate function enable."]
pub struct PIN_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PIN_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable alternate function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PIN_A::DISABLE)
    }
    #[doc = "Enable alternate function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PIN_A::ENABLE)
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
    #[doc = "Alternate function enable."]
    #[inline(always)]
    pub unsafe fn pin(&self, n: usize) -> PIN_R {
        PIN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - Alternate function enable."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN_R {
        PIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alternate function enable."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate function enable."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate function enable."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alternate function enable."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alternate function enable."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate function enable."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Alternate function enable."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alternate function enable."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alternate function enable."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Alternate function enable."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Alternate function enable."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alternate function enable."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alternate function enable."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Alternate function enable."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Alternate function enable."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Alternate function enable."]
    #[inline(always)]
    pub unsafe fn pin<const O: usize>(&mut self) -> PIN_W<O> {
        PIN_W { w: self }
    }
    #[doc = "Bit 0 - Alternate function enable."]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN_W<0> {
        PIN_W { w: self }
    }
    #[doc = "Bit 1 - Alternate function enable."]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN_W<1> {
        PIN_W { w: self }
    }
    #[doc = "Bit 2 - Alternate function enable."]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN_W<2> {
        PIN_W { w: self }
    }
    #[doc = "Bit 3 - Alternate function enable."]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN_W<3> {
        PIN_W { w: self }
    }
    #[doc = "Bit 4 - Alternate function enable."]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN_W<4> {
        PIN_W { w: self }
    }
    #[doc = "Bit 5 - Alternate function enable."]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN_W<5> {
        PIN_W { w: self }
    }
    #[doc = "Bit 6 - Alternate function enable."]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN_W<6> {
        PIN_W { w: self }
    }
    #[doc = "Bit 7 - Alternate function enable."]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN_W<7> {
        PIN_W { w: self }
    }
    #[doc = "Bit 8 - Alternate function enable."]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN_W<8> {
        PIN_W { w: self }
    }
    #[doc = "Bit 9 - Alternate function enable."]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN_W<9> {
        PIN_W { w: self }
    }
    #[doc = "Bit 10 - Alternate function enable."]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN_W<10> {
        PIN_W { w: self }
    }
    #[doc = "Bit 11 - Alternate function enable."]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN_W<11> {
        PIN_W { w: self }
    }
    #[doc = "Bit 12 - Alternate function enable."]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN_W<12> {
        PIN_W { w: self }
    }
    #[doc = "Bit 13 - Alternate function enable."]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN_W<13> {
        PIN_W { w: self }
    }
    #[doc = "Bit 14 - Alternate function enable."]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN_W<14> {
        PIN_W { w: self }
    }
    #[doc = "Bit 15 - Alternate function enable."]
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
#[doc = "Alternate function enable register\n\n Each bit corresponds to whether the corresponding GPIO multiplexing function is turned on.\n\n When the bit is set, the multiplexing function depends on the corresponding bits of registers `function_high` and `function_low`.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alternate_enable](index.html) module"]
pub struct ALTERNATE_ENABLE_SPEC;
impl crate::RegisterSpec for ALTERNATE_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alternate_enable::R](R) reader structure"]
impl crate::Readable for ALTERNATE_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alternate_enable::W](W) writer structure"]
impl crate::Writable for ALTERNATE_ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets alternate_enable to value 0xffff"]
impl crate::Resettable for ALTERNATE_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
