#[doc = "Register `reset_control` reader"]
pub struct R(crate::R<RESET_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reset_control` writer"]
pub struct W(crate::W<RESET_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_CONTROL_SPEC>;
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
impl From<crate::W<RESET_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `touch` reader - Touch sensor module reset"]
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
#[doc = "Field `touch` writer - Touch sensor module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `flash` reader - Flash controller module reset"]
pub struct FLASH_R(crate::FieldReader<bool, bool>);
impl FLASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flash` writer - Flash controller module reset"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `rsa` reader - RSA Montgomery coprocessor module reset"]
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
#[doc = "Field `rsa` writer - RSA Montgomery coprocessor module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `i2s` reader - Inter-Integrated Sound module reset"]
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
#[doc = "Field `i2s` writer - Inter-Integrated Sound module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pwm` reader - Pulse-width modulation module reset"]
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
#[doc = "Field `pwm` writer - Pulse-width modulation module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `adc` reader - Sar-adc module reset"]
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
#[doc = "Field `adc` writer - Sar-adc module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `timer` reader - Timer module reset"]
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
#[doc = "Field `timer` writer - Timer module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `gpio` reader - Gpio module reset"]
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
#[doc = "Field `gpio` writer - Gpio module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Fields `uart(0-5)` reader - Universal asynchronous transmitter/receiver reset"]
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
#[doc = "Fields `uart(0-5)` writer - Universal asynchronous transmitter/receiver reset"]
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
#[doc = "Field `i2c` reader - Inter-Integrated Circuit module reset"]
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
#[doc = "Field `i2c` writer - Inter-Integrated Circuit module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Fields `bus(0-1)` reader - Internal bus reset"]
pub struct BUS_R(crate::FieldReader<bool, bool>);
impl BUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `bus(0-1)` writer - Internal bus reset"]
pub struct BUS_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> BUS_W<'a> {
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
#[doc = "Field `apb` reader - APB bridge module reset"]
pub struct APB_R(crate::FieldReader<bool, bool>);
impl APB_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `apb` writer - APB bridge module reset"]
pub struct APB_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `dma` reader - Direct memory access module reset"]
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
#[doc = "Field `dma` writer - Direct memory access module reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `sdio_ahb` reader - Sdio-Ahb clock domain reset"]
pub struct SDIO_AHB_R(crate::FieldReader<bool, bool>);
impl SDIO_AHB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_AHB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_AHB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdio_ahb` writer - Sdio-Ahb clock domain reset"]
pub struct SDIO_AHB_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_AHB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Touch sensor module reset"]
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Flash controller module reset"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RSA Montgomery coprocessor module reset"]
    #[inline(always)]
    pub fn rsa(&self) -> RSA_R {
        RSA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Inter-Integrated Sound module reset"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Pulse-width modulation module reset"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Sar-adc module reset"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer module reset"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Gpio module reset"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub unsafe fn uart(&self, n: usize) -> UART_R {
        UART_R::new(((self.bits >> n + 10) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart0(&self) -> UART_R {
        UART_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART_R {
        UART_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart2(&self) -> UART_R {
        UART_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart3(&self) -> UART_R {
        UART_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart4(&self) -> UART_R {
        UART_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart5(&self) -> UART_R {
        UART_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Inter-Integrated Circuit module reset"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Internal bus reset"]
    #[inline(always)]
    pub unsafe fn bus(&self, n: usize) -> BUS_R {
        BUS_R::new(((self.bits >> n + 7) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal bus reset"]
    #[inline(always)]
    pub fn bus0(&self) -> BUS_R {
        BUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Internal bus reset"]
    #[inline(always)]
    pub fn bus1(&self) -> BUS_R {
        BUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - APB bridge module reset"]
    #[inline(always)]
    pub fn apb(&self) -> APB_R {
        APB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Direct memory access module reset"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sdio-Ahb clock domain reset"]
    #[inline(always)]
    pub fn sdio_ahb(&self) -> SDIO_AHB_R {
        SDIO_AHB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Touch sensor module reset"]
    #[inline(always)]
    pub fn touch(&mut self) -> TOUCH_W {
        TOUCH_W { w: self }
    }
    #[doc = "Bit 30 - Flash controller module reset"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 25 - RSA Montgomery coprocessor module reset"]
    #[inline(always)]
    pub fn rsa(&mut self) -> RSA_W {
        RSA_W { w: self }
    }
    #[doc = "Bit 24 - Inter-Integrated Sound module reset"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 22 - Pulse-width modulation module reset"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W {
        PWM_W { w: self }
    }
    #[doc = "Bit 21 - Sar-adc module reset"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 20 - Timer module reset"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 19 - Gpio module reset"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub unsafe fn uart(&mut self, n: usize) -> UART_W {
        UART_W {
            w: self,
            offset: n + 10,
        }
    }
    #[doc = "Bit 10 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 10,
        }
    }
    #[doc = "Bit 11 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 11,
        }
    }
    #[doc = "Bit 12 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 12,
        }
    }
    #[doc = "Bit 13 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 13,
        }
    }
    #[doc = "Bit 14 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 14,
        }
    }
    #[doc = "Bit 15 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart5(&mut self) -> UART_W {
        UART_W {
            w: self,
            offset: 15,
        }
    }
    #[doc = "Bit 9 - Inter-Integrated Circuit module reset"]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Internal bus reset"]
    #[inline(always)]
    pub unsafe fn bus(&mut self, n: usize) -> BUS_W {
        BUS_W {
            w: self,
            offset: n + 7,
        }
    }
    #[doc = "Bit 7 - Internal bus reset"]
    #[inline(always)]
    pub fn bus0(&mut self) -> BUS_W {
        BUS_W { w: self, offset: 7 }
    }
    #[doc = "Bit 8 - Internal bus reset"]
    #[inline(always)]
    pub fn bus1(&mut self) -> BUS_W {
        BUS_W { w: self, offset: 8 }
    }
    #[doc = "Bit 6 - APB bridge module reset"]
    #[inline(always)]
    pub fn apb(&mut self) -> APB_W {
        APB_W { w: self }
    }
    #[doc = "Bit 4 - Direct memory access module reset"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 3 - Sdio-Ahb clock domain reset"]
    #[inline(always)]
    pub fn sdio_ahb(&mut self) -> SDIO_AHB_W {
        SDIO_AHB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software module reset control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_control](index.html) module"]
pub struct RESET_CONTROL_SPEC;
impl crate::RegisterSpec for RESET_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_control::R](R) reader structure"]
impl crate::Readable for RESET_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_control::W](W) writer structure"]
impl crate::Writable for RESET_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets reset_control to value 0xffff_ffff"]
impl crate::Resettable for RESET_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
