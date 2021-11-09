#[doc = "Register `clock_mask` reader"]
pub struct R(crate::R<CLOCK_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_mask](index.html) module"]
pub struct CLOCK_MASK_SPEC;
impl crate::RegisterSpec for CLOCK_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_mask::R](R) reader structure"]
impl crate::Readable for CLOCK_MASK_SPEC {
    type Reader = R;
}
