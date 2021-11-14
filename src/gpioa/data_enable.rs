#[doc = "Register `data_enable` reader"]
pub struct R(crate::R<DATA_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "GPIO data read/write enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_enable](index.html) module"]
pub struct DATA_ENABLE_SPEC;
impl crate::RegisterSpec for DATA_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_enable::R](R) reader structure"]
impl crate::Readable for DATA_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets data_enable to value 0"]
impl crate::Resettable for DATA_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
