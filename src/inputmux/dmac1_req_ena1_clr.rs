#[doc = "Register `DMAC1_REQ_ENA1_CLR` writer"]
pub type W = crate::W<Dmac1ReqEna1ClrSpec>;
#[doc = "ESPI Channel 1 enable clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EspiCh1 {
    #[doc = "0: No Effect"]
    NoEffect = 0,
    #[doc = "1: Clears the ENA1 Bit"]
    ClrEna0Bit = 1,
}
impl From<EspiCh1> for bool {
    #[inline(always)]
    fn from(variant: EspiCh1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESPI_CH1` writer - ESPI Channel 1 enable clear"]
pub type EspiCh1W<'a, REG> = crate::BitWriter<'a, REG, EspiCh1>;
impl<'a, REG> EspiCh1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::NoEffect)
    }
    #[doc = "Clears the ENA1 Bit"]
    #[inline(always)]
    pub fn clr_ena0_bit(self) -> &'a mut crate::W<REG> {
        self.variant(EspiCh1::ClrEna0Bit)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<Dmac1ReqEna1ClrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ESPI Channel 1 enable clear"]
    #[inline(always)]
    pub fn espi_ch1(&mut self) -> EspiCh1W<Dmac1ReqEna1ClrSpec> {
        EspiCh1W::new(self, 0)
    }
}
#[doc = "DMAC1 request enable clear 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1_req_ena1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmac1ReqEna1ClrSpec;
impl crate::RegisterSpec for Dmac1ReqEna1ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac1_req_ena1_clr::W`](W) writer structure"]
impl crate::Writable for Dmac1ReqEna1ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC1_REQ_ENA1_CLR to value 0"]
impl crate::Resettable for Dmac1ReqEna1ClrSpec {
    const RESET_VALUE: u32 = 0;
}
