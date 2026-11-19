//! 組み込みの手続きの docstring (Emacs に同じ名前が無いものと, 動きが Emacs と違うもの).
//!
//! Emacs と同じ名前で動きが同じものは, `tools/emacs-commands/builtin_docs.py` が作る `emacs_docs.rs` に置きます.

use crate::scheme::emacs_docs;

/// 組み込みの手続きの docstring を返します (この表が先で, 次に Emacs の docstring).
///
/// # Arguments
///
/// * `name` - 手続きの名前.
///
/// # Returns
///
/// docstring (英語). 無ければ [`None`].
pub fn lookup(name: &str) -> Option<&'static str> {
    find(DOCS, name).or_else(|| find(emacs_docs::DOCS, name))
}

/// 名前で並んだ表から, docstring を引きます.
fn find(table: &'static [(&'static str, &'static str)], name: &str) -> Option<&'static str> {
    table
        .binary_search_by_key(&name, |(key, _)| key)
        .ok()
        .map(|index| table[index].1)
}

/// 名前と docstring (名前の順). `emacs_docs::DOCS` より先に引きます.
pub static DOCS: &[(&str, &str)] = &[
    ("all-key-bindings", "Return all the key bindings of the editor.\n\n(all-key-bindings)"),
    ("beginning-of-defun-position", "Return the position that `beginning-of-defun' moves to from POSITION.\n\n(beginning-of-defun-position POSITION &optional ARG)"),
    ("boolean?", "Return #t if OBJECT is a boolean (#t or #f).\n\n(boolean? OBJECT)"),
    ("buffer-read-only", "Return #t if the current buffer is read-only.\n\n(buffer-read-only)"),
    ("buffer-undo-list", "Return the undo list of the current buffer.\n\n(buffer-undo-list)"),
    ("call-shell-command", "Run COMMAND in a shell and return its output.\n\n(call-shell-command COMMAND)"),
    ("casify-region", "Convert the text between START and END according to MODE (upcase, downcase or capitalize).\n\n(casify-region MODE START END)"),
    ("comment-dwim-op", "Internal helper of `comment-dwim'.\n\n(comment-dwim-op MARK COLUMN INDENT-TABS-MODE)"),
    ("comment-region-op", "Internal helper of `comment-region'.\n\n(comment-region-op FROM TO INDENT-TABS-MODE)"),
    ("current-prefix-arg", "Return the prefix argument of the current command.\n\n(current-prefix-arg)"),
    ("deactivate-mark-requested", "Return #t if a command asked to deactivate the mark.\n\n(deactivate-mark-requested)"),
    ("directory-listing", "Return the names in the directory DIRECTORY, one per line, as a string.\n\n(directory-listing DIRECTORY)"),
    ("editor-command-names", "Return the names of the commands defined by the screen (front end).\n\n(editor-command-names)"),
    ("end-of-defun-position", "Return the position that `end-of-defun' moves to from POSITION.\n\n(end-of-defun-position POSITION ARG SCAN-ERRORS)"),
    ("eq?", "Return #t if A and B are the same object (numbers and symbols by value).\n\n(eq? A B)"),
    ("equal?", "Return #t if A and B have the same structure and contents.\n\n(equal? A B)"),
    ("eqv?", "Return #t if A and B are equivalent as primitive values.\n\n(eqv? A B)"),
    ("ert-delete-all-tests", "Delete all the tests defined by `ert-deftest'.\n\n(ert-delete-all-tests)"),
    ("ert-delete-test", "Delete the test named NAME.\n\n(ert-delete-test NAME)"),
    ("ert-fail", "Make the running test fail with DATA.\n\n(ert-fail DATA)"),
    ("ert-skip", "Skip the running test.\n\n(ert-skip DATA)"),
    ("ert-test-boundp", "Return #t if a test named NAME is defined.\n\n(ert-test-boundp NAME)"),
    ("even?", "Return #t if the integer N is even.\n\n(even? N)"),
    ("executing-kbd-macro-p", "Return #t while a keyboard macro is being executed.\n\n(executing-kbd-macro-p)"),
    ("file-directory?", "Return #t if FILENAME names a directory.\n\n(file-directory? FILENAME)"),
    ("file-exists?", "Return #t if the file FILENAME exists.\n\n(file-exists? FILENAME)"),
    ("file-local-mode", "Return the major mode that the file name of the current buffer selects.\n\n(file-local-mode)"),
    ("file-modification-time", "Return the modification time of FILENAME, in seconds since the epoch.\n\n(file-modification-time FILENAME)"),
    ("files--buffer-backed-up-p", "Return #t if the current buffer was already backed up.\n\n(files--buffer-backed-up-p)"),
    ("files--set-buffer-backed-up", "Record that the current buffer was backed up.\n\n(files--set-buffer-backed-up VALUE)"),
    ("files--set-buffer-file-name", "Set the file name of the current buffer to FILENAME.\n\n(files--set-buffer-file-name FILENAME)"),
    ("files--set-buffer-text", "Replace the text of the current buffer with TEXT.\n\n(files--set-buffer-text TEXT MODIFIED)"),
    ("fill-paragraph-op", "Internal helper of `fill-paragraph'.\n\n(fill-paragraph-op FILL-COLUMN FILL-PREFIX INDENT-TABS-MODE)"),
    ("fill-text-op", "Internal helper of `fill-region'.\n\n(fill-text-op START END FILL-COLUMN FILL-PREFIX INDENT-TABS-MODE)"),
    ("filter", "Return the elements of LIST for which PROCEDURE returns true.\n\n(filter PROCEDURE LIST)"),
    ("for-each", "Call PROCEDURE on each element of LIST, for effect.\n\n(for-each PROCEDURE LIST)"),
    ("forward-paragraph-position", "Return the position that `forward-paragraph' moves to from POSITION by N paragraphs.\n\n(forward-paragraph-position POSITION N)"),
    ("forward-sentence-position", "Return the position that `forward-sentence' moves to from POSITION by N sentences.\n\n(forward-sentence-position POSITION N)"),
    ("forward-word-position", "Return the position that `forward-word' moves to from POSITION by N words.\n\n(forward-word-position POSITION N)"),
    ("global-mark-pop", "Pop the global mark ring and return the entry.\n\n(global-mark-pop)"),
    ("grep-files", "Search the files under DIRECTORY and return the matching lines.\n\n(grep-files DIRECTORY)"),
    ("gui-selection-value", "Return the text of the system clipboard.\n\n(gui-selection-value)"),
    ("highlighted-regexps", "Return the regular expressions highlighted by `hi-lock'.\n\n(highlighted-regexps)"),
    ("indent-for-tab-op", "Internal helper of `indent-for-tab-command'.\n\n(indent-for-tab-op INDENT-TABS-MODE)"),
    ("indent-region-op", "Internal helper of `indent-region'.\n\n(indent-region-op START END INDENT-TABS-MODE)"),
    ("indent-sexp-op", "Internal helper of `indent-sexp'.\n\n(indent-sexp-op INDENT-TABS-MODE)"),
    ("integer?", "Return #t if OBJECT is an integer.\n\n(integer? OBJECT)"),
    ("kbd-macro-defining-p", "Return #t while a keyboard macro is being defined.\n\n(kbd-macro-defining-p)"),
    ("kbd-macro-last", "Return the keys of the last keyboard macro.\n\n(kbd-macro-last)"),
    ("kill-ring-length", "Return the number of entries in the kill ring.\n\n(kill-ring-length)"),
    ("list-ref", "Return the element of LIST at index K (counting from 0).\n\n(list-ref LIST K)"),
    ("list?", "Return #t if OBJECT is a proper list.\n\n(list? OBJECT)"),
    ("major-mode-name", "Return the name of the major mode of the current buffer.\n\n(major-mode-name)"),
    ("map", "Apply PROCEDURE to each element of LIST and return the list of results.\n\n(map PROCEDURE LIST)"),
    ("mark-defun-range", "Return the range that `mark-defun' marks.\n\n(mark-defun-range POSITION EXTEND ARG)"),
    ("marker-release", "Release the marker MARKER so that it is no longer updated.\n\n(marker-release MARKER)"),
    ("marker-set", "Point the marker MARKER at POSITION.\n\n(marker-set MARKER POSITION)"),
    ("message-log", "Return the text of the `*Messages*' log.\n\n(message-log)"),
    ("modulo", "Return the remainder of N1 divided by N2, with the sign of N2.\n\n(modulo N1 N2)"),
    ("newline-and-indent-op", "Internal helper of `newline'.  Insert a newline and indent the new line, like `electric-indent-mode'.\n\n(newline-and-indent-op INDENT-TABS-MODE)"),
    ("null?", "Return #t if OBJECT is the empty list.\n\n(null? OBJECT)"),
    ("number->string", "Return the decimal representation of the number N as a string.\n\n(number->string N)"),
    ("number?", "Return #t if OBJECT is a number.\n\n(number? OBJECT)"),
    ("odd?", "Return #t if the integer N is odd.\n\n(odd? N)"),
    ("outline-flag-region", "Hide (FLAG true) or show (FLAG false) the text between FROM and TO.\n\n(outline-flag-region FROM TO FLAG)"),
    ("outline-invisible-p", "Return #t if the text at POSITION is hidden by outline mode.\n\n(outline-invisible-p POSITION)"),
    ("pair?", "Return #t if OBJECT is a pair (a cons cell).\n\n(pair? OBJECT)"),
    ("plain-newline-and-indent-op", "Internal helper of `newline-and-indent'.  Delete the horizontal space before point, insert a newline and indent the new line only.\n\n(plain-newline-and-indent-op INDENT-TABS-MODE)"),
    ("procedure?", "Return #t if OBJECT is a procedure.\n\n(procedure? OBJECT)"),
    ("quit-requested-p", "Return #t if the user asked to quit (`C-g') while Lisp code was running.\n\n(quit-requested-p)"),
    ("quotient", "Return the integer quotient of N1 divided by N2, truncated toward zero.\n\n(quotient N1 N2)"),
    ("read-file-string", "Return the contents of the file FILENAME as a string.\n\n(read-file-string FILENAME)"),
    ("rectangle-mark-p", "Return #t if the region is a rectangle (`rectangle-mark-mode').\n\n(rectangle-mark-p)"),
    ("regexp-match", "Match the regular expression REGEXP against STRING and return the matched strings, or #f.\n\n(regexp-match REGEXP STRING)"),
    ("regexp-match-positions", "Like `regexp-match', but return the start and end positions.\n\n(regexp-match-positions REGEXP STRING)"),
    ("remainder", "Return the remainder of N1 divided by N2, with the sign of N1.\n\n(remainder N1 N2)"),
    ("restriction-restore", "Restore the narrowing saved by `restriction-save'.\n\n(restriction-restore)"),
    ("restriction-save", "Save the narrowing of the current buffer.\n\n(restriction-save)"),
    ("rust-grep", "Return the lines of FILES that match the regular expression PATTERN.\n\n(rust-grep PATTERN FILES)"),
    ("rust-zgrep", "Like `rust-grep', but expand .gz files before searching.\n\n(rust-zgrep PATTERN FILES)"),
    ("scan-error-reason", "Return the message of the scan error ERROR.\n\n(scan-error-reason ERROR)"),
    ("set-buffer-directory", "Set the default directory of the current buffer.\n\n(set-buffer-directory DIRECTORY)"),
    ("set-buffer-read-only", "Make the current buffer read-only (VALUE true) or writable (VALUE false).\n\n(set-buffer-read-only VALUE)"),
    ("set-buffer-undo-list", "Set the undo list of the current buffer to LIST.\n\n(set-buffer-undo-list LIST)"),
    ("set-current-prefix-arg", "Set the prefix argument of the current command to ARG.\n\n(set-current-prefix-arg ARG)"),
    ("set-deactivate-mark", "Ask (VALUE true) or cancel (VALUE false) the deactivation of the mark after the command.\n\n(set-deactivate-mark VALUE)"),
    ("set-major-mode-name", "Set the name of the major mode of the current buffer.\n\n(set-major-mode-name NAME)"),
    ("set-mark-position", "Set the position of the mark to POSITION.\n\n(set-mark-position POSITION)"),
    ("set-rectangle-mark", "Treat the region as a rectangle (VALUE true) or as ordinary text (VALUE false).\n\n(set-rectangle-mark VALUE)"),
    ("set-syntax-dialect", "Set the syntax dialect (elisp, scheme or fundamental) of the current buffer.\n\n(set-syntax-dialect DIALECT)"),
    ("set-tab-bar", "Set the names and the current one of the tabs shown by the screen.\n\n(set-tab-bar NAMES CURRENT)"),
    ("split-root-window", "Split the root window of the frame in two.\n\n(split-root-window DIRECTION)"),
    ("string->number", "Return the number STRING denotes, or #f if it is not a number.\n\n(string->number STRING)"),
    ("string->symbol", "Return the symbol whose name is STRING.\n\n(string->symbol STRING)"),
    ("string->utf8", "Return the UTF-8 bytes of STRING as a list of integers.\n\n(string->utf8 STRING)"),
    ("string-append", "Return the concatenation of the STRINGS.\n\n(string-append &rest STRINGS)"),
    ("string-downcase", "Return STRING converted to lower case.\n\n(string-downcase STRING)"),
    ("string-length", "Return the number of characters in STRING.\n\n(string-length STRING)"),
    ("string-match--op", "Internal helper of `string-match'.\n\n(string-match--op REGEXP STRING START)"),
    ("string-prefix?", "Return #t if STRING begins with PREFIX.\n\n(string-prefix? PREFIX STRING)"),
    ("string-suffix?", "Return #t if STRING ends with SUFFIX.\n\n(string-suffix? SUFFIX STRING)"),
    ("string-upcase", "Return STRING converted to upper case.\n\n(string-upcase STRING)"),
    ("string<?", "Return #t if the string A sorts before B.\n\n(string<? A B)"),
    ("string=?", "Return #t if the strings are equal.\n\n(string=? A B)"),
    ("string?", "Return #t if OBJECT is a string.\n\n(string? OBJECT)"),
    ("symbol->string", "Return the name of SYMBOL as a string.\n\n(symbol->string SYMBOL)"),
    ("symbol?", "Return #t if OBJECT is a symbol.\n\n(symbol? OBJECT)"),
    ("syntax-class-at", "Return the syntax class of the character at POSITION.\n\n(syntax-class-at POSITION)"),
    ("syntax-dialect", "Return the syntax dialect of the current buffer.\n\n(syntax-dialect)"),
    ("toggle-project-tree", "Show or hide the project tree of the screen.\n\n(toggle-project-tree)"),
    ("token-spans", "Return the syntax highlighting spans of the current buffer.\n\n(token-spans)"),
    ("up-list-position", "Return the position that `up-list' moves to from POSITION by N levels.\n\n(up-list-position POSITION N)"),
    ("window-balance", "Make all the windows of the frame the same size.\n\n(window-balance)"),
    ("write-file-string", "Write STRING to the file FILENAME.\n\n(write-file-string FILENAME STRING)"),
    ("zero?", "Return #t if the number N is zero.\n\n(zero? N)"),
];
