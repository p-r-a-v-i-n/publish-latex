use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: no argument provided");
        eprintln!("Usage: publish-latex <file.tex>");
        return;
    }

    let filename = &args[1];
    let path = Path::new(filename);

    if path.extension().and_then(|ext| ext.to_str()) != Some("tex") {
        eprintln!("Error: input must be a .tex file");
        return;
    }

    if !path.exists() {
        eprintln!("Error: file does not exist: {filename}");
        return;
    }

    if !path.is_file() {
        eprintln!("Error: input is not a file: {filename}");
        return;
    }

    println!("Found: {filename}");

    let latexmk_check = Command::new("latexmk").arg("--version").output();

    match latexmk_check {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Error: latexmk is installed but could not run successfully");
                return;
            }
        }
        Err(error) => {
            eprintln!("Error: failed to execute latexmk: {error}");
            eprintln!("Make sure latexmk is installed and available in PATH.");
            return;
        }
    }

    println!("Found latexmk");
    println!("Compiling {filename}...");

    let output = Command::new("latexmk")
        .args([
            "-pdf",
            "-interaction=nonstopmode",
            "-halt-on-error",
            filename,
        ])
        .output()
        .expect("failed to execute latexmk");

    if output.status.success() {
        let pdf_path = path.with_extension("pdf");

        // Make sure the expected PDF was actually generated.
        if pdf_path.exists() {
            println!("PDF generated successfully");
            println!("PDF: {}", pdf_path.display());
        } else {
            eprintln!(
                "Error: latexmk succeeded but PDF was not found: {}",
                pdf_path.display()
            );
        }
    } else {
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
    }
}