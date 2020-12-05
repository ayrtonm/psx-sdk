use super::Printer;
use crate::gpu::Color;
use crate::gpu::Vertex;
use crate::mmio::{dma, gpu};

impl<const N: usize> Default for UnsafePrinter<N> {
    fn default() -> Self {
        UnsafePrinter::new((0, 0), (8, 16), (0, 0), (320, 240), None)
    }
}

pub struct UnsafePrinter<const N: usize> {
    printer: Printer<N>,
    gpu_dma: dma::gpu::Channel,
    gp0: gpu::GP0,
    gp1: gpu::GP1,
}

impl<const N: usize> UnsafePrinter<N> {
    pub fn new<T, U, V, S>(
        cursor: T, font_size: U, box_offset: V, box_size: S, color: Option<Color>,
    ) -> Self
    where Vertex: From<T> + From<U> + From<V> + From<S> {
        unsafe {
            UnsafePrinter {
                printer: Printer::<N>::new(
                    cursor,
                    font_size,
                    box_offset,
                    box_size,
                    color,
                    &mut dma::otc::Channel::new(),
                ),
                gpu_dma: dma::gpu::Channel::new(),
                gp0: gpu::GP0::new(),
                gp1: gpu::GP1::new(),
            }
        }
    }

    pub fn load_font(&mut self) {
        self.printer.load_font(&mut self.gp1, &mut self.gpu_dma)
    }

    pub fn print<'a, M>(&mut self, msg: M)
    where M: IntoIterator<Item = &'a u8> {
        self.printer
            .print(msg, &mut self.gp0, &mut self.gp1, &mut self.gpu_dma)
    }
}