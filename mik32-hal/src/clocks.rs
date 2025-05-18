//! Настройка тактирования
use crate::pac::{
    Pm, WakeUp, pm::sys_clk_mux::ForceMux as ForceSysOsc, pm::sys_clk_mux::Mux as SysClkSrc,
    wake_up::clocks_sys::Force32kClk as FreqMonClkSrc,
};
use crate::prelude::*;
use crate::time::Hertz;

/// Расширение для `PM`
pub trait PmExt {
    /// Ограничивает `PM` для дальнейшей настройки
    fn constrain(self) -> PmCfg;
}

impl PmExt for Pm {
    fn constrain(self) -> PmCfg {
        PmCfg {
            ahb_div: 0,
            apb_m_div: 0,
            apb_p_div: 0,
            sys_clk_src: SysClkSrc::Osc32m,
            force_sys_osc: ForceSysOsc::Unfixed,
        }
    }
}

/// Ограниченный для настройки `PM`
pub struct PmCfg {
    ahb_div: u8,
    apb_m_div: u8,
    apb_p_div: u8,
    sys_clk_src: SysClkSrc,
    force_sys_osc: ForceSysOsc,
}

impl PmCfg {
    /// Устанавливает делитель шины `AHB`
    pub fn ahb_div(mut self, div: u8) -> Self {
        self.ahb_div = div;
        self
    }

    /// Устанавливает делитель шины `APB_M`
    pub fn apb_m_div(mut self, div: u8) -> Self {
        self.apb_m_div = div;
        self
    }

    /// Устанавливает делитель шины `APB_P`
    pub fn apb_p_div(mut self, div: u8) -> Self {
        self.apb_p_div = div;
        self
    }

    /// Устанавливает источник тактирования системы
    pub fn sys_clk_src(mut self, src: SysClkSrc) -> Self {
        self.sys_clk_src = src;
        self
    }

    /// Устанавливает запрет на переключение источника тактирования системы
    pub fn force_sys_osc(mut self, force_sys_osc: ForceSysOsc) -> Self {
        self.force_sys_osc = force_sys_osc;
        self
    }
}

/// Расширение для `WU`
pub trait WakeUpExt {
    /// Ограничивает `WU` для дальнейшей настройки
    fn constrain(self) -> WakeUpCfg;
}

impl WakeUpExt for WakeUp {
    fn constrain(self) -> WakeUpCfg {
        WakeUpCfg {
            lsi32k_adj: 8,
            hsi32m_adj: 128,
            freq_mon_clk_src: FreqMonClkSrc::Automatic,
        }
    }
}

/// Ограниченный для настройки `WU`
pub struct WakeUpCfg {
    lsi32k_adj: u8,
    hsi32m_adj: u8,
    freq_mon_clk_src: FreqMonClkSrc,
}

impl WakeUpCfg {
    /// Устанавливает поправочный коэффициент для `LSI32K`
    /// Может принимать значения от `0` до `15`
    pub fn lsi32k_adj(mut self, adj: u8) -> Self {
        assert!(adj < 16);
        self.lsi32k_adj = adj;
        self
    }

    /// Устанавливает поправочный коэффициент для `HSI32M`
    pub fn hsi32m_adj(mut self, adj: u8) -> Self {
        self.hsi32m_adj = adj;
        self
    }

    /// Устанавливает источник тактирования монитора частоты
    pub fn freq_mon_clk_src(mut self, src: FreqMonClkSrc) -> Self {
        self.freq_mon_clk_src = src;
        self
    }
}

/// "Замороженные" настройки тактирования
///
/// Существование экземпляра этой структуры означает, что частоты больше не могут быть изменены
pub struct Clocks {
    sys_clk: Hertz,
    ahb_clk: Hertz,
    apb_m_clk: Hertz,
    apb_p_clk: Hertz,
}

impl Clocks {
    /// Применяет и "замораживает" настройки частот
    pub fn freeze(pm_cfg: PmCfg, wu_cfg: WakeUpCfg) -> Self {
        let pm = unsafe { Pm::steal() };
        let wu = unsafe { WakeUp::steal() };

        // Установка источника тактироавния монитора частоты и поправочных коэффициентов
        wu.clocks_bu()
            .modify(|_, w| unsafe { w.adj_lsi32k().bits(wu_cfg.lsi32k_adj) });
        wu.clocks_sys().modify(|_, w| unsafe {
            w.adj_hsi32m()
                .bits(wu_cfg.hsi32m_adj)
                .force_32k_clk()
                .variant(wu_cfg.freq_mon_clk_src)
        });

        // Установка источника тактирования системы
        pm.sys_clk_mux().write(|w| {
            w.mux()
                .variant(pm_cfg.sys_clk_src)
                .force_mux()
                .variant(pm_cfg.force_sys_osc)
        });
        loop {
            let pm_read = pm.freq_status().read();
            let freq_status = match pm_cfg.sys_clk_src {
                SysClkSrc::Osc32m => pm_read.mask_osc32m(),
                SysClkSrc::Hsi32m => pm_read.mask_hsi32m(),
                SysClkSrc::Osc32k => pm_read.mask_osc32k(),
                SysClkSrc::Lsi32k => pm_read.mask_lsi32k(),
            };
            if freq_status.bit_is_set() {
                break;
            }
        }

        // Установка делителей частот
        pm.div_ahb()
            .write(|w| unsafe { w.bits(pm_cfg.ahb_div as _) });
        pm.div_apb_m()
            .write(|w| unsafe { w.bits(pm_cfg.apb_m_div as _) });
        pm.div_apb_p()
            .write(|w| unsafe { w.bits(pm_cfg.apb_p_div as _) });

        let sys_clk = match pm_cfg.sys_clk_src {
            SysClkSrc::Osc32m | SysClkSrc::Hsi32m => 32_000_000.Hz(),
            SysClkSrc::Osc32k | SysClkSrc::Lsi32k => 32_768.Hz(),
        };
        let ahb_clk = sys_clk / (pm_cfg.ahb_div as u32 + 1);
        let apb_m_clk = ahb_clk / (pm_cfg.apb_m_div as u32 + 1);
        let apb_p_clk = ahb_clk / (pm_cfg.apb_p_div as u32 + 1);

        Clocks {
            sys_clk,
            ahb_clk,
            apb_m_clk,
            apb_p_clk,
        }
    }

    /// Возвращает частоту системной шины
    pub fn sys_clk(&self) -> Hertz {
        self.sys_clk
    }

    /// Возвращает частоту шины `AHB`
    pub fn ahb_clk(&self) -> Hertz {
        self.ahb_clk
    }

    /// Возвращает частоту шины `APB_M`
    pub fn apb_m_clk(&self) -> Hertz {
        self.apb_m_clk
    }

    /// Возвращает частоту шины `APB_P`
    pub fn apb_p_clk(&self) -> Hertz {
        self.apb_p_clk
    }
}
