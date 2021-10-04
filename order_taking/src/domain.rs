use anyhow::Result;

pub struct WidgetCode(String);
pub struct GizmoCode(String);
pub enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

pub struct UnitQuantity(i32);
pub struct KilogramQuantity(f64);
pub enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

pub struct OrderId(String);
impl OrderId {
    pub fn new(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(anyhow!("OrderId must not be empty"))
        } else if value.len() > 50 {
            Err(anyhow!("OrderId must not be more than 50 chars"))
        } else {
            Ok(Self(value))
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub struct OrderLineId();
pub struct CustomerId();

pub struct CustomerInfor();
pub struct ShippingAddress();
pub struct BillingAddress();
pub struct Price();
pub struct BillingAmount();

pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: (),
    amount_to_bill: BillingAmount,
}

pub struct OrderLine {
    id: OrderLineId,
    oder_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}
