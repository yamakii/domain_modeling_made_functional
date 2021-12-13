use anyhow::{Context, Result};
use rust_decimal::{prelude::ToPrimitive, Decimal};

#[derive(Clone)]
pub struct String50(String);
impl String50 {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            bail!("Must not be empty")
        } else if value.len() > 50 {
            bail!("Must not be more than 50 chars")
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct WidgetCode(String);
pub struct GizmoCode(String);
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}
impl ProductCode {
    pub fn new(value: String) -> Result<Self> {
        if value.starts_with('W') {
            Ok(Self::Widget(WidgetCode(value)))
        } else if value.starts_with('G') {
            Ok(Self::Gizmo(GizmoCode(value)))
        } else {
            bail!("not implemented")
        }
    }
}

pub struct UnitQuantity(i32);
pub struct KilogramQuantity(Decimal);
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}
impl OrderQuantity {
    pub fn create(product_code: &ProductCode, value: Decimal) -> Result<Self> {
        match product_code {
            ProductCode::Widget(_) => {
                let value = value.to_i32().context("cannot parse to int")?;
                Ok(OrderQuantity::Unit(UnitQuantity(value)))
            }
            ProductCode::Gizmo(_) => Ok(OrderQuantity::Kilogram(KilogramQuantity(value))),
        }
    }

    pub fn value(&self) -> Decimal {
        match self {
            Self::Unit(u) => u.0.into(),
            Self::Kilogram(k) => k.0,
        }
    }
}

#[derive(Clone)]
pub struct OrderId(String50);
impl OrderId {
    pub fn new(value: String) -> Result<Self> {
        Ok(Self(String50::new(value)?))
    }

    pub fn value(&self) -> &str {
        self.0.value()
    }
}

pub struct OrderLineId(String50);
impl OrderLineId {
    pub fn new(value: String) -> Result<Self> {
        Ok(Self(String50::new(value)?))
    }

    pub fn value(&self) -> &str {
        self.0.value()
    }
}

pub struct CustomerId();

#[derive(new)]
pub struct PersonalName {
    first_name: String50,
    last_name: String50,
}

#[derive(new, Clone)]
pub struct EmailAddress(String);

#[derive(new)]
pub struct CustomerInfo {
    name: PersonalName,
    pub email_address: EmailAddress,
}

pub struct ZipCode(pub u16);

#[derive(new)]
pub struct Address {
    adress_line1: String50,
    adress_line2: Option<String50>,
    adress_line3: Option<String50>,
    adress_line4: Option<String50>,
    city: String50,
    zip_code: ZipCode,
}
pub struct ShippingAddress();
pub struct BillingAddress();
#[derive(new)]
pub struct Price(Decimal);
impl Price {
    pub fn create(value: Decimal) -> Result<Self> {
        // TODO; Return error if value is out of bounds
        Ok(Self(value))
    }

    pub fn multiply(&self, qty: Decimal) -> Result<Self> {
        Self::create(qty * self.0)
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}

pub struct BillingAmount(Decimal);

impl BillingAmount {
    pub fn create(value: Decimal) -> Result<Self> {
        // TODO; Return error if value is out of bounds
        Ok(Self(value))
    }

    pub fn sum_prices<'a, I>(prices: I) -> Result<Self>
    where
        I: Iterator<Item = &'a Price>,
    {
        let total = prices.map(|p| p.value()).sum();
        Self::create(total)
    }
}
