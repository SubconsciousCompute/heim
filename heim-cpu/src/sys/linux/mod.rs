use heim_common::{Result, Error};

mod times;
mod freq;
mod stats;

pub use self::times::*;
pub use self::freq::*;
pub use self::stats::*;

fn clock_ticks() -> Result<f64> {
    let result = unsafe {
        libc::sysconf(libc::_SC_CLK_TCK)
    };

    if result > 0 {
        Ok(result as f64)
    } else {
        Err(Error::last_os_error())
    }
}

lazy_static::lazy_static! {
    // Time units in USER_HZ or Jiffies
    pub static ref CLOCK_TICKS: f64 = clock_ticks()
        .expect("Unable to determine CPU number of ticks per second");
}
