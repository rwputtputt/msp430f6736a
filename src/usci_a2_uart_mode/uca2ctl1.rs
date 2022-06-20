#[doc = "Register `UCA2CTL1` reader"]
pub struct R(crate::R<UCA2CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA2CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA2CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA2CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA2CTL1` writer"]
pub struct W(crate::W<UCA2CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA2CTL1_SPEC>;
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
impl From<crate::W<UCA2CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA2CTL1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A2 Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca2ctl1](index.html) module"]
pub struct UCA2CTL1_SPEC;
impl crate::RegisterSpec for UCA2CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca2ctl1::R](R) reader structure"]
impl crate::Readable for UCA2CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca2ctl1::W](W) writer structure"]
impl crate::Writable for UCA2CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA2CTL1 to value 0"]
impl crate::Resettable for UCA2CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
