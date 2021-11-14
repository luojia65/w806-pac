#[doc = "Register `interrupt_clear` reader"]
pub struct R(crate::R<INTERRUPT_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Clear interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_clear](index.html) module"]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_clear::R](R) reader structure"]
impl crate::Readable for INTERRUPT_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
