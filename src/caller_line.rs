use backtrace::{Backtrace, BacktraceFrame, BacktraceSymbol};

pub fn caller_line(steps: usize) -> String {
    let (trace, curr_file, curr_line) = (Backtrace::new(), file!(), line!());
    let backtrace_symbol = backtrace_symbol(trace, curr_file, curr_line, steps);
    let filename = filename(&backtrace_symbol);
    let line_number = line_number(&backtrace_symbol);
    let previous_line = format!("{filename}:{line_number}");

    previous_line
}

fn backtrace_symbol(
    trace: Backtrace,
    curr_file: &str,
    curr_line: u32,
    steps: usize,
) -> Option<BacktraceSymbol> {
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
        .nth(1 + steps)
        .cloned();
    backtrace_symbol
}

fn filename(backtrace_symbol: &Option<BacktraceSymbol>) -> &str {
    let filename = backtrace_symbol
        .as_ref()
        .and_then(BacktraceSymbol::filename)
        .unwrap()
        .to_str()
        .unwrap();
    filename
}

fn line_number(backtrace_symbol: &Option<BacktraceSymbol>) -> u32 {
    let line_number = backtrace_symbol
        .as_ref()
        .and_then(BacktraceSymbol::lineno)
        .unwrap();
    line_number
}
