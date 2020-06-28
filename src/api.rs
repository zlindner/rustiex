// https://iexcloud.io/docs/api/#stock-prices
pub enum Prices {
    Book,
    Charts,
    DelayedQuote,
    ExtendedHoursQuote,
    HistoricalPrices,
    IntradayPrices,
    LargestTrades,
    OpenClosePrice,
    OHLC,
    PreviousDayPrice,
    PriceOnly,
    Quote,
    RealtimeQuote,
    VolumeByVenue,
}

impl Prices {}
enum Range {
    Max,
    FiveYears,
    TwoYears,
    OneYear,
    YearToDate,
    SixMonths,
    ThreeMonths, // TODO
    OneMonth,    // default
    OneMonthM,   // one month in 30 minute intervals
    FiveDays,
    FiveDaysM, // five days in 10 minute intervals,
    Date,      // TODO requires format YYYYMMDD
    Dynamic,   // uses OneMonth or OneDay depending on the day or week and time of day
}

impl Range {
    fn to_string(&self) -> str {
        match self {
            Self::Max => "max",
            Self::FiveYears => "5y",
            Self::TwoYears => "2y",
            Self::OneYear => "1y",
            Self::YearToDate => "ytd",
            Self::SixMonths => "6m",
            Self::ThreeMonths => "3m",
            Self::OneMonth => "1m",
            Self::OneMonthM => "1mm",
            Self::FiveDays => "5d",
            Self::FiveDaysM => "5dm",
            Self::Date => {
                // TODO not sure what to do here
            }
            Self::Dynamic => "dynamic",
        }
    }
}
