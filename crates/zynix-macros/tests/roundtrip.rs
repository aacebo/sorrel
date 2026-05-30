use zynix_macros::parse;

#[test]
fn raw_idents() {
    let r#foo = 1;
    assert_eq!(parse!(r#foo), 1);

    let r#fn = 2;
    assert_eq!(parse!(r#fn), 2);
}

#[test]
fn keywords() {
    parse! {
        struct S;

        impl S {
            pub fn make() -> Self {
                Self
            }

            pub fn id(self) -> Self {
                self
            }
        }
    }

    let _ = S::make().id();

    fn inner() -> i32 {
        parse! {
            let x = 3;

            match x {
                3 => x,
                _ => 0,
            }
        }
    }

    assert_eq!(inner(), 3);
}

#[test]
fn lifetimes() {
    fn first<'a>(xs: &'a [i32]) -> &'a i32 {
        parse!(&xs[0])
    }

    let v = [10, 20];
    assert_eq!(*first(&v), 10);

    parse! {
        fn _takes<'a>(_x: &'a str) {}
    }
}

#[test]
fn literals() {
    assert_eq!(parse!(1u8), 1u8);
    assert_eq!(parse!(1.5f64), 1.5f64);
    assert_eq!(parse!(0xffu16), 0xffu16);
    assert_eq!(parse!(0b1010), 0b1010);
    assert_eq!(parse!(1_000usize), 1_000usize);
    assert_eq!(parse!(b'x'), b'x');
    assert_eq!(parse!('c'), 'c');
    assert_eq!(parse!("s"), "s");
    assert_eq!(parse!(b"x"), b"x");
    assert_eq!(parse!(c"x"), c"x");
    assert_eq!(parse!(r#"raw"#), r#"raw"#);
}

#[test]
fn groups_and_multitoken() {
    assert_eq!(parse!((1 + (2 * 3))), 7);

    let v: Vec<i32> = parse!(vec![1, 2, 3]);
    assert_eq!(v, [1, 2, 3]);

    assert!(parse!(1 == 1 && 2 <= 3));
}
