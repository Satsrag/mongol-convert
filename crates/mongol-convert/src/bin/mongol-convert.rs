use mongol_convert::{translate_with_options, version, CodeType, TranslationOptions, Warning};
use std::io::{self, Read, Write};
use std::process::ExitCode;

fn usage(program: &str) -> String {
    format!(
        "usage: {program} translate --from <encoding> --to <encoding> [--repair-suffix-separators] [--no-restore-menk-shape-emoji] [text]"
    )
}

fn help(program: &str) -> String {
    format!(
        "Mongolian encoding converter\n\n\
Usage:\n  {program} translate --from <encoding> --to <encoding> [text]\n\n\
When [text] is omitted, mongol-convert reads UTF-8 text from stdin. Converted UTF-8 text is written to stdout without adding a newline.\n\n\
Options (after --to <encoding>, before text):\n  --repair-suffix-separators       Heuristically restore NNBSP before known suffixes in menk_letter/delehi input; report edits on stderr.\n  --no-restore-menk-shape-emoji   Do not restore legacy SoftBank/iOS emoji back to MenkShape PUA before decoding menk_shape input.\n  --  Treat the following argument as text even if it matches an option.\n\n\
Encodings:\n  zvvnmod\n  delehi\n  menk_shape\n  menk_letter\n  oyun\n  utn57\n  utn57_shape\n  z52\n\n\
oyun is not supported.\n"
    )
}

/// What a successful run writes: the text for stdout, and the warnings for stderr.
struct Output {
    text: String,
    warnings: Vec<Warning>,
}

impl Output {
    fn text(text: String) -> Self {
        Self {
            text,
            warnings: Vec::new(),
        }
    }
}

fn run(arguments: impl IntoIterator<Item = String>) -> Result<Output, String> {
    let mut arguments = arguments.into_iter();
    let program = arguments.next().unwrap_or_else(|| "mongol-convert".to_owned());

    let command = arguments.next();
    if matches!(command.as_deref(), Some("--help" | "-h")) && arguments.next().is_none() {
        return Ok(Output::text(help(&program)));
    }
    if matches!(command.as_deref(), Some("--version" | "-V")) && arguments.next().is_none() {
        return Ok(Output::text(format!("mongol-convert {}\n", version())));
    }
    if command.as_deref() != Some("translate") {
        return Err(usage(&program));
    }
    if arguments.next().as_deref() != Some("--from") {
        return Err(usage(&program));
    }
    let from = arguments.next().ok_or_else(|| usage(&program))?;
    if arguments.next().as_deref() != Some("--to") {
        return Err(usage(&program));
    }
    let to = arguments.next().ok_or_else(|| usage(&program))?;
    let mut options = TranslationOptions::default();
    let mut next = arguments.next();
    loop {
        match next.as_deref() {
            Some("--repair-suffix-separators") => {
                options.repair_suffix_separators = true;
                next = arguments.next();
            }
            Some("--no-restore-menk-shape-emoji") => {
                options.restore_menk_shape_emoji = false;
                next = arguments.next();
            }
            _ => break,
        }
    }
    if next.as_deref() == Some("--") {
        next = Some(arguments.next().ok_or_else(|| usage(&program))?);
    }
    let input = match next {
        Some(input) => input,
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|error| format!("could not read stdin: {error}"))?;
            input
        }
    };
    if arguments.next().is_some() {
        return Err(usage(&program));
    }

    let from = from
        .parse::<CodeType>()
        .map_err(|error| error.to_string())?;
    let to = to.parse::<CodeType>().map_err(|error| error.to_string())?;
    let translation =
        translate_with_options(from, to, &input, &options).map_err(|error| error.to_string())?;
    Ok(Output {
        text: translation.text,
        warnings: translation.warnings,
    })
}

fn main() -> ExitCode {
    match run(std::env::args()) {
        Ok(output) => {
            // Warnings go to stderr, one per line, so stdout stays the converted bytes alone.
            for warning in &output.warnings {
                eprintln!("mongol-convert: warning: {warning}");
            }
            match io::stdout().write_all(output.text.as_bytes()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("mongol-convert: could not write stdout: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(error) => {
            eprintln!("mongol-convert: {error}");
            ExitCode::FAILURE
        }
    }
}
