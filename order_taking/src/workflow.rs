use anyhow::Result;
use rust_decimal::Decimal;

use crate::domain::{
    Address, BillingAmount, CustomerInfo, EmailAddress, OrderId, OrderLineId, OrderQuantity,
    PersonalName, Price, ProductCode, String50, ZipCode,
};
// use async_trait::async_trait;

// ---------------------------------
// Input data
// ---------------------------------

pub struct UnvalidatedOrder {
    order_id: String,
    customer_info: UnvalidatedCustomer,
    shipping_address: UnvalidatedAddress,
    billing_address: UnvalidatedAddress,
    lines: Vec<UnValidatedOrderLine>,
}

pub struct UnvalidatedCustomer {
    first_name: String,
    last_name: String,
    email: String,
}

pub struct UnvalidatedAddress {
    address_line1: String,
    adress_line2: Option<String>,
    adress_line3: Option<String>,
    adress_line4: Option<String>,
    city: String,
    zip_code: u16,
}

pub struct UnValidatedOrderLine {
    id: String,
    oder_id: String,
    product_code: String,
    order_quantity: Decimal,
    price: i32,
}

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

pub type PlacedOrder = PricedOrder;

pub struct BillableOrderPlaced {
    order_id: OrderId,
    billing_address: Address,
    amount_to_bill: BillingAmount,
}

pub struct OrderAcknowledgmentSent {
    order_id: OrderId,
    email_address: EmailAddress,
}

pub enum PlaceOrderEvent {
    OrderPlaced(PlacedOrder),
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

pub fn place_order<T>(ctx: &T, unvalidated: UnvalidatedOrder) -> Result<Vec<PlaceOrderEvent>>
where
    T: CheckProductCodeExists
        + CheckAddressExists
        + GetProductPrice
        + CreateOrderAcknowledgmentLetter
        + SendOrderAcknowledgment,
{
    let validated = validate_order(ctx, unvalidated)?;
    let priced = price_order(ctx, validated)?;
    let acknowledgment_sent = acknowledgment_order(ctx, &priced);
    let events = create_events(priced, acknowledgment_sent);
    Ok(events)
}

// ---------------------------------
// Order life cycle
// ---------------------------------

pub struct ValidatedOrderLine {
    id: OrderLineId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
}

pub struct ValidatedOrder {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: Address,
    billing_address: Address,
    lines: Vec<ValidatedOrderLine>,
}

pub struct PricedOrderLine {
    id: OrderLineId,
    product_code: ProductCode,
    order_quantity: OrderQuantity,
    price: Price,
}

pub struct PricedOrder {
    id: OrderId,
    customer_info: CustomerInfo,
    shipping_address: Address,
    billing_address: Address,
    lines: Vec<PricedOrderLine>,
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

pub trait CheckProductCodeExists {
    fn exists_product_code(&self, product_code: &ProductCode) -> bool;
}

type CheckedAddress = UnvalidatedAddress;

pub trait CheckAddressExists {
    fn check_address_exists(
        &self,
        unvalidated_address: &UnvalidatedAddress,
    ) -> Result<CheckedAddress>;
}

fn validate_order<T>(ctx: &T, order: UnvalidatedOrder) -> Result<ValidatedOrder>
where
    T: CheckProductCodeExists + CheckAddressExists,
{
    let order_id = OrderId::new(order.order_id)?;
    let customer_info = to_customer_info(order.customer_info)?;
    let shipping_address = to_address(ctx, order.shipping_address)?;
    let billing_address = to_address(ctx, order.billing_address)?;
    let order_lines = order
        .lines
        .into_iter()
        .map(move |order_line| to_validate_order_line(ctx, order_line))
        .collect::<Result<Vec<_>>>()?;

    Ok(ValidatedOrder {
        id: order_id,
        customer_info,
        shipping_address,
        billing_address,
        lines: order_lines,
    })
}

fn to_customer_info(customer: UnvalidatedCustomer) -> Result<CustomerInfo> {
    Ok(CustomerInfo::new(
        PersonalName::new(
            String50::new(customer.first_name)?,
            String50::new(customer.last_name)?,
        ),
        EmailAddress::new(customer.email),
    ))
}

fn to_address<T: CheckAddressExists>(
    ctx: &T,
    unvalidated_address: UnvalidatedAddress,
) -> Result<Address> {
    let checked = ctx.check_address_exists(&unvalidated_address)?;
    Ok(Address::new(
        String50::new(checked.address_line1)?,
        checked.adress_line2.map(String50::new).transpose()?,
        checked.adress_line3.map(String50::new).transpose()?,
        checked.adress_line4.map(String50::new).transpose()?,
        String50::new(checked.city)?,
        ZipCode(checked.zip_code),
    ))
}

fn to_validate_order_line<T: CheckProductCodeExists>(
    ctx: &T,
    unvalidated_order_line: UnValidatedOrderLine,
) -> Result<ValidatedOrderLine> {
    let order_line_id = OrderLineId::new(unvalidated_order_line.id)?;
    let product_code = to_product_code(ctx, unvalidated_order_line.product_code)?;
    let order_quantity =
        OrderQuantity::create(&product_code, unvalidated_order_line.order_quantity)?;
    Ok(ValidatedOrderLine {
        id: order_line_id,
        product_code,
        order_quantity,
    })
}

fn to_product_code<T: CheckProductCodeExists>(
    ctx: &T,
    product_code: String,
) -> Result<ProductCode> {
    let product_code = ProductCode::new(product_code)?;
    if ctx.exists_product_code(&product_code) {
        Ok(product_code)
    } else {
        bail!("no exists product_code")
    }
}

// ----- Price order -----

pub trait GetProductPrice {
    fn get_product_price(&self, product_code: &ProductCode) -> Result<Price>;
}

fn price_order<T>(ctx: &T, validate_order: ValidatedOrder) -> Result<PricedOrder>
where
    T: GetProductPrice,
{
    let lines = validate_order
        .lines
        .into_iter()
        .map(|line| to_priced_order_line(ctx, line))
        .collect::<Result<Vec<_>>>()?;
    let amount_to_bill = BillingAmount::sum_prices(lines.iter().map(|line| &line.price))?;
    Ok(PricedOrder {
        id: validate_order.id,
        customer_info: validate_order.customer_info,
        shipping_address: validate_order.shipping_address,
        billing_address: validate_order.billing_address,
        lines,
        amount_to_bill,
    })
}

fn to_priced_order_line<T: GetProductPrice>(
    ctx: &T,
    line: ValidatedOrderLine,
) -> Result<PricedOrderLine> {
    let qty = line.order_quantity.value();
    let price = ctx.get_product_price(&line.product_code)?;
    let line_price = price.multiply(qty)?;
    Ok(PricedOrderLine {
        id: line.id,
        product_code: line.product_code,
        order_quantity: line.order_quantity,
        price: line_price,
    })
}
// ----- Acknowledgment order -----

pub trait CreateOrderAcknowledgmentLetter {
    fn create_order_acknowledgment_letter(&self, order: &PricedOrder) -> HtmlString;
}

pub struct HtmlString();

// #[async_trait]
pub trait SendOrderAcknowledgment {
    fn send_order_acknowledgment(&self, order: &OrderAcknowledgment) -> SendResult;
}

pub struct OrderAcknowledgment {
    email_address: EmailAddress,
    letter: HtmlString,
}

pub enum SendResult {
    Sent,
    NotSent,
}

fn acknowledgment_order<T>(ctx: &T, order: &PricedOrder) -> Option<OrderAcknowledgmentSent>
where
    T: CreateOrderAcknowledgmentLetter + SendOrderAcknowledgment,
{
    let letter = ctx.create_order_acknowledgment_letter(order);
    let acknowledgment = OrderAcknowledgment {
        email_address: order.customer_info.email_address.clone(),
        letter,
    };
    match ctx.send_order_acknowledgment(&acknowledgment) {
        SendResult::Sent => Some(OrderAcknowledgmentSent {
            order_id: order.id.clone(),
            email_address: order.customer_info.email_address.clone(),
        }),
        SendResult::NotSent => None,
    }
}

// ----- create events -----
fn create_events(
    order: PricedOrder,
    order_acknowledgment_sent: Option<OrderAcknowledgmentSent>,
) -> Vec<PlaceOrderEvent> {
    let event2 = order_acknowledgment_sent.map(PlaceOrderEvent::AcknowledgmentSent);
    let event3 = create_billing_event(&order).map(PlaceOrderEvent::BillableOrderPlaced);
    let event1 = Some(PlaceOrderEvent::OrderPlaced(order));
    vec![event1, event2, event3].into_iter().flatten().collect()
}

fn create_billing_event(order: &PlacedOrder) -> Option<BillableOrderPlaced> {
    let billing_amount = order.amount_to_bill.value();
    if billing_amount > 0.into() {
        Some(BillableOrderPlaced {
            order_id: order.id.clone(),
            billing_address: order.billing_address.clone(),
            amount_to_bill: order.amount_to_bill.clone(),
        })
    } else {
        None
    }
}
