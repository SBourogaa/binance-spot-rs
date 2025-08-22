use serde::{
    Deserialize,
    Deserializer, 
    Serialize, 
    Serializer
};

use crate::enums::TradeGroup;

/**
 * Account & symbol permissions.
 *
 * # Variants
 * - `Spot`: Spot trading permission.
 * - `Margin`: Margin trading permission.
 * - `Leveraged`: Leveraged trading permission.
 * - `TradeGroup`: Dynamic trading-group scopes (TRD_GRP_###).
 * - `Unknown`:  Any permission not recognised.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Spot,
    Margin,
    Leveraged,
    TradeGroup(TradeGroup),
    Unknown,
}

impl Serialize for Permission {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Permission::Spot => serializer.serialize_str("SPOT"),
            Permission::Margin => serializer.serialize_str("MARGIN"),
            Permission::Leveraged => serializer.serialize_str("LEVERAGED"),
            Permission::TradeGroup(group) => {
                serializer.serialize_str(&format!("TRD_GRP_{:03}", group.0))
            }
            Permission::Unknown => Err(serde::ser::Error::custom("Cannot serialize Unknown permission")),
        }
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match s.as_str() {
            "SPOT" => Ok(Permission::Spot),
            "MARGIN" => Ok(Permission::Margin),
            "LEVERAGED" => Ok(Permission::Leveraged),
            s if s.starts_with("TRD_GRP_") => {
                let digits = s.strip_prefix("TRD_GRP_")
                    .ok_or_else(|| serde::de::Error::custom("Invalid TRD_GRP format"))?;
                let id: u8 = digits.parse()
                    .map_err(|_| serde::de::Error::custom("Invalid trade group ID"))?;
                let group = TradeGroup::try_from(id)
                    .map_err(serde::de::Error::custom)?;
                Ok(Permission::TradeGroup(group))
            }
            _ => Ok(Permission::Unknown),
        }
    }
}