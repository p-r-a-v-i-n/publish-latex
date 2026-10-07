use std::env;
use std::path::Path;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: no argument provided");
        eprintln!("Usage: publish-latex <file.tex>");
        return ExitCode::FAILURE;
    }

    let filename = &args[1];
    let path = Path::new(filename);

    if path.extension().and_then(|ext| ext.to_str()) != Some("tex") {
        eprintln!("Error: input must be a .tex file");
        return ExitCode::FAILURE;
    }

    if !path.exists() {
        eprintln!("Error: file does not exist: {filename}");
        return ExitCode::FAILURE;
    }

    if !path.is_file() {
        eprintln!("Error: input is not a file: {filename}");
        return ExitCode::FAILURE;
    }

    println!("Found: {filename}");

    let latexmk_check = Command::new("latexmk").arg("--version").output();

    match latexmk_check {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Error: latexmk is installed but could not run successfully");
                return ExitCode::FAILURE;
            }
        }
        Err(error) => {
            eprintln!("Error: failed to execute latexmk: {error}");
            eprintln!("Make sure latexmk is installed and available in PATH.");
            return ExitCode::FAILURE;
        }
    }

    println!("Found latexmk");
    println!("Compiling {filename}...");

    let output = match Command::new("latexmk")
        .args([
            "-pdf",
            "-interaction=nonstopmode",
            "-halt-on-error",
            filename,
        ])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Error: failed to execute latexmk: {error}");
            return ExitCode::FAILURE;
        }
    };

    if !output.status.success() {
        eprintln!("LaTeX compilation failed:");

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.is_empty() {
            eprintln!("{stdout}");
        }

        if !stderr.is_empty() {
            eprintln!("Errors:");
            eprintln!("{stderr}");
        }

        return ExitCode::FAILURE;
    }

    let pdf_path = path.with_extension("pdf");

    if !pdf_path.exists() {
        eprintln!(
            "Error: latexmk succeeded but PDF was not found: {}",
            pdf_path.display()
        );

        return ExitCode::FAILURE;
    }

    println!("PDF generated successfully");
    println!("PDF: {}", pdf_path.display());

    ExitCode::SUCCESS
}
