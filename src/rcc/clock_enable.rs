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
#[doc = "Touch sensor module clock gate enable\n\n By deafult, touch sensor module clock gate is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOUCH_A {
    #[doc = "0: The clock gate is turned off."]
    DISABLE = 0,
    #[doc = "1: The clock gate is ruened on."]
    ENABLE = 1,
}
impl From<TOUCH_A> for bool {
    #[inline(always)]
    fn from(variant: TOUCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `touch` reader - Touch sensor module clock gate enable\n\n By deafult, touch sensor module clock gate is enabled."]
pub struct TOUCH_R(crate::FieldReader<bool, TOUCH_A>);
impl TOUCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUCH_A {
        match self.bits {
            false => TOUCH_A::DISABLE,
            true => TOUCH_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TOUCH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TOUCH_A::ENABLE
    }
}
impl core::ops::Deref for TOUCH_R {
    type Target = crate::FieldReader<bool, TOUCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `touch` writer - Touch sensor module clock gate enable\n\n By deafult, touch sensor module clock gate is enabled."]
pub struct TOUCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOUCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TOUCH_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TOUCH_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Secure digital input/output clock gate enable\n\n By deafult, SDIO master module clock gate is enabled."]
pub type SDIO_A = TOUCH_A;
#[doc = "Field `sdio` reader - Secure digital input/output clock gate enable\n\n By deafult, SDIO master module clock gate is enabled."]
pub type SDIO_R = TOUCH_R;
#[doc = "Field `sdio` writer - Secure digital input/output clock gate enable\n\n By deafult, SDIO master module clock gate is enabled."]
pub struct SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDIO_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDIO_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "RSA Montgomery coprocessor clock gate enable\n\n By deafult, RSA clock gate is enabled."]
pub type RSA_A = TOUCH_A;
#[doc = "Field `rsa` reader - RSA Montgomery coprocessor clock gate enable\n\n By deafult, RSA clock gate is enabled."]
pub type RSA_R = TOUCH_R;
#[doc = "Field `rsa` writer - RSA Montgomery coprocessor clock gate enable\n\n By deafult, RSA clock gate is enabled."]
pub struct RSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RSA_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RSA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Inter-Integrated Sound clock gate enable\n\n By deafult, I2S clock gate is enabled."]
pub type I2S_A = TOUCH_A;
#[doc = "Field `i2s` reader - Inter-Integrated Sound clock gate enable\n\n By deafult, I2S clock gate is enabled."]
pub type I2S_R = TOUCH_R;
#[doc = "Field `i2s` writer - Inter-Integrated Sound clock gate enable\n\n By deafult, I2S clock gate is enabled."]
pub struct I2S_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2S_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2S_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Pulse-width modulation module clock gate enable\n\n By deafult, PWM clock gate is enabled."]
pub type PWM_A = TOUCH_A;
#[doc = "Field `pwm` reader - Pulse-width modulation module clock gate enable\n\n By deafult, PWM clock gate is enabled."]
pub type PWM_R = TOUCH_R;
#[doc = "Field `pwm` writer - Pulse-width modulation module clock gate enable\n\n By deafult, PWM clock gate is enabled."]
pub struct PWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Analog-digital converter clock gate enable"]
pub type ADC_A = TOUCH_A;
#[doc = "Field `adc` reader - Analog-digital converter clock gate enable"]
pub type ADC_R = TOUCH_R;
#[doc = "Field `adc` writer - Analog-digital converter clock gate enable"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "General purpose input/output clock gate enable\n\n By deafult, GPIO clock gate is enabled."]
pub type GPIO_A = TOUCH_A;
#[doc = "Field `gpio` reader - General purpose input/output clock gate enable\n\n By deafult, GPIO clock gate is enabled."]
pub type GPIO_R = TOUCH_R;
#[doc = "Field `gpio` writer - General purpose input/output clock gate enable\n\n By deafult, GPIO clock gate is enabled."]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Timer module clock gate enable\n\n By deafult, timer clock gate is enabled."]
pub type TIMER_A = TOUCH_A;
#[doc = "Field `timer` reader - Timer module clock gate enable\n\n By deafult, timer clock gate is enabled."]
pub type TIMER_R = TOUCH_R;
#[doc = "Field `timer` writer - Timer module clock gate enable\n\n By deafult, timer clock gate is enabled."]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Direct memory access clock gate enable\n\n By deafult, DMA clock gate is enabled."]
pub type DMA_A = TOUCH_A;
#[doc = "Field `dma` reader - Direct memory access clock gate enable\n\n By deafult, DMA clock gate is enabled."]
pub type DMA_R = TOUCH_R;
#[doc = "Field `dma` writer - Direct memory access clock gate enable\n\n By deafult, DMA clock gate is enabled."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Universal asynchronous transmitter/receiver enable\n\n By deafult, UART clock gate is enabled."]
pub type UART_A = TOUCH_A;
#[doc = "Fields `uart(0-5)` reader - Universal asynchronous transmitter/receiver enable\n\n By deafult, UART clock gate is enabled."]
pub type UART_R = TOUCH_R;
#[doc = "Fields `uart(0-5)` writer - Universal asynchronous transmitter/receiver enable\n\n By deafult, UART clock gate is enabled."]
pub struct UART_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> UART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART_A::ENABLE)
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Inter-Integrated Circuit module enable\n\n By deafult, I2C clock gate is enabled."]
pub type I2C_A = TOUCH_A;
#[doc = "Field `i2c` reader - Inter-Integrated Circuit module enable\n\n By deafult, I2C clock gate is enabled."]
pub type I2C_R = TOUCH_R;
#[doc = "Field `i2c` writer - Inter-Integrated Circuit module enable\n\n By deafult, I2C clock gate is enabled."]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The clock gate is turned off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C_A::DISABLE)
    }
    #[doc = "The clock gate is ruened on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - Touch sensor module clock gate enable By deafult, touch sensor module clock gate is enabled."]
    #[inline(always)]
    pub fn touch(&self) -> TOUCH_R {
        TOUCH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Secure digital input/output clock gate enable By deafult, SDIO master module clock gate is enabled."]
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RSA Montgomery coprocessor clock gate enable By deafult, RSA clock gate is enabled."]
    #[inline(always)]
    pub fn rsa(&self) -> RSA_R {
        RSA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inter-Integrated Sound clock gate enable By deafult, I2S clock gate is enabled."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pulse-width modulation module clock gate enable By deafult, PWM clock gate is enabled."]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Analog-digital converter clock gate enable"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - General purpose input/output clock gate enable By deafult, GPIO clock gate is enabled."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timer module clock gate enable By deafult, timer clock gate is enabled."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Direct memory access clock gate enable By deafult, DMA clock gate is enabled."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Universal asynchronous transmitter/receiver enable\n\n By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub unsafe fn uart(&self, n: usize) -> UART_R {
        UART_R::new(((self.bits >> n + 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart0(&self) -> UART_R {
        UART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart1(&self) -> UART_R {
        UART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart2(&self) -> UART_R {
        UART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart3(&self) -> UART_R {
        UART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart4(&self) -> UART_R {
        UART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart5(&self) -> UART_R {
        UART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Inter-Integrated Circuit module enable By deafult, I2C clock gate is enabled."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Touch sensor module clock gate enable By deafult, touch sensor module clock gate is enabled."]
    #[inline(always)]
    pub fn touch(&mut self) -> TOUCH_W {
        TOUCH_W { w: self }
    }
    #[doc = "Bit 18 - Secure digital input/output clock gate enable By deafult, SDIO master module clock gate is enabled."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SDIO_W {
        SDIO_W { w: self }
    }
    #[doc = "Bit 16 - RSA Montgomery coprocessor clock gate enable By deafult, RSA clock gate is enabled."]
    #[inline(always)]
    pub fn rsa(&mut self) -> RSA_W {
        RSA_W { w: self }
    }
    #[doc = "Bit 15 - Inter-Integrated Sound clock gate enable By deafult, I2S clock gate is enabled."]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W {
        I2S_W { w: self }
    }
    #[doc = "Bit 13 - Pulse-width modulation module clock gate enable By deafult, PWM clock gate is enabled."]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W {
        PWM_W { w: self }
    }
    #[doc = "Bit 12 - Analog-digital converter clock gate enable"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 11 - General purpose input/output clock gate enable By deafult, GPIO clock gate is enabled."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 10 - Timer module clock gate enable By deafult, timer clock gate is enabled."]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 8 - Direct memory access clock gate enable By deafult, DMA clock gate is enabled."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Universal asynchronous transmitter/receiver enable\n\n By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub unsafe fn uart(&mut self, n: usize) -> UART_W {
        UART_W {
            w: self,
            offset: n + 1,
        }
    }
    #[doc = "Bit 1 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART_W {
        UART_W { w: self, offset: 1 }
    }
    #[doc = "Bit 2 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART_W {
        UART_W { w: self, offset: 2 }
    }
    #[doc = "Bit 3 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART_W {
        UART_W { w: self, offset: 3 }
    }
    #[doc = "Bit 4 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART_W {
        UART_W { w: self, offset: 4 }
    }
    #[doc = "Bit 5 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART_W {
        UART_W { w: self, offset: 5 }
    }
    #[doc = "Bit 6 - Universal asynchronous transmitter/receiver enable By deafult, UART clock gate is enabled."]
    #[inline(always)]
    pub fn uart5(&mut self) -> UART_W {
        UART_W { w: self, offset: 6 }
    }
    #[doc = "Bit 0 - Inter-Integrated Circuit module enable By deafult, I2C clock gate is enabled."]
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
#[doc = "Software configure clock gate enable\n\n By configuring this clock gating enable register, you can control the clock to shutdown a specified function, so as to achieve the purpose of shutting down a certain module.\n\n In order to provide the firmware with flexibility to control the power consumption of the system, the clock and reset module provides the clock gating function of each module in the system. When the clock of the corresponding module is turned off, the digital logic and clock tree of the module will stop working, which can reduce the dynamic power consumption of the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_enable](index.html) module"]
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
#[doc = "`reset()` method sets clock_enable to value 0x7fff"]
impl crate::Resettable for CLOCK_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff
    }
}
