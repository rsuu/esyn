impl<T> Zeroed for T {}

pub trait Zeroed: Sized {
    fn null() -> Self {
        // // WARN: UB
        // unsafe {
        //     let mut config = mem::MaybeUninit::<Config>::zeroed().assume_init();
        //     config.e = vec![8; 1000000];
        // }

        unsafe {
            let mut buf = vec![0; std::mem::size_of::<Self>()];
            let res = buf.as_ptr().cast::<Self>().read_unaligned();

            res
        }
    }
}

// REFS: https://github.com/rust-lang/rust/issues/54542#issuecomment-505789992
// use std::mem::{self, MaybeUninit};
// use std::ptr;
//
// let data = {
//     // Create an uninitialized array of `MaybeUninit`. The `assume_init` is
//     // safe because the type we are claiming to have initialized here is a
//     // bunch of `MaybeUninit`s, which do not require initialization.
//     let mut data: [MaybeUninit<Vec<u32>>; 1000] = unsafe {
//         MaybeUninit::uninit().assume_init()
//     };
//
//     // Dropping a `MaybeUninit` does nothing, so if there is a panic during this loop,
//     // we have a memory leak, but there is no memory safety issue.
//     for elem in &mut data[..] {
//         unsafe { ptr::write(elem.as_mut_ptr(), vec![42]); }
//     }
//
//     // Everything is initialized. Transmute the array to the
//     // initialized type.
//     unsafe { mem::transmute::<_, [Vec<u32>; 1000]>(data) }
// };
//
// assert_eq!(&data[0], &[42]);
