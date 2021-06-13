///Reader of register MACFFR
pub type R = crate::R<u32, super::MACFFR>;
///Writer for register MACFFR
pub type W = crate::W<u32, super::MACFFR>;
///Register MACFFR `reset()`'s with value 0
impl crate::ResetValue for super::MACFFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Promiscuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    ///0: Normal address filtering
    DISABLED = 0,
    ///1: Address filters pass all incoming frames regardless of their destination or source address
    ENABLED = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PM`
pub type PM_R = crate::R<bool, PM_A>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::DISABLED,
            true => PM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PM_A::ENABLED
    }
}
///Write proxy for field `PM`
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal address filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PM_A::DISABLED)
    }
    ///Address filters pass all incoming frames regardless of their destination or source address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PM_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Hash unicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HU_A {
    ///0: MAC performs a perfect destination address filtering for unicast frames
    PERFECT = 0,
    ///1: MAC performs destination address filtering of received unicast frames according to the hash table
    HASH = 1,
}
impl From<HU_A> for bool {
    #[inline(always)]
    fn from(variant: HU_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HU`
pub type HU_R = crate::R<bool, HU_A>;
impl HU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HU_A {
        match self.bits {
            false => HU_A::PERFECT,
            true => HU_A::HASH,
        }
    }
    ///Checks if the value of the field is `PERFECT`
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HU_A::PERFECT
    }
    ///Checks if the value of the field is `HASH`
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HU_A::HASH
    }
}
///Write proxy for field `HU`
pub struct HU_W<'a> {
    w: &'a mut W,
}
impl<'a> HU_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC performs a perfect destination address filtering for unicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HU_A::PERFECT)
    }
    ///MAC performs destination address filtering of received unicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HU_A::HASH)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Hash multicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HM_A {
    ///0: MAC performs a perfect destination address filtering for multicast frames
    PERFECT = 0,
    ///1: MAC performs destination address filtering of received multicast frames according to the hash table
    HASH = 1,
}
impl From<HM_A> for bool {
    #[inline(always)]
    fn from(variant: HM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HM`
pub type HM_R = crate::R<bool, HM_A>;
impl HM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HM_A {
        match self.bits {
            false => HM_A::PERFECT,
            true => HM_A::HASH,
        }
    }
    ///Checks if the value of the field is `PERFECT`
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HM_A::PERFECT
    }
    ///Checks if the value of the field is `HASH`
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HM_A::HASH
    }
}
///Write proxy for field `HM`
pub struct HM_W<'a> {
    w: &'a mut W,
}
impl<'a> HM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC performs a perfect destination address filtering for multicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut W {
        self.variant(HM_A::PERFECT)
    }
    ///MAC performs destination address filtering of received multicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut W {
        self.variant(HM_A::HASH)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Destination address unique filtering
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAIF_A {
    ///0: Normal filtering of frames
    NORMAL = 0,
    ///1: Address check block operates in inverse filtering mode for the DA address comparison
    INVERT = 1,
}
impl From<DAIF_A> for bool {
    #[inline(always)]
    fn from(variant: DAIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DAIF`
pub type DAIF_R = crate::R<bool, DAIF_A>;
impl DAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DAIF_A {
        match self.bits {
            false => DAIF_A::NORMAL,
            true => DAIF_A::INVERT,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAIF_A::NORMAL
    }
    ///Checks if the value of the field is `INVERT`
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == DAIF_A::INVERT
    }
}
///Write proxy for field `DAIF`
pub struct DAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DAIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal filtering of frames
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(DAIF_A::NORMAL)
    }
    ///Address check block operates in inverse filtering mode for the DA address comparison
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(DAIF_A::INVERT)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Pass all multicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAM_A {
    ///0: Filtering of multicast frames depends on HM
    DISABLED = 0,
    ///1: All received frames with a multicast destination address are passed
    ENABLED = 1,
}
impl From<PAM_A> for bool {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PAM`
pub type PAM_R = crate::R<bool, PAM_A>;
impl PAM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PAM_A {
        match self.bits {
            false => PAM_A::DISABLED,
            true => PAM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAM_A::ENABLED
    }
}
///Write proxy for field `PAM`
pub struct PAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PAM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Filtering of multicast frames depends on HM
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAM_A::DISABLED)
    }
    ///All received frames with a multicast destination address are passed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAM_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Broadcast frames disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFD_A {
    ///0: Address filters pass all received broadcast frames
    ENABLED = 0,
    ///1: Address filters filter all incoming broadcast frames
    DISABLED = 1,
}
impl From<BFD_A> for bool {
    #[inline(always)]
    fn from(variant: BFD_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `BFD`
pub type BFD_R = crate::R<bool, BFD_A>;
impl BFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BFD_A {
        match self.bits {
            false => BFD_A::ENABLED,
            true => BFD_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFD_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFD_A::DISABLED
    }
}
///Write proxy for field `BFD`
pub struct BFD_W<'a> {
    w: &'a mut W,
}
impl<'a> BFD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Address filters pass all received broadcast frames
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BFD_A::ENABLED)
    }
    ///Address filters filter all incoming broadcast frames
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BFD_A::DISABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Pass control frames
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCF_A {
    ///0: MAC prevents all control frames from reaching the application
    PREVENTALL = 0,
    ///1: MAC forwards all control frames to application except Pause
    FORWARDALLEXCEPTPAUSE = 1,
    ///2: MAC forwards all control frames to application even if they fail the address filter
    FORWARDALL = 2,
    ///3: MAC forwards control frames that pass the address filter
    FORWARDALLFILTERED = 3,
}
impl From<PCF_A> for u8 {
    #[inline(always)]
    fn from(variant: PCF_A) -> Self {
        variant as _
    }
}
///Reader of field `PCF`
pub type PCF_R = crate::R<u8, PCF_A>;
impl PCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCF_A {
        match self.bits {
            0 => PCF_A::PREVENTALL,
            1 => PCF_A::FORWARDALLEXCEPTPAUSE,
            2 => PCF_A::FORWARDALL,
            3 => PCF_A::FORWARDALLFILTERED,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `PREVENTALL`
    #[inline(always)]
    pub fn is_prevent_all(&self) -> bool {
        *self == PCF_A::PREVENTALL
    }
    ///Checks if the value of the field is `FORWARDALLEXCEPTPAUSE`
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCF_A::FORWARDALLEXCEPTPAUSE
    }
    ///Checks if the value of the field is `FORWARDALL`
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        *self == PCF_A::FORWARDALL
    }
    ///Checks if the value of the field is `FORWARDALLFILTERED`
    #[inline(always)]
    pub fn is_forward_all_filtered(&self) -> bool {
        *self == PCF_A::FORWARDALLFILTERED
    }
}
///Write proxy for field `PCF`
pub struct PCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///MAC prevents all control frames from reaching the application
    #[inline(always)]
    pub fn prevent_all(self) -> &'a mut W {
        self.variant(PCF_A::PREVENTALL)
    }
    ///MAC forwards all control frames to application except Pause
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALLEXCEPTPAUSE)
    }
    ///MAC forwards all control frames to application even if they fail the address filter
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALL)
    }
    ///MAC forwards control frames that pass the address filter
    #[inline(always)]
    pub fn forward_all_filtered(self) -> &'a mut W {
        self.variant(PCF_A::FORWARDALLFILTERED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
///Source address inverse filtering
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAIF_A {
    ///0: Source address filter operates normally
    NORMAL = 0,
    ///1: Source address filter operation inverted
    INVERT = 1,
}
impl From<SAIF_A> for bool {
    #[inline(always)]
    fn from(variant: SAIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SAIF`
pub type SAIF_R = crate::R<bool, SAIF_A>;
impl SAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAIF_A {
        match self.bits {
            false => SAIF_A::NORMAL,
            true => SAIF_A::INVERT,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SAIF_A::NORMAL
    }
    ///Checks if the value of the field is `INVERT`
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SAIF_A::INVERT
    }
}
///Write proxy for field `SAIF`
pub struct SAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Source address filter operates normally
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SAIF_A::NORMAL)
    }
    ///Source address filter operation inverted
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(SAIF_A::INVERT)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Source address filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAF_A {
    ///0: Source address ignored
    DISABLED = 0,
    ///1: MAC drops frames that fail the source address filter
    ENABLED = 1,
}
impl From<SAF_A> for bool {
    #[inline(always)]
    fn from(variant: SAF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SAF`
pub type SAF_R = crate::R<bool, SAF_A>;
impl SAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAF_A {
        match self.bits {
            false => SAF_A::DISABLED,
            true => SAF_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAF_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAF_A::ENABLED
    }
}
///Write proxy for field `SAF`
pub struct SAF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SAF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Source address ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAF_A::DISABLED)
    }
    ///MAC drops frames that fail the source address filter
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAF_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Hash or perfect filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPF_A {
    ///0: If HM or HU is set, only frames that match the Hash filter are passed
    HASHONLY = 0,
    ///1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    HASHORPERFECT = 1,
}
impl From<HPF_A> for bool {
    #[inline(always)]
    fn from(variant: HPF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `HPF`
pub type HPF_R = crate::R<bool, HPF_A>;
impl HPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HPF_A {
        match self.bits {
            false => HPF_A::HASHONLY,
            true => HPF_A::HASHORPERFECT,
        }
    }
    ///Checks if the value of the field is `HASHONLY`
    #[inline(always)]
    pub fn is_hash_only(&self) -> bool {
        *self == HPF_A::HASHONLY
    }
    ///Checks if the value of the field is `HASHORPERFECT`
    #[inline(always)]
    pub fn is_hash_or_perfect(&self) -> bool {
        *self == HPF_A::HASHORPERFECT
    }
}
///Write proxy for field `HPF`
pub struct HPF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///If HM or HU is set, only frames that match the Hash filter are passed
    #[inline(always)]
    pub fn hash_only(self) -> &'a mut W {
        self.variant(HPF_A::HASHONLY)
    }
    ///If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    #[inline(always)]
    pub fn hash_or_perfect(self) -> &'a mut W {
        self.variant(HPF_A::HASHORPERFECT)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Receive all
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RA_A {
    ///0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    DISABLED = 0,
    ///1: MAC receiver passes oll received frames on to the application
    ENABLED = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RA`
pub type RA_R = crate::R<bool, RA_A>;
impl RA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::DISABLED,
            true => RA_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RA_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RA_A::ENABLED
    }
}
///Write proxy for field `RA`
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RA_A::DISABLED)
    }
    ///MAC receiver passes oll received frames on to the application
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RA_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bit 7 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Source address filter
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W { w: self }
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W {
        HM_W { w: self }
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W { w: self }
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W { w: self }
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W {
        BFD_W { w: self }
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W { w: self }
    }
    ///Bit 7 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W { w: self }
    }
    ///Bit 8 - Source address filter
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W { w: self }
    }
    ///Bit 9 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W {
        HPF_W { w: self }
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
}
