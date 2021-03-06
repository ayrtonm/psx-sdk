use super::{kernel, RootCounter};
use crate::bios::fs::{File, MemCard};
use crate::bios::thread::Thread;
use crate::interrupt::IRQ;
use crate::timer;

// TODO: Complete this list and make names clearer than nocash defaults
pub enum Class<'a> {
    FileDescriptor(&'a File<'a, MemCard>),
    IRQ(IRQ),
    Exception,
    // There are two missing memory card event classes
    MemCard,
    RootCounter(RootCounter),
    Thread(&'a Thread),
}

impl<'a> From<Class<'a>> for u32 {
    fn from(cl: Class) -> u32 {
        match cl {
            Class::FileDescriptor(file) => file.fd().into(),
            Class::IRQ(irq) => match irq {
                IRQ::Vblank => 0xF0000001,
                IRQ::GPU => 0xF0000002,
                IRQ::CDROM => 0xF0000003,
                IRQ::DMA => 0xF0000004,
                IRQ::Timer0 => 0xF0000005,
                IRQ::Timer1 => 0xF0000006,
                // This isn't a typoe, timer 1 and 2's IRQs use the same event
                IRQ::Timer2 => 0xF0000006,
                IRQ::ControllerMemoryCard => 0xF0000008,
                IRQ::SPU => 0xF0000009,
                IRQ::ControllerPIO => 0xF000000A,
                IRQ::SIO => 0xF000000B,
            },
            Class::Exception => 0xF0000010,
            Class::MemCard => 0xF0000011,
            Class::RootCounter(rc) => match rc {
                RootCounter::Timer(timer) => match timer {
                    timer::Name::DotClock => 0xF200_0000,
                    timer::Name::Hblank => 0xF200_0001,
                    timer::Name::Fractional => 0xF200_0002,
                },
                RootCounter::Vblank => 0xF200_0003,
            },
            Class::Thread(thread) => 0xFF00_0000 | thread.handle(),
        }
    }
}

pub enum Spec {
    Zero = 0x0001,
    Interrupted = 0x0002,
    EOF = 0x0004,
    FileClosed = 0x0008,
    CmdAck = 0x0010,
    CmdDone = 0x0020,
    DataReady = 0x0040,
    DataEnd = 0x0080,
    TimeOut = 0x0100,
    UnknownCmd = 0x0200,
    EndOfReadBuffer = 0x0400,
    EndOfWriteBuffer = 0x0800,
    Interrupt = 0x1000,
    NewDevice = 0x2000,
    SysCall = 0x4000,
    Error = 0x8000,
    PreviousWriteError = 0x8001,
    DomainError = 0x0301,
    RangeError = 0x0302,
}

pub struct Event {
    ev: u32,
}

impl Event {
    pub fn open(class: Class, spec: Spec, callback: Option<fn()>) -> Option<Self> {
        let (mode, func) = match callback {
            Some(func) => (0x1000, func as *const u32),
            None => (0x2000, 0 as *const u32),
        };
        let ev = unsafe { kernel::open_event(class.into(), spec as u16, mode, func) };
        match ev {
            0xFFFF_FFFF => None,
            _ => Some(Self { ev }),
        }
    }

    pub fn close(self) {
        // Drop impl takes care of dropping the event
    }

    pub fn wait(&self) -> bool {
        unsafe { kernel::wait_event(self.ev) }
    }

    pub fn test(&self) -> bool {
        unsafe { kernel::test_event(self.ev) }
    }

    pub fn enable(&mut self) {
        unsafe { kernel::enable_event(self.ev) }
    }

    pub fn disable(&mut self) {
        unsafe { kernel::disable_event(self.ev) }
    }

    pub fn deliver(class: Class, spec: Spec) {
        unsafe { kernel::deliver_event(class.into(), spec as u16) }
    }

    pub fn undeliver(class: Class, spec: Spec) {
        unsafe { kernel::undeliver_event(class.into(), spec as u16) }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { kernel::close_event(self.ev) }
    }
}
