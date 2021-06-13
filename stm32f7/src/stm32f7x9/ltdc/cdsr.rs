///Reader of register CDSR
pub type R = crate::R<u32, super::CDSR>;
///Horizontal Synchronization display Status
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSYNCS_A {
    ///0: Currently not in HSYNC phase
    NOTACTIVE = 0,
    ///1: Currently in HSYNC phase
    ACTIVE = 1,
}
impl From<HSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: HSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HSYNCS`
pub type HSYNCS_R = crate::R<bool, HSYNCS_A>;
impl HSYNCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSYNCS_A {
        match self.bits {
            false => HSYNCS_A::NOTACTIVE,
            true => HSYNCS_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `NOTACTIVE`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HSYNCS_A::NOTACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HSYNCS_A::ACTIVE
    }
}
///Vertical Synchronization display Status
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSYNCS_A {
    ///0: Currently not in VSYNC phase
    NOTACTIVE = 0,
    ///1: Currently in VSYNC phase
    ACTIVE = 1,
}
impl From<VSYNCS_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNCS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VSYNCS`
pub type VSYNCS_R = crate::R<bool, VSYNCS_A>;
impl VSYNCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSYNCS_A {
        match self.bits {
            false => VSYNCS_A::NOTACTIVE,
            true => VSYNCS_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `NOTACTIVE`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VSYNCS_A::NOTACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VSYNCS_A::ACTIVE
    }
}
///Horizontal Data Enable display Status
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDES_A {
    ///0: Currently not in horizontal Data Enable phase
    NOTACTIVE = 0,
    ///1: Currently in horizontal Data Enable phase
    ACTIVE = 1,
}
impl From<HDES_A> for bool {
    #[inline(always)]
    fn from(variant: HDES_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HDES`
pub type HDES_R = crate::R<bool, HDES_A>;
impl HDES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDES_A {
        match self.bits {
            false => HDES_A::NOTACTIVE,
            true => HDES_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `NOTACTIVE`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == HDES_A::NOTACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == HDES_A::ACTIVE
    }
}
///Vertical Data Enable display Status
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDES_A {
    ///0: Currently not in vertical Data Enable phase
    NOTACTIVE = 0,
    ///1: Currently in vertical Data Enable phase
    ACTIVE = 1,
}
impl From<VDES_A> for bool {
    #[inline(always)]
    fn from(variant: VDES_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `VDES`
pub type VDES_R = crate::R<bool, VDES_A>;
impl VDES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VDES_A {
        match self.bits {
            false => VDES_A::NOTACTIVE,
            true => VDES_A::ACTIVE,
        }
    }
    ///Checks if the value of the field is `NOTACTIVE`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == VDES_A::NOTACTIVE
    }
    ///Checks if the value of the field is `ACTIVE`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VDES_A::ACTIVE
    }
}
impl R {
    ///Bit 3 - Horizontal Synchronization display Status
    #[inline(always)]
    pub fn hsyncs(&self) -> HSYNCS_R {
        HSYNCS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Vertical Synchronization display Status
    #[inline(always)]
    pub fn vsyncs(&self) -> VSYNCS_R {
        VSYNCS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Horizontal Data Enable display Status
    #[inline(always)]
    pub fn hdes(&self) -> HDES_R {
        HDES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Vertical Data Enable display Status
    #[inline(always)]
    pub fn vdes(&self) -> VDES_R {
        VDES_R::new((self.bits & 0x01) != 0)
    }
}
