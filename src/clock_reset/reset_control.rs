#[doc = "Register `reset_control` reader"]
pub struct R(crate::R<RESET_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_control](index.html) module"]
pub struct RESET_CONTROL_SPEC;
impl crate::RegisterSpec for RESET_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_control::R](R) reader structure"]
impl crate::Readable for RESET_CONTROL_SPEC {
    type Reader = R;
}
