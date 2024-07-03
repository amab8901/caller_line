use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};

pub fn caller_line() -> String {
    let (trace, curr_file, curr_line) = (Backtrace::new(), file!(), line!());
    let frames = trace.frames();
    let backtrace_symbol = frames
        .iter()
        .flat_map(BacktraceFrame::symbols)
        .skip_while(|s| {
            s.filename()
                .map(|p| !p.ends_with(curr_file))
                .unwrap_or(true)
                || s.lineno() != Some(curr_line)
        })
        .nth(2)
        .cloned();

    let filename = backtrace_symbol
        .as_ref()
        .and_then(BacktraceSymbol::filename)
        .unwrap()
        .to_str()
        .unwrap();

    let lineno = backtrace_symbol
        .as_ref()
        .and_then(BacktraceSymbol::lineno)
        .unwrap();

    let previous_line = format!("{filename}:{lineno}");

    previous_line
}
