#[rustfmt::skip]
pub(super) mod map {
    pub const GPIO_OFFSET:                              usize =        0x0020_0000;
    pub const UART_OFFSET:                              usize =        0x0020_1000;

    /// Physical devices.
    #[cfg(feature = "bsp_rpi3")]
    pub mod mmio {
        use super::*;

        pub const BASE:                                 usize =        0x3F00_0000;
        pub const GPIO_BASE:                            usize = BASE + GPIO_OFFSET;
        pub const PL011_UART_BASE:                      usize = BASE + UART_OFFSET;
    }

    /// Physical devices.
    #[cfg(feature = "bsp_rpi4")]
    pub mod mmio {
        use super::*;

        pub const BASE:                                 usize =        0xFE00_0000;
        pub const GPIO_BASE:                            usize = BASE + GPIO_OFFSET;
        pub const PL011_UART_BASE:                      usize = BASE + UART_OFFSET;
    }
}
