#![allow(dead_code)]

mod order_taking {
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
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Person {
        first: String,
        last: String,
    }

    enum OrderQuantity {
        UnitQuantity(u32),
        KilogramQuantity(f64),
    }

    #[test]
    fn types_works() {
        let person = Person {
            first: "Hiroshi".to_string(),
            last: "Yamaki".to_string(),
        };
        println!("{:?}", person);

        let unit_quantity = OrderQuantity::UnitQuantity(14);
        let kilogram_quantity = OrderQuantity::KilogramQuantity(77.9);

        let print_quantity = |quantity: OrderQuantity| match quantity {
            OrderQuantity::UnitQuantity(unit) => println!("{} units", unit),
            OrderQuantity::KilogramQuantity(kilo) => println!("{} kg", kilo),
        };

        print_quantity(unit_quantity);
        print_quantity(kilogram_quantity);
    }

    #[test]
    fn composed_types_work() {
        let f1 = |x: i32| x;
        let f2 = |x: Box<dyn Fn(i32) -> i32>, y: i32| x(y);
        println!("{:?}", f2(Box::new(f1), 1));

        fn f4(f: impl Fn(i32) -> i32, x: i32) -> i32 {
            f(x)
        }
        println!("{:?}", f4(f1, 1));

        fn f5(x: i32) -> impl Fn(i32) -> i32 {
            move |y: i32| x * y
        }
        println!("{:?}", f5(5)(4));
    }

    mod payment {
        struct CheckNumber(u32);
        struct CardNumber(String);
        enum CardType {
            Visa,
            Mastercard,
        }
        struct CreditCardInfo {
            card_type: CardType,
            card_number: CardNumber,
        }
        enum PaymentMethod {
            Cash,
            Check(CheckNumber),
            Card(CreditCardInfo),
        }
        struct PaymentAmount(f64);
        enum Currency {
            Eur,
            Usd,
        }
        struct Payment {
            amount: PaymentAmount,
            currency: Currency,
            method: PaymentMethod,
        }
        struct OrderId(u32);
        struct CustomerId(u32);

        // #[cfg(test)]
        // mod tests {
        //     use super::*;
        //     #[test]
        //     fn it_works() {
        //         let mut order_id = OrderId(2);
        //         let customer_id = CustomerId(3);
        //         order_id = customer_id;
        //     }
        // }
    }
}
