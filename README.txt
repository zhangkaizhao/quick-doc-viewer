# Quick doc viewer

A quick documentation viewer for developers to preview documentations.

## Features:

* Sorted all file listing powered by [walkdir]
* Special files, e.g. README.*, SUMMARY.*, etc. as navigation
* Preview of Lightweight Markup documentations, currently supports Markdown
* Support some additional Markdown features e.g. GFM table, etc. powered by [comrak]
* Files can be previewed by selected format
* Some directories e.g. `.git`, etc. useless for preview are ignored
* Support encodings powered by [chardetng] and [encodings_rs]

[chardetng]: https://github.com/hsivonen/chardetng
[comrak]: https://github.com/kivikakk/comrak
[encodings_rs]: https://github.com/hsivonen/encoding_rs
[walkdir]: https://github.com/BurntSushi/walkdir

### Usage:

0. Build the `quick-doc-viewer` command line program:

    git clone https://github.com/zhangkaizhao/quick-doc-viewer.git
    cd quick-doc-viewer/
    cargo build --release

1. Run the built `quick-doc-viewer` command line program under the directory of documentations:

    cd /path/to/documentations/
    /path/to/quick-doc-viewer

2. Open the URL as follow in the web browser to start preview:

    http://127.0.0.1:8080/
