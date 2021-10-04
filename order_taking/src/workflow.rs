use crate::domain::ProductCode;

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

struct PlaceOrderEvent {
    acknowledgement_sent: (),
    order_placed: (),
    billable_order_placed: (),
}

enum PlaceOrderEror {
    ValidationError(Vec<ValidationError>),
}

struct ValidationError {
    fieled_name: String,
    error_description: String,
}

fn validate_order<T>(_ctx: T, _order: UnvalidatedOrder) -> Result<ValidatedOrder, PlaceOrderEror>
where
    T: ChackProductCodeExists + CheckAddressExists,
{
    unimplemented!()
}

struct ValidatedOrder();

trait ChackProductCodeExists {
    fn exists_product_code(product_code: ProductCode) -> bool;
}

struct UnvalidatedAddress();
struct CheckedAddress();
struct AddressValidationError(String);

trait CheckAddressExists {
    fn check_address_exists(
        unvalidated_address: UnvalidatedAddress,
    ) -> Result<CheckedAddress, AddressValidationError>;
}
