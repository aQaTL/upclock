use anyhow::Result;

fn main() -> Result<()> {
	let uptime = uptime()?;
	println!("Up for {} seconds", uptime);
	Ok(())
}

#[cfg(target_os = "macos")]
fn uptime() -> std::io::Result<i64> {
	let mut uptime = libc::timespec {
		tv_sec: 0,
		tv_nsec: 0,
	};

	if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut uptime as *mut libc::timespec) }
		!= 0
	{
		return Err(std::io::Error::last_os_error());
	}
	return Ok(uptime.tv_sec);
}
