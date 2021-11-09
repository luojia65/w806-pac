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
#[doc = "Reset state register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
