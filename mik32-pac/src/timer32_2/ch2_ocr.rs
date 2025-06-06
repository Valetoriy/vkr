#[doc = "Register `CH2_OCR` reader"]
pub type R = crate::R<Ch2OcrSpec>;
#[doc = "Register `CH2_OCR` writer"]
pub type W = crate::W<Ch2OcrSpec>;
#[doc = "Field `OCR` reader - Значение таймера в режиме сравнения"]
pub type OcrR = crate::FieldReader<u32>;
#[doc = "Field `OCR` writer - Значение таймера в режиме сравнения"]
pub type OcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Значение таймера в режиме сравнения"]
    #[inline(always)]
    pub fn ocr(&self) -> OcrR {
        OcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Значение таймера в режиме сравнения"]
    #[inline(always)]
    pub fn ocr(&mut self) -> OcrW<Ch2OcrSpec> {
        OcrW::new(self, 0)
    }
}
#[doc = "Значение сравнения 2 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2OcrSpec;
impl crate::RegisterSpec for Ch2OcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_ocr::R`](R) reader structure"]
impl crate::Readable for Ch2OcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_ocr::W`](W) writer structure"]
impl crate::Writable for Ch2OcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH2_OCR to value 0"]
impl crate::Resettable for Ch2OcrSpec {}
