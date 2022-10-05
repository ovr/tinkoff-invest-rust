/// Денежная сумма в определенной валюте
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyValue {
    /// строковый ISO-код валюты
    #[prost(string, tag="1")]
    pub currency: ::prost::alloc::string::String,
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag="2")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag="3")]
    pub nano: i32,
}
/// Котировка - денежная сумма без указания валюты
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quotation {
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag="1")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag="2")]
    pub nano: i32,
}
/// Проверка активности стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Время проверки.
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Режим торгов инструмента
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён
    Unspecified = 0,
    /// Недоступен для торгов
    NotAvailableForTrading = 1,
    /// Период открытия торгов
    OpeningPeriod = 2,
    /// Период закрытия торгов
    ClosingPeriod = 3,
    /// Перерыв в торговле
    BreakInTrading = 4,
    /// Нормальная торговля
    NormalTrading = 5,
    /// Аукцион закрытия
    ClosingAuction = 6,
    /// Аукцион крупных пакетов
    DarkPoolAuction = 7,
    /// Дискретный аукцион
    DiscreteAuction = 8,
    /// Аукцион открытия
    OpeningAuctionPeriod = 9,
    /// Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice = 10,
    /// Сессия назначена
    SessionAssigned = 11,
    /// Сессия закрыта
    SessionClose = 12,
    /// Сессия открыта
    SessionOpen = 13,
    /// Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading = 14,
    /// Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading = 15,
    /// Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading = 16,
}
impl SecurityTradingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityTradingStatus::Unspecified => "SECURITY_TRADING_STATUS_UNSPECIFIED",
            SecurityTradingStatus::NotAvailableForTrading => "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING",
            SecurityTradingStatus::OpeningPeriod => "SECURITY_TRADING_STATUS_OPENING_PERIOD",
            SecurityTradingStatus::ClosingPeriod => "SECURITY_TRADING_STATUS_CLOSING_PERIOD",
            SecurityTradingStatus::BreakInTrading => "SECURITY_TRADING_STATUS_BREAK_IN_TRADING",
            SecurityTradingStatus::NormalTrading => "SECURITY_TRADING_STATUS_NORMAL_TRADING",
            SecurityTradingStatus::ClosingAuction => "SECURITY_TRADING_STATUS_CLOSING_AUCTION",
            SecurityTradingStatus::DarkPoolAuction => "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION",
            SecurityTradingStatus::DiscreteAuction => "SECURITY_TRADING_STATUS_DISCRETE_AUCTION",
            SecurityTradingStatus::OpeningAuctionPeriod => "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD",
            SecurityTradingStatus::TradingAtClosingAuctionPrice => "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE",
            SecurityTradingStatus::SessionAssigned => "SECURITY_TRADING_STATUS_SESSION_ASSIGNED",
            SecurityTradingStatus::SessionClose => "SECURITY_TRADING_STATUS_SESSION_CLOSE",
            SecurityTradingStatus::SessionOpen => "SECURITY_TRADING_STATUS_SESSION_OPEN",
            SecurityTradingStatus::DealerNormalTrading => "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING",
            SecurityTradingStatus::DealerBreakInTrading => "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING",
            SecurityTradingStatus::DealerNotAvailableForTrading => "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING",
        }
    }
}
/// Запрос расписания торгов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesRequest {
    /// Наименование биржи или расчетного календаря. </br>Если не передаётся, возвращается информация по всем доступным торговым площадкам.
    #[prost(string, tag="1")]
    pub exchange: ::prost::alloc::string::String,
    /// Начало периода по часовому поясу UTC.
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода по часовому поясу UTC.
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Список торговых площадок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesResponse {
    /// Список торговых площадок и режимов торгов.
    #[prost(message, repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<TradingSchedule>,
}
/// Данные по торговой площадке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedule {
    /// Наименование торговой площадки.
    #[prost(string, tag="1")]
    pub exchange: ::prost::alloc::string::String,
    /// Массив с торговыми и неторговыми днями.
    #[prost(message, repeated, tag="2")]
    pub days: ::prost::alloc::vec::Vec<TradingDay>,
}
/// Информация о времени торгов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingDay {
    /// Дата.
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак торгового дня на бирже.
    #[prost(bool, tag="2")]
    pub is_trading_day: bool,
    /// Время начала торгов по часовому поясу UTC.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания торгов по часовому поясу UTC.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия в часовом поясе UTC.
    #[prost(message, optional, tag="7")]
    pub opening_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания аукциона закрытия в часовом поясе UTC.
    #[prost(message, optional, tag="8")]
    pub closing_auction_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag="9")]
    pub evening_opening_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag="10")]
    pub evening_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag="11")]
    pub evening_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала основного клиринга в часовом поясе UTC.
    #[prost(message, optional, tag="12")]
    pub clearing_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания основного клиринга в часовом поясе UTC.
    #[prost(message, optional, tag="13")]
    pub clearing_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала премаркета в часовом поясе UTC.
    #[prost(message, optional, tag="14")]
    pub premarket_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания премаркета в часовом поясе UTC.
    #[prost(message, optional, tag="15")]
    pub premarket_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос получения инструмента по идентификатору.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentRequest {
    /// Тип идентификатора инструмента. Возможные значения: figi, ticker. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
    #[prost(enumeration="InstrumentIdType", tag="1")]
    pub id_type: i32,
    /// Идентификатор class_code. Обязателен при id_type = ticker.
    #[prost(string, tag="2")]
    pub class_code: ::prost::alloc::string::String,
    /// Идентификатор запрашиваемого инструмента.
    #[prost(string, tag="3")]
    pub id: ::prost::alloc::string::String,
}
/// Запрос получения инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentsRequest {
    /// Статус запрашиваемых инструментов. Возможные значения: \[InstrumentStatus\](#instrumentstatus)
    #[prost(enumeration="InstrumentStatus", tag="1")]
    pub instrument_status: i32,
}
/// Информация об облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondResponse {
    /// Информация об облигации.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Bond>,
}
/// Список облигаций.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsResponse {
    /// Массив облигаций.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Bond>,
}
/// Запрос купонов по облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC. Фильтрация по coupon_date (дата выплаты купона)
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC. Фильтрация по coupon_date (дата выплаты купона)
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Купоны по облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsResponse {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Coupon>,
}
/// Объект передачи информации о купоне облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coupon {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Дата выплаты купона.
    #[prost(message, optional, tag="2")]
    pub coupon_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номер купона.
    #[prost(int64, tag="3")]
    pub coupon_number: i64,
    /// (Опционально) Дата фиксации реестра для выплаты купона.
    #[prost(message, optional, tag="4")]
    pub fix_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Выплата на одну облигацию.
    #[prost(message, optional, tag="5")]
    pub pay_one_bond: ::core::option::Option<MoneyValue>,
    /// Тип купона.
    #[prost(enumeration="CouponType", tag="6")]
    pub coupon_type: i32,
    /// Начало купонного периода.
    #[prost(message, optional, tag="7")]
    pub coupon_start_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание купонного периода.
    #[prost(message, optional, tag="8")]
    pub coupon_end_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Купонный период в днях.
    #[prost(int32, tag="9")]
    pub coupon_period: i32,
}
/// Данные по валюте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyResponse {
    /// Информация о валюте.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Currency>,
}
/// Данные по валютам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrenciesResponse {
    /// Массив валют.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Currency>,
}
/// Данные по фонду.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfResponse {
    /// Информация о фонде.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Etf>,
}
/// Данные по фондам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfsResponse {
    /// Массив фондов.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Etf>,
}
/// Данные по фьючерсу.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureResponse {
    /// Информация о фьючерсу.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Future>,
}
/// Данные по фьючерсам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuturesResponse {
    /// Массив фьючерсов.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Future>,
}
/// Данные по опциону.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionResponse {
    /// Информация по опциону.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Option>,
}
/// Данные по опционам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionsResponse {
    /// Массив данных по опциону.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Option>,
}
/// Опцион.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag="2")]
    pub position_uid: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код.
    #[prost(string, tag="4")]
    pub class_code: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag="5")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="21")]
    pub trading_status: i32,
    /// Реальная площадка исполнения расчётов. Допустимые значения: [REAL_EXCHANGE_MOEX, REAL_EXCHANGE_RTS]
    #[prost(enumeration="RealExchange", tag="31")]
    pub real_exchange: i32,
    /// Направление опциона.
    #[prost(enumeration="OptionDirection", tag="41")]
    pub direction: i32,
    /// Тип расчетов по опциону.
    #[prost(enumeration="OptionPaymentType", tag="42")]
    pub payment_type: i32,
    /// Стиль опциона.
    #[prost(enumeration="OptionStyle", tag="43")]
    pub style: i32,
    /// Способ исполнения опциона.
    #[prost(enumeration="OptionSettlementType", tag="44")]
    pub settlement_type: i32,
    /// Название инструмента.
    #[prost(string, tag="101")]
    pub name: ::prost::alloc::string::String,
    /// Валюта.
    #[prost(string, tag="111")]
    pub currency: ::prost::alloc::string::String,
    /// Валюта, в которой оценивается контракт.
    #[prost(string, tag="112")]
    pub settlement_currency: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(string, tag="131")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag="132")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Биржа.
    #[prost(string, tag="141")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны рисков.
    #[prost(string, tag="151")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны рисков.
    #[prost(string, tag="152")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag="161")]
    pub sector: ::prost::alloc::string::String,
    /// Количество бумаг в лоте.
    #[prost(int32, tag="201")]
    pub lot: i32,
    /// Размер основного актива.
    #[prost(message, optional, tag="211")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска длинной позиции по клиенту.
    #[prost(message, optional, tag="221")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту.
    #[prost(message, optional, tag="222")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи лонг.
    #[prost(message, optional, tag="223")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи шорт.
    #[prost(message, optional, tag="224")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи лонг.
    #[prost(message, optional, tag="225")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи шорт.
    #[prost(message, optional, tag="226")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Минимальный шаг цены.
    #[prost(message, optional, tag="231")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Цена страйка.
    #[prost(message, optional, tag="241")]
    pub strike_price: ::core::option::Option<MoneyValue>,
    /// Дата истечения срока в формате UTC.
    #[prost(message, optional, tag="301")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата начала обращения контракта в формате UTC.
    #[prost(message, optional, tag="311")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата исполнения в формате UTC.
    #[prost(message, optional, tag="312")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой минутной свечи в формате UTC.
    #[prost(message, optional, tag="321")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи в формате UTC.
    #[prost(message, optional, tag="322")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag="401")]
    pub short_enabled_flag: bool,
    /// Возможность покупки/продажи на ИИС.
    #[prost(bool, tag="402")]
    pub for_iis_flag: bool,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="403")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="404")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="405")]
    pub sell_available_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="406")]
    pub for_qual_investor_flag: bool,
}
/// Данные по акции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareResponse {
    /// Информация об акции.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Share>,
}
/// Данные по акциям.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharesResponse {
    /// Массив акций.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<Share>,
}
/// Объект передачи информации об облигации.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bond {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag="7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag="8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag="13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="16")]
    pub exchange: ::prost::alloc::string::String,
    /// Количество выплат по купонам в год.
    #[prost(int32, tag="17")]
    pub coupon_quantity_per_year: i32,
    /// Дата погашения облигации в часовом поясе UTC.
    #[prost(message, optional, tag="18")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номинал облигации.
    #[prost(message, optional, tag="19")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Первоначальный номинал облигации.
    #[prost(message, optional, tag="20")]
    pub initial_nominal: ::core::option::Option<MoneyValue>,
    /// Дата выпуска облигации в часовом поясе UTC.
    #[prost(message, optional, tag="21")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения в часовом поясе UTC.
    #[prost(message, optional, tag="22")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения.
    #[prost(message, optional, tag="23")]
    pub placement_price: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату.
    #[prost(message, optional, tag="24")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="25")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="26")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag="27")]
    pub sector: ::prost::alloc::string::String,
    /// Форма выпуска. Возможные значения: </br>**documentary** — документарная; </br>**non_documentary** — бездокументарная.
    #[prost(string, tag="28")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Размер выпуска.
    #[prost(int64, tag="29")]
    pub issue_size: i64,
    /// Плановый размер выпуска.
    #[prost(int64, tag="30")]
    pub issue_size_plan: i64,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="31")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="32")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="33")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="34")]
    pub sell_available_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag="35")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag="36")]
    pub perpetual_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag="37")]
    pub amortization_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag="38")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="39")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="40")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="41")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="42")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="51")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="52")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="61")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="62")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации о валюте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag="7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag="8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag="13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="16")]
    pub exchange: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag="17")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="18")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="19")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="20")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="21")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="22")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="23")]
    pub sell_available_flag: bool,
    /// Строковый ISO-код валюты.
    #[prost(string, tag="24")]
    pub iso_currency_name: ::prost::alloc::string::String,
    /// Шаг цены.
    #[prost(message, optional, tag="25")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="26")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="27")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="28")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="29")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="52")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации об инвестиционном фонде.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etf {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag="7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag="8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag="13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="16")]
    pub exchange: ::prost::alloc::string::String,
    /// Размер фиксированной комиссии фонда.
    #[prost(message, optional, tag="17")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Возможные значения: </br>**equity** — акции;</br>**fixed_income** — облигации;</br>**mixed_allocation** — смешанный;</br>**money_market** — денежный рынок;</br>**real_estate** — недвижимость;</br>**commodity** — товары;</br>**specialty** — специальный;</br>**private_equity** — private equity;</br>**alternative_investment** — альтернативные инвестиции.
    #[prost(string, tag="18")]
    pub focus_type: ::prost::alloc::string::String,
    /// Дата выпуска в часовом поясе UTC.
    #[prost(message, optional, tag="19")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество акций фонда в обращении.
    #[prost(message, optional, tag="20")]
    pub num_shares: ::core::option::Option<Quotation>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag="23")]
    pub sector: ::prost::alloc::string::String,
    /// Частота ребалансировки.
    #[prost(string, tag="24")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag="29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="42")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации о фьючерсе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Future {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="4")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту.
    #[prost(message, optional, tag="6")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту.
    #[prost(message, optional, tag="7")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="8")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag="12")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="13")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="14")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата начала обращения контракта в часовом поясе UTC.
    #[prost(message, optional, tag="15")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата в часовом поясе UTC, до которой возможно проведение операций с фьючерсом.
    #[prost(message, optional, tag="16")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип фьючерса. Возможные значения: </br>**physical_delivery** — физические поставки; </br>**cash_settlement** — денежный эквивалент.
    #[prost(string, tag="17")]
    pub futures_type: ::prost::alloc::string::String,
    /// Тип актива. Возможные значения: </br>**commodity** — товар; </br>**currency** — валюта; </br>**security** — ценная бумага; </br>**index** — индекс.
    #[prost(string, tag="18")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag="19")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Размер основного актива.
    #[prost(message, optional, tag="20")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag="23")]
    pub sector: ::prost::alloc::string::String,
    /// Дата истечения срока в часов поясе UTC.
    #[prost(message, optional, tag="24")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag="29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag="34")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="42")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации об акции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag="7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag="8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag="13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="16")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата IPO акции в часовом поясе UTC.
    #[prost(message, optional, tag="17")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Размер выпуска.
    #[prost(int64, tag="18")]
    pub issue_size: i64,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="19")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="20")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag="21")]
    pub sector: ::prost::alloc::string::String,
    /// Плановый размер выпуска.
    #[prost(int64, tag="22")]
    pub issue_size_plan: i64,
    /// Номинал.
    #[prost(message, optional, tag="23")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="28")]
    pub sell_available_flag: bool,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag="29")]
    pub div_yield_flag: bool,
    /// Тип акции. Возможные значения: \[ShareType\](<https://tinkoff.github.io/investAPI/instruments#sharetype>)
    #[prost(enumeration="ShareType", tag="30")]
    pub share_type: i32,
    /// Шаг цены.
    #[prost(message, optional, tag="31")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="32")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="33")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="34")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="46")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="47")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос НКД по облигации
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// НКД облигации
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsResponse {
    /// Массив операций начисления купонов.
    #[prost(message, repeated, tag="1")]
    pub accrued_interests: ::prost::alloc::vec::Vec<AccruedInterest>,
}
/// Операция начисления купонов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccruedInterest {
    /// Дата и время выплаты в часовом поясе UTC.
    #[prost(message, optional, tag="1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Величина выплаты.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Quotation>,
    /// Величина выплаты в процентах от номинала.
    #[prost(message, optional, tag="3")]
    pub value_percent: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag="4")]
    pub nominal: ::core::option::Option<Quotation>,
}
/// Запрос информации о фьючерсе
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginRequest {
    /// Идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
}
/// Данные по фьючерсу
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginResponse {
    /// Гарантийное обеспечение при покупке.
    #[prost(message, optional, tag="1")]
    pub initial_margin_on_buy: ::core::option::Option<MoneyValue>,
    /// Гарантийное обеспечение при продаже.
    #[prost(message, optional, tag="2")]
    pub initial_margin_on_sell: ::core::option::Option<MoneyValue>,
    /// Шаг цены.
    #[prost(message, optional, tag="3")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Стоимость шага цены.
    #[prost(message, optional, tag="4")]
    pub min_price_increment_amount: ::core::option::Option<Quotation>,
}
/// Данные по инструменту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentResponse {
    /// Основная информация об инструменте.
    #[prost(message, optional, tag="1")]
    pub instrument: ::core::option::Option<Instrument>,
}
/// Объект передачи основной информации об инструменте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag="5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag="6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag="7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag="8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag="11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag="12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag="13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag="14")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag="15")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="16")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag="17")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="18")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration="SecurityTradingStatus", tag="19")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="20")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag="21")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag="22")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag="23")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="24")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="25")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration="RealExchange", tag="26")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="27")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="36")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="37")]
    pub for_qual_investor_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос дивидендов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC. Фильтрация происходит по параметру *record_date* (дата фиксации реестра).
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC. Фильтрация происходит по параметру *record_date* (дата фиксации реестра).
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Дивиденды.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsResponse {
    #[prost(message, repeated, tag="1")]
    pub dividends: ::prost::alloc::vec::Vec<Dividend>,
}
/// Информация о выплате.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dividend {
    /// Величина дивиденда на 1 ценную бумагу (включая валюту).
    #[prost(message, optional, tag="1")]
    pub dividend_net: ::core::option::Option<MoneyValue>,
    /// Дата фактических выплат в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата объявления дивидендов в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub declared_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Последний день (включительно) покупки для получения выплаты в часовом поясе UTC.
    #[prost(message, optional, tag="4")]
    pub last_buy_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип выплаты. Возможные значения: Regular Cash – регулярные выплаты, Cancelled – выплата отменена, Daily Accrual – ежедневное начисление, Return of Capital – возврат капитала, прочие типы выплат.
    #[prost(string, tag="5")]
    pub dividend_type: ::prost::alloc::string::String,
    /// Дата фиксации реестра в часовом поясе UTC.
    #[prost(message, optional, tag="6")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Регулярность выплаты. Возможные значения: Annual – ежегодная, Semi-Anl – каждые полгода, прочие типы выплат.
    #[prost(string, tag="7")]
    pub regularity: ::prost::alloc::string::String,
    /// Цена закрытия инструмента на момент ex_dividend_date.
    #[prost(message, optional, tag="8")]
    pub close_price: ::core::option::Option<MoneyValue>,
    /// Величина доходности.
    #[prost(message, optional, tag="9")]
    pub yield_value: ::core::option::Option<Quotation>,
    /// Дата и время создания записи в часовом поясе UTC.
    #[prost(message, optional, tag="10")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос актива по идентификатору.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRequest {
    /// uid-идентификатор актива.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// Данные по активу.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetResponse {
    /// Актив.
    #[prost(message, optional, tag="1")]
    pub asset: ::core::option::Option<AssetFull>,
}
/// Запрос списка активов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRequest {
}
/// Список активов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsResponse {
    /// Активы.
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFull {
    /// Уникальный идентификатор актива.
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration="AssetType", tag="2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Короткое наименование актива.
    #[prost(string, tag="4")]
    pub name_brief: ::prost::alloc::string::String,
    /// Описание актива.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Дата и время удаления актива.
    #[prost(message, optional, tag="6")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Тестирование клиентов.
    #[prost(string, repeated, tag="7")]
    pub required_tests: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Номер государственной регистрации.
    #[prost(string, tag="10")]
    pub gos_reg_code: ::prost::alloc::string::String,
    /// Код CFI.
    #[prost(string, tag="11")]
    pub cfi: ::prost::alloc::string::String,
    /// Код НРД инструмента.
    #[prost(string, tag="12")]
    pub code_nsd: ::prost::alloc::string::String,
    /// Статус актива.
    #[prost(string, tag="13")]
    pub status: ::prost::alloc::string::String,
    /// Бренд.
    #[prost(message, optional, tag="14")]
    pub brand: ::core::option::Option<Brand>,
    /// Дата и время последнего обновления записи.
    #[prost(message, optional, tag="15")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Код типа ц.б. по классификации Банка России.
    #[prost(string, tag="16")]
    pub br_code: ::prost::alloc::string::String,
    /// Наименование кода типа ц.б. по классификации Банка России.
    #[prost(string, tag="17")]
    pub br_code_name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag="18")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
    #[prost(oneof="asset_full::Ext", tags="8, 9")]
    pub ext: ::core::option::Option<asset_full::Ext>,
}
/// Nested message and enum types in `AssetFull`.
pub mod asset_full {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Валюта. Обязательно и заполняется только для type = "ASSET_TYPE_CURRENCY".
        #[prost(message, tag="8")]
        Currency(super::AssetCurrency),
        /// Ценная бумага. Обязательно и заполняется только для type = "ASSET_TYPE_SECURITY".
        #[prost(message, tag="9")]
        Security(super::AssetSecurity),
    }
}
/// Информация об активе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Уникальный идентификатор актива.
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration="AssetType", tag="2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag="4")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
}
/// Валюта.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetCurrency {
    /// ISO-код валюты.
    #[prost(string, tag="1")]
    pub base_currency: ::prost::alloc::string::String,
}
/// Ценная бумага.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSecurity {
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag="1")]
    pub isin: ::prost::alloc::string::String,
    /// Тип ценной бумаги.
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(oneof="asset_security::Ext", tags="3, 4, 5, 6, 7")]
    pub ext: ::core::option::Option<asset_security::Ext>,
}
/// Nested message and enum types in `AssetSecurity`.
pub mod asset_security {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Акция. Заполняется только для акций (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = share).
        #[prost(message, tag="3")]
        Share(super::AssetShare),
        /// Облигация. Заполняется только для облигаций (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = bond).
        #[prost(message, tag="4")]
        Bond(super::AssetBond),
        /// Структурная нота. Заполняется только для структурных продуктов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = sp).
        #[prost(message, tag="5")]
        Sp(super::AssetStructuredProduct),
        /// Фонд. Заполняется только для фондов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = etf).
        #[prost(message, tag="6")]
        Etf(super::AssetEtf),
        /// Клиринговый сертификат участия. Заполняется только для клиринговых сертификатов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = clearing_certificate).
        #[prost(message, tag="7")]
        ClearingCertificate(super::AssetClearingCertificate),
    }
}
/// Акция.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetShare {
    /// Тип акции.
    #[prost(enumeration="ShareType", tag="1")]
    pub r#type: i32,
    /// Объем выпуска (шт.).
    #[prost(message, optional, tag="2")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал.
    #[prost(message, optional, tag="3")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag="4")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Индекс (Bloomberg).
    #[prost(string, tag="5")]
    pub primary_index: ::prost::alloc::string::String,
    /// Ставка дивиденда (для привилегированных акций).
    #[prost(message, optional, tag="6")]
    pub dividend_rate: ::core::option::Option<Quotation>,
    /// Тип привилегированных акций.
    #[prost(string, tag="7")]
    pub preferred_share_type: ::prost::alloc::string::String,
    /// Дата IPO.
    #[prost(message, optional, tag="8")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата регистрации.
    #[prost(message, optional, tag="9")]
    pub registry_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag="10")]
    pub div_yield_flag: bool,
    /// Форма выпуска ФИ.
    #[prost(string, tag="11")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Дата размещения акции.
    #[prost(message, optional, tag="12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// ISIN базового актива.
    #[prost(string, tag="13")]
    pub repres_isin: ::prost::alloc::string::String,
    /// Объявленное количество шт.
    #[prost(message, optional, tag="14")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Количество акций в свободном обращении.
    #[prost(message, optional, tag="15")]
    pub total_float: ::core::option::Option<Quotation>,
}
/// Облигация.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetBond {
    /// Текущий номинал.
    #[prost(message, optional, tag="1")]
    pub current_nominal: ::core::option::Option<Quotation>,
    /// Наименование заемщика.
    #[prost(string, tag="2")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Объем эмиссии облигации (стоимость).
    #[prost(message, optional, tag="3")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag="4")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag="5")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Форма выпуска облигации.
    #[prost(string, tag="6")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Форма дохода облигации.
    #[prost(string, tag="7")]
    pub interest_kind: ::prost::alloc::string::String,
    /// Количество выплат в год.
    #[prost(int32, tag="8")]
    pub coupon_quantity_per_year: i32,
    /// Признак облигации с индексируемым номиналом.
    #[prost(bool, tag="9")]
    pub indexed_nominal_flag: bool,
    /// Признак субординированной облигации.
    #[prost(bool, tag="10")]
    pub subordinated_flag: bool,
    /// Признак обеспеченной облигации.
    #[prost(bool, tag="11")]
    pub collateral_flag: bool,
    /// Признак показывает, что купоны облигации не облагаются налогом (для mass market).
    #[prost(bool, tag="12")]
    pub tax_free_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag="13")]
    pub amortization_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag="14")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag="15")]
    pub perpetual_flag: bool,
    /// Дата погашения облигации.
    #[prost(message, optional, tag="16")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание и условия получения дополнительного дохода.
    #[prost(string, tag="17")]
    pub return_condition: ::prost::alloc::string::String,
    /// Дата выпуска облигации.
    #[prost(message, optional, tag="18")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения облигации.
    #[prost(message, optional, tag="19")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения облигации.
    #[prost(message, optional, tag="20")]
    pub placement_price: ::core::option::Option<Quotation>,
    /// Объявленное количество шт.
    #[prost(message, optional, tag="21")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
}
/// Структурная нота.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetStructuredProduct {
    /// Наименование заемщика.
    #[prost(string, tag="1")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag="2")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag="3")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Тип структурной ноты.
    #[prost(enumeration="StructuredProductType", tag="4")]
    pub r#type: i32,
    /// Стратегия портфеля.
    #[prost(string, tag="5")]
    pub logic_portfolio: ::prost::alloc::string::String,
    /// Тип базового актива.
    #[prost(enumeration="AssetType", tag="6")]
    pub asset_type: i32,
    /// Вид базового актива в зависимости от типа базового актива.
    #[prost(string, tag="7")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Барьер сохранности (в процентах).
    #[prost(message, optional, tag="8")]
    pub safety_barrier: ::core::option::Option<Quotation>,
    /// Дата погашения.
    #[prost(message, optional, tag="9")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Объявленное количество шт.
    #[prost(message, optional, tag="10")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Объем размещения.
    #[prost(message, optional, tag="11")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Дата размещения ноты.
    #[prost(message, optional, tag="12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag="13")]
    pub issue_kind: ::prost::alloc::string::String,
}
/// Фонд.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetEtf {
    /// Суммарные расходы фонда (в %).
    #[prost(message, optional, tag="1")]
    pub total_expense: ::core::option::Option<Quotation>,
    /// Барьерная ставка доходности после которой фонд имеет право на perfomance fee (в процентах).
    #[prost(message, optional, tag="2")]
    pub hurdle_rate: ::core::option::Option<Quotation>,
    /// Комиссия за успешные результаты фонда (в процентах).
    #[prost(message, optional, tag="3")]
    pub performance_fee: ::core::option::Option<Quotation>,
    /// Фиксированная комиссия за управление (в процентах).
    #[prost(message, optional, tag="4")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Тип распределения доходов от выплат по бумагам.
    #[prost(string, tag="5")]
    pub payment_type: ::prost::alloc::string::String,
    /// Признак необходимости выхода фонда в плюс для получения комиссии.
    #[prost(bool, tag="6")]
    pub watermark_flag: bool,
    /// Премия (надбавка к цене) при покупке доли в фонде (в процентах).
    #[prost(message, optional, tag="7")]
    pub buy_premium: ::core::option::Option<Quotation>,
    /// Ставка дисконта (вычет из цены) при продаже доли в фонде (в процентах).
    #[prost(message, optional, tag="8")]
    pub sell_discount: ::core::option::Option<Quotation>,
    /// Признак ребалансируемости портфеля фонда.
    #[prost(bool, tag="9")]
    pub rebalancing_flag: bool,
    /// Периодичность ребалансировки.
    #[prost(string, tag="10")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Тип управления.
    #[prost(string, tag="11")]
    pub management_type: ::prost::alloc::string::String,
    /// Индекс, который реплицирует (старается копировать) фонд.
    #[prost(string, tag="12")]
    pub primary_index: ::prost::alloc::string::String,
    /// База ETF.
    #[prost(string, tag="13")]
    pub focus_type: ::prost::alloc::string::String,
    /// Признак использования заемных активов (плечо).
    #[prost(bool, tag="14")]
    pub leveraged_flag: bool,
    /// Количество акций в обращении.
    #[prost(message, optional, tag="15")]
    pub num_share: ::core::option::Option<Quotation>,
    /// Признак обязательства по отчетности перед регулятором.
    #[prost(bool, tag="16")]
    pub ucits_flag: bool,
    /// Дата выпуска.
    #[prost(message, optional, tag="17")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание фонда.
    #[prost(string, tag="18")]
    pub description: ::prost::alloc::string::String,
    /// Описание индекса, за которым следует фонд.
    #[prost(string, tag="19")]
    pub primary_index_description: ::prost::alloc::string::String,
    /// Основные компании, в которые вкладывается фонд.
    #[prost(string, tag="20")]
    pub primary_index_company: ::prost::alloc::string::String,
    /// Срок восстановления индекса (после просадки).
    #[prost(message, optional, tag="21")]
    pub index_recovery_period: ::core::option::Option<Quotation>,
    /// IVAV-код.
    #[prost(string, tag="22")]
    pub inav_code: ::prost::alloc::string::String,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag="23")]
    pub div_yield_flag: bool,
    /// Комиссия на покрытие расходов фонда (в процентах).
    #[prost(message, optional, tag="24")]
    pub expense_commission: ::core::option::Option<Quotation>,
    /// Ошибка следования за индексом (в процентах).
    #[prost(message, optional, tag="25")]
    pub primary_index_tracking_error: ::core::option::Option<Quotation>,
    /// Плановая ребалансировка портфеля.
    #[prost(string, tag="26")]
    pub rebalancing_plan: ::prost::alloc::string::String,
    /// Ставки налогообложения дивидендов и купонов.
    #[prost(string, tag="27")]
    pub tax_rate: ::prost::alloc::string::String,
    /// Даты ребалансировок.
    #[prost(message, repeated, tag="28")]
    pub rebalancing_dates: ::prost::alloc::vec::Vec<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag="29")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag="30")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag="31")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Клиринговый сертификат участия.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClearingCertificate {
    /// Номинал.
    #[prost(message, optional, tag="1")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag="2")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Бренд.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    /// uid идентификатор бренда.
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    /// Наименование бренда.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Описание.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Информация о бренде.
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    /// Компания.
    #[prost(string, tag="5")]
    pub company: ::prost::alloc::string::String,
    /// Сектор.
    #[prost(string, tag="6")]
    pub sector: ::prost::alloc::string::String,
    /// Код страны риска.
    #[prost(string, tag="7")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска.
    #[prost(string, tag="8")]
    pub country_of_risk_name: ::prost::alloc::string::String,
}
/// Идентификаторы инструмента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInstrument {
    /// uid идентификатор инструмента.
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    /// figi идентификатор инструмента.
    #[prost(string, tag="2")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="3")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="4")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag="5")]
    pub class_code: ::prost::alloc::string::String,
    /// Массив связанных инструментов.
    #[prost(message, repeated, tag="6")]
    pub links: ::prost::alloc::vec::Vec<InstrumentLink>,
}
/// Связь с другим инструментом.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentLink {
    /// Тип связи.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// uid идентификатор связанного инструмента.
    #[prost(string, tag="2")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос списка избранных инструментов, входные параметры не требуются.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesRequest {
}
/// В ответ передаётся список избранных инструментов в качестве массива.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesResponse {
    /// Массив инструментов
    #[prost(message, repeated, tag="1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Массив избранных инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavoriteInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag="2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag="3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="11")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag="16")]
    pub otc_flag: bool,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="17")]
    pub api_trade_available_flag: bool,
}
/// Запрос редактирования списка избранных инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequest {
    /// Массив инструментов.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<EditFavoritesRequestInstrument>,
    /// Тип действия со списком.
    #[prost(enumeration="EditFavoritesActionType", tag="6")]
    pub action_type: i32,
}
/// Массив инструментов для редактирования списка избранных инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequestInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
}
/// Результат редактирования списка избранных инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesResponse {
    /// Массив инструментов
    #[prost(message, repeated, tag="1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Запрос справочника стран.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesRequest {
}
/// Справочник стран.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesResponse {
    /// Массив стран.
    #[prost(message, repeated, tag="1")]
    pub countries: ::prost::alloc::vec::Vec<CountryResponse>,
}
/// Данные о стране.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountryResponse {
    /// Двухбуквенный код страны.
    #[prost(string, tag="1")]
    pub alfa_two: ::prost::alloc::string::String,
    /// Трёхбуквенный код страны.
    #[prost(string, tag="2")]
    pub alfa_three: ::prost::alloc::string::String,
    /// Наименование страны.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Краткое наименование страны.
    #[prost(string, tag="4")]
    pub name_brief: ::prost::alloc::string::String,
}
/// Запрос на поиск инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentRequest {
    /// Строка поиска.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
}
/// Результат поиска инструментов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentResponse {
    /// Массив инструментов, удовлетворяющих условиям поиска.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentShort>,
}
/// Краткая информация об инструменте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentShort {
    /// Isin инструмента.
    #[prost(string, tag="1")]
    pub isin: ::prost::alloc::string::String,
    /// Figi инструмента.
    #[prost(string, tag="2")]
    pub figi: ::prost::alloc::string::String,
    /// Ticker инструмента.
    #[prost(string, tag="3")]
    pub ticker: ::prost::alloc::string::String,
    /// ClassCode инструмента.
    #[prost(string, tag="4")]
    pub class_code: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="5")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Название инструмента.
    #[prost(string, tag="6")]
    pub name: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="7")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag="8")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="11")]
    pub api_trade_available_flag: bool,
    /// Признак доступности для ИИС.
    #[prost(bool, tag="12")]
    pub for_iis_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag="26")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag="27")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag="28")]
    pub for_qual_investor_flag: bool,
}
/// Запрос списка брендов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsRequest {
}
/// Запрос бренда.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandRequest {
    /// Uid-идентификатор бренда.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// Список брендов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsResponse {
    /// Массив брендов.
    #[prost(message, repeated, tag="1")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
/// Тип купонов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CouponType {
    /// Неопределенное значение
    Unspecified = 0,
    /// Постоянный
    Constant = 1,
    /// Плавающий
    Floating = 2,
    /// Дисконт
    Discount = 3,
    /// Ипотечный
    Mortgage = 4,
    /// Фиксированный
    Fix = 5,
    /// Переменный
    Variable = 6,
    /// Прочее
    Other = 7,
}
impl CouponType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CouponType::Unspecified => "COUPON_TYPE_UNSPECIFIED",
            CouponType::Constant => "COUPON_TYPE_CONSTANT",
            CouponType::Floating => "COUPON_TYPE_FLOATING",
            CouponType::Discount => "COUPON_TYPE_DISCOUNT",
            CouponType::Mortgage => "COUPON_TYPE_MORTGAGE",
            CouponType::Fix => "COUPON_TYPE_FIX",
            CouponType::Variable => "COUPON_TYPE_VARIABLE",
            CouponType::Other => "COUPON_TYPE_OTHER",
        }
    }
}
/// Тип опциона по направлению сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionDirection {
    Unspecified = 0,
    Put = 1,
    Call = 2,
}
impl OptionDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionDirection::Unspecified => "OPTION_DIRECTION_UNSPECIFIED",
            OptionDirection::Put => "OPTION_DIRECTION_PUT",
            OptionDirection::Call => "OPTION_DIRECTION_CALL",
        }
    }
}
/// Тип расчетов по опциону.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionPaymentType {
    Unspecified = 0,
    Premium = 1,
    Marginal = 2,
}
impl OptionPaymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionPaymentType::Unspecified => "OPTION_PAYMENT_TYPE_UNSPECIFIED",
            OptionPaymentType::Premium => "OPTION_PAYMENT_TYPE_PREMIUM",
            OptionPaymentType::Marginal => "OPTION_PAYMENT_TYPE_MARGINAL",
        }
    }
}
/// Тип опциона по стилю.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionStyle {
    Unspecified = 0,
    American = 1,
    European = 2,
}
impl OptionStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionStyle::Unspecified => "OPTION_STYLE_UNSPECIFIED",
            OptionStyle::American => "OPTION_STYLE_AMERICAN",
            OptionStyle::European => "OPTION_STYLE_EUROPEAN",
        }
    }
}
/// Тип опциона по способу исполнения.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionSettlementType {
    OptionExecutionTypeUnspecified = 0,
    OptionExecutionTypePhysicalDelivery = 1,
    OptionExecutionTypeCashSettlement = 2,
}
impl OptionSettlementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionSettlementType::OptionExecutionTypeUnspecified => "OPTION_EXECUTION_TYPE_UNSPECIFIED",
            OptionSettlementType::OptionExecutionTypePhysicalDelivery => "OPTION_EXECUTION_TYPE_PHYSICAL_DELIVERY",
            OptionSettlementType::OptionExecutionTypeCashSettlement => "OPTION_EXECUTION_TYPE_CASH_SETTLEMENT",
        }
    }
}
/// Тип идентификатора инструмента. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentIdType {
    /// Значение не определено.
    InstrumentIdUnspecified = 0,
    /// Figi.
    Figi = 1,
    /// Ticker.
    Ticker = 2,
    /// Уникальный идентификатор.
    Uid = 3,
    /// Идентификатор позиции.
    PositionUid = 4,
}
impl InstrumentIdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentIdType::InstrumentIdUnspecified => "INSTRUMENT_ID_UNSPECIFIED",
            InstrumentIdType::Figi => "INSTRUMENT_ID_TYPE_FIGI",
            InstrumentIdType::Ticker => "INSTRUMENT_ID_TYPE_TICKER",
            InstrumentIdType::Uid => "INSTRUMENT_ID_TYPE_UID",
            InstrumentIdType::PositionUid => "INSTRUMENT_ID_TYPE_POSITION_UID",
        }
    }
}
/// Статус запрашиваемых инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentStatus {
    /// Значение не определено.
    Unspecified = 0,
    /// Базовый список инструментов (по умолчанию). Инструменты доступные для торговли через TINKOFF INVEST API.
    Base = 1,
    /// Список всех инструментов.
    All = 2,
}
impl InstrumentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentStatus::Unspecified => "INSTRUMENT_STATUS_UNSPECIFIED",
            InstrumentStatus::Base => "INSTRUMENT_STATUS_BASE",
            InstrumentStatus::All => "INSTRUMENT_STATUS_ALL",
        }
    }
}
/// Тип акций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShareType {
    /// Значение не определено.
    Unspecified = 0,
    /// Обыкновенная
    Common = 1,
    /// Привилегированная
    Preferred = 2,
    /// Американские депозитарные расписки
    Adr = 3,
    /// Глобальные депозитарные расписки
    Gdr = 4,
    /// Товарищество с ограниченной ответственностью
    Mlp = 5,
    /// Акции из реестра Нью-Йорка
    NyRegShrs = 6,
    /// Закрытый инвестиционный фонд
    ClosedEndFund = 7,
    /// Траст недвижимости
    Reit = 8,
}
impl ShareType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShareType::Unspecified => "SHARE_TYPE_UNSPECIFIED",
            ShareType::Common => "SHARE_TYPE_COMMON",
            ShareType::Preferred => "SHARE_TYPE_PREFERRED",
            ShareType::Adr => "SHARE_TYPE_ADR",
            ShareType::Gdr => "SHARE_TYPE_GDR",
            ShareType::Mlp => "SHARE_TYPE_MLP",
            ShareType::NyRegShrs => "SHARE_TYPE_NY_REG_SHRS",
            ShareType::ClosedEndFund => "SHARE_TYPE_CLOSED_END_FUND",
            ShareType::Reit => "SHARE_TYPE_REIT",
        }
    }
}
/// Тип актива.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Тип не определён.
    Unspecified = 0,
    /// Валюта.
    Currency = 1,
    /// Товар.
    Commodity = 2,
    /// Индекс.
    Index = 3,
    /// Ценная бумага.
    Security = 4,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            AssetType::Currency => "ASSET_TYPE_CURRENCY",
            AssetType::Commodity => "ASSET_TYPE_COMMODITY",
            AssetType::Index => "ASSET_TYPE_INDEX",
            AssetType::Security => "ASSET_TYPE_SECURITY",
        }
    }
}
/// Тип структурной ноты.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StructuredProductType {
    /// Тип не определён.
    SpTypeUnspecified = 0,
    /// Поставочный.
    SpTypeDeliverable = 1,
    /// Беспоставочный.
    SpTypeNonDeliverable = 2,
}
impl StructuredProductType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StructuredProductType::SpTypeUnspecified => "SP_TYPE_UNSPECIFIED",
            StructuredProductType::SpTypeDeliverable => "SP_TYPE_DELIVERABLE",
            StructuredProductType::SpTypeNonDeliverable => "SP_TYPE_NON_DELIVERABLE",
        }
    }
}
/// Тип действия со списком избранных инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EditFavoritesActionType {
    /// Тип не определён.
    Unspecified = 0,
    /// Добавить в список.
    Add = 1,
    /// Удалить из списка.
    Del = 2,
}
impl EditFavoritesActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EditFavoritesActionType::Unspecified => "EDIT_FAVORITES_ACTION_TYPE_UNSPECIFIED",
            EditFavoritesActionType::Add => "EDIT_FAVORITES_ACTION_TYPE_ADD",
            EditFavoritesActionType::Del => "EDIT_FAVORITES_ACTION_TYPE_DEL",
        }
    }
}
/// Реальная площадка исполнения расчётов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RealExchange {
    /// Тип не определён.
    Unspecified = 0,
    /// Московская биржа.
    Moex = 1,
    /// Санкт-Петербургская биржа.
    Rts = 2,
    /// Внебиржевой инструмент.
    Otc = 3,
}
impl RealExchange {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RealExchange::Unspecified => "REAL_EXCHANGE_UNSPECIFIED",
            RealExchange::Moex => "REAL_EXCHANGE_MOEX",
            RealExchange::Rts => "REAL_EXCHANGE_RTS",
            RealExchange::Otc => "REAL_EXCHANGE_OTC",
        }
    }
}
/// Generated client implementations.
pub mod instruments_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InstrumentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InstrumentsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InstrumentsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InstrumentsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод получения расписания торгов торговых площадок.
        pub async fn trading_schedules(
            &mut self,
            request: impl tonic::IntoRequest<super::TradingSchedulesRequest>,
        ) -> Result<tonic::Response<super::TradingSchedulesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения облигации по её идентификатору.
        pub async fn bond_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::BondResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка облигаций.
        pub async fn bonds(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::BondsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения графика выплат купонов по облигации.
        pub async fn get_bond_coupons(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBondCouponsRequest>,
        ) -> Result<tonic::Response<super::GetBondCouponsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBondCoupons",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения валюты по её идентификатору.
        pub async fn currency_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::CurrencyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка валют.
        pub async fn currencies(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::CurrenciesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения инвестиционного фонда по его идентификатору.
        pub async fn etf_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::EtfResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка инвестиционных фондов.
        pub async fn etfs(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::EtfsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения фьючерса по его идентификатору.
        pub async fn future_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::FutureResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка фьючерсов.
        pub async fn futures(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::FuturesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения опциона по его идентификатору.
        pub async fn option_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::OptionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/OptionBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка опционов.
        pub async fn options(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::OptionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Options",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения акции по её идентификатору.
        pub async fn share_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::ShareResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка акций.
        pub async fn shares(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::SharesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения накопленного купонного дохода по облигации.
        pub async fn get_accrued_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccruedInterestsRequest>,
        ) -> Result<tonic::Response<super::GetAccruedInterestsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения размера гарантийного обеспечения по фьючерсам.
        pub async fn get_futures_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFuturesMarginRequest>,
        ) -> Result<tonic::Response<super::GetFuturesMarginResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения основной информации об инструменте.
        pub async fn get_instrument_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::InstrumentResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод для получения событий выплаты дивидендов по инструменту.
        pub async fn get_dividends(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsRequest>,
        ) -> Result<tonic::Response<super::GetDividendsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения актива по его идентификатору.
        pub async fn get_asset_by(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetRequest>,
        ) -> Result<tonic::Response<super::AssetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssetBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка активов.
        pub async fn get_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetsRequest>,
        ) -> Result<tonic::Response<super::AssetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка избранных инструментов.
        pub async fn get_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFavoritesRequest>,
        ) -> Result<tonic::Response<super::GetFavoritesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFavorites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод редактирования списка избранных инструментов.
        pub async fn edit_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::EditFavoritesRequest>,
        ) -> Result<tonic::Response<super::EditFavoritesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EditFavorites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка стран.
        pub async fn get_countries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCountriesRequest>,
        ) -> Result<tonic::Response<super::GetCountriesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetCountries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод поиска инструмента.
        pub async fn find_instrument(
            &mut self,
            request: impl tonic::IntoRequest<super::FindInstrumentRequest>,
        ) -> Result<tonic::Response<super::FindInstrumentResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FindInstrument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка брендов.
        pub async fn get_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandsRequest>,
        ) -> Result<tonic::Response<super::GetBrandsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrands",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения бренда по его идентификатору.
        pub async fn get_brand_by(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrandBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос подписки или отписки на определённые биржевые данные.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataRequest {
    #[prost(oneof="market_data_request::Payload", tags="1, 2, 3, 4, 5, 6")]
    pub payload: ::core::option::Option<market_data_request::Payload>,
}
/// Nested message and enum types in `MarketDataRequest`.
pub mod market_data_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Запрос подписки на свечи.
        #[prost(message, tag="1")]
        SubscribeCandlesRequest(super::SubscribeCandlesRequest),
        /// Запрос подписки на стаканы.
        #[prost(message, tag="2")]
        SubscribeOrderBookRequest(super::SubscribeOrderBookRequest),
        /// Запрос подписки на ленту обезличенных сделок.
        #[prost(message, tag="3")]
        SubscribeTradesRequest(super::SubscribeTradesRequest),
        /// Запрос подписки на торговые статусы инструментов.
        #[prost(message, tag="4")]
        SubscribeInfoRequest(super::SubscribeInfoRequest),
        /// Запрос подписки на последние цены.
        #[prost(message, tag="5")]
        SubscribeLastPriceRequest(super::SubscribeLastPriceRequest),
        /// Запрос своих подписок.
        #[prost(message, tag="6")]
        GetMySubscriptions(super::GetMySubscriptions),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataServerSideStreamRequest {
    /// Запрос подписки на свечи.
    #[prost(message, optional, tag="1")]
    pub subscribe_candles_request: ::core::option::Option<SubscribeCandlesRequest>,
    /// Запрос подписки на стаканы.
    #[prost(message, optional, tag="2")]
    pub subscribe_order_book_request: ::core::option::Option<SubscribeOrderBookRequest>,
    /// Запрос подписки на ленту обезличенных сделок.
    #[prost(message, optional, tag="3")]
    pub subscribe_trades_request: ::core::option::Option<SubscribeTradesRequest>,
    /// Запрос подписки на торговые статусы инструментов.
    #[prost(message, optional, tag="4")]
    pub subscribe_info_request: ::core::option::Option<SubscribeInfoRequest>,
    /// Запрос подписки на последние цены.
    #[prost(message, optional, tag="5")]
    pub subscribe_last_price_request: ::core::option::Option<SubscribeLastPriceRequest>,
}
/// Пакет биржевой информации по подписке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataResponse {
    #[prost(oneof="market_data_response::Payload", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub payload: ::core::option::Option<market_data_response::Payload>,
}
/// Nested message and enum types in `MarketDataResponse`.
pub mod market_data_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Результат подписки на свечи.
        #[prost(message, tag="1")]
        SubscribeCandlesResponse(super::SubscribeCandlesResponse),
        /// Результат подписки на стаканы.
        #[prost(message, tag="2")]
        SubscribeOrderBookResponse(super::SubscribeOrderBookResponse),
        /// Результат подписки на поток обезличенных сделок.
        #[prost(message, tag="3")]
        SubscribeTradesResponse(super::SubscribeTradesResponse),
        /// Результат подписки на торговые статусы инструментов.
        #[prost(message, tag="4")]
        SubscribeInfoResponse(super::SubscribeInfoResponse),
        /// Свеча.
        #[prost(message, tag="5")]
        Candle(super::Candle),
        /// Сделки.
        #[prost(message, tag="6")]
        Trade(super::Trade),
        /// Стакан.
        #[prost(message, tag="7")]
        Orderbook(super::OrderBook),
        /// Торговый статус.
        #[prost(message, tag="8")]
        TradingStatus(super::TradingStatus),
        /// Проверка активности стрима.
        #[prost(message, tag="9")]
        Ping(super::Ping),
        /// Результат подписки на последние цены инструментов.
        #[prost(message, tag="10")]
        SubscribeLastPriceResponse(super::SubscribeLastPriceResponse),
        /// Последняя цена.
        #[prost(message, tag="11")]
        LastPrice(super::LastPrice),
    }
}
/// subscribeCandles | Изменения статуса подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration="SubscriptionAction", tag="1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на свечи.
    #[prost(message, repeated, tag="2")]
    pub instruments: ::prost::alloc::vec::Vec<CandleInstrument>,
    /// Флаг ожидания закрытия временного интервала для отправки свечи, применяется только для минутных свечей.
    #[prost(bool, tag="3")]
    pub waiting_close: bool,
}
/// Запрос изменения статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration="SubscriptionInterval", tag="2")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag="1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на свечи.
    #[prost(message, repeated, tag="2")]
    pub candles_subscriptions: ::prost::alloc::vec::Vec<CandleSubscription>,
}
/// Статус подписки на свечи.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration="SubscriptionInterval", tag="2")]
    pub interval: i32,
    /// Статус подписки.
    #[prost(enumeration="SubscriptionStatus", tag="3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag="4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос на изменение статуса подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration="SubscriptionAction", tag="1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на стаканы.
    #[prost(message, repeated, tag="2")]
    pub instruments: ::prost::alloc::vec::Vec<OrderBookInstrument>,
}
/// Запрос подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag="2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на стаканы.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag="1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на стаканы.
    #[prost(message, repeated, tag="2")]
    pub order_book_subscriptions: ::prost::alloc::vec::Vec<OrderBookSubscription>,
}
/// Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag="2")]
    pub depth: i32,
    /// Статус подписки.
    #[prost(enumeration="SubscriptionStatus", tag="3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag="4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration="SubscriptionAction", tag="1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на поток обезличенных сделок.
    #[prost(message, repeated, tag="2")]
    pub instruments: ::prost::alloc::vec::Vec<TradeInstrument>,
}
/// Запрос подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на поток обезличенных сделок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag="1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на поток сделок.
    #[prost(message, repeated, tag="2")]
    pub trade_subscriptions: ::prost::alloc::vec::Vec<TradeSubscription>,
}
/// Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration="SubscriptionStatus", tag="2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag="3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на торговый статус инструмента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration="SubscriptionAction", tag="1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на торговый статус.
    #[prost(message, repeated, tag="2")]
    pub instruments: ::prost::alloc::vec::Vec<InfoInstrument>,
}
/// Запрос подписки на торговый статус.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на торговый статус.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag="1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на торговый статус.
    #[prost(message, repeated, tag="2")]
    pub info_subscriptions: ::prost::alloc::vec::Vec<InfoSubscription>,
}
/// Статус подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration="SubscriptionStatus", tag="2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag="3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на последнюю цену инструмента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration="SubscriptionAction", tag="1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на последнюю цену.
    #[prost(message, repeated, tag="2")]
    pub instruments: ::prost::alloc::vec::Vec<LastPriceInstrument>,
}
/// Запрос подписки на последнюю цену.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на последнюю цену.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag="1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на последнюю цену.
    #[prost(message, repeated, tag="2")]
    pub last_price_subscriptions: ::prost::alloc::vec::Vec<LastPriceSubscription>,
}
/// Статус подписки на последнюю цену.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration="SubscriptionStatus", tag="2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag="3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет свечей в рамках стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечи.
    #[prost(enumeration="SubscriptionInterval", tag="2")]
    pub interval: i32,
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="3")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="4")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="5")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="6")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём сделок в лотах.
    #[prost(int64, tag="7")]
    pub volume: i64,
    /// Время начала интервала свечи в часовом поясе UTC.
    #[prost(message, optional, tag="8")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время последней сделки, вошедшей в свечу в часовом поясе UTC.
    #[prost(message, optional, tag="9")]
    pub last_trade_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag="10")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет стаканов в рамках стрима.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag="2")]
    pub depth: i32,
    /// Флаг консистентности стакана. **false** значит не все заявки попали в стакан по причинам сетевых задержек или нарушения порядка доставки.
    #[prost(bool, tag="3")]
    pub is_consistent: bool,
    /// Массив предложений.
    #[prost(message, repeated, tag="4")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Массив спроса.
    #[prost(message, repeated, tag="5")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Время формирования стакана в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Uid инструмента
    #[prost(string, tag="9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Массив предложений/спроса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="1")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество в лотах.
    #[prost(int64, tag="2")]
    pub quantity: i64,
}
/// Информация о сделке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration="TradeDirection", tag="2")]
    pub direction: i32,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество лотов.
    #[prost(int64, tag="4")]
    pub quantity: i64,
    /// Время сделки в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag="5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag="6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет изменения торгового статуса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStatus {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration="SecurityTradingStatus", tag="2")]
    pub trading_status: i32,
    /// Время изменения торгового статуса в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag="4")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag="5")]
    pub market_order_available_flag: bool,
    /// Uid инструмента
    #[prost(string, tag="6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос исторических свечей.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Интервал запрошенных свечей.
    #[prost(enumeration="CandleInterval", tag="4")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="5")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Список свечей.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesResponse {
    /// Массив свечей.
    #[prost(message, repeated, tag="1")]
    pub candles: ::prost::alloc::vec::Vec<HistoricCandle>,
}
/// Информация о свече.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricCandle {
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="1")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="2")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="3")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="4")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём торгов в лотах.
    #[prost(int64, tag="5")]
    pub volume: i64,
    /// Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак завершённости свечи. **false** значит, свеча за текущие интервал ещё сформирована не полностью.
    #[prost(bool, tag="7")]
    pub is_complete: bool,
}
/// Запрос получения последних цен.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesRequest {
    /// Массив figi-идентификаторов инструментов.
    #[prost(string, repeated, tag="1")]
    pub figi: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, repeated, tag="2")]
    pub instrument_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Список последних цен.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesResponse {
    /// Массив последних цен.
    #[prost(message, repeated, tag="1")]
    pub last_prices: ::prost::alloc::vec::Vec<LastPrice>,
}
/// Информация о цене.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPrice {
    /// Figi инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Последняя цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="2")]
    pub price: ::core::option::Option<Quotation>,
    /// Время получения последней цены в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag="3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag="11")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос стакана.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag="2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о стакане.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag="2")]
    pub depth: i32,
    /// Множество пар значений на покупку.
    #[prost(message, repeated, tag="3")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Множество пар значений на продажу.
    #[prost(message, repeated, tag="4")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="5")]
    pub last_price: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="6")]
    pub close_price: ::core::option::Option<Quotation>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag="8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Время получения цены последней сделки.
    #[prost(message, optional, tag="21")]
    pub last_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время получения цены закрытия.
    #[prost(message, optional, tag="22")]
    pub close_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время формирования стакана на бирже.
    #[prost(message, optional, tag="23")]
    pub orderbook_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag="9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusRequest {
    /// Идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о торговом статусе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration="SecurityTradingStatus", tag="2")]
    pub trading_status: i32,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag="3")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag="4")]
    pub market_order_available_flag: bool,
    /// Признак доступности торгов через API.
    #[prost(bool, tag="5")]
    pub api_trade_available_flag: bool,
    /// Uid инструмента
    #[prost(string, tag="6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос обезличенных сделок за последний час.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesRequest {
    /// Figi-идентификатор инструмента
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Обезличенных сделок за последний час.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesResponse {
    /// Массив сделок
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
/// Запрос активных подписок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMySubscriptions {
}
/// Запрос цен закрытия торговой сессии по инструментам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesRequest {
    /// Массив по инструментам.
    #[prost(message, repeated, tag="1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentClosePriceRequest>,
}
/// Запрос цен закрытия торговой сессии по инструменту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag="1")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Цены закрытия торговой сессии по инструментам.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesResponse {
    /// Массив по инструментам.
    #[prost(message, repeated, tag="1")]
    pub close_prices: ::prost::alloc::vec::Vec<InstrumentClosePriceResponse>,
}
/// Цена закрытия торговой сессии по инструменту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceResponse {
    /// Figi инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Uid инструмента.
    #[prost(string, tag="2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Цена закрытия торговой сессии.
    #[prost(message, optional, tag="11")]
    pub price: ::core::option::Option<Quotation>,
    /// Дата совершения торгов.
    #[prost(message, optional, tag="21")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Тип операции со списком подписок.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionAction {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Подписаться.
    Subscribe = 1,
    /// Отписаться.
    Unsubscribe = 2,
}
impl SubscriptionAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionAction::Unspecified => "SUBSCRIPTION_ACTION_UNSPECIFIED",
            SubscriptionAction::Subscribe => "SUBSCRIPTION_ACTION_SUBSCRIBE",
            SubscriptionAction::Unsubscribe => "SUBSCRIPTION_ACTION_UNSUBSCRIBE",
        }
    }
}
/// Интервал свечи.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionInterval {
    /// Интервал свечи не определён.
    Unspecified = 0,
    /// Минутные свечи.
    OneMinute = 1,
    /// Пятиминутные свечи.
    FiveMinutes = 2,
}
impl SubscriptionInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionInterval::Unspecified => "SUBSCRIPTION_INTERVAL_UNSPECIFIED",
            SubscriptionInterval::OneMinute => "SUBSCRIPTION_INTERVAL_ONE_MINUTE",
            SubscriptionInterval::FiveMinutes => "SUBSCRIPTION_INTERVAL_FIVE_MINUTES",
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionStatus {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Инструмент не найден.
    InstrumentNotFound = 2,
    /// Некорректный статус подписки, список возможных значений: \[SubscriptionAction\](<https://tinkoff.github.io/investAPI/marketdata#subscriptionaction>).
    SubscriptionActionIsInvalid = 3,
    /// Некорректная глубина стакана, доступные значения: 1, 10, 20, 30, 40, 50.
    DepthIsInvalid = 4,
    /// Некорректный интервал свечей, список возможных значений: \[SubscriptionInterval\](<https://tinkoff.github.io/investAPI/marketdata#subscriptioninterval>).
    IntervalIsInvalid = 5,
    /// Превышен лимит на общее количество подписок в рамках стрима, подробнее: [Лимитная политика](<https://tinkoff.github.io/investAPI/limits/>).
    LimitIsExceeded = 6,
    /// Внутренняя ошибка сервиса.
    InternalError = 7,
    /// Превышен лимит на количество запросов на подписки в течение установленного отрезка времени
    TooManyRequests = 8,
}
impl SubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionStatus::Unspecified => "SUBSCRIPTION_STATUS_UNSPECIFIED",
            SubscriptionStatus::Success => "SUBSCRIPTION_STATUS_SUCCESS",
            SubscriptionStatus::InstrumentNotFound => "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND",
            SubscriptionStatus::SubscriptionActionIsInvalid => "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID",
            SubscriptionStatus::DepthIsInvalid => "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID",
            SubscriptionStatus::IntervalIsInvalid => "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID",
            SubscriptionStatus::LimitIsExceeded => "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED",
            SubscriptionStatus::InternalError => "SUBSCRIPTION_STATUS_INTERNAL_ERROR",
            SubscriptionStatus::TooManyRequests => "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS",
        }
    }
}
/// Направление сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDirection {
    /// Направление сделки не определено.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl TradeDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDirection::Unspecified => "TRADE_DIRECTION_UNSPECIFIED",
            TradeDirection::Buy => "TRADE_DIRECTION_BUY",
            TradeDirection::Sell => "TRADE_DIRECTION_SELL",
        }
    }
}
/// Интервал свечей.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandleInterval {
    /// Интервал не определён.
    Unspecified = 0,
    /// 1 минута.
    CandleInterval1Min = 1,
    /// 5 минут.
    CandleInterval5Min = 2,
    /// 15 минут.
    CandleInterval15Min = 3,
    /// 1 час.
    Hour = 4,
    /// 1 день.
    Day = 5,
}
impl CandleInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandleInterval::Unspecified => "CANDLE_INTERVAL_UNSPECIFIED",
            CandleInterval::CandleInterval1Min => "CANDLE_INTERVAL_1_MIN",
            CandleInterval::CandleInterval5Min => "CANDLE_INTERVAL_5_MIN",
            CandleInterval::CandleInterval15Min => "CANDLE_INTERVAL_15_MIN",
            CandleInterval::Hour => "CANDLE_INTERVAL_HOUR",
            CandleInterval::Day => "CANDLE_INTERVAL_DAY",
        }
    }
}
/// Generated client implementations.
pub mod market_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод запроса исторических свечей по инструменту.
        pub async fn get_candles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCandlesRequest>,
        ) -> Result<tonic::Response<super::GetCandlesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод запроса последних цен по инструментам.
        pub async fn get_last_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPricesRequest>,
        ) -> Result<tonic::Response<super::GetLastPricesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения стакана по инструменту.
        pub async fn get_order_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderBookRequest>,
        ) -> Result<tonic::Response<super::GetOrderBookResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusRequest>,
        ) -> Result<tonic::Response<super::GetTradingStatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод запроса обезличенных сделок за последний час.
        pub async fn get_last_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastTradesRequest>,
        ) -> Result<tonic::Response<super::GetLastTradesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastTrades",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод запроса цен закрытия торговой сессии по инструментам.
        pub async fn get_close_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClosePricesRequest>,
        ) -> Result<tonic::Response<super::GetClosePricesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetClosePrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod market_data_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketDataStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataStreamServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Bi-directional стрим предоставления биржевой информации.
        pub async fn market_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MarketDataRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataStream",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///Server-side стрим предоставления биржевой информации.
        pub async fn market_data_server_side_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketDataServerSideStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataServerSideStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Запрос получения списка операций по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус запрашиваемых операций.
    #[prost(enumeration="OperationState", tag="4")]
    pub state: i32,
    /// Figi-идентификатор инструмента для фильтрации.
    #[prost(string, tag="5")]
    pub figi: ::prost::alloc::string::String,
}
/// Список операций.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsResponse {
    /// Массив операций.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
/// Данные по операции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Идентификатор операции.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции.
    #[prost(string, tag="2")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Валюта операции.
    #[prost(string, tag="3")]
    pub currency: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag="4")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Статус операции.
    #[prost(enumeration="OperationState", tag="6")]
    pub state: i32,
    /// Количество единиц инструмента.
    #[prost(int64, tag="7")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag="8")]
    pub quantity_rest: i64,
    /// Figi-идентификатор инструмента, связанного с операцией.
    #[prost(string, tag="9")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента. Возможные значения: </br>**bond** — облигация; </br>**share** — акция; </br>**currency** — валюта; </br>**etf** — фонд; </br>**futures** — фьючерс.
    #[prost(string, tag="10")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Дата и время операции в формате часовом поясе UTC.
    #[prost(message, optional, tag="11")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текстовое описание типа операции.
    #[prost(string, tag="12")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип операции.
    #[prost(enumeration="OperationType", tag="13")]
    pub operation_type: i32,
    /// Массив сделок.
    #[prost(message, repeated, tag="14")]
    pub trades: ::prost::alloc::vec::Vec<OperationTrade>,
}
/// Сделка по операции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationTrade {
    /// Идентификатор сделки.
    #[prost(string, tag="1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Дата и время сделки в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество инструментов.
    #[prost(int64, tag="3")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="4")]
    pub price: ::core::option::Option<MoneyValue>,
}
/// Запрос получения текущего портфеля по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Текущий портфель по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// Общая стоимость акций в портфеле в рублях.
    #[prost(message, optional, tag="1")]
    pub total_amount_shares: ::core::option::Option<MoneyValue>,
    /// Общая стоимость облигаций в портфеле в рублях.
    #[prost(message, optional, tag="2")]
    pub total_amount_bonds: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фондов в портфеле в рублях.
    #[prost(message, optional, tag="3")]
    pub total_amount_etf: ::core::option::Option<MoneyValue>,
    /// Общая стоимость валют в портфеле в рублях.
    #[prost(message, optional, tag="4")]
    pub total_amount_currencies: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фьючерсов в портфеле в рублях.
    #[prost(message, optional, tag="5")]
    pub total_amount_futures: ::core::option::Option<MoneyValue>,
    /// Текущая относительная доходность портфеля, в %.
    #[prost(message, optional, tag="6")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Список позиций портфеля.
    #[prost(message, repeated, tag="7")]
    pub positions: ::prost::alloc::vec::Vec<PortfolioPosition>,
    /// Идентификатор счёта пользователя.
    #[prost(string, tag="8")]
    pub account_id: ::prost::alloc::string::String,
}
/// Запрос позиций портфеля по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список позиций по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag="1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag="2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag="3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Признак идущей в данный момент выгрузки лимитов.
    #[prost(bool, tag="4")]
    pub limits_loading_in_progress: bool,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag="5")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag="6")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
}
/// Запрос доступного для вывода остатка.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Доступный для вывода остаток.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag="1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag="2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Заблокировано под гарантийное обеспечение фьючерсов.
    #[prost(message, repeated, tag="3")]
    pub blocked_guarantee: ::prost::alloc::vec::Vec<MoneyValue>,
}
/// Позиции портфеля.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioPosition {
    /// Figi-идентификатора инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="2")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag="3")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag="4")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag="5")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущий НКД.
    #[prost(message, optional, tag="6")]
    pub current_nkd: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции в пунктах (для фьючерсов). **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag="7")]
    pub average_position_price_pt: ::core::option::Option<Quotation>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента..
    #[prost(message, optional, tag="8")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag="9")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
    /// Количество лотов в портфеле.
    #[prost(message, optional, tag="10")]
    pub quantity_lots: ::core::option::Option<Quotation>,
    /// Заблокировано.
    #[prost(bool, tag="21")]
    pub blocked: bool,
}
/// Баланс позиции ценной бумаги.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSecurities {
    /// Figi-идентификатор бумаги.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Заблокировано.
    #[prost(int64, tag="2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag="3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag="4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag="5")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Заблокировано на бирже.
    #[prost(bool, tag="11")]
    pub exchange_blocked: bool,
    /// Тип инструмента.
    #[prost(string, tag="16")]
    pub instrument_type: ::prost::alloc::string::String,
}
/// Баланс фьючерса.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFutures {
    /// Figi-идентификатор фьючерса.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Заблокировано.
    #[prost(int64, tag="2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag="3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag="4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag="5")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Баланс опциона.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsOptions {
    /// Уникальный идентификатор позиции опциона.
    #[prost(string, tag="1")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag="2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Заблокировано.
    #[prost(int64, tag="11")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag="21")]
    pub balance: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportRequest {
    #[prost(oneof="broker_report_request::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<broker_report_request::Payload>,
}
/// Nested message and enum types in `BrokerReportRequest`.
pub mod broker_report_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="1")]
        GenerateBrokerReportRequest(super::GenerateBrokerReportRequest),
        #[prost(message, tag="2")]
        GetBrokerReportRequest(super::GetBrokerReportRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportResponse {
    #[prost(oneof="broker_report_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<broker_report_response::Payload>,
}
/// Nested message and enum types in `BrokerReportResponse`.
pub mod broker_report_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="1")]
        GenerateBrokerReportResponse(super::GenerateBrokerReportResponse),
        #[prost(message, tag="2")]
        GetBrokerReportResponse(super::GetBrokerReportResponse),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода в часовом поясе UTC.
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportResponse {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportRequest {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 1), значение по умолчанию: 0.
    #[prost(int32, tag="2")]
    pub page: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportResponse {
    #[prost(message, repeated, tag="1")]
    pub broker_report: ::prost::alloc::vec::Vec<BrokerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag="2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag="3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag="4")]
    pub page: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReport {
    /// Номер сделки.
    #[prost(string, tag="1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Номер поручения.
    #[prost(string, tag="2")]
    pub order_id: ::prost::alloc::string::String,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="3")]
    pub figi: ::prost::alloc::string::String,
    /// Признак исполнения.
    #[prost(string, tag="4")]
    pub execute_sign: ::prost::alloc::string::String,
    /// Дата и время заключения в часовом поясе UTC.
    #[prost(message, optional, tag="5")]
    pub trade_datetime: ::core::option::Option<::prost_types::Timestamp>,
    /// Торговая площадка.
    #[prost(string, tag="6")]
    pub exchange: ::prost::alloc::string::String,
    /// Режим торгов.
    #[prost(string, tag="7")]
    pub class_code: ::prost::alloc::string::String,
    /// Вид сделки.
    #[prost(string, tag="8")]
    pub direction: ::prost::alloc::string::String,
    /// Сокращённое наименование актива.
    #[prost(string, tag="9")]
    pub name: ::prost::alloc::string::String,
    /// Код актива.
    #[prost(string, tag="10")]
    pub ticker: ::prost::alloc::string::String,
    /// Цена за единицу.
    #[prost(message, optional, tag="11")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество.
    #[prost(int64, tag="12")]
    pub quantity: i64,
    /// Сумма (без НКД).
    #[prost(message, optional, tag="13")]
    pub order_amount: ::core::option::Option<MoneyValue>,
    /// НКД.
    #[prost(message, optional, tag="14")]
    pub aci_value: ::core::option::Option<Quotation>,
    /// Сумма сделки.
    #[prost(message, optional, tag="15")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Комиссия брокера.
    #[prost(message, optional, tag="16")]
    pub broker_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия биржи.
    #[prost(message, optional, tag="17")]
    pub exchange_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия клир. центра.
    #[prost(message, optional, tag="18")]
    pub exchange_clearing_commission: ::core::option::Option<MoneyValue>,
    /// Ставка РЕПО (%).
    #[prost(message, optional, tag="19")]
    pub repo_rate: ::core::option::Option<Quotation>,
    /// Контрагент/Брокер.
    #[prost(string, tag="20")]
    pub party: ::prost::alloc::string::String,
    /// Дата расчётов в часовом поясе UTC.
    #[prost(message, optional, tag="21")]
    pub clear_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата поставки в часовом поясе UTC.
    #[prost(message, optional, tag="22")]
    pub sec_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус брокера.
    #[prost(string, tag="23")]
    pub broker_status: ::prost::alloc::string::String,
    /// Тип дог.
    #[prost(string, tag="24")]
    pub separate_agreement_type: ::prost::alloc::string::String,
    /// Номер дог.
    #[prost(string, tag="25")]
    pub separate_agreement_number: ::prost::alloc::string::String,
    /// Дата дог.
    #[prost(string, tag="26")]
    pub separate_agreement_date: ::prost::alloc::string::String,
    /// Тип расчёта по сделке.
    #[prost(string, tag="27")]
    pub delivery_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerRequest {
    #[prost(oneof="get_dividends_foreign_issuer_request::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_request::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerRequest`.
pub mod get_dividends_foreign_issuer_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект запроса формирования отчёта.
        #[prost(message, tag="1")]
        GenerateDivForeignIssuerReport(super::GenerateDividendsForeignIssuerReportRequest),
        /// Объект запроса сформированного отчёта.
        #[prost(message, tag="2")]
        GetDivForeignIssuerReport(super::GetDividendsForeignIssuerReportRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerResponse {
    #[prost(oneof="get_dividends_foreign_issuer_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_response::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerResponse`.
pub mod get_dividends_foreign_issuer_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата задачи запуска формирования отчёта.
        #[prost(message, tag="1")]
        GenerateDivForeignIssuerReportResponse(super::GenerateDividendsForeignIssuerReportResponse),
        /// Отчёт "Справка о доходах за пределами РФ".
        #[prost(message, tag="2")]
        DivForeignIssuerReport(super::GetDividendsForeignIssuerReportResponse),
    }
}
/// Объект запроса формирования отчёта "Справка о доходах за пределами РФ".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag="2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag="3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект запроса сформированного отчёта "Справка о доходах за пределами РФ".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportRequest {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 0), значение по умолчанию: 0.
    #[prost(int32, tag="2")]
    pub page: i32,
}
/// Объект результата задачи запуска формирования отчёта "Справка о доходах за пределами РФ".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportResponse {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportResponse {
    #[prost(message, repeated, tag="1")]
    pub dividends_foreign_issuer_report: ::prost::alloc::vec::Vec<DividendsForeignIssuerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag="2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag="3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag="4")]
    pub page: i32,
}
/// Отчёт "Справка о доходах за пределами РФ".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsForeignIssuerReport {
    /// Дата фиксации реестра.
    #[prost(message, optional, tag="1")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата выплаты.
    #[prost(message, optional, tag="2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Наименование ценной бумаги.
    #[prost(string, tag="3")]
    pub security_name: ::prost::alloc::string::String,
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag="4")]
    pub isin: ::prost::alloc::string::String,
    /// Страна эмитента. Для депозитарных расписок указывается страна эмитента базового актива.
    #[prost(string, tag="5")]
    pub issuer_country: ::prost::alloc::string::String,
    /// Количество ценных бумаг.
    #[prost(int64, tag="6")]
    pub quantity: i64,
    /// Выплаты на одну бумагу
    #[prost(message, optional, tag="7")]
    pub dividend: ::core::option::Option<Quotation>,
    /// Комиссия внешних платёжных агентов.
    #[prost(message, optional, tag="8")]
    pub external_commission: ::core::option::Option<Quotation>,
    /// Сумма до удержания налога.
    #[prost(message, optional, tag="9")]
    pub dividend_gross: ::core::option::Option<Quotation>,
    /// Сумма налога, удержанного агентом.
    #[prost(message, optional, tag="10")]
    pub tax: ::core::option::Option<Quotation>,
    /// Итоговая сумма выплаты.
    #[prost(message, optional, tag="11")]
    pub dividend_amount: ::core::option::Option<Quotation>,
    /// Валюта.
    #[prost(string, tag="12")]
    pub currency: ::prost::alloc::string::String,
}
/// Запрос установки stream-соединения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по позициям и доходностям портфелей.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamResponse {
    #[prost(oneof="portfolio_stream_response::Payload", tags="1, 2, 3")]
    pub payload: ::core::option::Option<portfolio_stream_response::Payload>,
}
/// Nested message and enum types in `PortfolioStreamResponse`.
pub mod portfolio_stream_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag="1")]
        Subscriptions(super::PortfolioSubscriptionResult),
        /// Объект стриминга портфеля.
        #[prost(message, tag="2")]
        Portfolio(super::PortfolioResponse),
        /// Проверка активности стрима.
        #[prost(message, tag="3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<AccountSubscriptionStatus>,
}
/// Счет клиента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration="PortfolioSubscriptionStatus", tag="6")]
    pub subscription_status: i32,
}
/// Запрос списка операций по счёту с пагинацией.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorRequest {
    /// Идентификатор счёта клиента. Обязательный параметр для данного метода, остальные параметры опциональны.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента (Figi инструмента или uid инструмента)
    #[prost(string, tag="2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag="6")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag="7")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор элемента, с которого начать формировать ответ.
    #[prost(string, tag="11")]
    pub cursor: ::prost::alloc::string::String,
    /// Лимит количества операций. По умолчанию устанавливается значение **100**, максимальное значение 1000.
    #[prost(int32, tag="12")]
    pub limit: i32,
    /// Тип операции. Принимает значение из списка OperationType.
    #[prost(enumeration="OperationType", repeated, tag="13")]
    pub operation_types: ::prost::alloc::vec::Vec<i32>,
    /// Статус запрашиваемых операций, возможные значения указаны в OperationState.
    #[prost(enumeration="OperationState", tag="14")]
    pub state: i32,
    /// Флаг возвращать ли комиссии, по умолчанию false
    #[prost(bool, tag="15")]
    pub without_commissions: bool,
    /// Флаг получения ответа без массива сделок.
    #[prost(bool, tag="16")]
    pub without_trades: bool,
    /// Флаг не показывать overnight операций.
    #[prost(bool, tag="17")]
    pub without_overnights: bool,
}
/// Список операций по счёту с пагинацией.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorResponse {
    /// Признак, есть ли следующий элемент.
    #[prost(bool, tag="1")]
    pub has_next: bool,
    /// Следующий курсор.
    #[prost(string, tag="2")]
    pub next_cursor: ::prost::alloc::string::String,
    /// Список операций.
    #[prost(message, repeated, tag="6")]
    pub items: ::prost::alloc::vec::Vec<OperationItem>,
}
/// Данные об операции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItem {
    /// Курсор.
    #[prost(string, tag="1")]
    pub cursor: ::prost::alloc::string::String,
    /// Номер счета клиента.
    #[prost(string, tag="6")]
    pub broker_account_id: ::prost::alloc::string::String,
    /// Номер поручения.
    #[prost(string, tag="16")]
    pub id: ::prost::alloc::string::String,
    /// Номер родительского поручения.
    #[prost(string, tag="17")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Название операции.
    #[prost(string, tag="18")]
    pub name: ::prost::alloc::string::String,
    /// Дата поручения.
    #[prost(message, optional, tag="21")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип операции.
    #[prost(enumeration="OperationType", tag="22")]
    pub r#type: i32,
    /// Описание операции.
    #[prost(string, tag="23")]
    pub description: ::prost::alloc::string::String,
    /// Статус поручения.
    #[prost(enumeration="OperationState", tag="24")]
    pub state: i32,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag="31")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi.
    #[prost(string, tag="32")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag="33")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration="InstrumentType", tag="34")]
    pub instrument_kind: i32,
    /// Сумма операции.
    #[prost(message, optional, tag="41")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент.
    #[prost(message, optional, tag="42")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Комиссия.
    #[prost(message, optional, tag="43")]
    pub commission: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag="44")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag="45")]
    pub yield_relative: ::core::option::Option<Quotation>,
    /// Накопленный купонный доход.
    #[prost(message, optional, tag="46")]
    pub accrued_int: ::core::option::Option<MoneyValue>,
    /// Количество единиц инструмента.
    #[prost(int64, tag="51")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag="52")]
    pub quantity_rest: i64,
    /// Исполненный остаток.
    #[prost(int64, tag="53")]
    pub quantity_done: i64,
    /// Дата и время снятия заявки.
    #[prost(message, optional, tag="56")]
    pub cancel_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Причина отмены операции.
    #[prost(string, tag="57")]
    pub cancel_reason: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, optional, tag="61")]
    pub trades_info: ::core::option::Option<OperationItemTrades>,
}
/// Массив с информацией о сделках.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrades {
    #[prost(int32, tag="1")]
    pub trades_size: i32,
    #[prost(message, repeated, tag="6")]
    pub trades: ::prost::alloc::vec::Vec<OperationItemTrade>,
}
/// Сделка по операции.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrade {
    /// Номер сделки
    #[prost(string, tag="1")]
    pub num: ::prost::alloc::string::String,
    /// Дата сделки
    #[prost(message, optional, tag="6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество в единицах.
    #[prost(int64, tag="11")]
    pub quantity: i64,
    /// Цена.
    #[prost(message, optional, tag="16")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag="21")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag="22")]
    pub yield_relative: ::core::option::Option<Quotation>,
}
/// Запрос установки stream-соединения позиций.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по изменению позиций портфеля.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamResponse {
    #[prost(oneof="positions_stream_response::Payload", tags="1, 2, 3")]
    pub payload: ::core::option::Option<positions_stream_response::Payload>,
}
/// Nested message and enum types in `PositionsStreamResponse`.
pub mod positions_stream_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag="1")]
        Subscriptions(super::PositionsSubscriptionResult),
        /// Объект стриминга позиций.
        #[prost(message, tag="2")]
        Position(super::PositionData),
        /// Проверка активности стрима.
        #[prost(message, tag="3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<PositionsSubscriptionStatus>,
}
/// Счет клиента.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration="PositionsAccountSubscriptionStatus", tag="6")]
    pub subscription_status: i32,
}
/// Данные о позиции портфеля.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionData {
    /// Идентификатор счёта.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag="2")]
    pub money: ::prost::alloc::vec::Vec<PositionsMoney>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag="3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag="4")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag="5")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
    /// Дата и время операции в формате UTC.
    #[prost(message, optional, tag="6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Валютная позиция портфеля.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsMoney {
    /// Доступное количество валютный позиций.
    #[prost(message, optional, tag="1")]
    pub available_value: ::core::option::Option<MoneyValue>,
    /// Заблокированное количество валютный позиций.
    #[prost(message, optional, tag="2")]
    pub blocked_value: ::core::option::Option<MoneyValue>,
}
/// Статус запрашиваемых операций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Статус операции не определён
    Unspecified = 0,
    /// Исполнена.
    Executed = 1,
    /// Отменена.
    Canceled = 2,
    /// Исполняется.
    Progress = 3,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            OperationState::Executed => "OPERATION_STATE_EXECUTED",
            OperationState::Canceled => "OPERATION_STATE_CANCELED",
            OperationState::Progress => "OPERATION_STATE_PROGRESS",
        }
    }
}
/// Тип операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Тип операции не определён.
    Unspecified = 0,
    /// Пополнение брокерского счёта.
    Input = 1,
    /// Удержание НДФЛ по купонам.
    BondTax = 2,
    /// Вывод ЦБ.
    OutputSecurities = 3,
    /// Доход по сделке РЕПО овернайт.
    Overnight = 4,
    /// Удержание налога.
    Tax = 5,
    /// Полное погашение облигаций.
    BondRepaymentFull = 6,
    /// Продажа ЦБ с карты.
    SellCard = 7,
    /// Удержание налога по дивидендам.
    DividendTax = 8,
    /// Вывод денежных средств.
    Output = 9,
    /// Частичное погашение облигаций.
    BondRepayment = 10,
    /// Корректировка налога.
    TaxCorrection = 11,
    /// Удержание комиссии за обслуживание брокерского счёта.
    ServiceFee = 12,
    /// Удержание налога за материальную выгоду.
    BenefitTax = 13,
    /// Удержание комиссии за непокрытую позицию.
    MarginFee = 14,
    /// Покупка ЦБ.
    Buy = 15,
    /// Покупка ЦБ с карты.
    BuyCard = 16,
    /// Перевод ценных бумаг из другого депозитария.
    InputSecurities = 17,
    /// Продажа в результате Margin-call.
    SellMargin = 18,
    /// Удержание комиссии за операцию.
    BrokerFee = 19,
    /// Покупка в результате Margin-call.
    BuyMargin = 20,
    /// Выплата дивидендов.
    Dividend = 21,
    /// Продажа ЦБ.
    Sell = 22,
    /// Выплата купонов.
    Coupon = 23,
    /// Удержание комиссии SuccessFee.
    SuccessFee = 24,
    /// Передача дивидендного дохода.
    DividendTransfer = 25,
    /// Зачисление вариационной маржи.
    AccruingVarmargin = 26,
    /// Списание вариационной маржи.
    WritingOffVarmargin = 27,
    /// Покупка в рамках экспирации фьючерсного контракта.
    DeliveryBuy = 28,
    /// Продажа в рамках экспирации фьючерсного контракта.
    DeliverySell = 29,
    /// Комиссия за управление по счёту автоследования.
    TrackMfee = 30,
    /// Комиссия за результат по счёту автоследования.
    TrackPfee = 31,
    /// Удержание налога по ставке 15%.
    TaxProgressive = 32,
    /// Удержание налога по купонам по ставке 15%.
    BondTaxProgressive = 33,
    /// Удержание налога по дивидендам по ставке 15%.
    DividendTaxProgressive = 34,
    /// Удержание налога за материальную выгоду по ставке 15%.
    BenefitTaxProgressive = 35,
    /// Корректировка налога по ставке 15%.
    TaxCorrectionProgressive = 36,
    /// Удержание налога за возмещение по сделкам РЕПО по ставке 15%.
    TaxRepoProgressive = 37,
    /// Удержание налога за возмещение по сделкам РЕПО.
    TaxRepo = 38,
    /// Удержание налога по сделкам РЕПО.
    TaxRepoHold = 39,
    /// Возврат налога по сделкам РЕПО.
    TaxRepoRefund = 40,
    /// Удержание налога по сделкам РЕПО по ставке 15%.
    TaxRepoHoldProgressive = 41,
    /// Возврат налога по сделкам РЕПО по ставке 15%.
    TaxRepoRefundProgressive = 42,
    /// Выплата дивидендов на карту.
    DivExt = 43,
    /// Корректировка налога по купонам.
    TaxCorrectionCoupon = 44,
    /// Комиссия за валютный остаток.
    CashFee = 45,
    /// Комиссия за вывод валюты с брокерского счета.
    OutFee = 46,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            OperationType::Input => "OPERATION_TYPE_INPUT",
            OperationType::BondTax => "OPERATION_TYPE_BOND_TAX",
            OperationType::OutputSecurities => "OPERATION_TYPE_OUTPUT_SECURITIES",
            OperationType::Overnight => "OPERATION_TYPE_OVERNIGHT",
            OperationType::Tax => "OPERATION_TYPE_TAX",
            OperationType::BondRepaymentFull => "OPERATION_TYPE_BOND_REPAYMENT_FULL",
            OperationType::SellCard => "OPERATION_TYPE_SELL_CARD",
            OperationType::DividendTax => "OPERATION_TYPE_DIVIDEND_TAX",
            OperationType::Output => "OPERATION_TYPE_OUTPUT",
            OperationType::BondRepayment => "OPERATION_TYPE_BOND_REPAYMENT",
            OperationType::TaxCorrection => "OPERATION_TYPE_TAX_CORRECTION",
            OperationType::ServiceFee => "OPERATION_TYPE_SERVICE_FEE",
            OperationType::BenefitTax => "OPERATION_TYPE_BENEFIT_TAX",
            OperationType::MarginFee => "OPERATION_TYPE_MARGIN_FEE",
            OperationType::Buy => "OPERATION_TYPE_BUY",
            OperationType::BuyCard => "OPERATION_TYPE_BUY_CARD",
            OperationType::InputSecurities => "OPERATION_TYPE_INPUT_SECURITIES",
            OperationType::SellMargin => "OPERATION_TYPE_SELL_MARGIN",
            OperationType::BrokerFee => "OPERATION_TYPE_BROKER_FEE",
            OperationType::BuyMargin => "OPERATION_TYPE_BUY_MARGIN",
            OperationType::Dividend => "OPERATION_TYPE_DIVIDEND",
            OperationType::Sell => "OPERATION_TYPE_SELL",
            OperationType::Coupon => "OPERATION_TYPE_COUPON",
            OperationType::SuccessFee => "OPERATION_TYPE_SUCCESS_FEE",
            OperationType::DividendTransfer => "OPERATION_TYPE_DIVIDEND_TRANSFER",
            OperationType::AccruingVarmargin => "OPERATION_TYPE_ACCRUING_VARMARGIN",
            OperationType::WritingOffVarmargin => "OPERATION_TYPE_WRITING_OFF_VARMARGIN",
            OperationType::DeliveryBuy => "OPERATION_TYPE_DELIVERY_BUY",
            OperationType::DeliverySell => "OPERATION_TYPE_DELIVERY_SELL",
            OperationType::TrackMfee => "OPERATION_TYPE_TRACK_MFEE",
            OperationType::TrackPfee => "OPERATION_TYPE_TRACK_PFEE",
            OperationType::TaxProgressive => "OPERATION_TYPE_TAX_PROGRESSIVE",
            OperationType::BondTaxProgressive => "OPERATION_TYPE_BOND_TAX_PROGRESSIVE",
            OperationType::DividendTaxProgressive => "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE",
            OperationType::BenefitTaxProgressive => "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE",
            OperationType::TaxCorrectionProgressive => "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE",
            OperationType::TaxRepoProgressive => "OPERATION_TYPE_TAX_REPO_PROGRESSIVE",
            OperationType::TaxRepo => "OPERATION_TYPE_TAX_REPO",
            OperationType::TaxRepoHold => "OPERATION_TYPE_TAX_REPO_HOLD",
            OperationType::TaxRepoRefund => "OPERATION_TYPE_TAX_REPO_REFUND",
            OperationType::TaxRepoHoldProgressive => "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE",
            OperationType::TaxRepoRefundProgressive => "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE",
            OperationType::DivExt => "OPERATION_TYPE_DIV_EXT",
            OperationType::TaxCorrectionCoupon => "OPERATION_TYPE_TAX_CORRECTION_COUPON",
            OperationType::CashFee => "OPERATION_TYPE_CASH_FEE",
            OperationType::OutFee => "OPERATION_TYPE_OUT_FEE",
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PortfolioSubscriptionStatus {
    /// Тип не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Счёт не найден или недостаточно прав.
    AccountNotFound = 2,
    /// Произошла ошибка.
    InternalError = 3,
}
impl PortfolioSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PortfolioSubscriptionStatus::Unspecified => "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED",
            PortfolioSubscriptionStatus::Success => "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS",
            PortfolioSubscriptionStatus::AccountNotFound => "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND",
            PortfolioSubscriptionStatus::InternalError => "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR",
        }
    }
}
/// Тип инструмента.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentType {
    Unspecified = 0,
    /// Облигация.
    Bond = 1,
    /// Акция.
    Share = 2,
    /// Валюта.
    Currency = 3,
    /// Exchange-traded fund. Фонд.
    Etf = 4,
    /// Фьючерс.
    Futures = 5,
    /// Структурная нота.
    Sp = 6,
    /// Опцион.
    Option = 7,
}
impl InstrumentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentType::Unspecified => "INSTRUMENT_TYPE_UNSPECIFIED",
            InstrumentType::Bond => "INSTRUMENT_TYPE_BOND",
            InstrumentType::Share => "INSTRUMENT_TYPE_SHARE",
            InstrumentType::Currency => "INSTRUMENT_TYPE_CURRENCY",
            InstrumentType::Etf => "INSTRUMENT_TYPE_ETF",
            InstrumentType::Futures => "INSTRUMENT_TYPE_FUTURES",
            InstrumentType::Sp => "INSTRUMENT_TYPE_SP",
            InstrumentType::Option => "INSTRUMENT_TYPE_OPTION",
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionsAccountSubscriptionStatus {
    /// Тип не определён.
    PositionsSubscriptionStatusUnspecified = 0,
    /// Успешно.
    PositionsSubscriptionStatusSuccess = 1,
    /// Счёт не найден или недостаточно прав.
    PositionsSubscriptionStatusAccountNotFound = 2,
    /// Произошла ошибка.
    PositionsSubscriptionStatusInternalError = 3,
}
impl PositionsAccountSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusUnspecified => "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED",
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusSuccess => "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS",
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusAccountNotFound => "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND",
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusInternalError => "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR",
        }
    }
}
/// Generated client implementations.
pub mod operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OperationsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод получения списка операций по счёту.
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения портфеля по счёту.
        pub async fn get_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка позиций по счёту.
        pub async fn get_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения доступного остатка для вывода средств.
        pub async fn get_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<tonic::Response<super::WithdrawLimitsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения брокерского отчёта.
        pub async fn get_broker_report(
            &mut self,
            request: impl tonic::IntoRequest<super::BrokerReportRequest>,
        ) -> Result<tonic::Response<super::BrokerReportResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения отчёта "Справка о доходах за пределами РФ".
        pub async fn get_dividends_foreign_issuer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsForeignIssuerRequest>,
        ) -> Result<
            tonic::Response<super::GetDividendsForeignIssuerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetDividendsForeignIssuer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка операций по счёту с пагинацией.
        pub async fn get_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> Result<
            tonic::Response<super::GetOperationsByCursorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperationsByCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod operations_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OperationsStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OperationsStreamServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Server-side stream обновлений портфеля
        pub async fn portfolio_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PortfolioStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PortfolioStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        ///Server-side stream обновлений информации по изменению позиций портфеля
        pub async fn positions_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PositionsStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PositionsStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Запрос установки соединения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamRequest {
    /// Идентификаторы счетов.
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговых поручениях.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamResponse {
    #[prost(oneof="trades_stream_response::Payload", tags="1, 2")]
    pub payload: ::core::option::Option<trades_stream_response::Payload>,
}
/// Nested message and enum types in `TradesStreamResponse`.
pub mod trades_stream_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Информация об исполнении торгового поручения.
        #[prost(message, tag="1")]
        OrderTrades(super::OrderTrades),
        /// Проверка активности стрима.
        #[prost(message, tag="2")]
        Ping(super::Ping),
    }
}
/// Информация об исполнении торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrades {
    /// Идентификатор торгового поручения.
    #[prost(string, tag="1")]
    pub order_id: ::prost::alloc::string::String,
    /// Дата и время создания сообщения в часовом поясе UTC.
    #[prost(message, optional, tag="2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Направление сделки.
    #[prost(enumeration="OrderDirection", tag="3")]
    pub direction: i32,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="4")]
    pub figi: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, repeated, tag="5")]
    pub trades: ::prost::alloc::vec::Vec<OrderTrade>,
    /// Идентификатор счёта.
    #[prost(string, tag="6")]
    pub account_id: ::prost::alloc::string::String,
}
/// Информация о сделке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrade {
    /// Дата и время совершения сделки в часовом поясе UTC.
    #[prost(message, optional, tag="1")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена одного инструмента, по которой совершена сделка.
    #[prost(message, optional, tag="2")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество лотов в сделке.
    #[prost(int64, tag="3")]
    pub quantity: i64,
}
/// Запрос выставления торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag="2")]
    pub quantity: i64,
    /// Цена одного инструмента. Для получения стоимости лота требуется умножить на лотность инструмента. Игнорируется для рыночных поручений.
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration="OrderDirection", tag="4")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag="5")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration="OrderType", tag="6")]
    pub order_type: i32,
    /// Идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов.
    #[prost(string, tag="7")]
    pub order_id: ::prost::alloc::string::String,
}
/// Информация о выставлении поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderResponse {
    /// Идентификатор заявки.
    #[prost(string, tag="1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration="OrderExecutionReportStatus", tag="2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag="3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag="4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag="5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag="6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag="7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия рассчитанная при выставлении заявки.
    #[prost(message, optional, tag="8")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag="9")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату. Подробнее: [НКД при выставлении торговых поручений](<https://tinkoff.github.io/investAPI/head-orders#coupon>)
    #[prost(message, optional, tag="10")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration="OrderDirection", tag="12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Тип заявки.
    #[prost(enumeration="OrderType", tag="14")]
    pub order_type: i32,
    /// Дополнительные данные об исполнении заявки.
    #[prost(string, tag="15")]
    pub message: ::prost::alloc::string::String,
    /// Начальная цена заявки в пунктах (для фьючерсов).
    #[prost(message, optional, tag="16")]
    pub initial_order_price_pt: ::core::option::Option<Quotation>,
}
/// Запрос отмены торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    /// Номер счёта.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag="2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Результат отмены торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    /// Дата и время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос получения статуса торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderStateRequest {
    /// Номер счёта.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag="2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Запрос получения списка активных торговых поручений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersRequest {
    /// Номер счёта.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных торговых поручений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersResponse {
    /// Массив активных заявок.
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<OrderState>,
}
/// Информация о торговом поручении.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderState {
    /// Идентификатор заявки.
    #[prost(string, tag="1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration="OrderExecutionReportStatus", tag="2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag="3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag="4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag="5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag="6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag="7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по сделке.
    #[prost(message, optional, tag="8")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия, рассчитанная на момент подачи заявки.
    #[prost(message, optional, tag="9")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag="10")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление заявки.
    #[prost(enumeration="OrderDirection", tag="12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Стадии выполнения заявки.
    #[prost(message, repeated, tag="14")]
    pub stages: ::prost::alloc::vec::Vec<OrderStage>,
    /// Сервисная комиссия.
    #[prost(message, optional, tag="15")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    /// Валюта заявки.
    #[prost(string, tag="16")]
    pub currency: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration="OrderType", tag="17")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag="18")]
    pub order_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Сделки в рамках торгового поручения.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStage {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента..
    #[prost(message, optional, tag="1")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество лотов.
    #[prost(int64, tag="2")]
    pub quantity: i64,
    /// Идентификатор торговой операции.
    #[prost(string, tag="3")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос изменения выставленной заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceOrderRequest {
    /// Номер счета.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки на бирже.
    #[prost(string, tag="6")]
    pub order_id: ::prost::alloc::string::String,
    /// Новый идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов. Перезатирает старый ключ.
    #[prost(string, tag="7")]
    pub idempotency_key: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag="11")]
    pub quantity: i64,
    /// Цена за 1 инструмент.
    #[prost(message, optional, tag="12")]
    pub price: ::core::option::Option<Quotation>,
    /// Тип цены.
    #[prost(enumeration="PriceType", tag="13")]
    pub price_type: i32,
}
/// Направление операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    /// Значение не указано
    Unspecified = 0,
    /// Покупка
    Buy = 1,
    /// Продажа
    Sell = 2,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Unspecified => "ORDER_DIRECTION_UNSPECIFIED",
            OrderDirection::Buy => "ORDER_DIRECTION_BUY",
            OrderDirection::Sell => "ORDER_DIRECTION_SELL",
        }
    }
}
/// Тип заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Лимитная
    Limit = 1,
    /// Рыночная
    Market = 2,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
        }
    }
}
/// Текущий статус заявки (поручения)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderExecutionReportStatus {
    ExecutionReportStatusUnspecified = 0,
    /// Исполнена
    ExecutionReportStatusFill = 1,
    /// Отклонена
    ExecutionReportStatusRejected = 2,
    /// Отменена пользователем
    ExecutionReportStatusCancelled = 3,
    /// Новая
    ExecutionReportStatusNew = 4,
    /// Частично исполнена
    ExecutionReportStatusPartiallyfill = 5,
}
impl OrderExecutionReportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderExecutionReportStatus::ExecutionReportStatusUnspecified => "EXECUTION_REPORT_STATUS_UNSPECIFIED",
            OrderExecutionReportStatus::ExecutionReportStatusFill => "EXECUTION_REPORT_STATUS_FILL",
            OrderExecutionReportStatus::ExecutionReportStatusRejected => "EXECUTION_REPORT_STATUS_REJECTED",
            OrderExecutionReportStatus::ExecutionReportStatusCancelled => "EXECUTION_REPORT_STATUS_CANCELLED",
            OrderExecutionReportStatus::ExecutionReportStatusNew => "EXECUTION_REPORT_STATUS_NEW",
            OrderExecutionReportStatus::ExecutionReportStatusPartiallyfill => "EXECUTION_REPORT_STATUS_PARTIALLYFILL",
        }
    }
}
/// Тип цены.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    /// Значение не определено.
    Unspecified = 0,
    /// Цена в пунктах (только для фьючерсов и облигаций).
    Point = 1,
    /// Цена в валюте расчётов по инструменту.
    Currency = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::Point => "PRICE_TYPE_POINT",
            PriceType::Currency => "PRICE_TYPE_CURRENCY",
        }
    }
}
/// Generated client implementations.
pub mod orders_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrdersStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrdersStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Stream сделок пользователя
        pub async fn trades_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TradesStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/TradesStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод выставления заявки.
        pub async fn post_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод отмены биржевой заявки.
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения статуса торгового поручения.
        pub async fn get_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка активных заявок по счёту.
        pub async fn get_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод изменения выставленной заявки.
        pub async fn replace_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/ReplaceOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос получения счетов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsRequest {
}
/// Список счетов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsResponse {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
/// Информация о счёте.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Идентификатор счёта.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Тип счёта.
    #[prost(enumeration="AccountType", tag="2")]
    pub r#type: i32,
    /// Название счёта.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Статус счёта.
    #[prost(enumeration="AccountStatus", tag="4")]
    pub status: i32,
    /// Дата открытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag="5")]
    pub opened_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата закрытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag="6")]
    pub closed_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень доступа к текущему счёту (определяется токеном).
    #[prost(enumeration="AccessLevel", tag="7")]
    pub access_level: i32,
}
/// Запрос маржинальных показателей по счёту
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Маржинальные показатели по счёту.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesResponse {
    /// Ликвидная стоимость портфеля. Подробнее: [что такое ликвидный портфель?](<https://help.tinkoff.ru/margin-trade/short/liquid-portfolio/>).
    #[prost(message, optional, tag="1")]
    pub liquid_portfolio: ::core::option::Option<MoneyValue>,
    /// Начальная маржа — начальное обеспечение для совершения новой сделки. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag="2")]
    pub starting_margin: ::core::option::Option<MoneyValue>,
    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag="3")]
    pub minimal_margin: ::core::option::Option<MoneyValue>,
    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[prost(message, optional, tag="4")]
    pub funds_sufficiency_level: ::core::option::Option<Quotation>,
    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[prost(message, optional, tag="5")]
    pub amount_of_missing_funds: ::core::option::Option<MoneyValue>,
}
/// Запрос текущих лимитов пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffRequest {
}
/// Текущие лимиты пользователя.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffResponse {
    /// Массив лимитов пользователя по unary-запросам
    #[prost(message, repeated, tag="1")]
    pub unary_limits: ::prost::alloc::vec::Vec<UnaryLimit>,
    /// Массив лимитов пользователей для stream-соединений
    #[prost(message, repeated, tag="2")]
    pub stream_limits: ::prost::alloc::vec::Vec<StreamLimit>,
}
/// Лимит unary-методов.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnaryLimit {
    /// Количество unary-запросов в минуту
    #[prost(int32, tag="1")]
    pub limit_per_minute: i32,
    /// Названия методов
    #[prost(string, repeated, tag="2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Лимит stream-соединений.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    /// Максимальное количество stream-соединений
    #[prost(int32, tag="1")]
    pub limit: i32,
    /// Названия stream-методов
    #[prost(string, repeated, tag="2")]
    pub streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Запрос информации о пользователе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {
}
/// Информация о пользователе.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// Признак премиум клиента.
    #[prost(bool, tag="1")]
    pub prem_status: bool,
    /// Признак квалифицированного инвестора.
    #[prost(bool, tag="2")]
    pub qual_status: bool,
    /// Набор требующих тестирования инструментов и возможностей, с которыми может работать пользователь. \[Подробнее\](<https://tinkoff.github.io/investAPI/faq_users/>).
    #[prost(string, repeated, tag="3")]
    pub qualified_for_work_with: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Наименование тарифа пользователя.
    #[prost(string, tag="4")]
    pub tariff: ::prost::alloc::string::String,
}
/// Тип счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    /// Тип аккаунта не определён.
    Unspecified = 0,
    /// Брокерский счёт Тинькофф.
    Tinkoff = 1,
    /// ИИС счёт.
    TinkoffIis = 2,
    /// Инвесткопилка.
    InvestBox = 3,
}
impl AccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountType::Unspecified => "ACCOUNT_TYPE_UNSPECIFIED",
            AccountType::Tinkoff => "ACCOUNT_TYPE_TINKOFF",
            AccountType::TinkoffIis => "ACCOUNT_TYPE_TINKOFF_IIS",
            AccountType::InvestBox => "ACCOUNT_TYPE_INVEST_BOX",
        }
    }
}
/// Статус счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    /// Статус счёта не определён.
    Unspecified = 0,
    /// Новый, в процессе открытия.
    New = 1,
    /// Открытый и активный счёт.
    Open = 2,
    /// Закрытый счёт.
    Closed = 3,
}
impl AccountStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountStatus::Unspecified => "ACCOUNT_STATUS_UNSPECIFIED",
            AccountStatus::New => "ACCOUNT_STATUS_NEW",
            AccountStatus::Open => "ACCOUNT_STATUS_OPEN",
            AccountStatus::Closed => "ACCOUNT_STATUS_CLOSED",
        }
    }
}
/// Уровень доступа к счёту.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLevel {
    /// Уровень доступа не определён.
    AccountAccessLevelUnspecified = 0,
    /// Полный доступ к счёту.
    AccountAccessLevelFullAccess = 1,
    /// Доступ с уровнем прав "только чтение".
    AccountAccessLevelReadOnly = 2,
    /// Доступ отсутствует.
    AccountAccessLevelNoAccess = 3,
}
impl AccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessLevel::AccountAccessLevelUnspecified => "ACCOUNT_ACCESS_LEVEL_UNSPECIFIED",
            AccessLevel::AccountAccessLevelFullAccess => "ACCOUNT_ACCESS_LEVEL_FULL_ACCESS",
            AccessLevel::AccountAccessLevelReadOnly => "ACCOUNT_ACCESS_LEVEL_READ_ONLY",
            AccessLevel::AccountAccessLevelNoAccess => "ACCOUNT_ACCESS_LEVEL_NO_ACCESS",
        }
    }
}
/// Generated client implementations.
pub mod users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UsersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UsersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод получения счетов пользователя.
        pub async fn get_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Расчёт маржинальных показателей по счёту.
        pub async fn get_margin_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMarginAttributesRequest>,
        ) -> Result<tonic::Response<super::GetMarginAttributesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Запрос тарифа пользователя.
        pub async fn get_user_tariff(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserTariffRequest>,
        ) -> Result<tonic::Response<super::GetUserTariffResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения информации о пользователе.
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
/// Запрос открытия счёта в песочнице.
///
/// пустой запрос
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountRequest {
}
/// Номер открытого счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountResponse {
    /// Номер счёта
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Запрос закрытия счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountRequest {
    /// Номер счёта
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Результат закрытия счёта в песочнице.
///
/// пустой ответ
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountResponse {
}
/// Запрос пополнения счёта в песочнице.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInRequest {
    /// Номер счёта
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Сумма пополнения счёта в рублях
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<MoneyValue>,
}
/// Результат пополнения счёта, текущий баланс.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInResponse {
    /// Текущий баланс счёта
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<MoneyValue>,
}
/// Generated client implementations.
pub mod sandbox_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SandboxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SandboxServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SandboxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SandboxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод регистрации счёта в песочнице.
        pub async fn open_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::OpenSandboxAccountResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения счетов в песочнице.
        pub async fn get_sandbox_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод закрытия счёта в песочнице.
        pub async fn close_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::CloseSandboxAccountResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод выставления торгового поручения в песочнице.
        pub async fn post_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод изменения выставленной заявки.
        pub async fn replace_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/ReplaceSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения списка активных заявок по счёту в песочнице.
        pub async fn get_sandbox_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод отмены торгового поручения в песочнице.
        pub async fn cancel_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения статуса заявки в песочнице.
        pub async fn get_sandbox_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения позиций по виртуальному счёту песочницы.
        pub async fn get_sandbox_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения операций в песочнице по номеру счёта.
        pub async fn get_sandbox_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения операций в песочнице по номеру счета с пагинацией.
        pub async fn get_sandbox_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> Result<
            tonic::Response<super::GetOperationsByCursorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperationsByCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Метод получения портфолио в песочнице.
        pub async fn get_sandbox_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод пополнения счёта в песочнице.
        pub async fn sandbox_pay_in(
            &mut self,
            request: impl tonic::IntoRequest<super::SandboxPayInRequest>,
        ) -> Result<tonic::Response<super::SandboxPayInResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения доступного остатка для вывода средств в песочнице.
        pub async fn get_sandbox_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<tonic::Response<super::WithdrawLimitsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxWithdrawLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос выставления стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag="2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<Quotation>,
    /// Стоп-цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="4")]
    pub stop_price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration="StopOrderDirection", tag="5")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag="6")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип экспирации заявки.
    #[prost(enumeration="StopOrderExpirationType", tag="7")]
    pub expiration_type: i32,
    /// Тип заявки.
    #[prost(enumeration="StopOrderType", tag="8")]
    pub stop_order_type: i32,
    /// Дата и время окончания действия стоп-заявки в часовом поясе UTC. **Для ExpirationType = GoodTillDate заполнение обязательно**.
    #[prost(message, optional, tag="9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Результат выставления стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderResponse {
    /// Уникальный идентификатор стоп-заявки.
    #[prost(string, tag="1")]
    pub stop_order_id: ::prost::alloc::string::String,
}
/// Запрос получения списка активных стоп-заявок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных стоп-заявок.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersResponse {
    /// Массив стоп-заявок по счёту.
    #[prost(message, repeated, tag="1")]
    pub stop_orders: ::prost::alloc::vec::Vec<StopOrder>,
}
/// Запрос отмены выставленной стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag="1")]
    pub account_id: ::prost::alloc::string::String,
    /// Уникальный идентификатор стоп-заявки.
    #[prost(string, tag="2")]
    pub stop_order_id: ::prost::alloc::string::String,
}
/// Результат отмены выставленной стоп-заявки.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderResponse {
    /// Время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Информация о стоп-заявке.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopOrder {
    /// Идентификатор-идентификатор стоп-заявки.
    #[prost(string, tag="1")]
    pub stop_order_id: ::prost::alloc::string::String,
    /// Запрошено лотов.
    #[prost(int64, tag="2")]
    pub lots_requested: i64,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag="3")]
    pub figi: ::prost::alloc::string::String,
    /// Направление операции.
    #[prost(enumeration="StopOrderDirection", tag="4")]
    pub direction: i32,
    /// Валюта стоп-заявки.
    #[prost(string, tag="5")]
    pub currency: ::prost::alloc::string::String,
    /// Тип стоп-заявки.
    #[prost(enumeration="StopOrderType", tag="6")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag="7")]
    pub create_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время конвертации стоп-заявки в биржевую в часовом поясе UTC.
    #[prost(message, optional, tag="8")]
    pub activation_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время снятия заявки в часовом поясе UTC.
    #[prost(message, optional, tag="9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="10")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Цена активации стоп-заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag="11")]
    pub stop_price: ::core::option::Option<MoneyValue>,
}
/// Направление сделки стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderDirection {
    /// Значение не указано.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl StopOrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderDirection::Unspecified => "STOP_ORDER_DIRECTION_UNSPECIFIED",
            StopOrderDirection::Buy => "STOP_ORDER_DIRECTION_BUY",
            StopOrderDirection::Sell => "STOP_ORDER_DIRECTION_SELL",
        }
    }
}
/// Тип экспирации стоп-заявке.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderExpirationType {
    /// Значение не указано.
    Unspecified = 0,
    /// Действительно до отмены.
    GoodTillCancel = 1,
    /// Действительно до даты снятия.
    GoodTillDate = 2,
}
impl StopOrderExpirationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderExpirationType::Unspecified => "STOP_ORDER_EXPIRATION_TYPE_UNSPECIFIED",
            StopOrderExpirationType::GoodTillCancel => "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_CANCEL",
            StopOrderExpirationType::GoodTillDate => "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_DATE",
        }
    }
}
/// Тип стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderType {
    /// Значение не указано.
    Unspecified = 0,
    /// Take-profit заявка.
    TakeProfit = 1,
    /// Stop-loss заявка.
    StopLoss = 2,
    /// Stop-limit заявка.
    StopLimit = 3,
}
impl StopOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderType::Unspecified => "STOP_ORDER_TYPE_UNSPECIFIED",
            StopOrderType::TakeProfit => "STOP_ORDER_TYPE_TAKE_PROFIT",
            StopOrderType::StopLoss => "STOP_ORDER_TYPE_STOP_LOSS",
            StopOrderType::StopLimit => "STOP_ORDER_TYPE_STOP_LIMIT",
        }
    }
}
/// Generated client implementations.
pub mod stop_orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StopOrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StopOrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
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
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StopOrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StopOrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        ///Метод выставления стоп-заявки.
        pub async fn post_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostStopOrderRequest>,
        ) -> Result<tonic::Response<super::PostStopOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод получения списка активных стоп заявок по счёту.
        pub async fn get_stop_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStopOrdersRequest>,
        ) -> Result<tonic::Response<super::GetStopOrdersResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        ///Метод отмены стоп-заявки.
        pub async fn cancel_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelStopOrderRequest>,
        ) -> Result<tonic::Response<super::CancelStopOrderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
