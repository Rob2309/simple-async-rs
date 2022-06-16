#[macro_export]
macro_rules! async_main {
    ($($code:tt)*) => {
        fn main() {
            let mut executor = $crate::executor::Executor::new();

            executor.spawn(async {
                $($code)*
            });

            executor.run();
        }
    };
}
