#[doc = "Register `CH1_DST` reader"]
pub type R = crate::R<Ch1DstSpec>;
#[doc = "Register `CH1_DST` writer"]
pub type W = crate::W<Ch1DstSpec>;
#[doc = "Field `Dst` reader - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
pub type DstR = crate::FieldReader<u32>;
#[doc = "Field `Dst` writer - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
pub type DstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
    #[inline(always)]
    pub fn dst(&self) -> DstR {
        DstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
    #[inline(always)]
    pub fn dst(&mut self) -> DstW<Ch1DstSpec> {
        DstW::new(self, 0)
    }
}
#[doc = "Регистр адреса назначения канала 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1DstSpec;
impl crate::RegisterSpec for Ch1DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dst::R`](R) reader structure"]
impl crate::Readable for Ch1DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1_dst::W`](W) writer structure"]
impl crate::Writable for Ch1DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1_DST to value 0"]
impl crate::Resettable for Ch1DstSpec {}
