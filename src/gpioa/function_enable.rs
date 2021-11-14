#[doc = "Register `function_enable` reader"]
pub struct R(crate::R<FUNCTION_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Alternate function enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function_enable](index.html) module"]
pub struct FUNCTION_ENABLE_SPEC;
impl crate::RegisterSpec for FUNCTION_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function_enable::R](R) reader structure"]
impl crate::Readable for FUNCTION_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets function_enable to value 0"]
impl crate::Resettable for FUNCTION_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
