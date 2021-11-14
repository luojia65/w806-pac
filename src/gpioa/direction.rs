#[doc = "Register `direction` reader"]
pub struct R(crate::R<DIRECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIRECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIRECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIRECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GPIO direction register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [direction](index.html) module"]
pub struct DIRECTION_SPEC;
impl crate::RegisterSpec for DIRECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [direction::R](R) reader structure"]
impl crate::Readable for DIRECTION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets direction to value 0"]
impl crate::Resettable for DIRECTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
