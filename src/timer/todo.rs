#[doc = "Register `todo` reader"]
pub struct R(crate::R<TODO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TODO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TODO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TODO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `todo` writer"]
pub struct W(crate::W<TODO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TODO_SPEC>;
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
impl From<crate::W<TODO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TODO_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [todo](index.html) module"]
pub struct TODO_SPEC;
impl crate::RegisterSpec for TODO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [todo::R](R) reader structure"]
impl crate::Readable for TODO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [todo::W](W) writer structure"]
impl crate::Writable for TODO_SPEC {
    type Writer = W;
}
