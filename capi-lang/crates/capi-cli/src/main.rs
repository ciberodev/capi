use std::path::PathBuf;
use std::process::ExitCode;

use capi_driver::{run, DriverRequest};

fn main() -> ExitCode {
    let request = parse_args(std::env::args().skip(1));
    let response = run(request);

    if !response.stdout().is_empty() {
        print!("{}", response.stdout());
    }

    if !response.stderr().is_empty() {
        eprint!("{}", response.stderr());
    }

    ExitCode::from(response.status().code() as u8)
}

fn parse_args(args: impl IntoIterator<Item = String>) -> DriverRequest {
    let args = args.into_iter().collect::<Vec<_>>();

    match args.as_slice() {
        [] => DriverRequest::Help,
        [flag] if flag == "--help" || flag == "-h" => DriverRequest::Help,
        [flag] if flag == "--version" || flag == "-V" => DriverRequest::Version,
        [flag, kind, path] if flag == "--emit" && kind == "tokens" => DriverRequest::EmitTokens {
            path: PathBuf::from(path),
        },
        [flag, kind, path] if flag == "--emit" && kind == "ast" => DriverRequest::EmitAst {
            path: PathBuf::from(path),
        },
        [flag, kind, path] if flag == "--emit" && kind == "hir" => DriverRequest::EmitHir {
            path: PathBuf::from(path),
        },
        [flag, kind, _, extra, ..]
            if flag == "--emit" && matches!(kind.as_str(), "tokens" | "ast" | "hir") =>
        {
            DriverRequest::InvalidArguments {
                message: format!("unexpected argument '{extra}'"),
            }
        }
        [flag, kind, ..] if flag == "--emit" => DriverRequest::InvalidArguments {
            message: format!("unsupported emit kind '{kind}'"),
        },
        [flag] if flag.starts_with('-') => DriverRequest::InvalidArguments {
            message: format!("unknown option '{flag}'"),
        },
        [path] => DriverRequest::CheckSource {
            path: PathBuf::from(path),
        },
        [_, extra, ..] => DriverRequest::InvalidArguments {
            message: format!("unexpected argument '{extra}'"),
        },
    }
}
