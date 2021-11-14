#[doc = "Register `trigger_both` reader"]
pub struct R(crate::R<TRIGGER_BOTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGGER_BOTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGGER_BOTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGGER_BOTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt trigger on both sides selection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger_both](index.html) module"]
pub struct TRIGGER_BOTH_SPEC;
impl crate::RegisterSpec for TRIGGER_BOTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigger_both::R](R) reader structure"]
impl crate::Readable for TRIGGER_BOTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets trigger_both to value 0"]
impl crate::Resettable for TRIGGER_BOTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
