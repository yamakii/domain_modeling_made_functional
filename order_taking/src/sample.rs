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
