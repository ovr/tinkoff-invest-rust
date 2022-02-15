///Денежная сумма в определенной валюте
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyValue {
    /// строковый ISO-код валюты
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "3")]
    pub nano: i32,
}
///Котировка - денежная сумма без указания валюты
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quotation {
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "1")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "2")]
    pub nano: i32,
}
///Проверка активности стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    ///Время проверки.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Режим торгов инструмента
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    ///Торговый статус не определён
    Unspecified = 0,
    ///Недоступен для торгов
    NotAvailableForTrading = 1,
    ///Период открытия торгов
    OpeningPeriod = 2,
    ///Период закрытия торгов
    ClosingPeriod = 3,
    ///Перерыв в торговле
    BreakInTrading = 4,
    ///Нормальная торговля
    NormalTrading = 5,
    ///Аукцион закрытия
    ClosingAuction = 6,
    ///Аукцион крупных пакетов
    DarkPoolAuction = 7,
    ///Дискретный аукцион
    DiscreteAuction = 8,
    ///Аукцион открытия
    OpeningAuctionPeriod = 9,
    ///Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice = 10,
    ///Сессия назначена
    SessionAssigned = 11,
    ///Сессия закрыта
    SessionClose = 12,
    ///Сессия открыта
    SessionOpen = 13,
    ///Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading = 14,
    ///Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading = 15,
    ///Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading = 16,
}
///Запрос расписания торгов
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesRequest {
    ///Наименование биржи или расчетного календаря. </br>Если не передаётся, возвращается информация по всем доступным торговым площадкам.
    #[prost(string, tag = "1")]
    pub exchange: ::prost::alloc::string::String,
    ///Начало периода по часовому поясу UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание периода по часовому поясу UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
///Список торговых площадок
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesResponse {
    /// Список торговых площадок и режимов торгов.
    #[prost(message, repeated, tag = "1")]
    pub exchanges: ::prost::alloc::vec::Vec<TradingSchedule>,
}
///Данные по торговой площадке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedule {
    /// Наименование торговой площадки.
    #[prost(string, tag = "1")]
    pub exchange: ::prost::alloc::string::String,
    /// Массив с торговыми и неторговыми днями.
    #[prost(message, repeated, tag = "2")]
    pub days: ::prost::alloc::vec::Vec<TradingDay>,
}
///Информация о времени торгов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingDay {
    /// Дата.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак торгового дня на бирже.
    #[prost(bool, tag = "2")]
    pub is_trading_day: bool,
    /// Время начала торгов по часовому поясу UTC.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания торгов по часовому поясу UTC.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала подачи заявки по часовому поясу UTC.
    #[prost(message, optional, tag = "5")]
    pub market_order_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания подачи заявки по часовому поясу UTC.
    #[prost(message, optional, tag = "6")]
    pub market_order_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
///Запрос получения инструмента по идентификатору.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentRequest {
    /// Тип идентификатора инструмента. Возможные значения: figi, ticker. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
    #[prost(enumeration = "InstrumentIdType", tag = "1")]
    pub id_type: i32,
    /// Идентификатор class_code. Обязателен при id_type = ticker.
    #[prost(string, tag = "2")]
    pub class_code: ::prost::alloc::string::String,
    /// Идентификатор запрашиваемого инструмента.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
///Запрос получения инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentsRequest {
    ///Статус запрашиваемых инструментов. Возможные значения: \[InstrumentStatus\](#instrumentstatus)
    #[prost(enumeration = "InstrumentStatus", tag = "1")]
    pub instrument_status: i32,
}
///Информация об облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondResponse {
    /// Информация об облигации.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Bond>,
}
///Список облигаций.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsResponse {
    ///Массив облигаций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Bond>,
}
///Данные по валюте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyResponse {
    /// Информация о валюте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Currency>,
}
///Данные по валютам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrenciesResponse {
    ///Массив валют.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Currency>,
}
///Данные по фонду.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfResponse {
    /// Информация о фонде.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Etf>,
}
///Данные по фондам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfsResponse {
    ///Массив фондов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Etf>,
}
///Данные по фьючерсу.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureResponse {
    /// Информация о фьючерсу.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Future>,
}
///Данные по фьючерсам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuturesResponse {
    ///Массив фьючерсов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Future>,
}
///Данные по акции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareResponse {
    /// Информация об акции.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Share>,
}
///Данные по акциям.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharesResponse {
    ///Массив акций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Share>,
}
///Объект передачи информации об облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bond {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    ///Количество выплат по купонам в год.
    #[prost(int32, tag = "17")]
    pub coupon_quantity_per_year: i32,
    ///Дата погашения облигации в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Номинал облигации.
    #[prost(message, optional, tag = "19")]
    pub nominal: ::core::option::Option<MoneyValue>,
    ///Дата выпуска облигации в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата размещения в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Цена размещения.
    #[prost(message, optional, tag = "23")]
    pub placement_price: ::core::option::Option<MoneyValue>,
    ///Значение НКД (накопленного купонного дохода) на дату.
    #[prost(message, optional, tag = "24")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "25")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "26")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Сектор экономики.
    #[prost(string, tag = "27")]
    pub sector: ::prost::alloc::string::String,
    ///Форма выпуска. Возможные значения: </br>**documentary** — документарная; </br>**non_documentary** — бездокументарная.
    #[prost(string, tag = "28")]
    pub issue_kind: ::prost::alloc::string::String,
    ///Размер выпуска.
    #[prost(int64, tag = "29")]
    pub issue_size: i64,
    ///Плановый размер выпуска.
    #[prost(int64, tag = "30")]
    pub issue_size_plan: i64,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "31")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "32")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "33")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "34")]
    pub sell_available_flag: bool,
    ///Признак облигации с плавающим купоном.
    #[prost(bool, tag = "35")]
    pub floating_coupon_flag: bool,
    ///Признак бессрочной облигации.
    #[prost(bool, tag = "36")]
    pub perpetual_flag: bool,
    ///Признак облигации с амортизацией долга.
    #[prost(bool, tag = "37")]
    pub amortization_flag: bool,
    ///Шаг цены.
    #[prost(message, optional, tag = "38")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "39")]
    pub api_trade_available_flag: bool,
}
///Объект передачи информации о валюте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    ///Номинал.
    #[prost(message, optional, tag = "17")]
    pub nominal: ::core::option::Option<MoneyValue>,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "18")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "20")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "21")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "22")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "23")]
    pub sell_available_flag: bool,
    ///Строковый ISO-код валюты.
    #[prost(string, tag = "24")]
    pub iso_currency_name: ::prost::alloc::string::String,
    ///Шаг цены.
    #[prost(message, optional, tag = "25")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "26")]
    pub api_trade_available_flag: bool,
}
///Объект передачи информации об инвестиционном фонде.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etf {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    ///Размер фиксированной комиссии фонда.
    #[prost(message, optional, tag = "17")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    ///Возможные значения: </br>**equity** — акции;</br>**fixed_income** — облигации;</br>**mixed_allocation** — смешанный;</br>**money_market** — денежный рынок;</br>**real_estate** — недвижимость;</br>**commodity** — товары;</br>**specialty** — специальный;</br>**private_equity** — private equity;</br>**alternative_investment** — альтернативные инвестиции.
    #[prost(string, tag = "18")]
    pub focus_type: ::prost::alloc::string::String,
    ///Дата выпуска в часовом поясе UTC.
    #[prost(message, optional, tag = "19")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Количество акций фонда в обращении.
    #[prost(message, optional, tag = "20")]
    pub num_shares: ::core::option::Option<Quotation>,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    ///Частота ребалансировки.
    #[prost(string, tag = "24")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    ///Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
}
///Объект передачи информации о фьючерсе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Future {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "4")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по клиенту.
    #[prost(message, optional, tag = "6")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по клиенту.
    #[prost(message, optional, tag = "7")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "8")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций шорт.
    #[prost(bool, tag = "12")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "14")]
    pub exchange: ::prost::alloc::string::String,
    ///Дата начала обращения контракта в часовом поясе UTC.
    #[prost(message, optional, tag = "15")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата в часовом поясе UTC, до которой возможно проведение операций с фьючерсом.
    #[prost(message, optional, tag = "16")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Тип фьючерса. Возможные значения: </br>**physical_delivery** — физические поставки; </br>**cash_settlement** — денежный эквивалент.
    #[prost(string, tag = "17")]
    pub futures_type: ::prost::alloc::string::String,
    ///Тип актива. Возможные значения: </br>**commodity** — товар; </br>**currency** — валюта; </br>**security** — ценная бумага; </br>**index** — индекс.
    #[prost(string, tag = "18")]
    pub asset_type: ::prost::alloc::string::String,
    ///Основной актив.
    #[prost(string, tag = "19")]
    pub basic_asset: ::prost::alloc::string::String,
    ///Размер основного актива.
    #[prost(message, optional, tag = "20")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    ///Дата истечения срока в часов поясе UTC.
    #[prost(message, optional, tag = "24")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    ///Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
}
///Объект передачи информации об акции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    ///Дата IPO акции в часовом поясе UTC.
    #[prost(message, optional, tag = "17")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Размер выпуска.
    #[prost(int64, tag = "18")]
    pub issue_size: i64,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "20")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Сектор экономики.
    #[prost(string, tag = "21")]
    pub sector: ::prost::alloc::string::String,
    ///Плановый размер выпуска.
    #[prost(int64, tag = "22")]
    pub issue_size_plan: i64,
    ///Номинал.
    #[prost(message, optional, tag = "23")]
    pub nominal: ::core::option::Option<MoneyValue>,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    ///Признак наличия дивидендной доходности.
    #[prost(bool, tag = "29")]
    pub div_yield_flag: bool,
    ///Тип акции. Возможные значения: \[ShareType\](<https://tinkoff.github.io/investAPI/instruments#sharetype>)
    #[prost(enumeration = "ShareType", tag = "30")]
    pub share_type: i32,
    ///Шаг цены.
    #[prost(message, optional, tag = "31")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "32")]
    pub api_trade_available_flag: bool,
}
///Запрос НКД по облигации
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsRequest {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
///НКД облигации
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsResponse {
    ///Массив операций начисления купонов.
    #[prost(message, repeated, tag = "1")]
    pub accrued_interests: ::prost::alloc::vec::Vec<AccruedInterest>,
}
///Операция начисления купонов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccruedInterest {
    ///Дата и время выплаты в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    ///Величина выплаты.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Quotation>,
    ///Величина выплаты в процентах от номинала.
    #[prost(message, optional, tag = "3")]
    pub value_percent: ::core::option::Option<Quotation>,
    ///Номинал облигации.
    #[prost(message, optional, tag = "4")]
    pub nominal: ::core::option::Option<Quotation>,
}
///Запрос информации о фьючерсе
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginRequest {
    /// Идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
///Данные по фьючерсу
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginResponse {
    ///Гарантийное обеспечение при покупке.
    #[prost(message, optional, tag = "1")]
    pub initial_margin_on_buy: ::core::option::Option<MoneyValue>,
    ///Гарантийное обеспечение при продаже.
    #[prost(message, optional, tag = "2")]
    pub initial_margin_on_sell: ::core::option::Option<MoneyValue>,
    ///Шаг цены.
    #[prost(message, optional, tag = "3")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Стоимость шага цены.
    #[prost(message, optional, tag = "4")]
    pub min_price_increment_amount: ::core::option::Option<Quotation>,
}
///Данные по инструменту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentResponse {
    /// Основная информация об инструменте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Instrument>,
}
///Объект передачи основной информации об инструменте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instrument {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    ///Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    ///Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    ///Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    ///Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    ///Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    ///Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    ///Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    ///Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    ///Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    ///Название инструмента.
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    ///Торговая площадка.
    #[prost(string, tag = "15")]
    pub exchange: ::prost::alloc::string::String,
    ///Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "16")]
    pub country_of_risk: ::prost::alloc::string::String,
    ///Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "17")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    ///Тип инструмента.
    #[prost(string, tag = "18")]
    pub instrument_type: ::prost::alloc::string::String,
    ///Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "19")]
    pub trading_status: i32,
    ///Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "20")]
    pub otc_flag: bool,
    ///Признак доступности для покупки.
    #[prost(bool, tag = "21")]
    pub buy_available_flag: bool,
    ///Признак доступности для продажи.
    #[prost(bool, tag = "22")]
    pub sell_available_flag: bool,
    ///Шаг цены.
    #[prost(message, optional, tag = "23")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    ///Признак доступности торгов через API.
    #[prost(bool, tag = "24")]
    pub api_trade_available_flag: bool,
}
///Запрос дивидендов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsRequest {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
///Дивиденды.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends: ::prost::alloc::vec::Vec<Dividend>,
}
///Информация о выплате.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dividend {
    ///Величина дивиденда на 1 ценную бумагу (включая валюту).
    #[prost(message, optional, tag = "1")]
    pub dividend_net: ::core::option::Option<MoneyValue>,
    ///Дата фактических выплат в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата объявления дивидендов в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub declared_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Последний день (включительно) покупки для получения выплаты в часовом поясе UTC.
    #[prost(message, optional, tag = "4")]
    pub last_buy_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Тип выплаты. Возможные значения: Regular Cash – регулярные выплаты, Cancelled – выплата отменена, Daily Accrual – ежедневное начисление, Return of Capital – возврат капитала, прочие типы выплат.
    #[prost(string, tag = "5")]
    pub dividend_type: ::prost::alloc::string::String,
    ///Дата фиксации реестра в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Регулярность выплаты. Возможные значения: Annual – ежегодная, Semi-Anl – каждые полгода, прочие типы выплат.
    #[prost(string, tag = "7")]
    pub regularity: ::prost::alloc::string::String,
    ///Цена закрытия инструмента на момент ex_dividend_date.
    #[prost(message, optional, tag = "8")]
    pub close_price: ::core::option::Option<MoneyValue>,
    ///Величина доходности.
    #[prost(message, optional, tag = "9")]
    pub yield_value: ::core::option::Option<Quotation>,
    ///Дата и время создания записи в часовом поясе UTC.
    #[prost(message, optional, tag = "10")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
///Тип идентификатора инструмента. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentIdType {
    ///Значение не определено.
    InstrumentIdUnspecified = 0,
    ///Figi.
    Figi = 1,
    ///Ticker.
    Ticker = 2,
}
///Статус запрашиваемых инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentStatus {
    ///Значение не определено.
    Unspecified = 0,
    ///Базовый список инструментов (по умолчанию). Инструменты доступные для торговли через TINKOFF INVEST API.
    Base = 1,
    ///Список всех инструментов.
    All = 2,
}
///Тип акций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShareType {
    ///Значение не определено.
    Unspecified = 0,
    ///Обыкновенная
    Common = 1,
    ///Привилегированная
    Preferred = 2,
    ///Американские депозитарные расписки
    Adr = 3,
    ///Глобальные депозитарные расписки
    Gdr = 4,
    ///Товарищество с ограниченной ответственностью
    Mlp = 5,
    ///Акции из реестра Нью-Йорка
    NyRegShrs = 6,
    ///Закрытый инвестиционный фонд
    ClosedEndFund = 7,
    ///Траст недвижимости
    Reit = 8,
}
#[doc = r" Generated client implementations."]
pub mod instruments_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct InstrumentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InstrumentsServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InstrumentsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InstrumentsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            InstrumentsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод получения расписания торгов торговых площадок."]
        pub async fn trading_schedules(
            &mut self,
            request: impl tonic::IntoRequest<super::TradingSchedulesRequest>,
        ) -> Result<tonic::Response<super::TradingSchedulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/TradingSchedules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения облигации по её идентификатору."]
        pub async fn bond_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::BondResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/BondBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка облигаций."]
        pub async fn bonds(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::BondsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Bonds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения валюты по её идентификатору."]
        pub async fn currency_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::CurrencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/CurrencyBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка валют."]
        pub async fn currencies(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::CurrenciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Currencies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения инвестиционного фонда по его идентификатору."]
        pub async fn etf_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::EtfResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EtfBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка инвестиционных фондов."]
        pub async fn etfs(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::EtfsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Etfs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения фьючерса по его идентификатору."]
        pub async fn future_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::FutureResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FutureBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка фьючерсов."]
        pub async fn futures(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::FuturesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Futures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения акции по её идентификатору."]
        pub async fn share_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::ShareResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/ShareBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка акций."]
        pub async fn shares(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::SharesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Shares",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения накопленного купонного дохода по облигации."]
        pub async fn get_accrued_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccruedInterestsRequest>,
        ) -> Result<tonic::Response<super::GetAccruedInterestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAccruedInterests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения размера гарантийного обеспечения по фьючерсам."]
        pub async fn get_futures_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFuturesMarginRequest>,
        ) -> Result<tonic::Response<super::GetFuturesMarginResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFuturesMargin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения основной информации об инструменте."]
        pub async fn get_instrument_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::InstrumentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetInstrumentBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод для получения событий выплаты дивидендов по инструменту."]
        pub async fn get_dividends(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsRequest>,
        ) -> Result<tonic::Response<super::GetDividendsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetDividends",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///Запрос подписки или отписки на определённые биржевые данные.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataRequest {
    #[prost(oneof = "market_data_request::Payload", tags = "1, 2, 3, 4")]
    pub payload: ::core::option::Option<market_data_request::Payload>,
}
/// Nested message and enum types in `MarketDataRequest`.
pub mod market_data_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///Запрос подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesRequest(super::SubscribeCandlesRequest),
        ///Запрос подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookRequest(super::SubscribeOrderBookRequest),
        ///Запрос подписки на ленту обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesRequest(super::SubscribeTradesRequest),
        ///Запрос подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoRequest(super::SubscribeInfoRequest),
    }
}
///Пакет биржевой информации по подписке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataResponse {
    #[prost(
        oneof = "market_data_response::Payload",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub payload: ::core::option::Option<market_data_response::Payload>,
}
/// Nested message and enum types in `MarketDataResponse`.
pub mod market_data_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///Результат подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesResponse(super::SubscribeCandlesResponse),
        ///Результат подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookResponse(super::SubscribeOrderBookResponse),
        ///Результат подписки на поток обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesResponse(super::SubscribeTradesResponse),
        ///Результат подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoResponse(super::SubscribeInfoResponse),
        ///Свеча.
        #[prost(message, tag = "5")]
        Candle(super::Candle),
        ///Сделки.
        #[prost(message, tag = "6")]
        Trade(super::Trade),
        ///Стакан.
        #[prost(message, tag = "7")]
        Orderbook(super::OrderBook),
        ///Торговый статус.
        #[prost(message, tag = "8")]
        TradingStatus(super::TradingStatus),
        ///Проверка активности стрима.
        #[prost(message, tag = "9")]
        Ping(super::Ping),
    }
}
/// subscribeCandles | Изменения статуса подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesRequest {
    ///Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    ///Массив инструментов для подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<CandleInstrument>,
}
///Запрос изменения статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleInstrument {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
}
///Результат изменения статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesResponse {
    ///Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>)
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    ///Массив статусов подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub candles_subscriptions: ::prost::alloc::vec::Vec<CandleSubscription>,
}
///Статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleSubscription {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    ///Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
}
///Запрос на изменение статуса подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookRequest {
    ///Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    ///Массив инструментов для подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<OrderBookInstrument>,
}
///Запрос подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookInstrument {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
}
///Результат изменения статуса подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookResponse {
    ///Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>)
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    ///Массив статусов подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub order_book_subscriptions: ::prost::alloc::vec::Vec<OrderBookSubscription>,
}
///Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookSubscription {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    ///Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
}
///Изменение статуса подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesRequest {
    ///Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    ///Массив инструментов для подписки на поток обезличенных сделок.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<TradeInstrument>,
}
///Запрос подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeInstrument {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
///Результат изменения статуса подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesResponse {
    ///Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>)
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    ///Массив статусов подписки на поток сделок.
    #[prost(message, repeated, tag = "2")]
    pub trade_subscriptions: ::prost::alloc::vec::Vec<TradeSubscription>,
}
///Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeSubscription {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
}
///Изменение статуса подписки на торговый статус инструмента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoRequest {
    ///Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    ///Массив инструментов для подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<InfoInstrument>,
}
///Запрос подписки на торговый статус.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoInstrument {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
///Результат изменения статуса подписки на торговый статус.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoResponse {
    ///Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>)
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    ///Массив статусов подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub info_subscriptions: ::prost::alloc::vec::Vec<InfoSubscription>,
}
///Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoSubscription {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
}
///Пакет свечей в рамках стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Интервал свечи.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    ///Цена открытия за 1 лот.
    #[prost(message, optional, tag = "3")]
    pub open: ::core::option::Option<Quotation>,
    ///Максимальная цена за 1 лот.
    #[prost(message, optional, tag = "4")]
    pub high: ::core::option::Option<Quotation>,
    ///Минимальная цена за 1 лот.
    #[prost(message, optional, tag = "5")]
    pub low: ::core::option::Option<Quotation>,
    ///Цена закрытия за 1 лот.
    #[prost(message, optional, tag = "6")]
    pub close: ::core::option::Option<Quotation>,
    ///Объём сделок в лотах.
    #[prost(int64, tag = "7")]
    pub volume: i64,
    ///Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Пакет стаканов в рамках стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    ///Флаг консистентности стакана. **false** значит не все заявки попали в стакан по причинам сетевых задержек или нарушения порядка доставки.
    #[prost(bool, tag = "3")]
    pub is_consistent: bool,
    ///Массив предложений.
    #[prost(message, repeated, tag = "4")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    ///Массив спроса.
    #[prost(message, repeated, tag = "5")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    ///Время стакана в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Массив предложений/спроса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    ///Цена за 1 лот.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<Quotation>,
    ///Количество в лотах.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
}
///Информация о сделке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Направление сделки.
    #[prost(enumeration = "TradeDirection", tag = "2")]
    pub direction: i32,
    ///Цена за 1 лот.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    ///Количество лотов.
    #[prost(int64, tag = "4")]
    pub quantity: i64,
    ///Время сделки в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Пакет изменения торгового статуса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStatus {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    ///Время изменения торгового статуса в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Запрос исторических свечей.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesRequest {
    ///Figi-идентификатор инструмента
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    ///Интервал запрошенных свечей.
    #[prost(enumeration = "CandleInterval", tag = "4")]
    pub interval: i32,
}
///Список свечей.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesResponse {
    ///Массив свечей.
    #[prost(message, repeated, tag = "1")]
    pub candles: ::prost::alloc::vec::Vec<HistoricCandle>,
}
///Информация о свече.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricCandle {
    ///Цена открытия за 1 лот.
    #[prost(message, optional, tag = "1")]
    pub open: ::core::option::Option<Quotation>,
    ///Максимальная цена за 1 лот.
    #[prost(message, optional, tag = "2")]
    pub high: ::core::option::Option<Quotation>,
    ///Минимальная цена за 1 лот.
    #[prost(message, optional, tag = "3")]
    pub low: ::core::option::Option<Quotation>,
    ///Цена закрытия за 1 лот.
    #[prost(message, optional, tag = "4")]
    pub close: ::core::option::Option<Quotation>,
    ///Объём торгов в лотах.
    #[prost(int64, tag = "5")]
    pub volume: i64,
    ///Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    ///Признак завершённости свечи. **false** значит, свеча за текущие интервал ещё сформирована не полностью.
    #[prost(bool, tag = "7")]
    pub is_complete: bool,
}
///Запрос получения последних цен.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesRequest {
    ///Массив figi-идентификаторов инструментов.
    #[prost(string, repeated, tag = "1")]
    pub figi: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///Список последних цен.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesResponse {
    ///Массив последних цен.
    #[prost(message, repeated, tag = "1")]
    pub last_prices: ::prost::alloc::vec::Vec<LastPrice>,
}
///Информация о цене.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPrice {
    ///Идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Последняя цена за 1 лот.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    ///Время получения последней цены в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Запрос стакана.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookRequest {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
}
///Информация о стакане.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookResponse {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    ///Множество пар значений на покупку.
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    ///Множество пар значений на продажу.
    #[prost(message, repeated, tag = "4")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    ///Цена последней сделки.
    #[prost(message, optional, tag = "5")]
    pub last_price: ::core::option::Option<Quotation>,
    ///Цена закрытия.
    #[prost(message, optional, tag = "6")]
    pub close_price: ::core::option::Option<Quotation>,
}
///Запрос получения торгового статуса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusRequest {
    ///Идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
///Информация о торговом статусе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusResponse {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
}
///Тип операции со списком подписок.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionAction {
    ///Статус подписки не определён.
    Unspecified = 0,
    ///Подписаться.
    Subscribe = 1,
    ///Отписаться.
    Unsubscribe = 2,
}
///Интервал свечи.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionInterval {
    ///Интервал свечи не определён.
    Unspecified = 0,
    ///Минутные свечи.
    OneMinute = 1,
    ///Пятиминутные свечи.
    FiveMinutes = 2,
}
///Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionStatus {
    ///Статус подписки не определён.
    Unspecified = 0,
    ///Успешно.
    Success = 1,
    ///Инструмент не найден.
    InstrumentNotFound = 2,
    ///Некорректный статус подписки, список возможных значений: \[SubscriptionAction\](<https://tinkoff.github.io/investAPI/marketdata#subscriptionaction>)
    SubscriptionActionIsInvalid = 3,
    ///Некорректная глубина стакана, доступные значения: 1, 10, 20, 30, 40, 50.
    DepthIsInvalid = 4,
    ///Некорректный интервал свечей, список возможных значений: \[SubscriptionInterval\](<https://tinkoff.github.io/investAPI/marketdata#subscriptioninterval>)
    IntervalIsInvalid = 5,
    ///Превышен лимит подписок в рамках стрима, подробнее: [Лимитная политика](<https://tinkoff.github.io/investAPI/limits/>)
    LimitIsExceeded = 6,
    ///Внутренняя ошибка сервиса.
    InternalError = 7,
}
///Направление сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDirection {
    ///Направление сделки не определено.
    Unspecified = 0,
    ///Покупка.
    Buy = 1,
    ///Продажа.
    Sell = 2,
}
///Интервал свечей.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandleInterval {
    ///Интервал не определён.
    Unspecified = 0,
    ///1 минута.
    CandleInterval1Min = 1,
    ///5 минут.
    CandleInterval5Min = 2,
    ///15 минут.
    CandleInterval15Min = 3,
    ///1 час.
    Hour = 4,
    ///1 день.
    Day = 5,
}
#[doc = r" Generated client implementations."]
pub mod market_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MarketDataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод запроса исторических свечей по инструменту."]
        pub async fn get_candles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCandlesRequest>,
        ) -> Result<tonic::Response<super::GetCandlesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetCandles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод запроса последних цен по инструментам."]
        pub async fn get_last_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPricesRequest>,
        ) -> Result<tonic::Response<super::GetLastPricesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastPrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения стакана по инструменту."]
        pub async fn get_order_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderBookRequest>,
        ) -> Result<tonic::Response<super::GetOrderBookResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetOrderBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод запроса статуса торгов по инструментам."]
        pub async fn get_trading_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusRequest>,
        ) -> Result<tonic::Response<super::GetTradingStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod market_data_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataStreamServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MarketDataStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Bi-directional стрим предоставления биржевой информации."]
        pub async fn market_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MarketDataRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
///Запрос получения списка операций по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsRequest {
    ///Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Начало периода (по UTC)
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание периода (по UTC)
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    ///Статус запрашиваемых операций
    #[prost(enumeration = "OperationState", tag = "4")]
    pub state: i32,
    ///Figi-идентификатор инструмента для фильтрации
    #[prost(string, tag = "5")]
    pub figi: ::prost::alloc::string::String,
}
///Список операций.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsResponse {
    ///Массив операций
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
///Данные по операции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    ///Идентификатор операции
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///Идентификатор родительской операции
    #[prost(string, tag = "2")]
    pub parent_operation_id: ::prost::alloc::string::String,
    ///Валюта операции
    #[prost(string, tag = "3")]
    pub currency: ::prost::alloc::string::String,
    ///Сумма операции
    #[prost(message, optional, tag = "4")]
    pub payment: ::core::option::Option<MoneyValue>,
    ///Цена операции
    #[prost(message, optional, tag = "5")]
    pub price: ::core::option::Option<MoneyValue>,
    ///Статус операции
    #[prost(enumeration = "OperationState", tag = "6")]
    pub state: i32,
    ///Количество лотов инструмента
    #[prost(int64, tag = "7")]
    pub quantity: i64,
    ///Неисполненный остаток по сделке
    #[prost(int64, tag = "8")]
    pub quantity_rest: i64,
    ///Figi-идентификатор инструмента, связанного с операцией
    #[prost(string, tag = "9")]
    pub figi: ::prost::alloc::string::String,
    ///Тип инструмента. Возможные значения: </br>**bond** — облигация; </br>**share** — акция; </br>**currency** — валюта; </br>**etf** — фонд; </br>**futures** — фьючерс.
    #[prost(string, tag = "10")]
    pub instrument_type: ::prost::alloc::string::String,
    ///Дата и время операции в формате часовом поясе UTC
    #[prost(message, optional, tag = "11")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    ///Текстовое описание типа операции
    #[prost(string, tag = "12")]
    pub r#type: ::prost::alloc::string::String,
    ///Тип операции
    #[prost(enumeration = "OperationType", tag = "13")]
    pub operation_type: i32,
}
///Запрос получения текущего портфеля по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    ///Идентификатор счёта пользователя
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Текущий портфель по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    ///Общая стоимость акций в портфеле в рублях
    #[prost(message, optional, tag = "1")]
    pub total_amount_shares: ::core::option::Option<MoneyValue>,
    ///Общая стоимость облигаций в портфеле в рублях
    #[prost(message, optional, tag = "2")]
    pub total_amount_bonds: ::core::option::Option<MoneyValue>,
    ///Общая стоимость фондов в портфеле в рублях
    #[prost(message, optional, tag = "3")]
    pub total_amount_etf: ::core::option::Option<MoneyValue>,
    ///Общая стоимость валют в портфеле в рублях
    #[prost(message, optional, tag = "4")]
    pub total_amount_currencies: ::core::option::Option<MoneyValue>,
    ///Общая стоимость фьючерсов в портфеле в рублях
    #[prost(message, optional, tag = "5")]
    pub total_amount_futures: ::core::option::Option<MoneyValue>,
    ///Текущая доходность портфеля
    #[prost(message, optional, tag = "6")]
    pub expected_yield: ::core::option::Option<Quotation>,
    ///Список позиций портфеля
    #[prost(message, repeated, tag = "7")]
    pub positions: ::prost::alloc::vec::Vec<PortfolioPosition>,
}
///Запрос позиций портфеля по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    ///Идентификатор счёта пользователя
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Список позиций по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    ///Массив валютных позиций портфеля
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    ///Массив заблокированных валютных позиций портфеля
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    ///Список ценно-бумажных позиций портфеля
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    ///Признак идущей в данный момент выгрузки лимитов
    #[prost(bool, tag = "4")]
    pub limits_loading_in_progress: bool,
    ///Список фьючерсов портфеля
    #[prost(message, repeated, tag = "5")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
}
///Запрос доступного для вывода остатка.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsRequest {
    ///Идентификатор счёта пользователя
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Доступный для вывода остаток.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsResponse {
    ///Массив валютных позиций портфеля
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    ///Массив заблокированных валютных позиций портфеля
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    ///Заблокировано под гарантийное обеспечение фьючерсов
    #[prost(message, repeated, tag = "3")]
    pub blocked_guarantee: ::prost::alloc::vec::Vec<MoneyValue>,
}
///Позиции портфеля.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioPosition {
    ///Figi-идентификатора инструмента
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Тип инструмента
    #[prost(string, tag = "2")]
    pub instrument_type: ::prost::alloc::string::String,
    ///Количество инструмента в портфеле в штуках
    #[prost(message, optional, tag = "3")]
    pub quantity: ::core::option::Option<Quotation>,
    ///Средневзвешенная цена позиции
    #[prost(message, optional, tag = "4")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    ///Текущая рассчитанная доходность
    #[prost(message, optional, tag = "5")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущий НКД
    #[prost(message, optional, tag = "6")]
    pub current_nkd: ::core::option::Option<MoneyValue>,
    ///Средняя цена лота в позиции в пунктах (для фьючерсов)
    #[prost(message, optional, tag = "7")]
    pub average_position_price_pt: ::core::option::Option<Quotation>,
    ///Текущая цена инструмента
    #[prost(message, optional, tag = "8")]
    pub current_price: ::core::option::Option<MoneyValue>,
    ///Средняя цена лота в позиции по методу FIFO
    #[prost(message, optional, tag = "9")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
    ///Количество лотов в портфеле
    #[prost(message, optional, tag = "10")]
    pub quantity_lots: ::core::option::Option<Quotation>,
}
///Баланс позиции ценной бумаги.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSecurities {
    ///Figi-идентификатор бумаги
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Заблокировано
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    ///Текущий баланс
    #[prost(int64, tag = "3")]
    pub balance: i64,
}
///Баланс фьючерса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFutures {
    ///Figi-идентификатор фьючерса
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Заблокировано
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    ///Текущий баланс
    #[prost(int64, tag = "3")]
    pub balance: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportRequest {
    #[prost(oneof = "broker_report_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_request::Payload>,
}
/// Nested message and enum types in `BrokerReportRequest`.
pub mod broker_report_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportRequest(super::GenerateBrokerReportRequest),
        #[prost(message, tag = "2")]
        GetBrokerReportRequest(super::GetBrokerReportRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportResponse {
    #[prost(oneof = "broker_report_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_response::Payload>,
}
/// Nested message and enum types in `BrokerReportResponse`.
pub mod broker_report_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportResponse(super::GenerateBrokerReportResponse),
        #[prost(message, tag = "2")]
        GetBrokerReportResponse(super::GetBrokerReportResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportRequest {
    ///Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Начало периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    ///Окончание периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportResponse {
    ///Идентификатор задачи формирования брокерского отчёта
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportRequest {
    ///Идентификатор задачи формирования брокерского отчёта
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    ///Номер страницы отчета (начинается с 1), значение по умолчанию: 0
    #[prost(int32, tag = "2")]
    pub page: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub broker_report: ::prost::alloc::vec::Vec<BrokerReport>,
    ///Количество записей в отчете
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    ///Количество страниц с данными отчета (начинается с 0)
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    ///Текущая страница (начинается с 0)
    #[prost(int32, tag = "4")]
    pub page: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReport {
    ///Номер сделки
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    ///Номер поручения
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
    ///Figi-идентификатор инструмента
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    ///Признак исполнения
    #[prost(string, tag = "4")]
    pub execute_sign: ::prost::alloc::string::String,
    ///Дата и время заключения в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub trade_datetime: ::core::option::Option<::prost_types::Timestamp>,
    ///Торговая площадка
    #[prost(string, tag = "6")]
    pub exchange: ::prost::alloc::string::String,
    ///Режим торгов
    #[prost(string, tag = "7")]
    pub class_code: ::prost::alloc::string::String,
    ///Вид сделки
    #[prost(string, tag = "8")]
    pub direction: ::prost::alloc::string::String,
    ///Сокращённое наименование актива
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    ///Код актива
    #[prost(string, tag = "10")]
    pub ticker: ::prost::alloc::string::String,
    ///Цена за единицу
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<MoneyValue>,
    ///Количество
    #[prost(int64, tag = "12")]
    pub quantity: i64,
    ///Сумма (без НКД)
    #[prost(message, optional, tag = "13")]
    pub order_amount: ::core::option::Option<MoneyValue>,
    ///НКД
    #[prost(message, optional, tag = "14")]
    pub aci_value: ::core::option::Option<Quotation>,
    ///Сумма сделки
    #[prost(message, optional, tag = "15")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    ///Комиссия брокера
    #[prost(message, optional, tag = "16")]
    pub broker_commission: ::core::option::Option<MoneyValue>,
    ///Комиссия биржи
    #[prost(message, optional, tag = "17")]
    pub exchange_commission: ::core::option::Option<MoneyValue>,
    ///Комиссия клир. центра
    #[prost(message, optional, tag = "18")]
    pub exchange_clearing_commission: ::core::option::Option<MoneyValue>,
    ///Ставка РЕПО (%)
    #[prost(message, optional, tag = "19")]
    pub repo_rate: ::core::option::Option<Quotation>,
    ///Контрагент/Брокер
    #[prost(string, tag = "20")]
    pub party: ::prost::alloc::string::String,
    ///Дата расчётов в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub clear_value_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата поставки в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub sec_value_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Статус брокера
    #[prost(string, tag = "23")]
    pub broker_status: ::prost::alloc::string::String,
    ///Тип дог.
    #[prost(string, tag = "24")]
    pub separate_agreement_type: ::prost::alloc::string::String,
    ///Номер дог.
    #[prost(string, tag = "25")]
    pub separate_agreement_number: ::prost::alloc::string::String,
    ///Дата дог.
    #[prost(string, tag = "26")]
    pub separate_agreement_date: ::prost::alloc::string::String,
    ///Тип расчёта по сделке
    #[prost(string, tag = "27")]
    pub delivery_type: ::prost::alloc::string::String,
}
///Статус запрашиваемых операций
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    ///Статус операции не определён
    Unspecified = 0,
    ///Исполнена
    Executed = 1,
    ///Отменена
    Canceled = 2,
}
///Тип операции
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    ///Тип операции не определён
    Unspecified = 0,
    ///Завод денежных средств
    Input = 1,
    ///Удержание налога по купонам
    BondTax = 2,
    ///Вывод ЦБ
    OutputSecurities = 3,
    ///Доход по сделке РЕПО овернайт
    Overnight = 4,
    ///Удержание налога
    Tax = 5,
    ///Полное погашение облигаций
    BondRepaymentFull = 6,
    ///Продажа ЦБ с карты
    SellCard = 7,
    ///Удержание налога по дивидендам
    DividendTax = 8,
    ///Вывод денежных средств
    Output = 9,
    ///Частичное погашение облигаций
    BondRepayment = 10,
    ///Корректировка налога
    TaxCorrection = 11,
    ///Удержание комиссии за обслуживание брокерского счёта
    ServiceFee = 12,
    ///Удержание налога за материальную выгоду
    BenefitTax = 13,
    ///Удержание комиссии за непокрытую позицию
    MarginFee = 14,
    ///Покупка ЦБ
    Buy = 15,
    ///Покупка ЦБ с карты
    BuyCard = 16,
    ///Завод ЦБ
    InputSecurities = 17,
    ///Продажа в результате Margin-call
    SellMargin = 18,
    ///Удержание комиссии за операцию
    BrokerFee = 19,
    ///Покупка в результате Margin-call
    BuyMargin = 20,
    ///Выплата дивидендов
    Dividend = 21,
    ///Продажа ЦБ
    Sell = 22,
    ///Выплата купонов
    Coupon = 23,
    ///Удержание комиссии SuccessFee
    SuccessFee = 24,
    ///Передача дивидендного дохода
    DividendTransfer = 25,
    ///Зачисление вариационной маржи
    AccruingVarmargin = 26,
    ///Списание вариационной маржи
    WritingOffVarmargin = 27,
    ///Покупка в рамках экспирации фьючерсного контракта
    DeliveryBuy = 28,
    ///Продажа в рамках экспирации фьючерсного контракта
    DeliverySell = 29,
    ///Комиссия за управление по счёту автоследования
    TrackMfee = 30,
    ///Комиссия за результат по счёту автоследования
    TrackPfee = 31,
    ///Удержание налога по ставке 15%
    TaxProgressive = 32,
    ///Удержание налога по купонам по ставке 15%
    BondTaxProgressive = 33,
    ///Удержание налога по дивидендам по ставке 15%
    DividendTaxProgressive = 34,
    ///Удержание налога за материальную выгоду по ставке 15%
    BenefitTaxProgressive = 35,
    ///Корректировка налога по ставке 15%
    TaxCorrectionProgressive = 36,
    ///Удержание налога за возмещение по сделкам РЕПО по ставке 15%
    TaxRepoProgressive = 37,
    ///Удержание налога за возмещение по сделкам РЕПО
    TaxRepo = 38,
    ///Удержание налога по сделкам РЕПО
    TaxRepoHold = 39,
    ///Возврат налога по сделкам РЕПО
    TaxRepoRefund = 40,
    ///Удержание налога по сделкам РЕПО по ставке 15%
    TaxRepoHoldProgressive = 41,
    ///Возврат налога по сделкам РЕПО по ставке 15%
    TaxRepoRefundProgressive = 42,
    ///Выплата дивидендов на карту
    DivExt = 43,
}
#[doc = r" Generated client implementations."]
pub mod operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OperationsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод получения списка операций по счёту."]
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения портфеля по счёту."]
        pub async fn get_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPortfolio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка позиций по счёту."]
        pub async fn get_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения доступного остатка для вывода средств."]
        pub async fn get_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<tonic::Response<super::WithdrawLimitsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetWithdrawLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_broker_report(
            &mut self,
            request: impl tonic::IntoRequest<super::BrokerReportRequest>,
        ) -> Result<tonic::Response<super::BrokerReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetBrokerReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///Запрос установки соединения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamRequest {}
///Информация о торговых поручениях.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamResponse {
    #[prost(oneof = "trades_stream_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<trades_stream_response::Payload>,
}
/// Nested message and enum types in `TradesStreamResponse`.
pub mod trades_stream_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///Информация об исполнении торгового поручения.
        #[prost(message, tag = "1")]
        OrderTrades(super::OrderTrades),
        ///Проверка активности стрима.
        #[prost(message, tag = "2")]
        Ping(super::Ping),
    }
}
///Информация об исполнении торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrades {
    ///Идентификатор торгового поручения
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    ///Дата и время создания сообщения в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    ///Направление сделки (возможные значения)
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    ///Figi-идентификатор инструмента
    #[prost(string, tag = "4")]
    pub figi: ::prost::alloc::string::String,
    ///Массив сделок
    #[prost(message, repeated, tag = "5")]
    pub trades: ::prost::alloc::vec::Vec<OrderTrade>,
}
///Информация о сделке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrade {
    ///Дата и время совершения сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    ///Цена, по которой совершена сделка
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    ///Количество лотов в сделке
    #[prost(int64, tag = "3")]
    pub quantity: i64,
}
///Запрос выставления торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderRequest {
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    ///Цена лота.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    ///Направление операции.
    #[prost(enumeration = "OrderDirection", tag = "4")]
    pub direction: i32,
    ///Номер счёта.
    #[prost(string, tag = "5")]
    pub account_id: ::prost::alloc::string::String,
    ///Тип заявки.
    #[prost(enumeration = "OrderType", tag = "6")]
    pub order_type: i32,
    ///Идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов.
    #[prost(string, tag = "7")]
    pub order_id: ::prost::alloc::string::String,
}
///Информация о выставлении поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderResponse {
    ///Идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    ///Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    ///Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    ///Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    ///Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    ///Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    ///Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    ///Начальная комиссия. Комиссия рассчитанная при выставлении заявки.
    #[prost(message, optional, tag = "8")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    ///Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "9")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    ///Значение НКД (накопленного купонного дохода) на дату. Подробнее: [НКД при выставлении торговых поручений](<https://tinkoff.github.io/investAPI/head-orders#coupon>)
    #[prost(message, optional, tag = "10")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    ///Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    ///Начальная цена инструмента заявки.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    ///Тип заявки.
    #[prost(enumeration = "OrderType", tag = "14")]
    pub order_type: i32,
    ///Дополнительные данные об исполнении заявки.
    #[prost(string, tag = "15")]
    pub message: ::prost::alloc::string::String,
    ///Начальная цена заявки в пунктах (для фьючерсов).
    #[prost(message, optional, tag = "16")]
    pub initial_order_price_pt: ::core::option::Option<Quotation>,
}
///Запрос отмены торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    ///Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
///Результат отмены торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    ///Дата и время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Запрос получения статуса торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderStateRequest {
    ///Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
///Запрос получения списка активных торговых поручений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersRequest {
    ///Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Список активных торговых поручений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersResponse {
    ///Массив активных заявок.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<OrderState>,
}
///Информация о торговом поручении.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderState {
    ///Идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    ///Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    ///Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    ///Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    ///Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    ///Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    ///Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    ///Средняя цена позиции по сделке.
    #[prost(message, optional, tag = "8")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    ///Начальная комиссия. Комиссия, рассчитанная на момент подачи заявки.
    #[prost(message, optional, tag = "9")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    ///Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "10")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    ///Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    ///Направление заявки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    ///Начальная цена инструмента. Цена инструмента на момент выставления заявки.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    ///Стадии выполнения заявки.
    #[prost(message, repeated, tag = "14")]
    pub stages: ::prost::alloc::vec::Vec<OrderStage>,
    ///Сервисная комиссия.
    #[prost(message, optional, tag = "15")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    ///Валюта заявки.
    #[prost(string, tag = "16")]
    pub currency: ::prost::alloc::string::String,
    ///Тип заявки.
    #[prost(enumeration = "OrderType", tag = "17")]
    pub order_type: i32,
    ///Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub order_date: ::core::option::Option<::prost_types::Timestamp>,
}
///Сделки в рамках торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStage {
    ///Цена.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<MoneyValue>,
    ///Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    ///Идентификатор торговой операции.
    #[prost(string, tag = "3")]
    pub trade_id: ::prost::alloc::string::String,
}
///Направление операции
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    ///Значение не указано
    Unspecified = 0,
    ///Покупка
    Buy = 1,
    ///Продажа
    Sell = 2,
}
///Тип заявки
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    ///Значение не указано
    Unspecified = 0,
    ///Лимитная
    Limit = 1,
    ///Рыночная
    Market = 2,
}
///Текущий статус заявки (поручения)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderExecutionReportStatus {
    ExecutionReportStatusUnspecified = 0,
    ///Исполнена
    ExecutionReportStatusFill = 1,
    ///Отклонена
    ExecutionReportStatusRejected = 2,
    ///Отменена пользователем
    ExecutionReportStatusCancelled = 3,
    ///Новая
    ExecutionReportStatusNew = 4,
    ///Частично исполнена
    ExecutionReportStatusPartiallyfill = 5,
}
#[doc = r" Generated client implementations."]
pub mod orders_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersStreamServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OrdersStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Stream сделок пользователя"]
        pub async fn trades_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TradesStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/TradesStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод выставления заявки."]
        pub async fn post_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/PostOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод отмены биржевой заявки."]
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/CancelOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения статуса торгового поручения."]
        pub async fn get_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrderState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка активных заявок по счёту."]
        pub async fn get_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///Запрос получения счетов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsRequest {}
///Список счетов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsResponse {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
///Информация о счёте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Тип счёта.
    #[prost(enumeration = "AccountType", tag = "2")]
    pub r#type: i32,
    /// Название счёта.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Статус счёта.
    #[prost(enumeration = "AccountStatus", tag = "4")]
    pub status: i32,
    /// Дата открытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub opened_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата закрытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub closed_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень доступа к текущему счёту (определяется токеном).
    #[prost(enumeration = "AccessLevel", tag = "7")]
    pub access_level: i32,
}
///Запрос маржинальных показателей по счёту
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Маржинальные показатели по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesResponse {
    /// Ликвидная стоимость портфеля. Подробнее: [что такое ликвидный портфель?](<https://help.tinkoff.ru/margin-trade/short/liquid-portfolio/>).
    #[prost(message, optional, tag = "1")]
    pub liquid_portfolio: ::core::option::Option<MoneyValue>,
    /// Начальная маржа — начальное обеспечение для совершения новой сделки. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "2")]
    pub starting_margin: ::core::option::Option<MoneyValue>,
    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "3")]
    pub minimal_margin: ::core::option::Option<MoneyValue>,
    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[prost(message, optional, tag = "4")]
    pub funds_sufficiency_level: ::core::option::Option<Quotation>,
    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[prost(message, optional, tag = "5")]
    pub amount_of_missing_funds: ::core::option::Option<MoneyValue>,
}
///Запрос текущих лимитов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffRequest {}
///Текущие лимиты пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffResponse {
    ///Массив лимитов пользователя по unary-запросам
    #[prost(message, repeated, tag = "1")]
    pub unary_limits: ::prost::alloc::vec::Vec<UnaryLimit>,
    ///Массив лимитов пользователей для stream-соединений
    #[prost(message, repeated, tag = "2")]
    pub stream_limits: ::prost::alloc::vec::Vec<StreamLimit>,
}
///Лимит unary-методов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnaryLimit {
    ///Количество unary-запросов в минуту
    #[prost(int32, tag = "1")]
    pub limit_per_minute: i32,
    ///Названия методов
    #[prost(string, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///Лимит stream-соединений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    ///Максимальное количество stream-соединений
    #[prost(int32, tag = "1")]
    pub limit: i32,
    ///Названия stream-методов
    #[prost(string, repeated, tag = "2")]
    pub streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///Запрос информации о пользователе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {}
///Информация о пользователе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    ///Признак премиум клиента.
    #[prost(bool, tag = "1")]
    pub prem_status: bool,
    ///Признак квалифицированного инвестора.
    #[prost(bool, tag = "2")]
    pub qual_status: bool,
    ///Набор требующих тестирования инструментов и возможностей, с которыми может работать пользователь.
    #[prost(string, repeated, tag = "3")]
    pub qualified_for_work_with: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///Тип счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    ///Тип аккаунта не определён.
    Unspecified = 0,
    ///Брокерский счёт Тинькофф.
    Tinkoff = 1,
    ///ИИС счёт.
    TinkoffIis = 2,
    ///Инвесткопилка.
    InvestBox = 3,
}
///Статус счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    ///Статус счёта не определён.
    Unspecified = 0,
    ///Новый, в процессе открытия.
    New = 1,
    ///Открытый и активный счёт.
    Open = 2,
    ///Закрытый счёт.
    Closed = 3,
}
///Уровень доступа к счёту.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLevel {
    ///Уровень доступа не определён.
    AccountAccessLevelUnspecified = 0,
    ///Полный доступ к счёту.
    AccountAccessLevelFullAccess = 1,
    ///Доступ с уровнем прав "только чтение".
    AccountAccessLevelReadOnly = 2,
    ///Доступ отсутствует.
    AccountAccessLevelNoAccess = 3,
}
#[doc = r" Generated client implementations."]
pub mod users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UsersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UsersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UsersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод получения счетов пользователя."]
        pub async fn get_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Расчёт маржинальных показателей по счёту."]
        pub async fn get_margin_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMarginAttributesRequest>,
        ) -> Result<tonic::Response<super::GetMarginAttributesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetMarginAttributes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Запрос тарифа пользователя."]
        pub async fn get_user_tariff(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserTariffRequest>,
        ) -> Result<tonic::Response<super::GetUserTariffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetUserTariff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения информации о пользователе."]
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///Запрос открытия счёта в песочнице.
///
///пустой запрос
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountRequest {}
///Номер открытого счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountResponse {
    ///Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Запрос закрытия счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountRequest {
    ///Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Результат закрытия счёта в песочнице.
///
///пустой ответ
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountResponse {}
///Запрос пополнения счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInRequest {
    ///Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Сумма пополнения счёта в рублях
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<MoneyValue>,
}
///Результат пополнения счёта, текущий баланс.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInResponse {
    ///Текущий баланс счёта
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<MoneyValue>,
}
#[doc = r" Generated client implementations."]
pub mod sandbox_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SandboxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SandboxServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SandboxServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SandboxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SandboxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод регистрации счёта в песочнице."]
        pub async fn open_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::OpenSandboxAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/OpenSandboxAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения счетов в песочнице."]
        pub async fn get_sandbox_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод закрытия счёта в песочнице."]
        pub async fn close_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::CloseSandboxAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CloseSandboxAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод выставления торгового поручения в песочнице."]
        pub async fn post_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/PostSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка активных заявок по счёту в песочнице."]
        pub async fn get_sandbox_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод отмены торгового поручения в песочнице."]
        pub async fn cancel_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CancelSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения статуса заявки в песочнице."]
        pub async fn get_sandbox_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrderState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения позиций по виртуальному счёту песочницы."]
        pub async fn get_sandbox_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения операций в песочнице по номеру счёта."]
        pub async fn get_sandbox_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения портфолио в песочнице."]
        pub async fn get_sandbox_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPortfolio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод пополнения счёта в песочнице."]
        pub async fn sandbox_pay_in(
            &mut self,
            request: impl tonic::IntoRequest<super::SandboxPayInRequest>,
        ) -> Result<tonic::Response<super::SandboxPayInResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/SandboxPayIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
///Запрос выставления стоп-заявки
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderRequest {
    ///Figi-идентификатор инструмента
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    ///Количество лотов
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    ///Цена лота
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    ///Стоп-цена заявки
    #[prost(message, optional, tag = "4")]
    pub stop_price: ::core::option::Option<Quotation>,
    ///Направление операции
    #[prost(enumeration = "StopOrderDirection", tag = "5")]
    pub direction: i32,
    ///Номер счёта
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    ///Тип экспирации заявки
    #[prost(enumeration = "StopOrderExpirationType", tag = "7")]
    pub expiration_type: i32,
    ///Тип заявки
    #[prost(enumeration = "StopOrderType", tag = "8")]
    pub stop_order_type: i32,
    ///Дата и время окончания действия стоп-заявки в часовом поясе UTC. **Для ExpirationType = GoodTillDate заполнение обязательно**.
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
}
///Результат выставления стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderResponse {
    ///Уникальный идентификатор стоп-заявки.
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
}
///Запрос получения списка активных стоп-заявок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersRequest {
    ///Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
///Список активных стоп-заявок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersResponse {
    ///Массив стоп-заявок по счёту
    #[prost(message, repeated, tag = "1")]
    pub stop_orders: ::prost::alloc::vec::Vec<StopOrder>,
}
///Запрос отмены выставленной стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderRequest {
    ///Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    ///Уникальный идентификатор стоп-заявки.
    #[prost(string, tag = "2")]
    pub stop_order_id: ::prost::alloc::string::String,
}
///Результат отмены выставленной стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderResponse {
    ///Время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
///Информация о стоп-заявке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopOrder {
    ///Идентификатор-идентификатор стоп-заявки
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
    ///Запрошено лотов
    #[prost(int64, tag = "2")]
    pub lots_requested: i64,
    ///Figi-идентификатор инструмента
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    ///Направление операции
    #[prost(enumeration = "StopOrderDirection", tag = "4")]
    pub direction: i32,
    ///Валюта стоп-заявки
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    ///Тип стоп-заявки
    #[prost(enumeration = "StopOrderType", tag = "6")]
    pub order_type: i32,
    ///Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "7")]
    pub create_date: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата и время конвертации стоп-заявки в биржевую в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub activation_date_time: ::core::option::Option<::prost_types::Timestamp>,
    ///Дата и время снятия заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    ///Цена заявки
    #[prost(message, optional, tag = "10")]
    pub price: ::core::option::Option<MoneyValue>,
    ///Цена активации стоп-заявки
    #[prost(message, optional, tag = "11")]
    pub stop_price: ::core::option::Option<MoneyValue>,
}
///Направление сделки стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderDirection {
    ///Значение не указано
    Unspecified = 0,
    ///Покупка
    Buy = 1,
    ///Продажа
    Sell = 2,
}
///Тип экспирации стоп-заявке.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderExpirationType {
    ///Значение не указано.
    Unspecified = 0,
    ///Действительно до отмены.
    GoodTillCancel = 1,
    ///Действительно до даты снятия.
    GoodTillDate = 2,
}
///Тип стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderType {
    ///Значение не указано
    Unspecified = 0,
    ///Take-profit заявка
    TakeProfit = 1,
    ///Stop-loss заявка
    StopLoss = 2,
    ///Stop-limit заявка
    StopLimit = 3,
}
#[doc = r" Generated client implementations."]
pub mod stop_orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StopOrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StopOrdersServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StopOrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StopOrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StopOrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "Метод выставления стоп-заявки."]
        pub async fn post_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostStopOrderRequest>,
        ) -> Result<tonic::Response<super::PostStopOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/PostStopOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод получения списка активных стоп заявок по счёту."]
        pub async fn get_stop_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStopOrdersRequest>,
        ) -> Result<tonic::Response<super::GetStopOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/GetStopOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Метод отмены стоп-заявки."]
        pub async fn cancel_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelStopOrderRequest>,
        ) -> Result<tonic::Response<super::CancelStopOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/CancelStopOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
