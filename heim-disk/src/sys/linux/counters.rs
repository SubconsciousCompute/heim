use std::str::FromStr;
use std::ffi::CString;

use tokio_threadpool::blocking;

use heim_common::prelude::*;
use heim_common::utils::parse::ParseIterator;
use heim_common::units::iec::u64::Information;
use heim_common::units::iec::information::byte;
use heim_common::units::si::f64::Time;
use heim_common::units::si::time::second;

// Copied from the `psutil` sources:
//
// "man iostat" states that sectors are equivalent with blocks and have
// a size of 512 bytes. Despite this value can be queried at runtime
// via /sys/block/{DISK}/queue/hw_sector_size and results may vary
// between 1k, 2k, or 4k... 512 appears to be a magic constant used
// throughout Linux source code:
// * https://stackoverflow.com/a/38136179/376587
// * https://lists.gt.net/linux/kernel/2241060
// * https://github.com/giampaolo/psutil/issues/1305
// * https://github.com/torvalds/linux/blob/4f671fe2f9523a1ea206f63fe60a7c7b3a56d5c7/include/linux/bio.h#L99
// * https://lkml.org/lkml/2015/8/17/234
const DISK_SECTOR_SIZE: u64 = 512;


#[derive(Debug, Default, heim_derive::Getter)]
pub struct IoCounters {
    #[getter(as_str)]
    name: String,
    read_count: u64,
    write_count: u64,
    read_bytes: Information,
    write_bytes: Information,
    busy_time: Time,
    read_merged_count: u64,
    write_merged_count: u64,
}

impl IoCounters {
    pub fn device_name(&self) -> &str {
        self.name.as_str()
    }

    // Based on the sysstat code:
    // https://github.com/sysstat/sysstat/blob/1c711c1fd03ac638cfc1b25cdf700625c173fd2c/common.c#L200
    fn is_storage_device(&self) -> impl Future<Item=bool, Error=Error> {
        let path = CString::new(format!("/sys/block/{}", self.name.replace("/", "!")))
            // FIXME: propagate error
            .expect("Malformed device path");

        future::poll_fn(move || {
            blocking(|| {
                let result = unsafe {
                    libc::access(path.as_ptr(), libc::F_OK)
                };

                result == 0
            }).map_err(|_| panic!("The tokio threadpool shut down"))
        })
    }

}

impl FromStr for IoCounters {
    type Err = Error;

    // At the moment supports format used in Linux 2.6+,
    // except ignoring discard values introduced in Linux 4.18.
    //
    // https://www.kernel.org/doc/Documentation/iostats.txt
    // https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats
    fn from_str(s: &str) -> Result<IoCounters> {
        let mut parts = s.split_whitespace().skip(2);

        let name: String = parts.try_from_next()?;
        let read_count = parts.try_from_next()?;
        let read_merged_count = parts.try_from_next()?;
        let read_bytes = parts.try_from_next()
            .map(|bytes: u64| Information::new::<byte>(bytes * DISK_SECTOR_SIZE))?;
        let mut parts = parts.skip(1);
        let write_count = parts.try_from_next()?;
        let write_merged_count = parts.try_from_next()?;
        let write_bytes = parts.try_from_next()
            .map(|bytes: u64| Information::new::<byte>(bytes * DISK_SECTOR_SIZE))?;
        let mut parts = parts.skip(2);
        let busy_time = parts.try_from_next()
            .map(|seconds: u64| Time::new::<second>(seconds as f64))?;

        Ok(IoCounters {
            name,
            read_count,
            read_merged_count,
            read_bytes,
            write_count,
            write_merged_count,
            write_bytes,
            busy_time,
        })
    }
}

pub fn io_counters() -> impl Stream<Item=IoCounters, Error=Error> {
    utils::fs::read_lines_into("/proc/diskstats")
}

pub fn io_counters_physical() -> impl Stream<Item=IoCounters, Error=Error> {
    io_counters()
        .and_then(|device| {
            device.is_storage_device().map(|value| (value, device))
        })
        .filter_map(|(is_storage, device)| {
            if is_storage {
                Some(device)
            } else {
                None
            }
        })
}
