#[doc = "Register `debug_control` reader"]
pub struct R(crate::R<DEBUG_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Debug control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_control](index.html) module"]
pub struct DEBUG_CONTROL_SPEC;
impl crate::RegisterSpec for DEBUG_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_control::R](R) reader structure"]
impl crate::Readable for DEBUG_CONTROL_SPEC {
    type Reader = R;
}
