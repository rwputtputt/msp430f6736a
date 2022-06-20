#[doc = "Register `LCDBM14` reader"]
pub struct R(crate::R<LCDBM14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDBM14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDBM14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDBM14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDBM14` writer"]
pub struct W(crate::W<LCDBM14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDBM14_SPEC>;
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
impl From<crate::W<LCDBM14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDBM14_SPEC>) -> Self {
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
#[doc = "LCD Blinking Memory 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdbm14](index.html) module"]
pub struct LCDBM14_SPEC;
impl crate::RegisterSpec for LCDBM14_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdbm14::R](R) reader structure"]
impl crate::Readable for LCDBM14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdbm14::W](W) writer structure"]
impl crate::Writable for LCDBM14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDBM14 to value 0"]
impl crate::Resettable for LCDBM14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
