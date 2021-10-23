use crate::domain::{
    Address, BillingAddress, BillingAmount, CustomerInfo, OrderId, OrderLineId, OrderQuantity,
    Price, ProductCode,
};
use async_trait::async_trait;

// ---------------------------------
// Input data
// ---------------------------------

pub struct UnvalidatedOrder {
    order_id: String,
    customer_info: UnvalidatedCustomer,
    shipping_address: UnvalidatedAddress,
}

pub struct UnvalidatedCustomer {
    name: String,
    email: String,
}

pub struct UnvalidatedAddress();

// ---------------------------------
// Input Command
// ---------------------------------

pub struct Command<Data> {
    data: Data,
    timestamp: (),
    user_id: (),
}

pub type PlaceOrderCommand = Command<UnvalidatedOrder>;

pub enum OrderTakingCommand {
    Place(PlaceOrderCommand),
    Change(),
    Cancel(),
}

// ---------------------------------
// Public API
// ---------------------------------

pub type OrderPlaced = PricedOrder;

pub struct BillableOrderPlaced {
    order_id: OrderId,
    billing_address: BillingAddress,
    amount_to_bill: BillingAmount,
}

pub struct OrderAcknowledgmentSent {
    order_id: OrderId,
    email_address: EmailAddress,
}

pub enum PlaceOrderEvent {
    OrderPlaced(OrderPlaced),
    BillableOrderPlaced(BillableOrderPlaced),
    AcknowledgmentSent(OrderAcknowledgmentSent),
}

pub enum PlaceOrderEror {
    ValidationError(Vec<ValidationError>),
}

pub struct ValidationError {
    fieled_name: String,
    error_description: String,
}

pub async fn place_order(
    _order: PlaceOrderCommand,
) -> Result<Vec<PlaceOrderEvent>, PlaceOrderEror> {
    unimplemented!()
}

// ---------------------------------
// Order life cycle
// ---------------------------------

pub struct ValidatedOrderLine {
    id: OrderLineId,
    oder_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

pub struct ValidatedOrder {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: Address,
    billing_address: Address,
    order_lines: Vec<ValidatedOrderLine>,
}

pub struct PricedOrderLine {
    id: OrderLineId,
    oder_id: OrderId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

pub struct PricedOrder {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: Address,
    billing_address: Address,
    order_lines: Vec<ValidatedOrderLine>,
    amount_to_bill: BillingAmount,
}

pub enum Order {
    Unvalidated(UnvalidatedOrder),
    Validated(ValidatedOrder),
    Priced(PricedOrder),
}

// ---------------------------------
// Definitions of Internal Steps
// ---------------------------------

// ----- Validate order -----

trait CheckProductCodeExists {
    fn exists_product_code(product_code: ProductCode) -> bool;
}

struct CheckedAddress();
struct AddressValidationError(String);

#[async_trait]
trait CheckAddressExists {
    async fn check_address_exists(
        unvalidated_address: UnvalidatedAddress,
    ) -> Result<CheckedAddress, AddressValidationError>;
}

async fn validate_order<T>(
    _ctx: T,
    _order: UnvalidatedOrder,
) -> Result<ValidatedOrder, Vec<ValidationError>>
where
    T: CheckProductCodeExists + CheckAddressExists,
{
    unimplemented!()
}

// ----- Price order -----

trait GetProductPrice {
    fn get_product_price(product_code: ProductCode) -> Result<PricedOrder, PricingEror>;
}

struct PricingEror(String);

fn price_order<T>(_ctx: T, _order: ValidatedOrder) -> Result<PricedOrder, PricingEror>
where
    T: GetProductPrice,
{
    unimplemented!()
}

// ----- Acknowledgment order -----

trait CreateOrderAcknowledgmentLetter {
    fn create_order_acknowledgment_letter(_order: PricedOrder) -> HtmlString;
}

struct HtmlString();

#[async_trait]
trait SendOrderAcknowledgment {
    async fn send_order_acknowledgment(
        _order: OrderAcknowledgment,
    ) -> Option<OrderAcknowledgmentSent>;
}

struct EmailAddress();

struct OrderAcknowledgment {
    email_address: EmailAddress,
    letter: HtmlString,
}

async fn acknowledgment_order<T>(_ctx: T, _order: PricedOrder) -> Option<OrderAcknowledgmentSent>
where
    T: CreateOrderAcknowledgmentLetter + SendOrderAcknowledgment,
{
    unimplemented!()
}

// ----- create events -----

fn create_events(_order: PricedOrder) -> Vec<PlaceOrderEvent> {
    unimplemented!()
}
