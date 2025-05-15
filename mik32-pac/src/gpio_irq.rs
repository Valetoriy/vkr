#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    state: State,
    line_mux: LineMux,
    interrupt: Interrupt,
    enable_set: EnableSet,
    enable_clear: EnableClear,
    edge: Edge,
    level: Level,
    level_set: LevelSet,
    level_clear: LevelClear,
    any_edge_set: AnyEdgeSet,
    any_edge_clear: AnyEdgeClear,
    clear: Clear,
}
impl RegisterBlock {
    #[doc = "0x00 - Текущее состояние линий после мультиплексоров. Номер бита соответствует линии запроса прерывания"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x04 - Управление мультиплексорами всех 8-ми линий. 4 бита кодируют выбор 1-го из 10-ти доступных портов: MUX_0 – \\[3:0\\]; MUX_1 – \\[7:4\\]; MUX_2 – \\[11:8\\]; MUX_3 – \\[15:12\\]; MUX_4 – \\[19:16\\]; MUX_5 – \\[23:20\\]; MUX_6 – \\[27:24\\]; MUX_7 – \\[31:28\\] Внимание!!! Схемотехническая ошибка в MIK32V2 ограничила чтение регистра LINE_MUX 8-ю младшими битами, а остальные читаются как «0». Запись осуществляется корректно во все 32 бита. Поэтому регистр помечен доступным только на запись. Примечание: На входы неиспользуемых линиий рекомендуется подать «заглушку» в виде логического «0» записью числа 10 в настройках мультиплексоров"]
    #[inline(always)]
    pub const fn line_mux(&self) -> &LineMux {
        &self.line_mux
    }
    #[doc = "0x08 - Текущее состояние флагов запросов прерывания. Номер бита соответствует номеру линии"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x0c - Разрешение запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» разрешает запрос от соответствующей линии"]
    #[inline(always)]
    pub const fn enable_set(&self) -> &EnableSet {
        &self.enable_set
    }
    #[doc = "0x10 - Запрет запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» запрещает запрос от соответствующей линии"]
    #[inline(always)]
    pub const fn enable_clear(&self) -> &EnableClear {
        &self.enable_clear
    }
    #[doc = "0x14 - Выбор типа запросов прерывания по событию. При чтении – текущий тип запросов: 0: по уровню; 1: по событию (фронт или спад). Запись «1» – запрос формируется по событию для соответствующей линии"]
    #[inline(always)]
    pub const fn edge(&self) -> &Edge {
        &self.edge
    }
    #[doc = "0x18 - Выбор типа запросов прерывания по уровню. При чтении – текущий тип запроса (инвертированный): 0: по событию (фронт или спад); 1: по уровню. Запись «1» – запрос формируется по уровню для соответствующей линии"]
    #[inline(always)]
    pub const fn level(&self) -> &Level {
        &self.level
    }
    #[doc = "0x1c - Выбор фронта и высокого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по фронту или уровню логической «1» для соответствующей линии"]
    #[inline(always)]
    pub const fn level_set(&self) -> &LevelSet {
        &self.level_set
    }
    #[doc = "0x20 - Выбор спада и низкого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по спаду или уровню логического «0» для соответствующей линии"]
    #[inline(always)]
    pub const fn level_clear(&self) -> &LevelClear {
        &self.level_clear
    }
    #[doc = "0x24 - Выбор запросов прерывания по любому событию. Запись «1» – запрос формируется по любому изменению соответствующей линии"]
    #[inline(always)]
    pub const fn any_edge_set(&self) -> &AnyEdgeSet {
        &self.any_edge_set
    }
    #[doc = "0x28 - Отмена выбора запросов прерывания по любому событию. Запись «1» – запрос не формируется по любому изменению соответствующей линии"]
    #[inline(always)]
    pub const fn any_edge_clear(&self) -> &AnyEdgeClear {
        &self.any_edge_clear
    }
    #[doc = "0x2c - Сброс флагов запросов прерывания по событию. Запись «1» – очищает флаг запроса прерывания для соответствующей линии"]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
}
#[doc = "STATE (r) register accessor: Текущее состояние линий после мультиплексоров. Номер бита соответствует линии запроса прерывания\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Текущее состояние линий после мультиплексоров. Номер бита соответствует линии запроса прерывания"]
pub mod state;
#[doc = "LINE_MUX (w) register accessor: Управление мультиплексорами всех 8-ми линий. 4 бита кодируют выбор 1-го из 10-ти доступных портов: MUX_0 – \\[3:0\\]; MUX_1 – \\[7:4\\]; MUX_2 – \\[11:8\\]; MUX_3 – \\[15:12\\]; MUX_4 – \\[19:16\\]; MUX_5 – \\[23:20\\]; MUX_6 – \\[27:24\\]; MUX_7 – \\[31:28\\] Внимание!!! Схемотехническая ошибка в MIK32V2 ограничила чтение регистра LINE_MUX 8-ю младшими битами, а остальные читаются как «0». Запись осуществляется корректно во все 32 бита. Поэтому регистр помечен доступным только на запись. Примечание: На входы неиспользуемых линиий рекомендуется подать «заглушку» в виде логического «0» записью числа 10 в настройках мультиплексоров\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line_mux::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line_mux`] module"]
#[doc(alias = "LINE_MUX")]
pub type LineMux = crate::Reg<line_mux::LineMuxSpec>;
#[doc = "Управление мультиплексорами всех 8-ми линий. 4 бита кодируют выбор 1-го из 10-ти доступных портов: MUX_0 – \\[3:0\\]; MUX_1 – \\[7:4\\]; MUX_2 – \\[11:8\\]; MUX_3 – \\[15:12\\]; MUX_4 – \\[19:16\\]; MUX_5 – \\[23:20\\]; MUX_6 – \\[27:24\\]; MUX_7 – \\[31:28\\] Внимание!!! Схемотехническая ошибка в MIK32V2 ограничила чтение регистра LINE_MUX 8-ю младшими битами, а остальные читаются как «0». Запись осуществляется корректно во все 32 бита. Поэтому регистр помечен доступным только на запись. Примечание: На входы неиспользуемых линиий рекомендуется подать «заглушку» в виде логического «0» записью числа 10 в настройках мультиплексоров"]
pub mod line_mux;
#[doc = "INTERRUPT (r) register accessor: Текущее состояние флагов запросов прерывания. Номер бита соответствует номеру линии\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`] module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Текущее состояние флагов запросов прерывания. Номер бита соответствует номеру линии"]
pub mod interrupt;
#[doc = "ENABLE_SET (rw) register accessor: Разрешение запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» разрешает запрос от соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_set`] module"]
#[doc(alias = "ENABLE_SET")]
pub type EnableSet = crate::Reg<enable_set::EnableSetSpec>;
#[doc = "Разрешение запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» разрешает запрос от соответствующей линии"]
pub mod enable_set;
#[doc = "ENABLE_CLEAR (rw) register accessor: Запрет запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» запрещает запрос от соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_clear`] module"]
#[doc(alias = "ENABLE_CLEAR")]
pub type EnableClear = crate::Reg<enable_clear::EnableClearSpec>;
#[doc = "Запрет запросов прерывания. При чтении: текущее состояние разрешений запросов. Запись «1» запрещает запрос от соответствующей линии"]
pub mod enable_clear;
#[doc = "EDGE (rw) register accessor: Выбор типа запросов прерывания по событию. При чтении – текущий тип запросов: 0: по уровню; 1: по событию (фронт или спад). Запись «1» – запрос формируется по событию для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`edge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge`] module"]
#[doc(alias = "EDGE")]
pub type Edge = crate::Reg<edge::EdgeSpec>;
#[doc = "Выбор типа запросов прерывания по событию. При чтении – текущий тип запросов: 0: по уровню; 1: по событию (фронт или спад). Запись «1» – запрос формируется по событию для соответствующей линии"]
pub mod edge;
#[doc = "LEVEL (rw) register accessor: Выбор типа запросов прерывания по уровню. При чтении – текущий тип запроса (инвертированный): 0: по событию (фронт или спад); 1: по уровню. Запись «1» – запрос формируется по уровню для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level`] module"]
#[doc(alias = "LEVEL")]
pub type Level = crate::Reg<level::LevelSpec>;
#[doc = "Выбор типа запросов прерывания по уровню. При чтении – текущий тип запроса (инвертированный): 0: по событию (фронт или спад); 1: по уровню. Запись «1» – запрос формируется по уровню для соответствующей линии"]
pub mod level;
#[doc = "LEVEL_SET (rw) register accessor: Выбор фронта и высокого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по фронту или уровню логической «1» для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level_set`] module"]
#[doc(alias = "LEVEL_SET")]
pub type LevelSet = crate::Reg<level_set::LevelSetSpec>;
#[doc = "Выбор фронта и высокого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по фронту или уровню логической «1» для соответствующей линии"]
pub mod level_set;
#[doc = "LEVEL_CLEAR (rw) register accessor: Выбор спада и низкого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по спаду или уровню логического «0» для соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`level_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level_clear`] module"]
#[doc(alias = "LEVEL_CLEAR")]
pub type LevelClear = crate::Reg<level_clear::LevelClearSpec>;
#[doc = "Выбор спада и низкого уровня для запросов прерывания. При чтении: 0: по спаду или уровню логического «0»; 1: по фронту или уровню логической «1». Запись «1» – запрос формируется по спаду или уровню логического «0» для соответствующей линии"]
pub mod level_clear;
#[doc = "ANY_EDGE_SET (rw) register accessor: Выбор запросов прерывания по любому событию. Запись «1» – запрос формируется по любому изменению соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_edge_set`] module"]
#[doc(alias = "ANY_EDGE_SET")]
pub type AnyEdgeSet = crate::Reg<any_edge_set::AnyEdgeSetSpec>;
#[doc = "Выбор запросов прерывания по любому событию. Запись «1» – запрос формируется по любому изменению соответствующей линии"]
pub mod any_edge_set;
#[doc = "ANY_EDGE_CLEAR (rw) register accessor: Отмена выбора запросов прерывания по любому событию. Запись «1» – запрос не формируется по любому изменению соответствующей линии\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_edge_clear`] module"]
#[doc(alias = "ANY_EDGE_CLEAR")]
pub type AnyEdgeClear = crate::Reg<any_edge_clear::AnyEdgeClearSpec>;
#[doc = "Отмена выбора запросов прерывания по любому событию. Запись «1» – запрос не формируется по любому изменению соответствующей линии"]
pub mod any_edge_clear;
#[doc = "CLEAR (w) register accessor: Сброс флагов запросов прерывания по событию. Запись «1» – очищает флаг запроса прерывания для соответствующей линии\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`] module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Сброс флагов запросов прерывания по событию. Запись «1» – очищает флаг запроса прерывания для соответствующей линии"]
pub mod clear;
