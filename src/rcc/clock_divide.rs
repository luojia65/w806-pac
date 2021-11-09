#[doc = "Register `clock_divide` reader"]
pub struct R(crate::R<CLOCK_DIVIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_DIVIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_DIVIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_DIVIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Software clock division configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_divide](index.html) module"]
pub struct CLOCK_DIVIDE_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_divide::R](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDE_SPEC {
    type Reader = R;
}
