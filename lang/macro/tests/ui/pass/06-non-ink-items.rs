use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod noop {
    #[ink(storage)]
    struct Noop {}

    impl Noop {
        #[ink(constructor)]
        fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        fn noop(&self) {}
    }

    struct NonInkStruct {}
    enum NonInkEnum {}
    impl NonInkStruct {
        fn do_nothing() {}
    }

    type NonInkTypeAlias = u32;
}

fn main() {}
