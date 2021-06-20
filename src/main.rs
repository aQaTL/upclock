use anyhow::Result;
use humantime::format_duration;
use std::convert::TryInto;
use std::time::Duration;

fn main() -> Result<()> {
	let uptime = uptime()?;
	println!("Up for {}", format_duration(uptime));
	Ok(())
}

#[cfg(target_os = "macos")]
fn uptime() -> Result<Duration> {
	let mut uptime = libc::timespec {
		tv_sec: 0,
		tv_nsec: 0,
	};

	if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut uptime as *mut libc::timespec) }
		!= 0
	{
		return Err(std::io::Error::last_os_error().into());
	}
	return Ok(Duration::from_secs(uptime.tv_sec.try_into()?));
}
