use crate::rng::{SplitMix64, XoRoShiRo128Plus, XorShift32};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

/// uses the system time to seed an XorShift32
///
/// see `TimeSeededXorShift32::generate`
pub struct TimeSeededXorShift32;

impl TimeSeededXorShift32 {
    pub fn generate() -> Result<XorShift32, SystemTimeError> {
        let current_time =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() % u64::MAX as u128;
        let mut temp_splitmix = SplitMix64::wrap(current_time as u64);
        let seed = temp_splitmix.mix() as u32;

        Ok(XorShift32::wrap(seed))
    }
}

/// uses the system time to seed a SplitMix64
///
/// see `TimeSeededSplitMix64::generate`
pub struct TimeSeededSplitMix64;

impl TimeSeededSplitMix64 {
    pub fn generate() -> Result<SplitMix64, SystemTimeError> {
        let current_time =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() % u64::MAX as u128;
        Ok(SplitMix64::wrap(current_time as u64))
    }
}

/// uses the system time to seed a XoRoShiRo128Plus
///
/// see `TimeSeededXoRoShiRo128Plus::generate`
pub struct TimeSeededXoRoShiRo128Plus;

impl TimeSeededXoRoShiRo128Plus {
    pub fn generate() -> Result<XoRoShiRo128Plus, SystemTimeError> {
        let current_time =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() % u64::MAX as u128;
        let mut temp_splitmix = SplitMix64::wrap(current_time as u64);
        let seed = [temp_splitmix.mix(), temp_splitmix.mix()];

        Ok(XoRoShiRo128Plus::wrap(seed))
    }
}
