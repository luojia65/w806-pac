#[doc = "Register `clock_div` reader"]
pub struct R(crate::R<CLOCK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_div](index.html) module"]
pub struct CLOCK_DIV_SPEC;
impl crate::RegisterSpec for CLOCK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_div::R](R) reader structure"]
impl crate::Readable for CLOCK_DIV_SPEC {
    type Reader = R;
}
