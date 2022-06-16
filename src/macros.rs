#[macro_export]
macro_rules! async_main {
    ($($code:tt)*) => {
        fn main() {
            let executor = $crate::executor::Executor::new();

            executor.spawn(async {
                $($code)*
            });

            executor.run();
        }
    };
}
