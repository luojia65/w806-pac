#[doc = "Register `clock_select` reader"]
pub struct R(crate::R<CLOCK_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_select](index.html) module"]
pub struct CLOCK_SELECT_SPEC;
impl crate::RegisterSpec for CLOCK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_select::R](R) reader structure"]
impl crate::Readable for CLOCK_SELECT_SPEC {
    type Reader = R;
}
