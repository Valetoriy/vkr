#[doc = "Register `DATA8` reader"]
pub type R = crate::R<Data8Spec>;
#[doc = "Register `DATA8` writer"]
pub type W = crate::W<Data8Spec>;
#[doc = "Field `DATA8` reader - Входные или выходные данные"]
pub type Data8R = crate::FieldReader;
#[doc = "Field `DATA8` writer - Входные или выходные данные"]
pub type Data8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data8(&mut self) -> Data8W<Data8Spec> {
        Data8W::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных 8 бит. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 \"Load access fault\")\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data8Spec;
impl crate::RegisterSpec for Data8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data8::R`](R) reader structure"]
impl crate::Readable for Data8Spec {}
#[doc = "`write(|w| ..)` method takes [`data8::W`](W) writer structure"]
impl crate::Writable for Data8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA8 to value 0"]
impl crate::Resettable for Data8Spec {}
