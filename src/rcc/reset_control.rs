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
#[doc = "Touch sensor module reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUCH_A {
    #[doc = "0: Reset the corresponding peripheral."]
    RESET = 0,
    #[doc = "1: Release the reset state the corresponding peripheral."]
    RELEASE = 1,
}
impl From<TOUCH_A> for bool {
    #[inline(always)]
    fn from(variant: TOUCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `touch` reader - Touch sensor module reset"]
pub struct TOUCH_R(crate::FieldReader<bool>);
impl TOUCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUCH_A {
        match self.bits {
            false => TOUCH_A::RESET,
            true => TOUCH_A::RELEASE,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == TOUCH_A::RESET
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        **self == TOUCH_A::RELEASE
    }
}
impl core::ops::Deref for TOUCH_R {
    type Target = crate::FieldReader<bool>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TOUCH_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(TOUCH_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Flash controller module reset"]
pub use TOUCH_A as FLASH_A;
#[doc = "Field `flash` reader - Flash controller module reset"]
pub use TOUCH_R as FLASH_R;
#[doc = "Field `flash` writer - Flash controller module reset"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FLASH_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(FLASH_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "RSA Montgomery coprocessor module reset"]
pub use TOUCH_A as RSA_A;
#[doc = "Field `rsa` reader - RSA Montgomery coprocessor module reset"]
pub use TOUCH_R as RSA_R;
#[doc = "Field `rsa` writer - RSA Montgomery coprocessor module reset"]
pub struct RSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RSA_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(RSA_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Inter-Integrated Sound module reset"]
pub use TOUCH_A as I2S_A;
#[doc = "Field `i2s` reader - Inter-Integrated Sound module reset"]
pub use TOUCH_R as I2S_R;
#[doc = "Field `i2s` writer - Inter-Integrated Sound module reset"]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2S_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(I2S_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Pulse-width modulation module reset"]
pub use TOUCH_A as PWM_A;
#[doc = "Field `pwm` reader - Pulse-width modulation module reset"]
pub use TOUCH_R as PWM_R;
#[doc = "Field `pwm` writer - Pulse-width modulation module reset"]
pub struct PWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWM_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(PWM_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Sar-adc module reset"]
pub use TOUCH_A as ADC_A;
#[doc = "Field `adc` reader - Sar-adc module reset"]
pub use TOUCH_R as ADC_R;
#[doc = "Field `adc` writer - Sar-adc module reset"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADC_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(ADC_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Timer module reset"]
pub use TOUCH_A as TIMER_A;
#[doc = "Field `timer` reader - Timer module reset"]
pub use TOUCH_R as TIMER_R;
#[doc = "Field `timer` writer - Timer module reset"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIMER_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(TIMER_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Gpio module reset"]
pub use TOUCH_A as GPIO_A;
#[doc = "Field `gpio` reader - Gpio module reset"]
pub use TOUCH_R as GPIO_R;
#[doc = "Field `gpio` writer - Gpio module reset"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIO_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(GPIO_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Universal asynchronous transmitter/receiver reset"]
pub use TOUCH_A as UART_A;
#[doc = "Fields `uart(0-5)` reader - Universal asynchronous transmitter/receiver reset"]
pub use TOUCH_R as UART_R;
#[doc = "Fields `uart(0-5)` const generic writer - Universal asynchronous transmitter/receiver reset"]
pub struct UART_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> UART_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(UART_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(UART_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "Inter-Integrated Circuit module reset"]
pub use TOUCH_A as I2C_A;
#[doc = "Field `i2c` reader - Inter-Integrated Circuit module reset"]
pub use TOUCH_R as I2C_R;
#[doc = "Field `i2c` writer - Inter-Integrated Circuit module reset"]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(I2C_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Internal bus reset"]
pub use TOUCH_A as BUS_A;
#[doc = "Fields `bus(1-2)` reader - Internal bus reset"]
pub use TOUCH_R as BUS_R;
#[doc = "Fields `bus(1-2)` const generic writer - Internal bus reset"]
pub struct BUS_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> BUS_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BUS_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(BUS_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << O)) | ((value as u32 & 1) << O);
        self.w
    }
}
#[doc = "APB bridge module reset"]
pub use TOUCH_A as APB_A;
#[doc = "Field `apb` reader - APB bridge module reset"]
pub use TOUCH_R as APB_R;
#[doc = "Field `apb` writer - APB bridge module reset"]
pub struct APB_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(APB_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(APB_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Direct memory access module reset"]
pub use TOUCH_A as DMA_A;
#[doc = "Field `dma` reader - Direct memory access module reset"]
pub use TOUCH_R as DMA_R;
#[doc = "Field `dma` writer - Direct memory access module reset"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(DMA_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Sdio-Ahb clock domain reset"]
pub use TOUCH_A as SDIO_AHB_A;
#[doc = "Field `sdio_ahb` reader - Sdio-Ahb clock domain reset"]
pub use TOUCH_R as SDIO_AHB_R;
#[doc = "Field `sdio_ahb` writer - Sdio-Ahb clock domain reset"]
pub struct SDIO_AHB_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_AHB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_AHB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the corresponding peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDIO_AHB_A::RESET)
    }
    #[doc = "Release the reset state the corresponding peripheral."]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(SDIO_AHB_A::RELEASE)
    }
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Touch sensor module reset"]
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Flash controller module reset"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 25 - RSA Montgomery coprocessor module reset"]
    #[inline(always)]
    pub fn rsa(&self) -> RSA_R {
        RSA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Inter-Integrated Sound module reset"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - Pulse-width modulation module reset"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Sar-adc module reset"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer module reset"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Gpio module reset"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub unsafe fn uart(&self, n: usize) -> UART_R {
        UART_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    #[doc = "Bit 10 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart0(&self) -> UART_R {
        UART_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART_R {
        UART_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart2(&self) -> UART_R {
        UART_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart3(&self) -> UART_R {
        UART_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart4(&self) -> UART_R {
        UART_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart5(&self) -> UART_R {
        UART_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 9 - Inter-Integrated Circuit module reset"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Internal bus reset"]
    #[inline(always)]
    pub unsafe fn bus(&self, n: usize) -> BUS_R {
        BUS_R::new(((self.bits >> (n - 1 + 7)) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal bus reset"]
    #[inline(always)]
    pub fn bus1(&self) -> BUS_R {
        BUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal bus reset"]
    #[inline(always)]
    pub fn bus2(&self) -> BUS_R {
        BUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - APB bridge module reset"]
    #[inline(always)]
    pub fn apb(&self) -> APB_R {
        APB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Direct memory access module reset"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Sdio-Ahb clock domain reset"]
    #[inline(always)]
    pub fn sdio_ahb(&self) -> SDIO_AHB_R {
        SDIO_AHB_R::new(((self.bits >> 3) & 1) != 0)
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
    pub unsafe fn uart<const O: usize>(&mut self) -> UART_W<O> {
        UART_W { w: self }
    }
    #[doc = "Bit 10 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART_W<10> {
        UART_W { w: self }
    }
    #[doc = "Bit 11 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART_W<11> {
        UART_W { w: self }
    }
    #[doc = "Bit 12 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART_W<12> {
        UART_W { w: self }
    }
    #[doc = "Bit 13 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART_W<13> {
        UART_W { w: self }
    }
    #[doc = "Bit 14 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART_W<14> {
        UART_W { w: self }
    }
    #[doc = "Bit 15 - Universal asynchronous transmitter/receiver reset"]
    #[inline(always)]
    pub fn uart5(&mut self) -> UART_W<15> {
        UART_W { w: self }
    }
    #[doc = "Bit 9 - Inter-Integrated Circuit module reset"]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Internal bus reset"]
    #[inline(always)]
    pub unsafe fn bus<const O: usize>(&mut self) -> BUS_W<O> {
        BUS_W { w: self }
    }
    #[doc = "Bit 7 - Internal bus reset"]
    #[inline(always)]
    pub fn bus1(&mut self) -> BUS_W<7> {
        BUS_W { w: self }
    }
    #[doc = "Bit 8 - Internal bus reset"]
    #[inline(always)]
    pub fn bus2(&mut self) -> BUS_W<8> {
        BUS_W { w: self }
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
#[doc = "Software module reset control\n\n The chip provides the soft reset function of each subsystem, and the subsystem reset can be achieved by setting the corresponding bit of this register to 0. However, the reset state will not be automatically cleared. To restore normal operation, the corresponding bit of this register must be set to 1.\n\n The soft reset function does not reset the CPU and watchdog. In this register, the reset operation of APB, BUS1 and BUS2 (corresponding to APB bus, system bus and data bus) is not recommended, which will cause system access device abnormality.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_control](index.html) module"]
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
