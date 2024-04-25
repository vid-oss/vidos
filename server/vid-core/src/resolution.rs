use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display(fmt = "{width}x{height} px")]
pub struct Resolution {
    width: u32,
    height: u32,
}
