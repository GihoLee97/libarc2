/// Convert a value into a `Vec<u32>`.
pub trait ToU32s {

    /// Convert an object, typically a register, into a serialisable
    /// vector of u32s. These are the values that are actually written
    /// to the ArC2 buffer.
    fn as_u32s(&self) -> Vec<u32>;
}


pub mod opcode {

    use num_derive::{FromPrimitive, ToPrimitive};
    use super::ToU32s;

    /// Opcodes designate an ArC2 operation
    ///
    /// An [`OpCode`] is typically the first register in an ArC2 instruction
    /// and may be followed by a series of arguments.
    #[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
    #[repr(u32)]
    pub enum OpCode {
        /// Set a DAC configuration.
        SetDAC         = 0x00000001,
        /// Enable a DAC configuration previously set with [`OpCode::SetDAC`].
        UpdateDAC      = 0x00000002,
        /// Read Current operation.
        CurrentRead    = 0x00000004,
        /// Read voltage operation.
        VoltageRead    = 0x00000008,
        /// Set selector.
        UpdateSelector = 0x00000010,
        /// Set logic levels.
        UpdateLogic    = 0x00000020,
        /// Update channel configuration.
        UpdateChannel  = 0x00000040,
        /// Clear instrument buffer.
        Clear          = 0x00000080,
        /// Configure for high speed pulse operation.
        HSPulseConfig  = 0x00000100,
        /// Initiate a high speed pulse operation.
        HSPulseStart   = 0x00000200,
        /// Modify channel configuration.
        ModifyChannel  = 0x00000400,
        /// Set DAC Offsets (currently nop).
        SetDACOffset   = 0x00001000
    }

    impl ToU32s for OpCode {
        fn as_u32s(&self) -> Vec<u32> {
            [*self as u32].to_vec()
        }
    }

}


pub mod empty {

    use super::ToU32s;

    /// An empty register, typically used to pad instructions
    /// to full length.
    ///
    /// An [`Empty`] should never need to be invoked manually
    /// as empty instructions are added appropriately
    /// when an instruction is compiled.
    pub struct Empty(u32);

    impl Empty {
        /// Create a new empty register
        pub fn new() -> Empty {
            Empty(0x0)
        }
    }

    impl ToU32s for Empty {
        fn as_u32s(&self) -> Vec<u32> {
            [self.0].to_vec()
        }
    }

}

pub mod terminate {

    use super::ToU32s;

    /// Register used to terminate instructions.
    ///
    /// As with [`Empty`][`super::empty::Empty`] this should not need
    /// to be invoked manually as it is typically compiled into
    /// an instruction.
    pub struct Terminate(u32);

    impl Terminate {
        pub fn new() -> Terminate {
            Terminate(0x80008000)
        }
    }

    impl ToU32s for Terminate {
        fn as_u32s(&self) -> Vec<u32> {
            [self.0].to_vec()
        }
    }


}

pub mod dacmask {
    use super::ToU32s;
    use bitflags::bitflags;

    bitflags! {
        /// DAC channel selection register.
        ///
        /// [`DACMask`] is used to create a suitable bitmask for a given
        /// selection of channels. Output channels in ArC2 are organised
        /// in 8 clusters and there is also an additional auxilliary DAC
        /// used for internal configuration. Each cluster contains 8
        /// possible channels that can be toggled up to a total of 64.
        /// This is usually paired with the
        /// [`OpCode::UpdateDAC`][`super::opcode::OpCode::UpdateDAC`]
        /// opcode to set the voltages of a channel. All the DACs are
        /// organised in halves so if one wants to address channel
        /// 3 (zero-indexed) would have to toggle the bits that correspond
        /// to DAC0; 1st half ([`DACMask::CH00_03`]), whereas for channel 29
        /// that would have to be DAC3; 2nd half ([`DACMask::CH28_31`]).
        ///
        /// Although one can create a suitable bitmask manually this
        /// implementation provides some convenient functions that can
        /// be used instead of calculating which of 16 halves corresponds
        /// to a given channel. This is abstracted away under functions
        /// [`set_channel`][`DACMask::set_channel`] and
        /// [`unset_channel`][`DACMask::unset_channel`].
        ///
        /// ## Example
        /// ```
        /// use libarc2::register::DACMask;
        ///
        /// // Create a new DAC bitmask
        /// let mut clusters = DACMask::NONE;
        ///
        /// clusters.set_channels(&[2, 3, 50, 61]);
        ///
        /// assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH48_51 |
        ///     DACMask::CH60_63);
        /// assert_eq!(clusters.as_u32(), 0x00009001);
        /// clusters.set_channel(12);
        /// assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH12_15 |
        ///     DACMask::CH48_51 | DACMask::CH60_63);
        /// assert_eq!(clusters.as_u32(), 0x00009009);
        ///
        /// clusters.unset_channel(61);
        /// assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH12_15 |
        ///     DACMask::CH48_51);
        /// assert_eq!(clusters.as_u32(), 0x00001009);
        ///
        /// clusters.clear();
        /// assert_eq!(clusters, DACMask::NONE);
        /// assert_eq!(clusters.as_u32(), 0x0);
        /// ```
        pub struct DACMask: u32 {
            /// No Flags; invalid state
            const NONE    = 0b00000000000000000000000000000000;
            /// DAC0; first half
            const CH00_03 = 0b00000000000000000000000000000001;
            /// DAC0; second half
            const CH04_07 = 0b00000000000000000000000000000010;
            /// DAC1; first half
            const CH08_11 = 0b00000000000000000000000000000100;
            /// DAC1; second half
            const CH12_15 = 0b00000000000000000000000000001000;
            /// DAC2; first half
            const CH16_19 = 0b00000000000000000000000000010000;
            /// DAC2; second half
            const CH20_23 = 0b00000000000000000000000000100000;
            /// DAC3; first half
            const CH24_27 = 0b00000000000000000000000001000000;
            /// DAC3; second half
            const CH28_31 = 0b00000000000000000000000010000000;
            /// DAC4; first half
            const CH32_35 = 0b00000000000000000000000100000000;
            /// DAC4; second half
            const CH36_39 = 0b00000000000000000000001000000000;
            /// DAC5; first half
            const CH40_43 = 0b00000000000000000000010000000000;
            /// DAC5; second half
            const CH44_47 = 0b00000000000000000000100000000000;
            /// DAC6; first half
            const CH48_51 = 0b00000000000000000001000000000000;
            /// DAC6; second half
            const CH52_55 = 0b00000000000000000010000000000000;
            /// DAC7; first half
            const CH56_59 = 0b00000000000000000100000000000000;
            /// DAC7; second half
            const CH60_63 = 0b00000000000000001000000000000000;
            /// AUX DAC0
            const AUX0    = 0b00000000000000010000000000000000;
            /// AUX DAC1
            const AUX1    = 0b00000000000000100000000000000000;
            /// All channels of DAC0
            const DAC0    = Self::CH00_03.bits | Self::CH04_07.bits;
            /// All channels of DAC1
            const DAC1    = Self::CH08_11.bits | Self::CH12_15.bits;
            /// All channels of DAC2
            const DAC2    = Self::CH16_19.bits | Self::CH20_23.bits;
            /// All channels of DAC3
            const DAC3    = Self::CH24_27.bits | Self::CH28_31.bits;
            /// All channels of DAC4
            const DAC4    = Self::CH32_35.bits | Self::CH36_39.bits;
            /// All channels of DAC5
            const DAC5    = Self::CH40_43.bits | Self::CH44_47.bits;
            /// All channels of DAC6
            const DAC6    = Self::CH48_51.bits | Self::CH52_55.bits;
            /// All channels of DAC7
            const DAC7    = Self::CH56_59.bits | Self::CH60_63.bits;
            /// All channels
            const ALL = Self::CH00_03.bits | Self::CH04_07.bits |
                        Self::CH08_11.bits | Self::CH12_15.bits |
                        Self::CH16_19.bits | Self::CH20_23.bits |
                        Self::CH24_27.bits | Self::CH28_31.bits |
                        Self::CH32_35.bits | Self::CH36_39.bits |
                        Self::CH40_43.bits | Self::CH44_47.bits |
                        Self::CH48_51.bits | Self::CH52_55.bits |
                        Self::CH56_59.bits | Self::CH60_63.bits;
        }
    }

    const CHANMAP: [DACMask; 64] = [
        DACMask::CH00_03, DACMask::CH00_03, DACMask::CH00_03, DACMask::CH00_03,
        DACMask::CH04_07, DACMask::CH04_07, DACMask::CH04_07, DACMask::CH04_07,
        DACMask::CH08_11, DACMask::CH08_11, DACMask::CH08_11, DACMask::CH08_11,
        DACMask::CH12_15, DACMask::CH12_15, DACMask::CH12_15, DACMask::CH12_15,
        DACMask::CH16_19, DACMask::CH16_19, DACMask::CH16_19, DACMask::CH16_19,
        DACMask::CH20_23, DACMask::CH20_23, DACMask::CH20_23, DACMask::CH20_23,
        DACMask::CH24_27, DACMask::CH24_27, DACMask::CH24_27, DACMask::CH24_27,
        DACMask::CH28_31, DACMask::CH28_31, DACMask::CH28_31, DACMask::CH28_31,
        DACMask::CH32_35, DACMask::CH32_35, DACMask::CH32_35, DACMask::CH32_35,
        DACMask::CH36_39, DACMask::CH36_39, DACMask::CH36_39, DACMask::CH36_39,
        DACMask::CH40_43, DACMask::CH40_43, DACMask::CH40_43, DACMask::CH40_43,
        DACMask::CH44_47, DACMask::CH44_47, DACMask::CH44_47, DACMask::CH44_47,
        DACMask::CH48_51, DACMask::CH48_51, DACMask::CH48_51, DACMask::CH48_51,
        DACMask::CH52_55, DACMask::CH52_55, DACMask::CH52_55, DACMask::CH52_55,
        DACMask::CH56_59, DACMask::CH56_59, DACMask::CH56_59, DACMask::CH56_59,
        DACMask::CH60_63, DACMask::CH60_63, DACMask::CH60_63, DACMask::CH60_63
    ];

    impl DACMask {

        /// Enable the specified channel.
        pub fn set_channel(&mut self, chan: usize) {
            self.set_channels(&[chan]);
        }

        /// Disable the specified channel.
        pub fn unset_channel(&mut self, chan: usize) {
            self.unset_channels(&[chan]);
        }

        /// Enable the specified channels.
        pub fn set_channels(&mut self, chans: &[usize]) {
            for c in chans {
                *self |= CHANMAP[*c];
            }
        }

        /// Disable the specified channels.
        pub fn unset_channels(&mut self, chans: &[usize]) {
            for c in chans {
                *self &= !CHANMAP[*c];
            }
        }

        /// Clear all channels. This is effectively [`DACMask::NONE`].
        pub fn clear(&mut self) {
            self.bits = 0;
        }

        /// Get the representation of this bitmas ask u32.
        pub fn as_u32(&self) -> u32 {
            u32::from(self)
        }
    }

    impl From<&DACMask> for u32 {
        fn from(clusters: &DACMask) -> u32 {
            clusters.bits() as u32
        }
    }

    impl ToU32s for DACMask {
        fn as_u32s(&self) -> Vec<u32> {
            [self.as_u32()].to_vec()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::DACMask;

        #[test]
        fn test_dac_mask() {

            let mut clusters = DACMask::NONE;

            clusters.set_channels(&[2, 3, 50, 61]);
            assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH48_51 |
                DACMask::CH60_63);
            assert_eq!(clusters.as_u32(), 0x00009001);

            clusters.set_channel(12);
            assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH12_15 |
                DACMask::CH48_51 | DACMask::CH60_63);
            assert_eq!(clusters.as_u32(), 0x00009009);

            clusters.unset_channel(61);
            assert_eq!(clusters, DACMask::CH00_03 | DACMask::CH12_15 |
                DACMask::CH48_51);
            assert_eq!(clusters.as_u32(), 0x00001009);

            clusters.clear();
            assert_eq!(clusters, DACMask::NONE);
            assert_eq!(clusters.as_u32(), 0x0);

        }
    }

}

pub mod channelconf {

    use super::ToU32s;
    use std::convert::TryFrom;
    use bitvec::prelude::{BitVec, Msb0, BitStore, BitSlice, bitarr};
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    const CHANSIZE: usize = 3;

    /// Channel configurations currently supported by ArC2.
    /// Use these with [`ChannelConf`] to control
    /// individual ArC2 channels.
    #[derive(Clone, Copy, FromPrimitive, Debug)]
    #[repr(u8)]
    pub enum ChannelState {
        /// Open channel; channel will not be connected
        /// to anything.
        Open = 0b001,
        /// Channel is GND
        CloseGND = 0b010,
        /// Channel cap to GND
        CapGND = 0b011,
        /// Channel is set for arbitrary voltage operation
        VoltArb = 0b100,
        /// Channel is set for arbitrary current operation
        CurArb = 0b101,
        /// High-Speed pulse channel
        HiSpeed = 0b110,
    }

    impl ChannelState {
        fn as_bools(&self) -> [bool; CHANSIZE] {

            let mut bools: [bool; CHANSIZE] = [false; CHANSIZE];

            for i in 0..CHANSIZE {
                bools[i] = ((*self as u8 >> i) & 1) == 1
            }

            bools
        }

        fn from_bools(bools: &[bool; CHANSIZE]) -> ChannelState {
            let mut bitarr = bitarr![Msb0, u8; 0; 8];

            for i in 0..CHANSIZE {
               bitarr.set(8-CHANSIZE+i, bools[i])
            }

            let value: [u8; 1] = bitarr.value();
            ChannelState::from_u8(value[0] as u8).unwrap()

        }

        fn from_bitslice(bools: &BitSlice<Msb0, u32>) -> Result<ChannelState, String> {

            let len: usize;

            if bools.len() < CHANSIZE {
                return Err(String::from("Supplied slice is too small"));
            }

            if bools.len() > 8 {
                len = 8;
            } else {
                len = bools.len()
            }

            let mut bitarr = bitarr![Msb0, u8; 0; 8];

            for i in 0..len {
               bitarr.set(8-len+i, bools[i])
            }

            let value: [u8; 1] = bitarr.value();
            Ok(ChannelState::from_u8(value[0] as u8).unwrap())
        }

    }

    impl From<&[bool; CHANSIZE]> for ChannelState {
        fn from(bools: &[bool; CHANSIZE]) -> ChannelState {
            ChannelState::from_bools(&bools)
        }
    }

    impl TryFrom<&BitSlice<Msb0, u32>> for ChannelState {

        type Error = String;

        fn try_from(v: &BitSlice<Msb0, u32>) -> Result<Self, Self::Error> {
            ChannelState::from_bitslice(v)
        }
    }

    /// A set of DAC channel output configuration.
    ///
    /// A `ChannelConf` is currently designed for 3 bits per channel for
    /// a total of 64 channels (192-bits). The underlying implementation uses a
    /// [`BitVec`][bitvec::vec::BitVec] storing MSB bits and backed by [`u32`]s.
    /// This matches the structure that ArC2 is expecting for the channel
    /// configuration. `ChannelConf` is typically paired with
    /// [`OpCode::UpdateChannel`][`super::opcode::OpCode::UpdateChannel`].
    ///
    /// To create a new register call [`ChannelConf::new()`] with the
    /// desired number of channels. For typical ArC2 scenarios this should be 64.
    /// By default the register is populated with zeros (which is an invalid
    /// status for ArC2) and must be configured appropriately by setting the
    /// invididual channels to a [`ChannelState`] value. The register will take
    /// care of flipping the correct bits in the internal representation in order
    /// to have a consistent 32bit representation.
    ///
    /// **See also**: [`ChannelState`] for the available channel configurations.
    ///
    /// ## Examples
    ///
    /// ```
    /// use libarc2::register::{ChannelConf, ChannelState, ToU32s};
    ///
    /// // Initialise a new channel configuration register
    /// let mut reg = ChannelConf::new(64);
    ///
    /// // Number of allocated channels
    /// let nchan = reg.len();
    ///
    /// // Set channel 31 to High speed pulsing mode
    /// reg.set(31, ChannelState::HiSpeed);
    ///
    /// // Set all channels to arbitrary voltage operation
    /// reg.set_all(ChannelState::VoltArb);
    ///
    /// // Traverse channels (non-consuming iterator)
    /// for channel in &reg {
    ///     println!("{:?}", channel);
    /// }
    ///
    /// // Print the internal representation
    /// // Should return
    /// //  0x92492492
    /// //  0x49249249
    /// //  0x24924924
    /// //  0x92492492
    /// //  0x49249249
    /// //  0x24924924
    /// for value in reg.as_u32s() {
    ///    println!("0x{:x}", value);
    /// }
    /// ```
    pub struct ChannelConf {
        bits: BitVec<Msb0, u32>,
    }

    impl ChannelConf {

        /// Create a new register with the specified number of
        /// channels. This will be expanded to `CHANSIZE` × channels
        /// in the internal bit vector representation.
        pub fn new(channels: usize) -> ChannelConf {
            // CHANSIZE bits for each channel
            let vec: BitVec<Msb0, u32> = BitVec::repeat(false, channels*CHANSIZE);

            ChannelConf { bits: vec }
        }

        /// Set a channel to a [`ChannelState`] value
        pub fn set(&mut self, idx: usize, val: ChannelState) {
            let bits = self.bits.as_mut_bitslice();
            let bools = val.as_bools();

            for i in 0..bools.len() {
                bits.set(CHANSIZE * idx + i, bools[CHANSIZE-1-i]);
            }
        }

        /// Get the [`state`][`ChannelState`] of a channel
        pub fn get(&self, idx: usize) -> ChannelState {
            let v = &self.bits[idx*CHANSIZE..(idx+1)*CHANSIZE];

            ChannelState::try_from(v).unwrap()
        }

        /// Get the number of allocated channels
        pub fn len(&self) -> usize {
            // len is always a multiple of CHANSIZE
            self.bits.len() / CHANSIZE
        }

        /// Set the status of all channels to the same value
        pub fn set_all(&mut self, val: ChannelState) {
            let nchannels = self.len();

            for i in 0..nchannels {
                self.set(i, val);
            }
        }

        /// Get the serialisable format of this register specified
        /// as a slice of whatever the internal representation is. This
        /// is presently a [`u32`] as this is the size of words that
        /// ArC2 is expecting as input.
        pub fn as_slice(&self) -> &[u32] {
            self.bits.as_raw_slice()
        }
    }

    impl ToU32s for ChannelConf {
        fn as_u32s(&self) -> Vec<u32> {
            let bits = self.bits.as_raw_slice();
            bits.to_vec()
        }
    }

    #[doc(hidden)]
    pub struct ChannelConfIterator<'a> {
        register: &'a ChannelConf,
        index: usize,
    }

    impl<'a> IntoIterator for &'a ChannelConf {

        type Item = ChannelState;
        type IntoIter = ChannelConfIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            ChannelConfIterator {
                register: self,
                index: 0,
            }
        }

    }

    impl<'a> Iterator for ChannelConfIterator<'a> {

        type Item = ChannelState;

        fn next(&mut self) -> Option<ChannelState> {
            if self.index >= self.register.len() {
                return None;
            }

            let v = self.register.get(self.index);
            self.index += 1;
            Some(v)
        }

    }


    #[cfg(test)]
    mod tests {

        use super::{ChannelConf, ChannelState};
        use crate::registers::ToU32s;
        use assert_matches::assert_matches;

        #[test]
        fn get_channel() {
            let mut v = ChannelConf::new(64);
            v.set(50, ChannelState::VoltArb);
            let res = v.get(50);
            assert_matches!(res, ChannelState::VoltArb);

            v.set(0, ChannelState::Open);
            let res = v.get(0);
            assert_matches!(res, ChannelState::Open);

            v.set(63, ChannelState::HiSpeed);
            let res = v.get(63);
            assert_matches!(res, ChannelState::HiSpeed);
        }

        #[test]
        fn channel_len() {
            let v = ChannelConf::new(64);
            assert_eq!(v.len(), 64);
        }

        #[test]
        fn bools_to_status() {
            let status0 = ChannelState::from(&[false, true, false]);
            assert_matches!(status0, ChannelState::CloseGND);

            let status1 = ChannelState::from(&[true, false, false]);
            assert_matches!(status1, ChannelState::VoltArb);

            let status2 = ChannelState::from(&[false, false, true]);
            assert_matches!(status2, ChannelState::Open);
        }

        #[test]
        fn all_channel_test() {
            let mut v = ChannelConf::new(64);
            v.set_all(ChannelState::VoltArb);

            for channel in &v {
                assert_matches!(channel, ChannelState::VoltArb);
            }

            let slice = v.as_u32s();

            assert_eq!(slice[0], 0x92492492);
            assert_eq!(slice[1], 0x49249249);
            assert_eq!(slice[2], 0x24924924);
            assert_eq!(slice[3], 0x92492492);
            assert_eq!(slice[4], 0x49249249);
            assert_eq!(slice[5], 0x24924924);
        }
    }

}

pub mod sourceconf {
    use super::ToU32s;
    use bitvec::prelude::{BitVec, Msb0, BitField};
    use num_derive::{FromPrimitive, ToPrimitive};
    use num_traits::{FromPrimitive};

    /// Current source configuration.
    ///
    /// When a current source is in operation this enum
    /// specifies its operation state and it is part of the
    /// [`SourceConf`] register.
    #[derive(Clone, Copy, FromPrimitive, ToPrimitive, Debug)]
    #[repr(u8)]
    pub enum CurrentSourceState {
        /// Maintain current status (default)
        Maintain   = 0b00000000,
        /// Disconnect current source
        Open       = 0b00000001,
        /// Arbitrary voltage operation
        VoltageArb = 0b00000010,
        /// High speed pulse operation
        HiSpeed    = 0b00000011
    }


    /// Output source configuration
    ///
    /// This register configures the status of the output source and
    /// it's usually followed by a
    /// [`ChannelConf`][`crate::register::ChannelConf`].
    /// There are two things that are specified by this register. The
    /// *output digipot* and the state of the *current source*.
    pub struct SourceConf {
        bits: BitVec<Msb0, u32>
    }

    impl SourceConf {

        /// Create a new source configuration register. This will
        /// initialise the digipot to a safe value (`0x1CD` or roughly
        /// 11 kΩ).
        pub fn new() -> SourceConf {
            let mut vec: BitVec<Msb0, u32> = BitVec::repeat(false, 32);
            let bits = vec.as_mut_bitslice();
            bits[0..10].store(0x1CD as u16);

            SourceConf { bits: vec }
        }

        /// Set digipot raw value. This is clamped between
        /// 0x000 and 0x300 to keep the instrument safe.
        pub fn set_digipot(&mut self, val: u16) {
            let actual_val;
            if val > 0x300 {
                actual_val = 0x300;
            } else {
                actual_val = val;
            }

            let bits = self.bits.as_mut_bitslice();
            bits[0..10].store(actual_val);
        }

        /// Get the current digipot raw value.
        pub fn get_digipot(&self) -> u16 {
            self.bits[0..10].load()
        }

        /// Set state output. See [`CurrentSourceState`] for possible
        /// values.
        pub fn set_cursource_state(&mut self, val: CurrentSourceState) {
            self.bits[28..32].store(val as u8);
        }

        /// Retrieves the current source state stores in this register.
        pub fn get_cursource_state(&self) -> CurrentSourceState {
            let val = self.bits[24..32].load::<u8>();
            CurrentSourceState::from_u8(val).unwrap()
        }
    }

    impl ToU32s for SourceConf {
        fn as_u32s(&self) -> Vec<u32> {
            let bits = self.bits.as_raw_slice();
            bits.to_vec()
        }
    }

    #[cfg(test)]
    mod tests {

        use super::{SourceConf, CurrentSourceState, ToU32s};
        use assert_matches::assert_matches;

        #[test]
        fn test_sourceconf() {
            let mut c = SourceConf::new();
            c.set_digipot(0x200);
            assert_eq!(c.get_digipot(), 0x200);

            let slice = c.as_u32s();
            assert_eq!(slice[0], 0x80000000);

            c.set_cursource_state(CurrentSourceState::HiSpeed);
            assert_matches!(c.get_cursource_state(),
                CurrentSourceState::HiSpeed);

            let slice = c.as_u32s();
            assert_eq!(slice[0], 0x80000003);

            // Digipot must never exceed 0x300
            c.set_digipot(0x400);
            assert_eq!(c.get_digipot(), 0x300);

        }
    }
}

pub mod dacvoltage {

    use super::ToU32s;

    /*macro_rules! vidx {
        ($val:expr, $offset:expr, $slope:expr) => {
            match ((($val + $offset)/($slope)) as f64).round() {
                c if c < 0.0 => 0u16,
                c if c > 65535.0 => 0xFFFFu16,
                c => c as u16
            }
        };

        ($val:expr) => {
            vidx!($val, 10.0, 3.05179e-4)
        }
    }*/

    const ZERO: u32 = 0x80008000;

    /// Voltage configuration for DACs
    ///
    /// This struct is used to configure the output voltages of the on-board
    /// DACs. DAC voltage are represented with a `u16` value, `0x0000` being
    /// the lowest possible voltage and `0xFFFF` being the highest. Assuming
    /// the absolute maximum voltages are the same for two polarities value
    /// `0x8000` is 0.0 volts which is the default value used when creating
    /// this register.
    ///
    /// DACs have two outputs, Vhigh and Vlow and typically Vhigh > Vlow in
    /// most circumstances. In normal measurement scenarios both output
    /// voltages should be the same and the user is advised to use the
    /// [`DACVoltage::set()`] function to set a voltage for a DAC. By
    /// default a new `DACVoltage` has four different channels as this
    /// is exactly the number of channels that fit in 1/2 of a DAC cluster.
    /// See documentation of [`DACMask`][`crate::register::DACMask`] for
    /// more details on that.
    ///
    /// ## Examples
    /// ```
    /// use libarc2::register::DACVoltage;
    ///
    /// // Make a new register
    /// let mut reg0 = DACVoltage::new();
    /// // Set both values of the second channel
    /// reg0.set(1, 0x8534);
    /// assert_eq!(reg0.get(1), (0x8534, 0x8534));
    ///
    /// let mut reg1 = DACVoltage::new();
    /// // Set the high voltage of the third channel
    /// reg1.set_high(2, 0x8534);
    /// assert_eq!(reg1.get(2), (0x8000, 0x8534));
    /// ```
    pub struct DACVoltage {
        values: Vec<u32>
    }

    impl DACVoltage {

        /// Create a new register with four channels
        pub fn new() -> DACVoltage {
            DACVoltage::new_with_size(4)
        }

        fn new_with_size(size: usize) -> DACVoltage {
            let mut vec: Vec<u32> = Vec::with_capacity(size);

            for _ in 0..size {
                vec.push(ZERO);
            }

            DACVoltage { values: vec }
        }

        /// Set the Vhigh value of a specified channel index
        pub fn set_high(&mut self, idx: usize, voltage: u16) {
            self.values[idx] = (voltage as u32) << 16 |
                (self.values[idx] & 0xFFFF);
        }

        /// Get the Vhigh value of a specified channel index
        pub fn get_high(&self, idx: usize) -> u16 {
            ((self.values[idx] & 0xFFFF0000) >> 16) as u16
        }

        /// Set the Vlow value of a specified channel index
        pub fn set_low(&mut self, idx: usize, voltage: u16) {
            self.values[idx] |= voltage as u32;
        }

        /// Get the Vlow value of a specified channel index
        pub fn get_low(&self, idx: usize) -> u16 {
            (self.values[idx] & 0xFFFF) as u16
        }

        /// Set both Vhigh and Vlow of a specified channel index
        pub fn set(&mut self, idx: usize, voltage: u16) {
            self.set_low(idx, voltage);
            self.set_high(idx, voltage);
        }

        /// Get both Vhigh and Vlow of a specified channel index.
        /// The first `u16` of the tuple is Vlow, the second Vhigh.
        pub fn get(&self, idx: usize) -> (u16, u16) {
            (self.get_low(idx), self.get_high(idx))
        }

        /// Number of configured channels
        pub fn len(&self) -> usize {
            self.values.len()
        }

    }

    impl ToU32s for DACVoltage {
        fn as_u32s(&self) -> Vec<u32> {
            self.values.clone()
        }
    }

    #[cfg(test)]
    mod tests {

        use super::{DACVoltage};

        #[test]
        fn dacvoltage_new() {
            let v = DACVoltage::new();
            for value in v.values {
                assert_eq!(value, 0x80008000);
            }
        }

        #[test]
        fn dacvoltage_set_high() {
            let mut v = DACVoltage::new();
            v.set_high(3, 0xA0A0);

            assert_eq!(v.values[3], 0xA0A08000);
            assert_eq!(v.get_high(3), 0xA0A0);
            assert_eq!(v.get_low(3), 0x8000);
        }

        #[test]
        fn dacvoltage_set_low() {
            let mut v = DACVoltage::new();
            v.set_low(2, 0x90F3);

            assert_eq!(v.values[2], 0x800090F3);
            assert_eq!(v.get_high(2), 0x8000);
            assert_eq!(v.get_low(2), 0x90F3);
        }

        #[test]
        fn dacvoltage_set_both() {
            let mut v = DACVoltage::new();

            v.set(1, 0x8534);
            assert_eq!(v.values[1], 0x85348534);
            assert_eq!(v.get_low(1), 0x8534);
            assert_eq!(v.get_high(1), 0x8534);
            assert_eq!(v.get(1), (0x8534, 0x8534));
        }

    }

}


pub mod u32mask {

    use super::ToU32s;
    use bitvec::prelude::{BitVec, Msb0};

    /// A trait denoting a word size; ie how many words
    /// a register is using.
    pub trait WordSize {
        const WORDS: usize;
    }

    /// One word
    pub struct Wx1;
    impl WordSize for Wx1 {
        const WORDS: usize = 1;
    }

    /// Two words
    pub struct Wx2;
    impl WordSize for Wx2 {
        const WORDS: usize = 2;
    }

    /// Three words
    pub struct Wx3;
    impl WordSize for Wx3 {
        const WORDS: usize = 3;
    }

    /// Four words
    pub struct Wx4;
    impl WordSize for Wx4 {
        const WORDS: usize = 4;
    }

    /// A generic bitmask of the specified word size
    pub struct U32Mask<T> {
        _words: T,
        bits: BitVec<Msb0, u32>,
    }


    impl<T: WordSize> U32Mask<T> {

        /// Set a channel to enabled (`true`) or disabled (`false`).
        pub fn set_enabled(&mut self, idx: usize, status: bool) {
            let len = self.bits.len();
            let bits = self.bits.as_mut_bitslice();
            bits.set(len-1-idx, status)
        }

        /// Get the state of a channel, enabled (`true`) or disabled (`false`).
        pub fn get_enabled(&self, idx: usize) -> bool {
            let len = self.bits.len();
            self.bits[len-1-idx]
        }

        /// Get the number of allocated channels.
        pub fn len(&self) -> usize {
            self.bits.len()
        }

        /// Set all channels to enabled (`true`) or disabled (`false`).
        pub fn set_enabled_all(&mut self, status: bool) {
            let len = self.bits.len();
            let bits = self.bits.as_mut_bitslice();
            for i in 0..len {
                bits.set(len-1-i, status)
            }
        }

        /// Toggle selected channel.
        pub fn toggle(&mut self, idx: usize) {
            self.set_enabled(idx, !self.get_enabled(idx));
        }

        /// Get the serialisable format of this register specified
        /// as a slice of whatever the internal representation is. This
        /// is presently a [`u32`] as this is the size of words that
        /// ArC2 is expecting as input.
        pub fn as_slice(&self) -> &[u32] {
            self.bits.as_raw_slice()
        }

    }

    impl<T: WordSize> ToU32s for U32Mask<T> {
        fn as_u32s(&self) -> Vec<u32> {
            self.bits.as_raw_slice().to_vec()
        }
    }

    impl U32Mask<Wx1> {
        pub fn new() -> U32Mask<Wx1> {
            let vec: BitVec<Msb0, u32> = BitVec::repeat(false, Wx1::WORDS*32);
            U32Mask { _words: Wx1{}, bits: vec }
        }
    }

    impl U32Mask<Wx2> {
        pub fn new() -> U32Mask<Wx2> {
            let vec: BitVec<Msb0, u32> = BitVec::repeat(false, Wx2::WORDS*32);
            U32Mask { _words: Wx2{}, bits: vec }
        }
    }

    impl U32Mask<Wx3> {
        pub fn new() -> U32Mask<Wx1> {
            let vec: BitVec<Msb0, u32> = BitVec::repeat(false, Wx3::WORDS*32);
            U32Mask { _words: Wx1{}, bits: vec }
        }
    }

    impl U32Mask<Wx4> {
        pub fn new() -> U32Mask<Wx2> {
            let vec: BitVec<Msb0, u32> = BitVec::repeat(false, Wx4::WORDS*32);
            U32Mask { _words: Wx2{}, bits: vec }
        }
    }
}


pub mod adcmask {

    use super::u32mask::{Wx2, U32Mask};


    /// Measurement channel configuration bitmask.
    ///
    /// An `ADCMask` is used to configure the measurement channels. Essentially
    /// it defines which channels the Read Current or Read Voltage operation be
    /// applied.
    ///
    /// See [`U32Mask`][`crate::register::U32Mask`] for details and
    /// methods.
    ///
    /// ## Example
    /// ```
    /// use libarc2::register::{ADCMask, ToU32s};
    ///
    /// let mut chan = ADCMask::new();
    ///
    /// // set some channels
    /// chan.set_enabled(31, true);
    /// chan.set_enabled(0, true);
    /// chan.set_enabled(62, true);
    ///
    /// assert_eq!(chan.get_enabled(31), true);
    ///
    /// // u32 representation
    /// assert_eq!(chan.as_u32s(), &[0x40000000, 0x80000001]);
    /// ```
    pub type ADCMask = U32Mask<Wx2>;


    #[cfg(test)]
    mod tests {
        use super::ADCMask;
        use crate::registers::ToU32s;

        #[test]
        fn get_set_channel() {
            let mut v = ADCMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);
            v.set_enabled(62, true);

            assert_eq!(v.get_enabled(31), true);
            assert_eq!(v.get_enabled(0), true);
            assert_eq!(v.get_enabled(62), true);

            v.set_enabled(62, false);
            assert_eq!(v.get_enabled(62), false);

        }

        #[test]
        fn get_set_all_channels() {
            let mut v = ADCMask::new();
            v.set_enabled_all(true);

            for c in 0..v.len() {
                assert_eq!(v.get_enabled(c), true);
            }

        }

        #[test]
        fn repr() {
            let mut v = ADCMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);
            v.set_enabled(62, true);

            assert_eq!(&v.as_u32s(), &[0x40000000, 0x80000001]);

        }

        #[test]
        fn toggle() {
            let mut v = ADCMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);
            v.set_enabled(62, true);

            assert_eq!(v.get_enabled(31), true);

            v.toggle(31);
            assert_eq!(v.get_enabled(31), false);
        }

    }
}

pub mod iomask {

    use super::u32mask::{Wx1, U32Mask};


    /// I/O channel configuration bitmask.
    ///
    /// An `IOMask` is used to configure the I/O channels of ArC2. Essentially
    /// it defines which channels will be configured during the Update I/O
    /// instruction.
    ///
    /// See [`U32Mask`][`crate::register::U32Mask`] for details and
    /// methods.
    ///
    /// ## Example
    /// ```
    /// use libarc2::register::{IOMask, ToU32s};
    ///
    /// let mut chan = IOMask::new();
    ///
    /// // set some channels
    /// chan.set_enabled(31, true);
    /// chan.set_enabled(0, true);
    ///
    /// assert_eq!(chan.get_enabled(31), true);
    ///
    /// // u32 representation
    /// assert_eq!(chan.as_u32s(), &[0x80000001]);
    /// ```
    pub type IOMask = U32Mask<Wx1>;


    #[cfg(test)]
    mod tests {
        use super::IOMask;
        use crate::registers::ToU32s;

        #[test]
        fn get_set_channel() {
            let mut v = IOMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);

            assert_eq!(v.get_enabled(31), true);
            assert_eq!(v.get_enabled(0), true);

            v.set_enabled(31, false);
            assert_eq!(v.get_enabled(31), false);

        }

        #[test]
        fn get_set_all_channels() {
            let mut v = IOMask::new();
            v.set_enabled_all(true);

            for c in 0..v.len() {
                assert_eq!(v.get_enabled(c), true);
            }

        }

        #[test]
        fn repr() {
            let mut v = IOMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);

            assert_eq!(&v.as_u32s(), &[0x80000001]);

        }

        #[test]
        fn toggle() {
            let mut v = IOMask::new();
            v.set_enabled(31, true);
            v.set_enabled(0, true);

            assert_eq!(v.get_enabled(31), true);

            v.toggle(31);
            assert_eq!(v.get_enabled(31), false);
        }

    }
}
