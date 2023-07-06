# cite-key-files 

GitHub action to verify Bibtex cite keys match a file

## Example

The following action checks whether all Bibtex cite keys are found as files in the directory of the biblography.

```yaml
name: Verify cite keys
on: push

jobs:
  verify_cite_keys:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v3
      - name: Verify cite keys
        uses: emanuelbuholzer/cite-key-files@cite-key-files
        with:
          bibliography: literature/bibliography.bib
```