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
	Ok(Duration::from_secs(uptime.tv_sec.try_into()?))
}

#[cfg(target_os = "linux")]
fn uptime() -> Result<Duration> {
	let mut sysinfo = libc::sysinfo {
		uptime: 0,
		loads: [0; 3],
		totalram: 0,
		freeram: 0,
		sharedram: 0,
		bufferram: 0,
		totalswap: 0,
		freeswap: 0,
		procs: 0,
		pad: 0,
		totalhigh: 0,
		freehigh: 0,
		mem_unit: 0,
		_f: [],
	};

	if unsafe { libc::sysinfo(&mut sysinfo as *mut libc::sysinfo) } != 0 {
		return Err(std::io::Error::last_os_error().into());
	}

	Ok(Duration::from_secs(sysinfo.uptime.try_into()?))
}
