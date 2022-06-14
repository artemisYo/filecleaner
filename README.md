# File Cleaner
... is a simple Rust app that recursively deletes all files that end on '~'.
There are probably thousands of such apps out there and they all are probably more customizable and/or faster, however I've made my own, since I want to get better at coding, and the best way to do that is by practice.
The code is small (40 lines) and does not feature any dependencies other than rusts path, filesystem and args standard library.
- - -
The code has a token constant, which determines which files are deleted.
It simply reads the last character of a file's name and, given it matches the "ENDING_TOKEN", deletes said file.
When using it you simply have to call the executable with the drop-in directory path as an argument.
