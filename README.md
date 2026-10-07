## publish-latex

`publish-latex` helps you keep your resume updated without keeping multiple PDF files locally.

What I noticed is that I had to use Overleaf, where I had to paste the LaTeX code, compile it, and then download the PDF. I had to repeat this process frequently whenever I updated my resume.

`publish-latex` generates a link to your resume where other people can view it directly.

You can make last-minute changes to your resume, and people will still have access to the latest version without needing you to share the PDF again.

### GitHub Action

Add a workflow to the repository containing your LaTeX document:

```yaml id="ml18wm"
name: Build Resume

on:
  push:
    branches:
      - main

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest

    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Build resume
        uses: YOUR-GITHUB-USERNAME/publish-latex@main
        with:
          source: resume.tex

      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

### Steps needed from your end

1. Your resume filename should currently be `resume.tex`, and it should be present at the root of your repository.

2. You need to enable GitHub Pages for the repository. Go to **Settings → Pages → Build and deployment**, and select **GitHub Actions** as the source.

3. If you are using GitHub Free, the repository needs to be public to use GitHub Pages.

4. Once deployed, you can access your resume at:

```text id="vwabtr"
https://githubusername.github.io/repo-name/document.pdf
```

For example:

```text id="mvrwo7"
https://john.github.io/resume/document.pdf
```

Whenever you update `resume.tex` and push the changes, the PDF will be rebuilt and the same link will show the latest version.

### Current status

This project is still in its early stage. Currently, it focuses on compiling a LaTeX resume and publishing the generated PDF through GitHub Pages.