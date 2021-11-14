#[doc = "Register `pull_down` reader"]
pub struct R(crate::R<PULL_DOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULL_DOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULL_DOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULL_DOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Internal pull down register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pull_down](index.html) module"]
pub struct PULL_DOWN_SPEC;
impl crate::RegisterSpec for PULL_DOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pull_down::R](R) reader structure"]
impl crate::Readable for PULL_DOWN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pull_down to value 0"]
impl crate::Resettable for PULL_DOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
