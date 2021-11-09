#[doc = "Register `clock_enable` reader"]
pub struct R(crate::R<CLOCK_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clock_enable` writer"]
pub struct W(crate::W<CLOCK_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_ENABLE_SPEC>;
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
impl From<crate::W<CLOCK_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch` reader - Touch sensor module clock gate enable"]
pub struct TOUCH_R(crate::FieldReader<bool, bool>);
impl TOUCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `touch` writer - Touch sensor module clock gate enable"]
pub struct TOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `sdio` reader - Secure digital input/output clock gate enable"]
pub struct SDIO_R(crate::FieldReader<bool, bool>);
impl SDIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdio` writer - Secure digital input/output clock gate enable"]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `rsa` reader - RSA Montgomery coprocessor clock gate enable"]
pub struct RSA_R(crate::FieldReader<bool, bool>);
impl RSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsa` writer - RSA Montgomery coprocessor clock gate enable"]
pub struct RSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `i2s` reader - Inter-Integrated Sound clock gate enable"]
pub struct I2S_R(crate::FieldReader<bool, bool>);
impl I2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2s` writer - Inter-Integrated Sound clock gate enable"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `pwm` reader - Pulse-width modulation module clock gate enable"]
pub struct PWM_R(crate::FieldReader<bool, bool>);
impl PWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm` writer - Pulse-width modulation module clock gate enable"]
pub struct PWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `adc` reader - Analog-digital converter clock gate enable"]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc` writer - Analog-digital converter clock gate enable"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `gpio` reader - General purpose input/output clock gate enable"]
pub struct GPIO_R(crate::FieldReader<bool, bool>);
impl GPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio` writer - General purpose input/output clock gate enable"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `timer` reader - Timer module clock gate enable"]
pub struct TIMER_R(crate::FieldReader<bool, bool>);
impl TIMER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer` writer - Timer module clock gate enable"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `dma` reader - Direct memory access clock gate enable"]
pub struct DMA_R(crate::FieldReader<bool, bool>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma` writer - Direct memory access clock gate enable"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Fields `uart(0-5)` reader - Universal asynchronous transmitter/receiver %s enable"]
pub struct UART_R(crate::FieldReader<bool, bool>);
impl UART_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `uart(0-5)` writer - Universal asynchronous transmitter/receiver %s enable"]
pub struct UART_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> UART_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Field `i2c` reader - Inter-Integrated Circuit module enable"]
pub struct I2C_R(crate::FieldReader<bool, bool>);
impl I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c` writer - Inter-Integrated Circuit module enable"]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - Touch sensor module clock gate enable"]
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Secure digital input/output clock gate enable"]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RSA Montgomery coprocessor clock gate enable"]
    #[inline(always)]
    pub fn rsa(&self) -> RSA_R {
        RSA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inter-Integrated Sound clock gate enable"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pulse-width modulation module clock gate enable"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog-digital converter clock gate enable"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - General purpose input/output clock gate enable"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer module clock gate enable"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Direct memory access clock gate enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Universal asynchronous transmitter/receiver (0-5) enable"]
    #[inline(always)]
    pub unsafe fn uart(&self, n: usize) -> UART_R {
        UART_R::new(((self.bits >> n + 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Universal asynchronous transmitter/receiver 0 enable"]
    #[inline(always)]
    pub fn uart0(&self) -> UART_R {
        UART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Universal asynchronous transmitter/receiver 1 enable"]
    #[inline(always)]
    pub fn uart1(&self) -> UART_R {
        UART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Universal asynchronous transmitter/receiver 2 enable"]
    #[inline(always)]
    pub fn uart2(&self) -> UART_R {
        UART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Universal asynchronous transmitter/receiver 3 enable"]
    #[inline(always)]
    pub fn uart3(&self) -> UART_R {
        UART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Universal asynchronous transmitter/receiver 4 enable"]
    #[inline(always)]
    pub fn uart4(&self) -> UART_R {
        UART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Universal asynchronous transmitter/receiver 5 enable"]
    #[inline(always)]
    pub fn uart5(&self) -> UART_R {
        UART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Inter-Integrated Circuit module enable"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Touch sensor module clock gate enable"]
    #[inline(always)]
    pub fn touch(&mut self) -> TOUCH_W {
        TOUCH_W { w: self }
    }
    #[doc = "Bit 18 - Secure digital input/output clock gate enable"]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 16 - RSA Montgomery coprocessor clock gate enable"]
    #[inline(always)]
    pub fn rsa(&mut self) -> RSA_W {
        RSA_W { w: self }
    }
    #[doc = "Bit 15 - Inter-Integrated Sound clock gate enable"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 13 - Pulse-width modulation module clock gate enable"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W {
        PWM_W { w: self }
    }
    #[doc = "Bit 12 - Analog-digital converter clock gate enable"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 11 - General purpose input/output clock gate enable"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 10 - Timer module clock gate enable"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 8 - Direct memory access clock gate enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Universal asynchronous transmitter/receiver (0-5) enable"]
    #[inline(always)]
    pub unsafe fn uart(&mut self, n: usize) -> UART_W {
        UART_W {
            w: self,
            offset: n + 1,
        }
    }
    #[doc = "Bit 1 - Universal asynchronous transmitter/receiver 0 enable"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART_W {
        UART_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Universal asynchronous transmitter/receiver 1 enable"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART_W {
        UART_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Universal asynchronous transmitter/receiver 2 enable"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART_W {
        UART_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Universal asynchronous transmitter/receiver 3 enable"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART_W {
        UART_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Universal asynchronous transmitter/receiver 4 enable"]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART_W {
        UART_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Universal asynchronous transmitter/receiver 5 enable"]
    #[inline(always)]
    pub fn uart5(&mut self) -> UART_W {
        UART_W { w: self, offset: 6 }
    }
    #[doc = "Bit 0 - Inter-Integrated Circuit module enable"]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software configure clock gate enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_enable](index.html) module"]
pub struct CLOCK_ENABLE_SPEC;
impl crate::RegisterSpec for CLOCK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_enable::R](R) reader structure"]
impl crate::Readable for CLOCK_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_enable::W](W) writer structure"]
impl crate::Writable for CLOCK_ENABLE_SPEC {
    type Writer = W;
}
