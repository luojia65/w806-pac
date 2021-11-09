#[doc = "Register `clock_enable` reader"]
pub struct R(crate::R<CLOCK_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Software configure clock gate enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_enable](index.html) module"]
pub struct CLOCK_ENABLE_SPEC;
impl crate::RegisterSpec for CLOCK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_enable::R](R) reader structure"]
impl crate::Readable for CLOCK_ENABLE_SPEC {
    type Reader = R;
}
