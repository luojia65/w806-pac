#[doc = "Register `reset_status` reader"]
pub struct R(crate::R<RESET_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_status](index.html) module"]
pub struct RESET_STATUS_SPEC;
impl crate::RegisterSpec for RESET_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_status::R](R) reader structure"]
impl crate::Readable for RESET_STATUS_SPEC {
    type Reader = R;
}
