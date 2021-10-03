use anyhow::Result;

struct WidgetCode(String);
struct GizmoCode(String);
enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

struct UnitQuantity(i32);
struct KilogramQuantity(f64);
enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

struct OrderId(String);
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

struct OrderLineId();
struct CustomerId();

struct CustomerInfor();
struct ShippingAddress();
struct BillingAddress();
struct Price();
struct BillingAmount();

struct Order {
    id: OrderId,
    customer_id: CustomerId,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: (),
    amount_to_bill: BillingAmount,
}

struct OrderLine {
    id: OrderLineId,
    oder_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}
