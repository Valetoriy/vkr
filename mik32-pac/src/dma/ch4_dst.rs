#[doc = "Register `CH4_DST` reader"]
pub type R = crate::R<Ch4DstSpec>;
#[doc = "Register `CH4_DST` writer"]
pub type W = crate::W<Ch4DstSpec>;
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
    pub fn dst(&mut self) -> DstW<Ch4DstSpec> {
        DstW::new(self, 0)
    }
}
#[doc = "Регистр адреса назначения канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4DstSpec;
impl crate::RegisterSpec for Ch4DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_dst::R`](R) reader structure"]
impl crate::Readable for Ch4DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_dst::W`](W) writer structure"]
impl crate::Writable for Ch4DstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4_DST to value 0"]
impl crate::Resettable for Ch4DstSpec {}
