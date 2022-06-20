#[doc = "Register `TA3CCR1` reader"]
pub struct R(crate::R<TA3CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TA3CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TA3CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TA3CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TA3CCR1` writer"]
pub struct W(crate::W<TA3CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TA3CCR1_SPEC>;
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
impl From<crate::W<TA3CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TA3CCR1_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer3_A2 Capture/Compare 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ta3ccr1](index.html) module"]
pub struct TA3CCR1_SPEC;
impl crate::RegisterSpec for TA3CCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ta3ccr1::R](R) reader structure"]
impl crate::Readable for TA3CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ta3ccr1::W](W) writer structure"]
impl crate::Writable for TA3CCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TA3CCR1 to value 0"]
impl crate::Resettable for TA3CCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
