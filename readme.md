A simple command-line tool to organize files by their extensions into categorized folders.
Features

Automatically sorts files into categories (Images, Documents, Videos, Audio, Code, etc.)
Dry run mode to preview changes before organizing
List files by category without moving them

# Dry run to see what would happen (recommended first step)
cargo run -- organize -s ~/Downloads --dry-run

# Actually organize the files
cargo run -- organize -s ~/Downloads

# List the contents of a folder
cargo run -- list -s ~/Downloads

# Categories
Files are organized into these folders:

Images - jpg, png, gif, svg, etc.
Documents - pdf, doc, txt, etc.
Videos - mp4, avi, mkv, etc.
Audio - mp3, wav, flac, etc.
Code - rs, py, js, java, etc.
Archives - zip, rar, 7z, etc.
Spreadsheets - xls, xlsx, csv, etc.
Presentations - ppt, pptx, etc.
Web - html, css, json, etc.
Executables - exe, msi, app, etc.
Others - everything else