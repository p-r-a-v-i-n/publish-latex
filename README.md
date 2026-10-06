## publish-latex

`publish-latex` is a small Rust CLI and GitHub Action for compiling LaTeX documents into PDFs.

The goal is to automate the workflow of compiling frequently updated LaTeX documents such as resumes, reports, and CVs.

### GitHub Action

Add a workflow to the repository containing your LaTeX document:

```yaml
name: Build LaTeX

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - uses: YOUR-GITHUB-USERNAME/publish-latex@main
        with:
          source: resume.tex
```