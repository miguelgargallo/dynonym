//! Error types and handling

error_chain! {
    foreign_links {
        // errors from other crates here
    }

    errors {
        // own errors here
    }
}

pub fn handle(err: &Error) -> ! {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    // TODO print "error:" in red
    writeln!(stderr, "error: {}", err).expect(errmsg);

    for err in err.iter().skip(1) {
        writeln!(stderr, "caused by: {}", err).expect(errmsg);
    }

    // The backtrace is not always generated. Try to run with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = err.backtrace() {
        writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    ::std::process::exit(1);
}