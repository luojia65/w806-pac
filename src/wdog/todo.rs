#[doc = "Register `todo` reader"]
pub struct R(crate::R<TODO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TODO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TODO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TODO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "??\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [todo](index.html) module"]
pub struct TODO_SPEC;
impl crate::RegisterSpec for TODO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [todo::R](R) reader structure"]
impl crate::Readable for TODO_SPEC {
    type Reader = R;
}
