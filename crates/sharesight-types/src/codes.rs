use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Market {
    /// New Zealand Stock Exchange
    NZX,
    /// Australian Stock Exchange
    ASX,
    /// New York Stock Exchange
    NYSE,
    /// NASDAQ
    NASDAQ,
    /// London Stock Exchange
    LSE,
    /// Australian Fund
    FundAU,
    /// New Zealand Fund
    FundNZ,
    /// ASX mFund
    #[serde(rename = "mFund")]
    MFund,
    /// Euronext St
    EURONEXT,
    /// Canadian Securities Exchange
    CNSX,
    /// Toronto Stock Exchange
    TSE,
    /// Toronto Venture Exchange
    CVE,
    /// Hong Kong Stock Exchange
    HKG,
    /// Other Market
    OTHER,
    /// Singapore Exchange
    SGX,
    /// Johannesburg Stock Exchange
    JSE,
    /// Deutsche Boerse
    FRA,
    /// Swiss Exchange
    SWX,
    /// Tokyo Stock Exchange
    TYO,
    /// Borsa Italiana Milan
    BIT,
    /// Bolsa de Madrid
    BME,
    /// Bombay Stock Exchange
    BSE,
    /// National Stock Exchange of India
    NSE,
    /// Foreign Exchange Currency
    FX,
    /// Canadian Fund
    FundCA,
    /// NYSE American (AMEX)
    AMEX,
    /// OTC Bulletin Board
    OTCBB,
    /// Other OTC
    OTC,
    /// Shenzhen Stock Exchange
    SHE,
    /// Korea Exchange
    KRX,
    /// Taiwan Stock Exchange
    TAI,
    /// NASDAQ OMX Group MFQS
    MFQS,
    /// CBOE BATS
    BATS,
    /// UK Fund
    FundUK,
    /// Nasdaq Nordic Stockholm
    STO,
    /// Nasdaq Nordic Copenhagen
    CSE,
    /// Nasdaq Nordic Iceland
    ICE,
    /// Nasdaq Nordic Helsinki
    HEL,
    /// Oslo Stock Exchange
    OSL,
    /// Bursa Malaysia
    KLS,
    /// Shanghai Stock Exchange
    SHG,
    /// Euronext Dublin
    DUB,
    /// Moscow Exchange
    MISX,
    /// Stock Exchange of Thailand
    BKK,
    /// Warsaw Stock Exchange
    WAR,
    /// Korea Exchange (KOSDAQ)
    KOSDAQ,
    /// Tel Aviv Stock Exchange
    TLV,
    /// Budapest Stock Exchange
    BDP,
    /// NEO Exchange
    NEO,
    /// US Fund
    FundUS,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeDescription {
    /// Buy
    Buy,
    /// Sell
    Sell,
    /// Split
    Split,
    /// Bonus
    Bonus,
    /// Consolidation
    Consold,
    /// Cancellation
    Cancel,
    /// Return of Capital
    CapitalReturn,
    /// Opening Balance
    OpeningBalance,
    /// Adjust Cost Base
    AdjustCostBase,
    /// Merge (Cancel)
    MergeCancel,
    /// Merge (Buy)
    MergeBuy,
    /// Capital Call
    CapitalCall,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum PayoutDescription {
    /// Dividend
    #[serde(rename = "DIV")]
    Dividend,
    /// Capital Repayment
    #[serde(rename = "REP")]
    CapitalRepayment,
    /// Interest Payment
    #[serde(rename = "INT")]
    InterestPayment,
    /// Distribution
    #[serde(rename = "DIS")]
    Distribution,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Country {
    /// Afghanistan
    #[serde(rename = "AF")]
    Afghanistan,
    /// Albania
    #[serde(rename = "AL")]
    Albania,
    /// Algeria
    #[serde(rename = "DZ")]
    Algeria,
    /// American Samoa
    #[serde(rename = "AS")]
    AmericanSamoa,
    /// Andorra
    #[serde(rename = "AD")]
    Andorra,
    /// Angola
    #[serde(rename = "AO")]
    Angola,
    /// Anguilla
    #[serde(rename = "AI")]
    Anguilla,
    /// Antarctica
    #[serde(rename = "AQ")]
    Antarctica,
    /// Antigua and Barbuda
    #[serde(rename = "AG")]
    AntiguaAndBarbuda,
    /// Argentina
    #[serde(rename = "AR")]
    Argentina,
    /// Armenia
    #[serde(rename = "AM")]
    Armenia,
    /// Aruba
    #[serde(rename = "AW")]
    Aruba,
    /// Asia
    #[serde(rename = "_AS")]
    Asia,
    /// Australia
    #[serde(rename = "AU")]
    Australia,
    /// Austria
    #[serde(rename = "AT")]
    Austria,
    /// Azerbaijan
    #[serde(rename = "AZ")]
    Azerbaijan,
    /// Bahamas
    #[serde(rename = "BS")]
    Bahamas,
    /// Bahrain
    #[serde(rename = "BH")]
    Bahrain,
    /// Bangladesh
    #[serde(rename = "BD")]
    Bangladesh,
    /// Barbados
    #[serde(rename = "BB")]
    Barbados,
    /// Belarus
    #[serde(rename = "BY")]
    Belarus,
    /// Belgium
    #[serde(rename = "BE")]
    Belgium,
    /// Belize
    #[serde(rename = "BZ")]
    Belize,
    /// Benin
    #[serde(rename = "BJ")]
    Benin,
    /// Bermuda
    #[serde(rename = "BM")]
    Bermuda,
    /// Bhutan
    #[serde(rename = "BT")]
    Bhutan,
    /// Bitcoin
    #[serde(rename = "XB")]
    Bitcoin,
    /// Bolivia
    #[serde(rename = "BO")]
    Bolivia,
    /// Bonaire, Sint Eustatius and Saba
    #[serde(rename = "BQ")]
    BonaireSintEustatiusAndSaba,
    /// Bosnia and Herzegovina
    #[serde(rename = "BA")]
    BosniaAndHerzegovina,
    /// Botswana
    #[serde(rename = "BW")]
    Botswana,
    /// Bouvet Island
    #[serde(rename = "BV")]
    BouvetIsland,
    /// Brazil
    #[serde(rename = "BR")]
    Brazil,
    /// British Indian Ocean Territory
    #[serde(rename = "IO")]
    BritishIndianOceanTerritory,
    /// Brunei
    #[serde(rename = "BN")]
    Brunei,
    /// Bulgaria
    #[serde(rename = "BG")]
    Bulgaria,
    /// Burkina Faso
    #[serde(rename = "BF")]
    BurkinaFaso,
    /// Burundi
    #[serde(rename = "BI")]
    Burundi,
    /// Cambodia
    #[serde(rename = "KH")]
    Cambodia,
    /// Cameroon
    #[serde(rename = "CM")]
    Cameroon,
    /// Canada
    #[serde(rename = "CA")]
    Canada,
    /// Cape Verde
    #[serde(rename = "CV")]
    CapeVerde,
    /// Cayman Islands
    #[serde(rename = "KY")]
    CaymanIslands,
    /// Central African Republic
    #[serde(rename = "CF")]
    CentralAfricanRepublic,
    /// Chad
    #[serde(rename = "TD")]
    Chad,
    /// Chile
    #[serde(rename = "CL")]
    Chile,
    /// China
    #[serde(rename = "CN")]
    China,
    /// China, (Offshore)
    #[serde(rename = "_CN")]
    ChinaOffshore,
    /// Christmas Island
    #[serde(rename = "CX")]
    ChristmasIsland,
    /// Cocos (Keeling) Islands
    #[serde(rename = "CC")]
    CocosKeelingIslands,
    /// Colombia
    #[serde(rename = "CO")]
    Colombia,
    /// Comoros
    #[serde(rename = "KM")]
    Comoros,
    /// Congo Republic of the Democratic
    #[serde(rename = "CG")]
    CongoRepublicOfTheDemocratic,
    /// Congo-Brazzaville
    #[serde(rename = "CD")]
    CongoBrazzaville,
    /// Cook Islands
    #[serde(rename = "CK")]
    CookIslands,
    /// Costa Rica
    #[serde(rename = "CR")]
    CostaRica,
    /// Croatia (Hrvatska)
    #[serde(rename = "HR")]
    Croatia,
    /// Cuba
    #[serde(rename = "CU")]
    Cuba,
    /// Curaçao
    #[serde(rename = "CW")]
    Curaçao,
    /// Cyprus
    #[serde(rename = "CY")]
    Cyprus,
    /// Cyprus (pre-Euro)
    #[serde(rename = "XCY")]
    CyprusPreEuro,
    /// Czech Republic
    #[serde(rename = "CZ")]
    CzechRepublic,
    /// Denmark
    #[serde(rename = "DK")]
    Denmark,
    /// Djibouti
    #[serde(rename = "DJ")]
    Djibouti,
    /// Dominica
    #[serde(rename = "DM")]
    Dominica,
    /// Dominican Republic
    #[serde(rename = "DO")]
    DominicanRepublic,
    /// East Timor
    #[serde(rename = "TP")]
    EastTimorP,
    /// East Timor
    #[serde(rename = "TL")]
    EastTimorL,
    /// Ecuador
    #[serde(rename = "EC")]
    Ecuador,
    /// Egypt
    #[serde(rename = "EG")]
    Egypt,
    /// El Salvador
    #[serde(rename = "SV")]
    ElSalvador,
    /// Equatorial Guinea
    #[serde(rename = "GQ")]
    EquatorialGuinea,
    /// Eritrea
    #[serde(rename = "ER")]
    Eritrea,
    /// Estonia
    #[serde(rename = "EE")]
    Estonia,
    /// Estonia (pre-Euro)
    #[serde(rename = "XEE")]
    EstoniaPreEuro,
    /// Ethiopia
    #[serde(rename = "ET")]
    Ethiopia,
    /// Europe
    #[serde(rename = "XS")]
    Europe,
    /// Falkland Islands (Malvinas)
    #[serde(rename = "FK")]
    FalklandIslands,
    /// Faroe Islands
    #[serde(rename = "FO")]
    FaroeIslands,
    /// Fiji
    #[serde(rename = "FJ")]
    Fiji,
    /// Finland
    #[serde(rename = "FI")]
    Finland,
    /// France
    #[serde(rename = "FR")]
    France,
    /// French Guiana
    #[serde(rename = "GF")]
    FrenchGuiana,
    /// French Polynesia
    #[serde(rename = "PF")]
    FrenchPolynesia,
    /// French Southern Territories
    #[serde(rename = "TF")]
    FrenchSouthernTerritories,
    /// Gabon
    #[serde(rename = "GA")]
    Gabon,
    /// Gambia
    #[serde(rename = "GM")]
    Gambia,
    /// Georgia
    #[serde(rename = "GE")]
    Georgia,
    /// Germany
    #[serde(rename = "DE")]
    Germany,
    /// Ghana
    #[serde(rename = "GH")]
    Ghana,
    /// Gibraltar
    #[serde(rename = "GI")]
    Gibraltar,
    /// Greece
    #[serde(rename = "GR")]
    Greece,
    /// Greenland
    #[serde(rename = "GL")]
    Greenland,
    /// Grenada
    #[serde(rename = "GD")]
    Grenada,
    /// Guadeloupe
    #[serde(rename = "GP")]
    Guadeloupe,
    /// Guam
    #[serde(rename = "GU")]
    Guam,
    /// Guatemala
    #[serde(rename = "GT")]
    Guatemala,
    /// Guernsey
    #[serde(rename = "GG")]
    Guernsey,
    /// Guinea
    #[serde(rename = "GN")]
    Guinea,
    /// Guinea-Bissau
    #[serde(rename = "GW")]
    GuineaBissau,
    /// Guyana
    #[serde(rename = "GY")]
    Guyana,
    /// Haiti
    #[serde(rename = "HT")]
    Haiti,
    /// Heard and Mc Donald Islands
    #[serde(rename = "HM")]
    HeardAndMcDonaldIslands,
    /// Honduras
    #[serde(rename = "HN")]
    Honduras,
    /// Hong Kong
    #[serde(rename = "HK")]
    HongKong,
    /// Hungary
    #[serde(rename = "HU")]
    Hungary,
    /// Iceland
    #[serde(rename = "IS")]
    Iceland,
    /// India
    #[serde(rename = "IN")]
    India,
    /// Indonesia
    #[serde(rename = "ID")]
    Indonesia,
    /// International
    #[serde(rename = "_IN")]
    International,
    /// Iran (Islamic Republic of)
    #[serde(rename = "IR")]
    Iran,
    /// Iraq
    #[serde(rename = "IQ")]
    Iraq,
    /// Ireland
    #[serde(rename = "IE")]
    Ireland,
    /// Isle of Man
    #[serde(rename = "IM")]
    IsleOfMan,
    /// Israel
    #[serde(rename = "IL")]
    Israel,
    /// Italy
    #[serde(rename = "IT")]
    Italy,
    /// Ivory Coast
    #[serde(rename = "CI")]
    IvoryCoast,
    /// Jamaica
    #[serde(rename = "JM")]
    Jamaica,
    /// Japan
    #[serde(rename = "JP")]
    Japan,
    /// Jersey
    #[serde(rename = "JE")]
    Jersey,
    /// Jordan
    #[serde(rename = "JO")]
    Jordan,
    /// Kazakhstan
    #[serde(rename = "KZ")]
    Kazakhstan,
    /// Kenya
    #[serde(rename = "KE")]
    Kenya,
    /// Kiribati
    #[serde(rename = "KI")]
    Kiribati,
    /// Kuwait
    #[serde(rename = "KW")]
    Kuwait,
    /// Kyrgyzstan
    #[serde(rename = "KG")]
    Kyrgyzstan,
    /// Lao Peoples Democratic Republic
    #[serde(rename = "LA")]
    LaoPeoplesDemocraticRepublic,
    /// Latin America
    #[serde(rename = "_LA")]
    LatinAmerica,
    /// Latvia
    #[serde(rename = "LV")]
    Latvia,
    /// Latvia (pre-Euro)
    #[serde(rename = "XLV")]
    LatviaPreEuro,
    /// Lebanon
    #[serde(rename = "LB")]
    Lebanon,
    /// Lesotho
    #[serde(rename = "LS")]
    Lesotho,
    /// Liberia
    #[serde(rename = "LR")]
    Liberia,
    /// Libyan Arab Jamahiriya
    #[serde(rename = "LY")]
    LibyanArabJamahiriya,
    /// Liechtenstein
    #[serde(rename = "LI")]
    Liechtenstein,
    /// Lithuania
    #[serde(rename = "LT")]
    Lithuania,
    /// Lithuania (pre-Euro)
    #[serde(rename = "XLT")]
    LithuaniaPreEuro,
    /// Luxembourg
    #[serde(rename = "LU")]
    Luxembourg,
    /// Macau
    #[serde(rename = "MO")]
    Macau,
    /// Madagascar
    #[serde(rename = "MG")]
    Madagascar,
    /// Malawi
    #[serde(rename = "MW")]
    Malawi,
    /// Malaysia
    #[serde(rename = "MY")]
    Malaysia,
    /// Maldives
    #[serde(rename = "MV")]
    Maldives,
    /// Mali
    #[serde(rename = "ML")]
    Mali,
    /// Malta
    #[serde(rename = "MT")]
    Malta,
    /// Malta (pre-Euro)
    #[serde(rename = "XMT")]
    MaltaPreEuro,
    /// Marshall Islands
    #[serde(rename = "MH")]
    MarshallIslands,
    /// Martinique
    #[serde(rename = "MQ")]
    Martinique,
    /// Mauritania
    #[serde(rename = "MR")]
    Mauritania,
    /// Mauritius
    #[serde(rename = "MU")]
    Mauritius,
    /// Mayotte
    #[serde(rename = "YT")]
    Mayotte,
    /// Mexico
    #[serde(rename = "MX")]
    Mexico,
    /// Micronesia (Federated States of)
    #[serde(rename = "FM")]
    Micronesia,
    /// Middle East
    #[serde(rename = "_ME")]
    MiddleEast,
    /// Moldova (Republic of)
    #[serde(rename = "MD")]
    MoldovaRepublicOf,
    /// Monaco
    #[serde(rename = "MC")]
    Monaco,
    /// Mongolia
    #[serde(rename = "MN")]
    Mongolia,
    /// Montenegro
    #[serde(rename = "ME")]
    Montenegro,
    /// Montserrat
    #[serde(rename = "MS")]
    Montserrat,
    /// Morocco
    #[serde(rename = "MA")]
    Morocco,
    /// Mozambique
    #[serde(rename = "MZ")]
    Mozambique,
    /// Myanmar
    #[serde(rename = "MM")]
    Myanmar,
    /// Namibia
    #[serde(rename = "NA")]
    Namibia,
    /// Nauru
    #[serde(rename = "NR")]
    Nauru,
    /// Nepal
    #[serde(rename = "NP")]
    Nepal,
    /// Netherlands
    #[serde(rename = "NL")]
    Netherlands,
    /// Netherlands Antilles
    #[serde(rename = "AN")]
    NetherlandsAntilles,
    /// New Caledonia
    #[serde(rename = "NC")]
    NewCaledonia,
    /// New Zealand
    #[serde(rename = "NZ")]
    NewZealand,
    /// Nicaragua
    #[serde(rename = "NI")]
    Nicaragua,
    /// Niger
    #[serde(rename = "NE")]
    Niger,
    /// Nigeria
    #[serde(rename = "NG")]
    Nigeria,
    /// Niue
    #[serde(rename = "NU")]
    Niue,
    /// Norfolk Island
    #[serde(rename = "NF")]
    NorfolkIsland,
    /// North Korea
    #[serde(rename = "KP")]
    NorthKorea,
    /// North Macedonia
    #[serde(rename = "MK")]
    NorthMacedonia,
    /// Northern Mariana Islands
    #[serde(rename = "MP")]
    NorthernMarianaIslands,
    /// Norway
    #[serde(rename = "NO")]
    Norway,
    /// Oman
    #[serde(rename = "OM")]
    Oman,
    /// Pakistan
    #[serde(rename = "PK")]
    Pakistan,
    /// Palau
    #[serde(rename = "PW")]
    Palau,
    /// Panama
    #[serde(rename = "PA")]
    Panama,
    /// Papua New Guinea
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    /// Paraguay
    #[serde(rename = "PY")]
    Paraguay,
    /// Peru
    #[serde(rename = "PE")]
    Peru,
    /// Philippines
    #[serde(rename = "PH")]
    Philippines,
    /// Pitcairn
    #[serde(rename = "PN")]
    Pitcairn,
    /// Poland
    #[serde(rename = "PL")]
    Poland,
    /// Portugal
    #[serde(rename = "PT")]
    Portugal,
    /// Puerto Rico
    #[serde(rename = "PR")]
    PuertoRico,
    /// Qatar
    #[serde(rename = "QA")]
    Qatar,
    /// Reunion
    #[serde(rename = "RE")]
    Reunion,
    /// Romania
    #[serde(rename = "RO")]
    Romania,
    /// Russian Federation
    #[serde(rename = "RU")]
    RussianFederation,
    /// Rwanda
    #[serde(rename = "RW")]
    Rwanda,
    /// Saint Barthélemy
    #[serde(rename = "BL")]
    SaintBarthélemy,
    /// Saint Helena, Ascension and Tristan da Cunha
    #[serde(rename = "SH")]
    SaintHelena,
    /// Saint Kitts
    #[serde(rename = "KN")]
    SaintKitts,
    /// Saint Lucia
    #[serde(rename = "LC")]
    SaintLucia,
    /// Saint Martin
    #[serde(rename = "MF")]
    SaintMartin,
    /// Saint Pierre and Miquelon
    #[serde(rename = "PM")]
    SaintPierreAndMiquelon,
    /// Saint Vincent Grenadines
    #[serde(rename = "VC")]
    SaintVincentGrenadines,
    /// Samoa
    #[serde(rename = "WS")]
    Samoa,
    /// San Marino
    #[serde(rename = "SM")]
    SanMarino,
    /// Sao Tome and Principe
    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,
    /// Saudi Arabia
    #[serde(rename = "SA")]
    SaudiArabia,
    /// Senegal
    #[serde(rename = "SN")]
    Senegal,
    /// Serbia
    #[serde(rename = "RS")]
    Serbia,
    /// Seychelles
    #[serde(rename = "SC")]
    Seychelles,
    /// Sierra Leone
    #[serde(rename = "SL")]
    SierraLeone,
    /// Singapore
    #[serde(rename = "SG")]
    Singapore,
    /// Sint Maarten
    #[serde(rename = "SX")]
    SintMaarten,
    /// Slovakia (Slovak Republic)
    #[serde(rename = "SK")]
    Slovakia,
    /// Slovakia (pre-Euro)
    #[serde(rename = "XSK")]
    SlovakiaPreEuro,
    /// Slovenia
    #[serde(rename = "SI")]
    Slovenia,
    /// Solomon Islands
    #[serde(rename = "SB")]
    SolomonIslands,
    /// Somalia
    #[serde(rename = "SO")]
    Somalia,
    /// South Africa
    #[serde(rename = "ZA")]
    SouthAfrica,
    /// South Georgia and the South Sandwich Islands
    #[serde(rename = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    /// South Korea
    #[serde(rename = "KR")]
    SouthKorea,
    /// South Sudan
    #[serde(rename = "SS")]
    SouthSudan,
    /// Spain
    #[serde(rename = "ES")]
    Spain,
    /// Sri Lanka
    #[serde(rename = "LK")]
    SriLanka,
    /// State of Palestine
    #[serde(rename = "PS")]
    StateOfPalestine,
    /// Sudan
    #[serde(rename = "SD")]
    Sudan,
    /// Suriname
    #[serde(rename = "SR")]
    Suriname,
    /// Svalbard and Jan Mayen Islands
    #[serde(rename = "SJ")]
    SvalbardAndJanMayenIslands,
    /// Swaziland
    #[serde(rename = "SZ")]
    Swaziland,
    /// Sweden
    #[serde(rename = "SE")]
    Sweden,
    /// Switzerland
    #[serde(rename = "CH")]
    Switzerland,
    /// Syrian Arab Republic
    #[serde(rename = "SY")]
    SyrianArabRepublic,
    /// Taiwan
    #[serde(rename = "TW")]
    Taiwan,
    /// Tajikistan
    #[serde(rename = "TJ")]
    Tajikistan,
    /// Tanzania
    #[serde(rename = "TZ")]
    Tanzania,
    /// Thailand
    #[serde(rename = "TH")]
    Thailand,
    /// Togo
    #[serde(rename = "TG")]
    Togo,
    /// Tokelau
    #[serde(rename = "TK")]
    Tokelau,
    /// Tonga
    #[serde(rename = "TO")]
    Tonga,
    /// Trinidad and Tobago
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    /// Tunisia
    #[serde(rename = "TN")]
    Tunisia,
    /// Turkey
    #[serde(rename = "TR")]
    Turkey,
    /// Turkmenistan
    #[serde(rename = "TM")]
    Turkmenistan,
    /// Turks and Caicos Islands
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    /// Tuvalu
    #[serde(rename = "TV")]
    Tuvalu,
    /// Uganda
    #[serde(rename = "UG")]
    Uganda,
    /// Ukraine
    #[serde(rename = "UA")]
    Ukraine,
    /// United Arab Emirates
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    /// United Kingdom
    #[serde(rename = "GB")]
    UnitedKingdom,
    /// United States
    #[serde(rename = "US")]
    UnitedStates,
    /// United States Minor Outlying Islands
    #[serde(rename = "UM")]
    UnitedStatesMinorOutlyingIslands,
    /// Unknown
    #[serde(rename = "UN")]
    Unknown,
    /// Uruguay
    #[serde(rename = "UY")]
    Uruguay,
    /// Uzbekistan
    #[serde(rename = "UZ")]
    Uzbekistan,
    /// Vanuatu
    #[serde(rename = "VU")]
    Vanuatu,
    /// Vatican City State (Holy See)
    #[serde(rename = "VA")]
    VaticanCityState,
    /// Venezuela
    #[serde(rename = "VE")]
    Venezuela,
    /// Vietnam
    #[serde(rename = "VN")]
    Vietnam,
    /// Virgin Islands (British)
    #[serde(rename = "VG")]
    VirginIslandsBritish,
    /// Virgin Islands (US)
    #[serde(rename = "VI")]
    VirginIslandsUS,
    /// Wallis and Futuna Islands
    #[serde(rename = "WF")]
    WallisAndFutunaIslands,
    /// Western Sahara
    #[serde(rename = "EH")]
    WesternSahara,
    /// Yemen
    #[serde(rename = "YE")]
    Yemen,
    /// Zambia
    #[serde(rename = "ZM")]
    Zambia,
    /// Zimbabwe
    #[serde(rename = "ZW")]
    Zimbabwe,
    /// Åland Islands
    #[serde(rename = "AX")]
    ÅlandIslands,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Currency {
    /// Afghani
    AFN,
    /// Albanian Lek
    ALL,
    /// Algerian Dinar
    DZD,
    /// Argentine Peso
    ARS,
    /// Armenian Dram
    AMD,
    /// Australian Dollars
    AUD,
    /// Azerbaijani Manat
    AZN,
    /// Bahamian Dollar
    BSD,
    /// Bahraini Dinar
    BHD,
    /// Baht
    THB,
    /// Barbadian Dollar
    BBD,
    /// Belarusian Ruble
    BYN,
    /// Belizean Dollar
    BZD,
    /// Bermudian Dollar
    BMD,
    /// Bitcoin
    XBT,
    /// Bitshares
    BTS,
    /// Bolivar
    VEF,
    /// Boliviano
    BOB,
    /// Bosnia and Herzegovina Convertible Mark
    BAM,
    /// Brazil Real
    BRL,
    /// Brunei Dollar
    BND,
    /// Burundi Franc
    BIF,
    /// CFA Franc BCEAO
    XOF,
    /// CFA Franc BEAC
    XAF,
    /// CFP Franc
    XPF,
    /// Canadian Dollar
    CAD,
    /// Caymanian Dollar
    KYD,
    /// Chilean Peso
    CLP,
    /// Chinese Yuan (Offshore)
    CNH,
    /// Colombian Peso
    COP,
    /// Comoran Franc
    KMF,
    /// Congolese Frank
    CDF,
    /// Cordoba Oro
    NIO,
    /// Costa Rican Colon
    CRC,
    /// Croatian Kuna
    HRK,
    /// Cuban Peso
    CUP,
    /// Cypriot Pound
    CYP,
    /// Dalasi
    GMD,
    /// Danish Krone
    DKK,
    /// Dash
    DAS,
    /// Dinar
    SDG,
    /// Dirham
    MAD,
    /// Dirham
    AED,
    /// Djiboutian Franc
    DJF,
    /// Dobra
    STD,
    /// Dominican Peso
    DOP,
    /// Dong
    VND,
    /// East Caribbean Dollar
    XCD,
    /// Egyptian Pound
    EGP,
    /// Escudo
    CVE,
    /// Estonian Kroon
    EEK,
    /// Ethereum
    ETH,
    /// Ethiopian Birr
    ETB,
    /// Euros
    EUR,
    /// Falkland Pound
    FKP,
    /// Fijian Dollar
    FJD,
    /// Forint
    HUF,
    /// Gibraltar Pound
    GIP,
    /// Gourde
    HTG,
    /// Guinean Franc
    GNF,
    /// Guyanaese Dollar
    GYD,
    /// Hong Kong Dollar
    HKD,
    /// Hryvnia
    UAH,
    /// Icelandic Krona
    ISK,
    /// Indian Rupee
    INR,
    /// Indonesian Rupiah
    IDR,
    /// Iranian Rial
    IRR,
    /// Iraqi Dinar
    IQD,
    /// Jamaican Dollar
    JMD,
    /// Japanese Yen
    JPY,
    /// Jordanian Dinar
    JOD,
    /// Kazahstani Tenge
    KZT,
    /// Kenyan Shilling
    KES,
    /// Kina
    PGK,
    /// Kip
    LAK,
    /// Koruna
    CZK,
    /// Koruna
    SKK,
    /// Krona
    SEK,
    /// Kuwaiti Dinar
    KWD,
    /// Kwacha
    ZMK,
    /// Kyat
    MMK,
    /// Lari
    GEL,
    /// Lat
    LVL,
    /// Lebanese Pound
    LBP,
    /// Lempira
    HNL,
    /// Leone
    SLL,
    /// Leu
    MDL,
    /// Leu
    RON,
    /// Lev
    BGN,
    /// Liberian Dollar
    LRD,
    /// Libyan Dinar
    LYD,
    /// Lilangeni
    SZL,
    /// Lira
    TRY,
    /// Lita
    LTL,
    /// Litecoin
    LTC,
    /// Loti
    LSL,
    /// Macanese Pataca
    MOP,
    /// Macedonian Denar
    MKD,
    /// Malagasy Franc
    MGA,
    /// Malawian Kwacha
    MWK,
    /// Maldivian Rufiyaa
    MVR,
    /// Maltese Lira
    MTL,
    /// Manat
    TMT,
    /// Mauritian Rupee
    MUR,
    /// Metical
    MZN,
    /// Mexican Peso
    MXN,
    /// Monero
    XMR,
    /// Naira
    NGN,
    /// Namibian Dollar
    NAD,
    /// Nepalese Rupee
    NPR,
    /// Netherlands Antilles Guilder
    ANG,
    /// New Taiwan Dollar
    TWD,
    /// New Zealand Dollars
    NZD,
    /// North Korean Won
    KPW,
    /// Norwegian Krone
    NOK,
    /// Nuevo Sol
    PEN,
    /// Omani Rial
    OMR,
    /// Ouguiya
    MRO,
    /// Pakistani Rupee
    PKR,
    /// Palanga
    TOP,
    /// Panamaian Balboa
    PAB,
    /// Paraguayan Guaraní
    PYG,
    /// Philippine Peso
    PHP,
    /// Pounds Sterling
    GBP,
    /// Pula
    BWP,
    /// Qatari Riyal
    QAR,
    /// Quetzal
    GTQ,
    /// Rand
    ZAR,
    /// Rial
    YER,
    /// Riel
    KHR,
    /// Ringgit
    MYR,
    /// Ripple
    XRP,
    /// Riyal
    SAR,
    /// Ruble
    RUB,
    /// Rwanda Franc
    RWF,
    /// Salvadoran Colon
    SVC,
    /// Serbian Dinar
    RSD,
    /// Seychellois Rupee
    SCR,
    /// Shekel
    ILS,
    /// Shilling
    SOS,
    /// Shilling
    TZS,
    /// Shilling
    UGX,
    /// Singapore Dollar
    SGD,
    /// Solomon Islands Dollar
    SBD,
    /// Som
    KGS,
    /// South Korean Won
    KRW,
    /// Sri Lankan Rupee
    LKR,
    /// Stellar
    XLM,
    /// Sucre
    ECS,
    /// Surinamese Guilder
    SRD,
    /// Swiss Franc
    CHF,
    /// Syrian Pound
    SYP,
    /// Tajikistan Ruble
    TJS,
    /// Taka
    BDT,
    /// Trinidad and Tobago Dollar
    TTD,
    /// Tugrik
    MNT,
    /// Tunisian Dinar
    TND,
    /// United States Dollar
    USD,
    /// Uruguayan Peso
    UYU,
    /// Uzbekistani Soʻm
    UZS,
    /// Vanuatu Vatu
    VUV,
    /// Yuan Renminbi
    CNY,
    /// Zimbabwe Dollar
    ZWD,
    /// Zloty
    PLN,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SaleAllocationMethod {
    /// Average Cost
    #[serde(rename = "average")]
    AverageCost,
    /// Portfolio Default
    #[serde(rename = "default")]
    PortfolioDefault,
    /// First In, First Out
    #[serde(rename = "fifo")]
    FirstInFirstOut,
    /// Last In, First Out
    #[serde(rename = "lifo")]
    LastInFirstOut,
    /// Maximise Gain
    #[serde(rename = "maximise_cr")]
    MaximiseGain,
    /// Minimise Gain
    #[serde(rename = "minimise_cr")]
    MinimiseGain,
    /// Minimise CGT
    #[serde(rename = "ss_minimise")]
    MinimiseCgt,
}
