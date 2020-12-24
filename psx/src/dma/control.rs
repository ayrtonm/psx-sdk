use super::Channel;

use crate::mmio::Address;
use crate::value;
use crate::value::LoadMut;

/// [DMA Control](http://problemkaputt.de/psx-spx.htm#dmachannels) register at `0x1F80_10F0`.
/// Used to enable DMA channels and set priorities.
pub struct Control;
/// A [`value::Value`] alias for [`Control`].
pub type Value<'r> = value::Value<'r, u32, Control>;
/// A [`value::MutValue`] alias for [`Control`].
pub type MutValue<'r> = value::MutValue<'r, u32, Control>;

impl Address<u32> for Control {
    const ADDRESS: u32 = 0x1F80_10F0;
}
impl LoadMut<u32> for Control {}

impl Control {
    #[inline(always)]
    const fn enable_bit(ch: Channel) -> u32 {
        let bit = (ch as u32 * 4) + 3;
        1 << bit
    }

    const enable_bits: u32 = {
        Self::enable_bit(Channel::MDECin) |
            Self::enable_bit(Channel::MDECout) |
            Self::enable_bit(Channel::GPU) |
            Self::enable_bit(Channel::CDROM) |
            Self::enable_bit(Channel::SPU) |
            Self::enable_bit(Channel::PIO) |
            Self::enable_bit(Channel::OTC)
    };
}

impl Value<'_> {
    /// Checks if the given DMA channel is enabled.
    #[inline(always)]
    pub fn enabled(&self, ch: Channel) -> bool {
        self.contains(Control::enable_bit(ch))
    }
}

impl MutValue<'_> {
    /// Enables the given DMA channel.
    #[inline(always)]
    pub fn enable(self, ch: Channel) -> Self {
        self.set(Control::enable_bit(ch))
    }

    /// Disables the given DMA channel.
    #[inline(always)]
    pub fn disable(self, ch: Channel) -> Self {
        self.clear(Control::enable_bit(ch))
    }

    /// Enables all DMA channels.
    #[inline(always)]
    pub fn enable_all(self) -> Self {
        self.set(Control::enable_bits)
    }

    /// Disables all DMA channels.
    #[inline(always)]
    pub fn disable_all(self) -> Self {
        self.clear(Control::enable_bits)
    }
}
