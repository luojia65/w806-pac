#[doc = "Register `direction` reader"]
pub struct R(crate::R<DIRECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `direction` writer"]
pub struct W(crate::W<DIRECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIRECTION_SPEC>;
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
impl From<crate::W<DIRECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIRECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "GPIO pin read/write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: The GPIO pin is input."]
    INPUT = 0,
    #[doc = "1: The GPIO pin is output."]
    OUTPUT = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `pin(0-15)` reader - GPIO pin read/write enable"]
pub struct PIN_R(crate::FieldReader<bool, PIN_A>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::INPUT,
            true => PIN_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == PIN_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        **self == PIN_A::OUTPUT
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `pin(0-15)` writer - GPIO pin read/write enable"]
pub struct PIN_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The GPIO pin is input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN_A::INPUT)
    }
    #[doc = "The GPIO pin is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN_A::OUTPUT)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `pin(0-15)` const generic writer - GPIO pin read/write enable"]
pub struct PIN_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PIN_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The GPIO pin is input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN_A::INPUT)
    }
    #[doc = "The GPIO pin is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
impl R {
    #[doc = "GPIO pin read/write enable"]
    #[inline(always)]
    pub unsafe fn pin(&self, n: usize) -> PIN_R {
        PIN_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "GPIO pin read/write enable"]
    #[inline(always)]
    pub unsafe fn pin(&mut self, n: usize) -> PIN_W {
        PIN_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN_CGW<0> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 1 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN_CGW<1> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 2 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN_CGW<2> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 3 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN_CGW<3> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 4 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN_CGW<4> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 5 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN_CGW<5> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 6 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN_CGW<6> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 7 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN_CGW<7> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 8 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN_CGW<8> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 9 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN_CGW<9> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 10 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN_CGW<10> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 11 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN_CGW<11> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 12 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN_CGW<12> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 13 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN_CGW<13> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 14 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN_CGW<14> {
        PIN_CGW { w: self }
    }
    #[doc = "Bit 15 - GPIO pin read/write enable"]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN_CGW<15> {
        PIN_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [direction](index.html) module"]
pub struct DIRECTION_SPEC;
impl crate::RegisterSpec for DIRECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [direction::R](R) reader structure"]
impl crate::Readable for DIRECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [direction::W](W) writer structure"]
impl crate::Writable for DIRECTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets direction to value 0"]
impl crate::Resettable for DIRECTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
