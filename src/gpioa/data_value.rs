#[doc = "Register `data_value` reader"]
pub struct R(crate::R<DATA_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `data_value` writer"]
pub struct W(crate::W<DATA_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DATA_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO data value read/write register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_value](index.html) module"]
pub struct DATA_VALUE_SPEC;
impl crate::RegisterSpec for DATA_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_value::R](R) reader structure"]
impl crate::Readable for DATA_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_value::W](W) writer structure"]
impl crate::Writable for DATA_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets data_value to value 0x180b"]
impl crate::Resettable for DATA_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x180b
    }
}
