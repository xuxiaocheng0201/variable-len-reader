macro_rules! write_float_future {
    (varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_float_future!(cfg(feature = "async_float_varint"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    (long_varint, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        write_float_future!(cfg(feature = "async_float_varint_long"), $primitive, $future, $poll_func, $struct_buf, $internal_struct);
    };
    ($feature: meta, $primitive: ty, $future: ident, $poll_func: ident, $struct_buf: ident, $internal_struct: ident) => {
        #[$feature]
        #[cfg_attr(docsrs, doc($feature))]
        #[derive(Debug)]
        struct $struct_buf {
            internal: $internal_struct,
        }
        #[$feature]
        impl $struct_buf {
            fn new(num: $primitive) -> Self {
                Self { internal: $internal_struct::new(num.to_bits()) }
            }
            fn reset(&mut self, num: $primitive) {
                self.internal.reset(num.to_bits());
            }
        }
        #[$feature]
        $crate::pin_project_lite::pin_project! {
            #[cfg_attr(docsrs, doc($feature))]
            #[derive(Debug)]
            #[project(!Unpin)]
            #[must_use = "futures do nothing unless you `.await` or poll them"]
            pub struct $future<'a, W: ?Sized> {
                #[pin]
                writer: &'a mut W,
                inner: $struct_buf,
            }
        }
        #[$feature]
        impl<'a, W: $crate::AsyncVariableWritable + Unpin + ?Sized> Future for $future<'a, W> {
            type Output = ::core::result::Result<usize>;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                let mut me = self.project();
                W::$poll_func(Pin::new(&mut *me.writer), cx, me.inner)
            }
        }
    };
}
macro_rules! write_float_poll {
    (varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_float_poll!(cfg(feature = "async_float_varint"), $poll_func, $poll_internal, $struct_buf);
    };
    (long_varint, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_float_poll!(cfg(feature = "async_float_varint_long"), $poll_func, $poll_internal, $struct_buf);
    };
    ($feature: meta, $poll_func: ident, $poll_internal: ident, $struct_buf: ident) => {
        write_signed_poll!($feature, $poll_func, $poll_internal, $struct_buf); // The same code.
    };
}
macro_rules! write_float_func {
    (varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_float_func!(cfg(feature = "async_float_varint"), $primitive, $func, $future, $struct_buf);
    };
    (long_varint, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_float_func!(cfg(feature = "async_float_varint_long"), $primitive, $func, $future, $struct_buf);
    };
    ($feature: meta, $primitive: ty, $func: ident, $future: ident, $struct_buf: ident) => {
        write_signed_func!($feature, $primitive, $func, $future, $struct_buf); // The same code.
    };
}
macro_rules! define_write_float_futures {
    () => {
        write_float_future!(varint, f32, WriteF32Varint, poll_write_f32_varint, InternalWriteF32Varint, InternalWriteU32Varint);
        write_float_future!(long_varint, f32, WriteF32Varint2Le, poll_write_f32_varint_2_le, InternalWriteF32Varint2Le, InternalWriteU32Varint2Le);
        write_float_future!(long_varint, f32, WriteF32Varint2Be, poll_write_f32_varint_2_be, InternalWriteF32Varint2Be, InternalWriteU32Varint2Be);
        write_float_future!(long_varint, f32, WriteF32Varint4Le, poll_write_f32_varint_4_le, InternalWriteF32Varint4Le, InternalWriteU32Varint4Le);
        write_float_future!(long_varint, f32, WriteF32Varint4Be, poll_write_f32_varint_4_be, InternalWriteF32Varint4Be, InternalWriteU32Varint4Be);

        write_float_future!(varint, f64, WriteF64Varint, poll_write_f64_varint, InternalWriteF64Varint, InternalWriteU64Varint);
        write_float_future!(long_varint, f64, WriteF64Varint2Le, poll_write_f64_varint_2_le, InternalWriteF64Varint2Le, InternalWriteU64Varint2Le);
        write_float_future!(long_varint, f64, WriteF64Varint2Be, poll_write_f64_varint_2_be, InternalWriteF64Varint2Be, InternalWriteU64Varint2Be);
        write_float_future!(long_varint, f64, WriteF64Varint4Le, poll_write_f64_varint_4_le, InternalWriteF64Varint4Le, InternalWriteU64Varint4Le);
        write_float_future!(long_varint, f64, WriteF64Varint4Be, poll_write_f64_varint_4_be, InternalWriteF64Varint4Be, InternalWriteU64Varint4Be);
        write_float_future!(long_varint, f64, WriteF64Varint8Le, poll_write_f64_varint_8_le, InternalWriteF64Varint8Le, InternalWriteU64Varint8Le);
        write_float_future!(long_varint, f64, WriteF64Varint8Be, poll_write_f64_varint_8_be, InternalWriteF64Varint8Be, InternalWriteU64Varint8Be);
    };
}
macro_rules! define_write_float_poll {
    () => {
        write_float_poll!(varint, poll_write_f32_varint, poll_write_u32_varint, InternalWriteF32Varint);
        write_float_poll!(long_varint, poll_write_f32_varint_2_le, poll_write_u32_varint_2_le, InternalWriteF32Varint2Le);
        write_float_poll!(long_varint, poll_write_f32_varint_2_be, poll_write_u32_varint_2_be, InternalWriteF32Varint2Be);
        write_float_poll!(long_varint, poll_write_f32_varint_4_le, poll_write_u32_varint_4_le, InternalWriteF32Varint4Le);
        write_float_poll!(long_varint, poll_write_f32_varint_4_be, poll_write_u32_varint_4_be, InternalWriteF32Varint4Be);

        write_float_poll!(varint, poll_write_f64_varint, poll_write_u64_varint, InternalWriteF64Varint);
        write_float_poll!(long_varint, poll_write_f64_varint_2_le, poll_write_u64_varint_2_le, InternalWriteF64Varint2Le);
        write_float_poll!(long_varint, poll_write_f64_varint_2_be, poll_write_u64_varint_2_be, InternalWriteF64Varint2Be);
        write_float_poll!(long_varint, poll_write_f64_varint_4_le, poll_write_u64_varint_4_le, InternalWriteF64Varint4Le);
        write_float_poll!(long_varint, poll_write_f64_varint_4_be, poll_write_u64_varint_4_be, InternalWriteF64Varint4Be);
        write_float_poll!(long_varint, poll_write_f64_varint_8_le, poll_write_u64_varint_8_le, InternalWriteF64Varint8Le);
        write_float_poll!(long_varint, poll_write_f64_varint_8_be, poll_write_u64_varint_8_be, InternalWriteF64Varint8Be);
    };
}
macro_rules! define_write_float_func {
    () => {
        write_float_func!(varint, f32, write_f32_varint, WriteF32Varint, InternalWriteF32Varint);
        write_float_func!(long_varint, f32, write_f32_varint_2_le, WriteF32Varint2Le, InternalWriteF32Varint2Le);
        write_float_func!(long_varint, f32, write_f32_varint_2_be, WriteF32Varint2Be, InternalWriteF32Varint2Be);
        write_float_func!(long_varint, f32, write_f32_varint_4_le, WriteF32Varint4Le, InternalWriteF32Varint4Le);
        write_float_func!(long_varint, f32, write_f32_varint_4_be, WriteF32Varint4Be, InternalWriteF32Varint4Be);

        write_float_func!(varint, f64, write_f64_varint, WriteF64Varint, InternalWriteF64Varint);
        write_float_func!(long_varint, f64, write_f64_varint_2_le, WriteF64Varint2Le, InternalWriteF64Varint2Le);
        write_float_func!(long_varint, f64, write_f64_varint_2_be, WriteF64Varint2Be, InternalWriteF64Varint2Be);
        write_float_func!(long_varint, f64, write_f64_varint_4_le, WriteF64Varint4Le, InternalWriteF64Varint4Le);
        write_float_func!(long_varint, f64, write_f64_varint_4_be, WriteF64Varint4Be, InternalWriteF64Varint4Be);
        write_float_func!(long_varint, f64, write_f64_varint_8_le, WriteF64Varint8Le, InternalWriteF64Varint8Le);
        write_float_func!(long_varint, f64, write_f64_varint_8_be, WriteF64Varint8Be, InternalWriteF64Varint8Be);
    };
}
define_write_float_futures!();
