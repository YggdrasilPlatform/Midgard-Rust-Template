///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Register Reload Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIF_A {
    ///0: No register reload
    NORELOAD = 0,
    ///1: Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)
    RELOAD = 1,
}
impl From<RRIF_A> for bool {
    #[inline(always)]
    fn from(variant: RRIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RRIF`
pub type RRIF_R = crate::R<bool, RRIF_A>;
impl RRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRIF_A {
        match self.bits {
            false => RRIF_A::NORELOAD,
            true => RRIF_A::RELOAD,
        }
    }
    ///Checks if the value of the field is `NORELOAD`
    #[inline(always)]
    pub fn is_no_reload(&self) -> bool {
        *self == RRIF_A::NORELOAD
    }
    ///Checks if the value of the field is `RELOAD`
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == RRIF_A::RELOAD
    }
}
///Transfer Error interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERRIF_A {
    ///0: No transfer error
    NOERROR = 0,
    ///1: Transfer error interrupt generated when a bus error occurs
    ERROR = 1,
}
impl From<TERRIF_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TERRIF`
pub type TERRIF_R = crate::R<bool, TERRIF_A>;
impl TERRIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TERRIF_A {
        match self.bits {
            false => TERRIF_A::NOERROR,
            true => TERRIF_A::ERROR,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == TERRIF_A::NOERROR
    }
    ///Checks if the value of the field is `ERROR`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == TERRIF_A::ERROR
    }
}
///FIFO Underrun Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUIF_A {
    ///0: No FIFO underrun
    NOUNDERRUN = 0,
    ///1: FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO
    UNDERRUN = 1,
}
impl From<FUIF_A> for bool {
    #[inline(always)]
    fn from(variant: FUIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FUIF`
pub type FUIF_R = crate::R<bool, FUIF_A>;
impl FUIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FUIF_A {
        match self.bits {
            false => FUIF_A::NOUNDERRUN,
            true => FUIF_A::UNDERRUN,
        }
    }
    ///Checks if the value of the field is `NOUNDERRUN`
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == FUIF_A::NOUNDERRUN
    }
    ///Checks if the value of the field is `UNDERRUN`
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == FUIF_A::UNDERRUN
    }
}
///Line Interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIF_A {
    ///0: Programmed line not reached
    NOTREACHED = 0,
    ///1: Line interrupt generated when a programmed line is reached
    REACHED = 1,
}
impl From<LIF_A> for bool {
    #[inline(always)]
    fn from(variant: LIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LIF`
pub type LIF_R = crate::R<bool, LIF_A>;
impl LIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LIF_A {
        match self.bits {
            false => LIF_A::NOTREACHED,
            true => LIF_A::REACHED,
        }
    }
    ///Checks if the value of the field is `NOTREACHED`
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == LIF_A::NOTREACHED
    }
    ///Checks if the value of the field is `REACHED`
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == LIF_A::REACHED
    }
}
impl R {
    ///Bit 3 - Register Reload Interrupt Flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Transfer Error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Line Interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 0x01) != 0)
    }
}
