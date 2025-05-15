#[doc = "Register `DATA16` reader"]
pub type R = crate::R<Data16Spec>;
#[doc = "Register `DATA16` writer"]
pub type W = crate::W<Data16Spec>;
#[doc = "Field `DATA16` reader - Входные или выходные данные"]
pub type Data16R = crate::FieldReader<u16>;
#[doc = "Field `DATA16` writer - Входные или выходные данные"]
pub type Data16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data16(&self) -> Data16R {
        Data16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data16(&mut self) -> Data16W<Data16Spec> {
        Data16W::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных 16 бит. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 \"Load access fault\")\n\nYou can [`read`](crate::Reg::read) this register and get [`data16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data16Spec;
impl crate::RegisterSpec for Data16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data16::R`](R) reader structure"]
impl crate::Readable for Data16Spec {}
#[doc = "`write(|w| ..)` method takes [`data16::W`](W) writer structure"]
impl crate::Writable for Data16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA16 to value 0"]
impl crate::Resettable for Data16Spec {}
