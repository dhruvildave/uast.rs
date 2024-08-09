# UAST: Unicode Aware Saṃskṛta Transliteration

> [!IMPORTANT]
> This is a minimalist Rust implementation. Please prefer using the full implementation at https://uast.dev for
> web-version or https://github.com/aneri0x4f/uast-cli for CLI version.

A tool for transliterating and typing Saṃskṛta in the easiest
computer, language, and human-friendly way

To install this program, you will need to install [Rust](https://rust-lang.org)

After that, simply run:

```bash
make
```

To remove,

```bash
make clean
```

If you use this repository, please cite the following paper:

```bibtex
@misc{uast_2022,
  doi = {10.48550/ARXIV.2203.14277},
  url = {https://arxiv.org/abs/2203.14277},
  author = {Dalwadi, Aneri and Dave, Dhruvil},
  keywords = {Human-Computer Interaction (cs.HC), FOS: Computer and information sciences, H.5.2},
  title = {UAST: Unicode Aware Sanskrit Transliteration},
  publisher = {arXiv},
  year = {2022},
  copyright = {Creative Commons Attribution 4.0 International}
}
```

![poster](/uast_poster.png)

Devanāgarī is the writing system that is adapted by various languages like Sanskrit (IAST:
_saṃskṛta_). International Alphabet of Sanskrit Transliteration (IAST) is a transliteration scheme for romanisation of
Sanskrit language. IAST makes use of diacritics to represent various characters. On a computer, these are represented
using Unicode standard which differs from how the Sanskrit language behaves at a very fundamental level. This results in
an issue that is encountered while designing typesetting software for _devanāgarī_ and IAST. We hereby discuss the
problems and provide a solution that solves the issue of incompatibilities between various transliteration and encoding
schemes.

Web version URL: https://uast.dev

Click on the encoding name for available options.