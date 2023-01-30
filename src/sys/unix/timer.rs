use std::io::{self, Error, ErrorKind};
use std::time::Duration;
use std::os::fd::RawFd;
use std::convert::TryInto;
//use std::os::unix::io::{AsRawFd, FromRawFd};
use crate::{
    unix::SourceFd,
    event::Source,
    Registry,
    Token,
    Interest
};


/// Test docs
#[derive(Debug, Copy, Clone)]
pub struct Timer {
    #[cfg(any(
        target_os = "android",
        target_os = "linux",
    ))]
    inner: RawFd,

    // The current timer duration
    //duration: Duration,
}

#[cfg(any(
    target_os = "android",
    target_os = "linux",
))]
impl Source for Timer {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.inner).register(registry, token, interests)
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.inner).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        SourceFd(&self.inner).deregister(registry)
    }
}

/*
#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "ios",
    target_os = "macos",
))]
impl Source for Timer {
    fn register(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        let mut kevent = kevent!(
            0,
            libc::EVFILT_TIMER,
            libc::EV_ADD | libc::EV_CLEAR | libc::EV_RECEIPT | libc::EV_ENABLE,
            token.0,
            self.duration.,
        );

        libc::kevent {
            ident: $id as libc::uintptr_t,
            filter: $filter as Filter,
            flags: $flags,
            udata: $data as UData,
            ..unsafe { mem::zeroed() }
        }

        syscall!(kevent(self.kq, &kevent, 1, &mut kevent, 1, ptr::null())).and_then(|_| {
            if (kevent.flags & libc::EV_ERROR) != 0 && kevent.data != 0 {
                Err(io::Error::from_raw_os_error(kevent.data as i32))
            } else {
                Ok(())
            }
        })
    }

    fn reregister(
        &mut self,
        registry: &Registry,
        token: Token,
        interests: Interest,
    ) -> io::Result<()> {
        SourceFd(&self.inner).reregister(registry, token, interests)
    }

    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        SourceFd(&self.inner).deregister(registry)
    }
} */


impl Timer {
    /// docs
    pub fn new(duration: Duration) -> io::Result<Self> {
        //let seconds: i32 = duration.as_secs().try_into().or(Err(Error::from(ErrorKind::InvalidInput)))?;
        Self::platform_new(duration)
    }

    #[cfg(any(
        target_os = "android",
        target_os = "linux",
    ))]
    fn platform_new(duration: Duration) -> io::Result<Self> {
        let timeout = libc::timespec {
            tv_sec: duration.as_secs().try_into().or(Err(Error::from(ErrorKind::InvalidInput)))?,
            tv_nsec: 0,
        };
        let mut value = libc::itimerspec {
            it_interval: timeout.clone(),
            it_value: timeout,

        };
        let timer = syscall!(timerfd_create(libc::CLOCK_MONOTONIC, libc::TFD_NONBLOCK | libc::TFD_CLOEXEC))?;

        syscall!(timerfd_settime(timer, libc::TFD_TIMER_ABSTIME, &mut value as _, core::ptr::null_mut()))?;
        Ok(Self {
            inner: timer,
            //duration,
        })
    }

    /// docs
    pub fn reset(&mut self, _duration: Duration) -> io::Result<()> {
        
        Err(Error::from(ErrorKind::InvalidInput))
    }
}