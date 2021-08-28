cfg_if::cfg_if! {
    if #[cfg(feature = "online")] {
        mod api;
        mod lang;
        pub use lang::Lang;
        mod online;
        pub use online::translate;
    } else if #[cfg(feature = "local")] {
        mod local;
        pub use local::{translate, Lang};
    }
}
