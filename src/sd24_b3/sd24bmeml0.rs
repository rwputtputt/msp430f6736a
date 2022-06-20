#[doc = "Register `SD24BMEML0` reader"]
pub struct R(crate::R<SD24BMEML0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD24BMEML0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD24BMEML0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD24BMEML0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SD24BMEML0` writer"]
pub struct W(crate::W<SD24BMEML0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD24BMEML0_SPEC>;
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
impl From<crate::W<SD24BMEML0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD24BMEML0_SPEC>) -> Self {
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
#[doc = "SD24B Channel 0 Conversion Memory Low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd24bmeml0](index.html) module"]
pub struct SD24BMEML0_SPEC;
impl crate::RegisterSpec for SD24BMEML0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd24bmeml0::R](R) reader structure"]
impl crate::Readable for SD24BMEML0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd24bmeml0::W](W) writer structure"]
impl crate::Writable for SD24BMEML0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SD24BMEML0 to value 0"]
impl crate::Resettable for SD24BMEML0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
