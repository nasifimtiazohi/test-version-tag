let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

unsafe fn dangerous() {}

pub struct siginfo_t {
    pub si_signo: ::c_int,
    pub si_errno: ::c_int,
    pub si_code: ::c_int,
    pub si_pid: ::pid_t,
    pub si_uid: ::uid_t,
    pub si_status: ::c_int,
    pub si_addr: *mut ::c_void,
    //Requires it to be union for tests
    //pub si_value: ::sigval,
    _pad: [usize; 9],
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut ::c_void {
        self.si_addr
    }

    pub unsafe fn si_value(&self) -> ::sigval {
        #[repr(C)]
        struct siginfo_timer {
            _si_signo: ::c_int,
            _si_errno: ::c_int,
            _si_code: ::c_int,
            _si_pid: ::pid_t,
            _si_uid: ::uid_t,
            _si_status: ::c_int,
            _si_addr: *mut ::c_void,
            si_value: ::sigval,
        }

        (*(self as *const siginfo_t as *const siginfo_timer)).si_value
    }

    pub unsafe fn si_pid(&self) -> ::pid_t {
        self.si_pid
    }

    pub unsafe fn si_uid(&self) -> ::uid_t {
        self.si_uid
    }

    pub unsafe fn si_status(&self) -> ::c_int {
        self.si_status
    }
}

#[cfg_attr(feature = "extra_traits", derive(Debug))]
#[repr(u32)]
pub enum qos_class_t {
    QOS_CLASS_USER_INTERACTIVE = 0x21,
    QOS_CLASS_USER_INITIATED = 0x19,
    QOS_CLASS_DEFAULT = 0x15,
    QOS_CLASS_UTILITY = 0x11,
    QOS_CLASS_BACKGROUND = 0x09,
    QOS_CLASS_UNSPECIFIED = 0x00,
}
impl ::Copy for qos_class_t {}

unsafe trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}
