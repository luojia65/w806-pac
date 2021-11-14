#[doc = "Register `pull_up` reader"]
pub struct R(crate::R<PULL_UP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULL_UP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULL_UP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULL_UP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Internal pull up mask register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pull_up](index.html) module"]
pub struct PULL_UP_SPEC;
impl crate::RegisterSpec for PULL_UP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pull_up::R](R) reader structure"]
impl crate::Readable for PULL_UP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pull_up to value 0"]
impl crate::Resettable for PULL_UP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
