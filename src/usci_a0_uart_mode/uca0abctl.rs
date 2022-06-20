#[doc = "Register `UCA0ABCTL` reader"]
pub struct R(crate::R<UCA0ABCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0ABCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0ABCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0ABCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0ABCTL` writer"]
pub struct W(crate::W<UCA0ABCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0ABCTL_SPEC>;
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
impl From<crate::W<UCA0ABCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0ABCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCABDEN` reader - Auto Baud Rate detect enable"]
pub type UCABDEN_R = crate::BitReader<bool>;
#[doc = "Field `UCABDEN` writer - Auto Baud Rate detect enable"]
pub type UCABDEN_W<'a> = crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, 0>;
#[doc = "Field `UCBTOE` reader - Break Timeout error"]
pub type UCBTOE_R = crate::BitReader<bool>;
#[doc = "Field `UCBTOE` writer - Break Timeout error"]
pub type UCBTOE_W<'a> = crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, 2>;
#[doc = "Field `UCSTOE` reader - Sync-Field Timeout error"]
pub type UCSTOE_R = crate::BitReader<bool>;
#[doc = "Field `UCSTOE` writer - Sync-Field Timeout error"]
pub type UCSTOE_W<'a> = crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, 3>;
#[doc = "Field `UCDELIM0` reader - Break Sync Delimiter 0"]
pub type UCDELIM0_R = crate::BitReader<bool>;
#[doc = "Field `UCDELIM0` writer - Break Sync Delimiter 0"]
pub type UCDELIM0_W<'a> = crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, 4>;
#[doc = "Field `UCDELIM1` reader - Break Sync Delimiter 1"]
pub type UCDELIM1_R = crate::BitReader<bool>;
#[doc = "Field `UCDELIM1` writer - Break Sync Delimiter 1"]
pub type UCDELIM1_W<'a> = crate::BitWriter<'a, u8, UCA0ABCTL_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&self) -> UCABDEN_R {
        UCABDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&self) -> UCBTOE_R {
        UCBTOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&self) -> UCSTOE_R {
        UCSTOE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim0(&self) -> UCDELIM0_R {
        UCDELIM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    pub fn ucdelim1(&self) -> UCDELIM1_R {
        UCDELIM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto Baud Rate detect enable"]
    #[inline(always)]
    pub fn ucabden(&mut self) -> UCABDEN_W {
        UCABDEN_W::new(self)
    }
    #[doc = "Bit 2 - Break Timeout error"]
    #[inline(always)]
    pub fn ucbtoe(&mut self) -> UCBTOE_W {
        UCBTOE_W::new(self)
    }
    #[doc = "Bit 3 - Sync-Field Timeout error"]
    #[inline(always)]
    pub fn ucstoe(&mut self) -> UCSTOE_W {
        UCSTOE_W::new(self)
    }
    #[doc = "Bit 4 - Break Sync Delimiter 0"]
    #[inline(always)]
    pub fn ucdelim0(&mut self) -> UCDELIM0_W {
        UCDELIM0_W::new(self)
    }
    #[doc = "Bit 5 - Break Sync Delimiter 1"]
    #[inline(always)]
    pub fn ucdelim1(&mut self) -> UCDELIM1_W {
        UCDELIM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A0 LIN Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0abctl](index.html) module"]
pub struct UCA0ABCTL_SPEC;
impl crate::RegisterSpec for UCA0ABCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0abctl::R](R) reader structure"]
impl crate::Readable for UCA0ABCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0abctl::W](W) writer structure"]
impl crate::Writable for UCA0ABCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0ABCTL to value 0"]
impl crate::Resettable for UCA0ABCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
