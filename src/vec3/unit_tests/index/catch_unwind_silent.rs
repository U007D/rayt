use std::panic::{UnwindSafe, take_hook, set_hook, catch_unwind};
use std::thread::Result as ThreadResult;

pub fn catch_unwind_silent<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> ThreadResult<R> {
    let orig_hook = take_hook();
    set_hook(Box::new(|_| {}));
    let res = catch_unwind(f);
    set_hook(orig_hook);
    res
}
