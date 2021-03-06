// #![cfg(test)]

// use std::any::Any;

// mod traits {
//     pub trait Foo: crate::TraitcastFrom {
//         fn foo(&mut self) -> i64;
//     }

//     pub trait Bar: crate::TraitcastFrom {
//         fn bar(&self) -> i64;
//     }

//     pub trait Baz: crate::TraitcastFrom {
//         fn baz(self: Box<Self>) -> i64;
//     }

//     crate::traitcast_to_trait!(Foo, Foo_Traitcast);
//     crate::traitcast_to_trait!(Bar, Bar_Traitcast);
//     crate::traitcast_to_trait!(Baz, Baz_Traitcast);
// }

// mod structs {
//     use crate::tests::traits::{Foo, Bar, Baz};
//     pub struct A {
//         pub x: i64
//     }

//     pub struct B {
//         pub y: i64
//     }

//     impl Foo for A {
//         fn foo(&mut self) -> i64 {
//             self.x += 1;
//             self.x
//         }
//     }

//     impl Bar for A {
//         fn bar(&self) -> i64 {
//             self.x
//         }
//     }

//     impl Foo for B {
//         fn foo(&mut self) -> i64 {
//             self.y *= 2;
//             self.y
//         }
//     }

//     impl Baz for B {
//         fn baz(self: Box<Self>) -> i64 {
//             self.y
//         }
//     }

//     crate::traitcast_to_impl!(Foo, A);
//     crate::traitcast_to_impl!(Bar, A);
//     crate::traitcast_to_impl!(Foo, B);
//     crate::traitcast_to_impl!(Baz, B);
// }

// use traits::*;
// use structs::*;

// use crate::Traitcast;

// #[test]
// fn test_traitcast() {
//     let mut x: Box<Any> = Box::new(A { x: 0 });
//     let mut y: Box<Any> = Box::new(B { y: 1 });

//     {
//         // Can cast from Any to Bar
//         let x: &dyn Bar = (*x).cast_ref().unwrap();
//         assert_eq!(x.bar(), 0);

//         // Can cast from Bar to Foo
//         assert!(crate::cast_ref::<dyn Bar, dyn Foo>(x).is_some());

//         // Any to Bar cast fails when the type does not implement Bar
//         assert!(crate::cast_ref::<dyn Any, dyn Bar>(&*y).is_none());
//     }

//     {
//         // Can cast from Any to Foo
//         let x: &mut dyn Foo = (*x).cast_mut().unwrap();
//         assert_eq!(x.foo(), 1);
//         assert_eq!(x.foo(), 2);

//         // Can cast from Foo to Bar
//         let x: &mut dyn Bar = x.cast_mut().unwrap();
//         assert_eq!(x.bar(), 2);

//         // Can also cast B from Any to Foo
//         let y: &mut dyn Foo = (*y).cast_mut().unwrap();
//         assert_eq!(y.foo(), 2);
//         assert_eq!(y.foo(), 4);

//         // Can cast from Foo to Baz
//         assert!(crate::cast_mut::<dyn Foo, dyn Baz>(y).is_some());
//     }

//     {
//         // Any to Baz fails when the type does not implement Baz
//         assert!(crate::cast_box::<dyn Any, dyn Baz>(x).is_err());

//         let mut y: Box<dyn Baz> = y.cast_box().unwrap();
//         // Can cast from Baz to Foo
//         {
//             let y: &mut dyn Foo = (*y).cast_mut().unwrap();
//             assert_eq!(y.foo(), 8);
//         }

//         assert_eq!(y.baz(), 8);
//     }
// }
