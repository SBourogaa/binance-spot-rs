use super::error_categories::impl_from_code;

impl_from_code!(RequestError,
    // Request Issues (11xx series)
    // These errors occur due to problems with request format, parameters, or data validation.

    -1100 => IllegalChars,
    -1101 => TooManyParameters,
    -1102 => MandatoryParamEmptyOrMalformed,
    -1103 => UnknownParam,
    -1104 => UnreadParameters,
    -1105 => ParamEmpty,
    -1106 => ParamNotRequired,
    -1108 => ParamOverflow,
    -1111 => BadPrecision,
    -1112 => NoDepth,
    -1114 => TifNotRequired,
    -1115 => InvalidTif,
    -1116 => InvalidOrderType,
    -1117 => InvalidSide,
    -1118 => EmptyNewClOrdId,
    -1119 => EmptyOrgClOrdId,
    -1120 => BadInterval,
    -1121 => BadSymbol,
    -1122 => InvalidSymbolStatus,
    -1125 => InvalidListenKey,
    -1127 => MoreThanXXHours,
    -1128 => OptionalParamsBadCombo,
    -1130 => InvalidParameter,
    -1134 => BadStrategyType,
    -1135 => InvalidJson,
    -1139 => InvalidTickerType,
    -1145 => InvalidCancelRestrictions,
    -1151 => DuplicateSymbols,
    -1152 => InvalidSbeHeader,
    -1153 => UnsupportedSchemaId,
    -1155 => SbeDisabled,
    -1158 => OcoOrderTypeRejected,
    -1160 => OcoIcebergQtyTimeInForce,
    -1161 => DeprecatedSchema,
    -1165 => BuyOcoLimitMustBeBelow,
    -1166 => SellOcoLimitMustBeAbove,
    -1168 => BothOcoOrdersCannotBeLimit,
    -1169 => InvalidTagNumber,
    -1170 => TagNotDefinedInMessage,
    -1171 => TagAppearsMoreThanOnce,
    -1172 => TagOutOfOrder,
    -1173 => GroupFieldsOutOfOrder,
    -1174 => InvalidComponent,
    -1175 => ResetSeqNumSupport,
    -1176 => AlreadyLoggedIn,
    -1177 => GarbledMessage,
    -1178 => BadSenderCompId,
    -1179 => BadSeqNum,
    -1180 => ExpectedLogon,
    -1181 => TooManyMessages,
    -1182 => ParamsBadCombo,
    -1183 => NotAllowedInDropCopySessions,
    -1184 => DropCopySessionNotAllowed,
    -1185 => DropCopySessionRequired,
    -1186 => NotAllowedInOrderEntrySessions,
    -1187 => NotAllowedInMarketDataSessions,
    -1188 => IncorrectNumInGroupCount,
    -1189 => DuplicateEntriesInAGroup,
    -1190 => InvalidRequestId,
    -1191 => TooManySubscriptions,
    -1194 => InvalidTimeUnit,
    -1196 => BuyOcoStopLossMustBeAbove,
    -1197 => SellOcoStopLossMustBeBelow,
    -1198 => BuyOcoTakeProfitMustBeBelow,
    -1199 => SellOcoTakeProfitMustBeAbove,
);

impl RequestError {
    /**
     * Returns whether this error is due to missing required parameters.
     */
    pub fn is_missing_parameter(&self) -> bool {
        matches!(
            self,
            Self::MandatoryParamEmptyOrMalformed
                | Self::ParamEmpty
                | Self::EmptyNewClOrdId
                | Self::EmptyOrgClOrdId
        )
    }

    /**
     * Returns whether this error is due to invalid parameter values.
     */
    pub fn is_invalid_parameter(&self) -> bool {
        matches!(
            self,
            Self::IllegalChars
                | Self::InvalidParameter
                | Self::BadPrecision
                | Self::InvalidTif
                | Self::InvalidOrderType
                | Self::InvalidSide
                | Self::BadInterval
                | Self::BadSymbol
                | Self::InvalidSymbolStatus
                | Self::BadStrategyType
                | Self::InvalidJson
                | Self::InvalidTickerType
                | Self::InvalidCancelRestrictions
        )
    }

    /**
     * Returns whether this error is due to parameter combination issues.
     */
    pub fn is_parameter_combination(&self) -> bool {
        matches!(
            self,
            Self::TooManyParameters
                | Self::UnknownParam
                | Self::UnreadParameters
                | Self::ParamNotRequired
                | Self::TifNotRequired
                | Self::OptionalParamsBadCombo
                | Self::ParamsBadCombo
                | Self::DuplicateSymbols
        )
    }

    /**
     * Returns whether this error is related to OCO orders.
     */
    pub fn is_oco_related(&self) -> bool {
        matches!(
            self,
            Self::OcoOrderTypeRejected
                | Self::OcoIcebergQtyTimeInForce
                | Self::BuyOcoLimitMustBeBelow
                | Self::SellOcoLimitMustBeAbove
                | Self::BothOcoOrdersCannotBeLimit
                | Self::BuyOcoStopLossMustBeAbove
                | Self::SellOcoStopLossMustBeBelow
                | Self::BuyOcoTakeProfitMustBeBelow
                | Self::SellOcoTakeProfitMustBeAbove
        )
    }

    /**
     * Returns user-friendly error message with guidance.
     */
    pub fn user_message(&self) -> &'static str {
        match self {
            Self::IllegalChars => "Request contains illegal characters",
            Self::TooManyParameters => "Too many parameters in request",
            Self::MandatoryParamEmptyOrMalformed => "Required parameter is missing or invalid",
            Self::UnknownParam => "Unknown parameter in request",
            Self::UnreadParameters => "Extra parameters were ignored",
            Self::ParamEmpty => "Parameter cannot be empty",
            Self::ParamNotRequired => "Parameter not needed for this request",
            Self::ParamOverflow => "Parameter value is too large",
            Self::BadPrecision => "Parameter has too many decimal places",
            Self::NoDepth => "No orders available for this symbol",
            Self::TifNotRequired => "Time in force not needed for this order type",
            Self::InvalidTif => "Invalid time in force value",
            Self::InvalidOrderType => "Invalid order type",
            Self::InvalidSide => "Invalid order side (must be BUY or SELL)",
            Self::EmptyNewClOrdId => "Client order ID cannot be empty",
            Self::EmptyOrgClOrdId => "Original client order ID cannot be empty",
            Self::BadInterval => "Invalid time interval",
            Self::BadSymbol => "Invalid trading symbol",
            Self::InvalidSymbolStatus => "Invalid symbol status filter",
            Self::InvalidListenKey => "Listen key is invalid or expired",
            Self::MoreThanXXHours => "Time range is too large",
            Self::OptionalParamsBadCombo => "Invalid combination of optional parameters",
            Self::InvalidParameter => "Parameter value is invalid",
            Self::BadStrategyType => "Strategy type must be >= 1000000",
            Self::InvalidJson => "Invalid JSON format",
            Self::InvalidTickerType => "Invalid ticker type",
            Self::InvalidCancelRestrictions => "Invalid cancel restrictions",
            Self::DuplicateSymbols => "Symbol appears multiple times",
            Self::OcoOrderTypeRejected => "Order type not supported in OCO",
            Self::OcoIcebergQtyTimeInForce => "OCO iceberg orders require GTC time in force",
            Self::BuyOcoLimitMustBeBelow => "Buy OCO limit order must be below market price",
            Self::SellOcoLimitMustBeAbove => "Sell OCO limit order must be above market price",
            Self::BothOcoOrdersCannotBeLimit => "OCO orders cannot both be limit orders",
            Self::BuyOcoStopLossMustBeAbove => "Buy OCO stop loss must be above market price",
            Self::SellOcoStopLossMustBeBelow => "Sell OCO stop loss must be below market price",
            Self::BuyOcoTakeProfitMustBeBelow => "Buy OCO take profit must be below market price",
            Self::SellOcoTakeProfitMustBeAbove => "Sell OCO take profit must be above market price",
            _ => "Request parameter error - please check your request format",
        }
    }
}
