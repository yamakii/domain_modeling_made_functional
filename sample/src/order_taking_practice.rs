use std::marker::PhantomData;

struct WidgetCode(String);
struct GizmoCode(String);
enum ProductCode {
    Widget(WidgetCode),
    Gizmo(GizmoCode),
}

struct UnitQuantity(i32);
struct KilogramQuantity(f32);
enum OrderQuantity {
    Unit(UnitQuantity),
    Kilogram(KilogramQuantity),
}

struct OrderIdLineId();
struct CustomerId();

struct CustomerInfo();
struct ShippingAddress();
struct BillingAdress();
struct Price();
struct BillingAmount();

struct Order {
    order_id: Id<Order>,
    customer_id: CustomerId,
    shipping_adress: ShippingAddress,
    billing_adress: BillingAdress,
    order_lines: Vec<OrderLine>,
    amount_to_bill: BillingAmount,
}

struct OrderLine {
    id: Id<OrderLine>,
    order_id: Id<Order>,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

struct NonEmptyList<T> {
    first: T,
    rest: Vec<T>,
}

#[derive(PartialEq, PartialOrd)]
struct UnvalidatedOrder(Id<UnvalidatedOrder>);
struct ValidatedOrder(Id<ValidatedOrder>);
struct ValidationError();
type ValidationResult<T> = Result<T, ValidationError>;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Id<T> {
    id: u64,
    _phantom: PhantomData<T>,
}
impl<T> Id<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> u64 {
        self.id
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Code<T> {
    code: String,
    _phantom: PhantomData<T>,
}
impl<T> Code<T> {
    pub fn new(id: String) -> Self {
        Self {
            code: id,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> String {
        self.code.clone()
    }
}
