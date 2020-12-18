use super::{ALL_IRQS, IRQ};
use crate::mmio::irq;
use crate::mmio::register::{Read, Update, Write};

impl irq::Mask {
    /// Returns the [`IRQ`]\(s\) enabled by [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enabled(&self) -> impl Iterator<Item = IRQ> {
        let val = unsafe { self.read() };
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (val & (1 << irq as u32) != 0).then_some(irq))
    }

    /// Returns the [`IRQ`]\(s\) disabled by
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disabled(&self) -> impl Iterator<Item = IRQ> {
        let val = unsafe { self.read() };
        ALL_IRQS
            .iter()
            .filter_map(move |&irq| (val & (1 << irq as u32) == 0).then_some(irq))
    }

    /// Enables the given [`IRQ`]\(s\) by setting the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enable<I>(&mut self, interrupts: I) -> &mut Self
    where I: IntoIterator<Item = IRQ> {
        unsafe {
            self.update(|val| {
                interrupts
                    .into_iter()
                    .fold(val, |val, irq| val | (1 << irq as u32))
            })
        }
        self
    }

    /// Enables all [`IRQ`]s by setting the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn enable_all(&mut self) -> &mut Self {
        unsafe { self.write(0x0000_07FF) }
        self
    }

    /// Disables the given [`IRQ`]\(s\) by clearing the corresponding bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disable<I>(&mut self, interrupts: I) -> &mut Self
    where I: IntoIterator<Item = IRQ> {
        unsafe {
            self.update(|val| {
                interrupts
                    .into_iter()
                    .fold(val, |val, irq| val & !(1 << irq as u32))
            })
        }
        self
    }

    /// Disables all [`IRQ`]s by clearing the relevant bits of
    /// [I_MASK](http://problemkaputt.de/psx-spx.htm#interrupts).
    pub fn disable_all(&mut self) -> &mut Self {
        unsafe { self.write(0x0000_0000) }
        self
    }
}
