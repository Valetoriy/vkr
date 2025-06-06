#[doc = "Register `BLOCK` reader"]
pub type R = crate::R<BlockSpec>;
#[doc = "Register `BLOCK` writer"]
pub type W = crate::W<BlockSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Буфер данных для шифрования\n\nYou can [`read`](crate::Reg::read) this register and get [`block::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockSpec;
impl crate::RegisterSpec for BlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block::R`](R) reader structure"]
impl crate::Readable for BlockSpec {}
#[doc = "`write(|w| ..)` method takes [`block::W`](W) writer structure"]
impl crate::Writable for BlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLOCK to value 0"]
impl crate::Resettable for BlockSpec {}
