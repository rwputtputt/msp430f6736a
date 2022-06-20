#[doc = "Register `RTCPS` reader"]
pub struct R(crate::R<RTCPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS` writer"]
pub struct W(crate::W<RTCPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS_SPEC>;
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
impl From<crate::W<RTCPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS_SPEC>) -> Self {
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
#[doc = "Real Timer Prescale Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps](index.html) module"]
pub struct RTCPS_SPEC;
impl crate::RegisterSpec for RTCPS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps::R](R) reader structure"]
impl crate::Readable for RTCPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps::W](W) writer structure"]
impl crate::Writable for RTCPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPS to value 0"]
impl crate::Resettable for RTCPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
