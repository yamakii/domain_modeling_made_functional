use crate::domain::{BillingAddress, BillingAmount, OrderId, ProductCode};
use async_trait::async_trait;

struct Command<Data> {
    data: Data,
    timestamp: (),
    user_id: (),
}

enum OrderTakingCommand {
    Place(Command<UnvalidatedOrder>),
    Change(),
    Cancel(),
}

fn place_order(_order: UnvalidatedOrder) -> Result<PlaceOrderEvent, PlaceOrderEror> {
    unimplemented!()
}

struct UnvalidatedOrder {
    order_id: String,
    customer_info: (),
    shipping_address: (),
}

enum PlaceOrderEvent {
    OrderPlaced(OrderPlaced),
    BillableOrderPlaced(BillableOrderPlaced),
    AcknowledgmentSent(OrderAcknowledgmentSent),
}

enum PlaceOrderEror {
    ValidationError(Vec<ValidationError>),
}

struct ValidationError {
    fieled_name: String,
    error_description: String,
}

async fn validate_order<T>(
    _ctx: T,
    _order: UnvalidatedOrder,
) -> Result<ValidatedOrder, PlaceOrderEror>
where
    T: CheckProductCodeExists + CheckAddressExists,
{
    unimplemented!()
}

struct ValidatedOrder();

trait CheckProductCodeExists {
    fn exists_product_code(product_code: ProductCode) -> bool;
}

struct UnvalidatedAddress();
struct CheckedAddress();
struct AddressValidationError(String);

#[async_trait]
trait CheckAddressExists {
    async fn check_address_exists(
        unvalidated_address: UnvalidatedAddress,
    ) -> Result<CheckedAddress, AddressValidationError>;
}

fn price_order<T>(_ctx: T, _order: ValidatedOrder) -> Result<PricedOrder, PricingEror>
where
    T: GetProductPrice,
{
    unimplemented!()
}

struct PricedOrder();
struct PricingEror(String);

trait GetProductPrice {
    fn get_product_price(product_code: ProductCode) -> Result<PricedOrder, PricingEror>;
}

async fn acknowledgment_order<T>(_ctx: T, _order: PricedOrder) -> Option<OrderAcknowledgmentSent>
where
    T: CreateOrderAcknowledgmentLetter + SendOrderAcknowledgment,
{
    unimplemented!()
}

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

struct OrderAcknowledgmentSent {
    order_id: OrderId,
    email_address: EmailAddress,
}

type OrderPlaced = PricedOrder;
struct BillableOrderPlaced {
    order_id: OrderId,
    billing_address: BillingAddress,
    amount_to_bill: BillingAmount,
}

fn create_events(_order: PricedOrder) -> Vec<PlaceOrderEvent> {
    unimplemented!()
}
