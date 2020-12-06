#[allow(unused_macros)]
macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}