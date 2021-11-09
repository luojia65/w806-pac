#[doc = "Register `i2s_clock` reader"]
pub struct R(crate::R<I2S_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Inter-Integrated Sound clock configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_clock](index.html) module"]
pub struct I2S_CLOCK_SPEC;
impl crate::RegisterSpec for I2S_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_clock::R](R) reader structure"]
impl crate::Readable for I2S_CLOCK_SPEC {
    type Reader = R;
}
