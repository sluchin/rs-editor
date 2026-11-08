;;; compile.scm --- compile, grep and next-error -*- scheme -*-

;; Emacs の grep.el と compile.el に当たるもの (`compile`, `recompile`, `grep`, `next-error`, `previous-error`,
;; `compile-goto-error`).
;; 結果のバッファ (`*grep*`, `*xref*`) は `ファイル:行:行の文字` の行を並べ, `next-error` はその行を順にたどる.
;; `*compilation*` は, コマンドの出力のうち, `compilation-error-regexp-alist` に合う行をたどる.
;; コマンドは, 終わるまで待つ (Emacs は, 裏で動かす).

(define next-error-last-buffer #f
  "The name of the buffer `next-error' reads the locations from, or #f.")

(define next-error-directory ""
  "The directory the file names of `next-error-last-buffer' are relative to.")

(define next-error--start 1
  "The position where the locations of `next-error-last-buffer' start.")

(define next-error--end 1
  "The position where the locations of `next-error-last-buffer' end.")

(define compile--sources '()
  "The buffers with locations: each is (NAME DIRECTORY START END KIND), where KIND is grep, compilation or occur.")

(define (compile--set-source name directory start end . kind)
  "Make the buffer NAME the source of `next-error': its locations lie between START and END.
KIND is grep (the default; lines `FILE:LINE:TEXT') or compilation (lines that match
`compilation-error-regexp-alist')."
  (set! compile--sources
        (cons (list name directory start end (if (pair? kind) (car kind) 'grep))
              (filter (lambda (source) (not (equal? (car source) name))) compile--sources)))
  (compile--use-source name))

(define (compile--source name)
  "Return the entry of `compile--sources' for the buffer NAME, or #f."
  (assoc name compile--sources))

(define (compile--use-source name)
  "Make the buffer NAME `next-error-last-buffer', with its directory and region of locations."
  (let ((source (compile--source name)))
    (set! next-error-last-buffer name)
    (when source
      (set! next-error-directory (list-ref source 1))
      (set! next-error--start (list-ref source 2))
      (set! next-error--end (list-ref source 3)))))

(define compilation-error-regexp-alist
  '(("^ *File \"([^\"\n]+)\", line ([0-9]+)" 1 2 #f)
    ("^([^:\n \t0-9][^:\n]*|[0-9]+[^0-9:\n][^:\n]*):([0-9]+)(?:[.:]([0-9]+))?:" 1 2 3))
  "The messages `compile' and `next-error' find in the compilation output.
Each is (REGEXP FILE LINE COLUMN): the numbers of the groups of the file name, the line and the
column (#f if there is none).  The default forms are those of Python (`File \"FILE\", line LINE')
and GNU (`FILE:LINE:COL:', `FILE:LINE.COL:' or `FILE:LINE:').  For the messages of Rust, add
(\"^ *--> ([^:\\n ]+):([0-9]+):([0-9]+)\" 1 2 3).")

(define (compile--compilation-hit text)
  "Return (FILE LINE COLUMN) if the line TEXT is a message of `compilation-error-regexp-alist', or #f."
  (let loop ((rules compilation-error-regexp-alist))
    (if (null? rules)
        #f
        (let* ((rule (car rules))
               (found (regexp-match (car rule) text)))
          (if found
              (let ((column (and (list-ref rule 3) (list-ref found (list-ref rule 3)))))
                (list (list-ref found (cadr rule))
                      (string->number (list-ref found (caddr rule)))
                      (and column (string->number column))))
              (loop (cdr rules)))))))

(define (compile--hit text)
  "Return (FILE LINE) if the line TEXT is `FILE:LINE:TEXT', or #f."
  (let ((found (regexp-match "^([^:]+):([0-9]+):" text)))
    (and found (list (cadr found) (string->number (caddr found))))))

(define (compile--line-hit)
  "Return the location on the line at point, or #f if the line is outside the locations."
  (let ((source (compile--source (buffer-name))))
    (if source
        (and (>= (line-beginning-position) (list-ref source 2))
             (<= (line-end-position) (list-ref source 3))
             ((if (eq? (list-ref source 4) 'compilation) compile--compilation-hit compile--hit)
              (buffer-substring (line-beginning-position) (line-end-position))))
        (and (>= (line-beginning-position) next-error--start)
             (<= (line-end-position) next-error--end)
             (compile--hit (buffer-substring (line-beginning-position) (line-end-position)))))))

(define (compile--kind name)
  "Return the kind (grep or compilation) of the locations of the buffer NAME."
  (let ((source (compile--source name)))
    (if source (list-ref source 4) 'grep)))

(define (compile--scan forward)
  "Move point to the next (or previous) line with a location and return it.
If there is none, leave point where it was and return #f."
  (let ((origin (point)))
    (let loop ()
      (let ((pos (if forward (+ (line-end-position) 1) (- (line-beginning-position) 1))))
        (cond ((or (> pos (point-max)) (< pos (point-min)))
               (goto-char origin)
               #f)
              (else
               (goto-char pos)
               (let ((hit (compile--line-hit)))
                 (cond (hit (goto-char (line-beginning-position)) hit)
                       (else (loop))))))))))

(define (compile--default-directory)
  "Return the directory `grep' starts from (that of the Dired buffer if dired.scm is loaded, else of the visited file)."
  (if (fboundp 'dired--default-directory)
      (dired--default-directory)
      (let ((file (buffer-file-name)))
        (or (and file (file-name-directory file)) "~/"))))

(define (compile--visit hit other-window)
  "Visit the file, line and column (if any) of HIT, in another window if OTHER-WINDOW."
  (let* ((file (car hit))
         (path (if (string-prefix? "/" file) file (string-append next-error-directory file)))
         (buffer (find-file-noselect path)))
    (if other-window
        (window--pop-to-buffer-other-window buffer)
        (pop-to-buffer-same-window buffer))
    (goto-char (point-min))
    (forward-line (- (cadr hit) 1))
    (if (and (pair? (cddr hit)) (caddr hit))
        (move-to-column (max 0 (- (caddr hit) 1))))))

(define (compile--past-message kind forward)
  (if (eq? kind 'compilation)
      (if forward "Past last error" "Moved back before first error")
      (if forward "Moved past last grep hit" "Moved back before first grep hit")))

(define (compile--locus-message kind)
  "Show `KIND locus from BUFFER' like the `next-error' of Emacs (`next-error-verbose')."
  (message (format "%s locus from %s" kind next-error-last-buffer)))

(define (compile--next-error forward . args)
  "Visit the next (or previous) location of `next-error-last-buffer'.
With a number in ARGS, move that many locations; with a third element true, start from the first."
  (let ((name next-error-last-buffer)
        (count (if (pair? args) (car args) 1))
        (reset (and (pair? args) (pair? (cdr args)) (cadr args))))
    (if (not (and name (get-buffer name)))
        (user-error "No buffers contain error message locations"))
    (compile--use-source name)
    (if (eq? (compile--kind name) 'occur)
        (let ((target (with-current-buffer name (occur-next-error (if forward count (- count))))))
          (if (equal? (buffer-name) name)
              (window--pop-to-buffer-other-window (car target))
              (pop-to-buffer-same-window (car target)))
          (goto-char (cadr target)))
        (compile--next-error-1 name forward count))
    (compile--locus-message (cond (reset "First") ((eq? forward (< count 0)) "Previous") (else "Next")))
    #f))

(define (compile--next-error-1 name forward count)
  "Visit the COUNTth next (or previous) location of the grep or compilation results in the buffer NAME."
  (let* ((forward (if (< count 0) (not forward) forward))
         (hit (with-current-buffer name
                (let loop ((n (max 1 (abs count))) (hit #f))
                  (if (= n 0)
                      hit
                      (let ((next (compile--scan forward)))
                        (if next (loop (- n 1) next) #f)))))))
    (if (not hit)
        (user-error (compile--past-message (compile--kind name) forward)))
    (compile--visit hit (equal? (buffer-name) name))))

(define-command (next-error . args)
  "Visit the next location of the compilation, grep (or xref) results.
With a numeric argument N, move N locations (backward if N is negative)."
  (compile--next-error #t (simple--arg args)))

(define-command (previous-error . args)
  "Visit the previous location of the compilation, grep (or xref) results.
With a numeric argument N, move N locations back (forward if N is negative)."
  (compile--next-error #f (simple--arg args)))

(define-command (first-error . args)
  "Visit the first location of the compilation, grep (or xref) results."
  (let ((name next-error-last-buffer))
    (if (not (and name (get-buffer name)))
        (user-error "No buffers contain error message locations"))
    (with-current-buffer name (goto-char (point-min)))
    (compile--next-error #t 1 #t)))

(define (compile--no-select thunk)
  "Call THUNK (that visits a location), then select the window of `next-error-last-buffer' again."
  (let ((name next-error-last-buffer))
    (thunk)
    (if (and name (get-buffer name)) (window--pop-to-buffer-other-window name))))

(define-command (next-error-no-select . args)
  "Move point to the next location in the results, and display the source in another window.
With a numeric argument N, move N locations."
  (let ((n (simple--arg args)))
    (compile--no-select (lambda () (compile--next-error #t n)))))

(define-command (previous-error-no-select . args)
  "Move point to the previous location in the results, and display the source in another window.
With a numeric argument N, move N locations back."
  (let ((n (simple--arg args)))
    (compile--no-select (lambda () (compile--next-error #f n)))))

(define-command (compile-goto-error . args)
  "Visit the location on the current line of the results, in another window."
  (let ((hit (compile--line-hit)))
    (if (not hit)
        (user-error "No error here"))
    (compile--use-source (buffer-name))
    (compile--visit hit #t)
    (compile--locus-message "Current")
    #f))

;;; compile

(define compile-command "make -k "
  "The last shell command used to do a compilation; default for next compilation.")

(define compilation-read-command #t
  "True means `compile' reads the compilation command to use (otherwise, only with a prefix argument).")

(define compilation-directory #f
  "The directory of the last `compile' (or of the compilation buffer), used by `recompile'.")

(define compile--directory #f
  "The directory `compile' runs in instead of that of the current buffer, or #f (`project-compile' sets it).")

(define compilation-arguments #f
  "The command of the last compilation, as a list (COMMAND), or #f.")

(define (compile--duration elapsed)
  "Return the duration ELAPSED (seconds) as Emacs writes it: `0.12 s', `12.3 s' or `1:02:03'."
  (cond ((< elapsed 10) (format "%s s" (compile--fixed elapsed 100)))
        ((< elapsed 60) (format "%s s" (compile--fixed elapsed 10)))
        (else (let* ((s (quotient elapsed 1)) (h (quotient s 3600)) (m (quotient (modulo s 3600) 60)))
                (format "%d:%s:%s" h (compile--pad m) (compile--pad (modulo s 60)))))))

(define (compile--pad n)
  (if (< n 10) (format "0%d" n) (format "%d" n)))

(define (compile--fixed x scale)
  "Return X rounded to 1/SCALE (10 or 100), with the digits after the point."
  (let* ((n (quotient (+ (* x scale) 0.5) 1))
         (frac (remainder n scale)))
    (format "%d.%s" (quotient n scale) (if (and (= scale 100) (< frac 10)) (format "0%d" frac) (format "%d" frac)))))

(define (compile--read-command command)
  (read-string "Compile command: " command 'compile-history))

(define (compile--save-buffers)
  "Ask whether to save the modified file buffers, if there are any."
  (if (pair? (filter (lambda (name) (and (buffer-file-name name) (buffer-modified-p name))) (buffer-list)))
      (save-some-buffers)))

(define (compile--directory-name dir)
  (if (string-suffix? "/" dir) dir (string-append dir "/")))

(define (compilation-start command)
  "Run COMMAND in the shell and show its output in the *compilation* buffer.
The output shows in another window; `next-error' visits the messages in it.  Return the buffer name."
  (let* ((name "*compilation*")
         (directory (compile--directory-name (expand-file-name (or compilation-directory (compile--default-directory)))))
         (start-time (current-time))
         (start 1)
         (end 1)
         (result #f))
    (get-buffer-create name)
    (with-current-buffer name
      (set-buffer-read-only #f)
      (erase-buffer)
      (insert (format "-*- mode: compilation; default-directory: %S -*-\n" (grep--home-name directory)))
      (insert (format "Compilation started at %s\n\n" (format-time-string "%a %b %e %H:%M:%S")))
      (insert command "\n")
      (set! start (point))
      (set! result (call-shell-command command #f directory))
      (insert (cadr result))
      (if (not (bolp)) (insert "\n"))
      (set! end (point))
      (let ((status (if (equal? (car result) 0)
                        "finished"
                        (format "exited abnormally with code %d" (car result)))))
        (insert (format "\nCompilation %s at %s, duration %s\n"
                        status (format-time-string "%a %b %e %H:%M:%S")
                        (compile--duration (- (current-time) start-time))))
        (goto-char (point-min))
        (set-buffer-modified-p #f)
        (set-buffer-read-only #t)
        (compilation-mode)
        (compile--set-source name directory start end 'compilation)
        (set! compilation-directory directory)
        (set! compilation-arguments (list command))
        (display-buffer name)
        (message (format "Compilation %s" status))))
    name))

(define-command (compile . args)
  "Compile the program including the current buffer.  Default: run `make'.
Runs COMMAND, a shell command, in the directory of the current buffer, and shows the
output in the buffer `*compilation*'.  You can then use the command \\[next-error]
to find the next error message and move to the source code that caused it.
Interactively, prompts for the command if the variable `compilation-read-command' is
true; otherwise uses `compile-command'.  With a prefix argument, always prompts."
  (let ((command (cond ((pair? args) (car args))
                       ((or compilation-read-command (current-prefix-arg))
                        (compile--read-command compile-command))
                       (else compile-command))))
    (set! compile-command command)
    (compile--save-buffers)
    (set! compilation-directory compile--directory)
    (compilation-start command)
    #f))

(define-command (recompile . args)
  "Re-compile the program including the current buffer.
Run the command of the last compilation again in its directory (otherwise, `compile-command'
in the directory of the current buffer).  With a prefix argument, edit the command first."
  (let* ((edit (if (pair? args) (car args) (current-prefix-arg)))
         (command (if compilation-arguments (car compilation-arguments) compile-command))
         (command (if edit (compile--read-command command) command)))
    (compile--save-buffers)
    (compilation-start command)
    #f))

(define (compile--scan-in-buffer forward count)
  "Move point to the COUNTth next (or previous) message in this buffer, without visiting it."
  (let ((origin (point)))
    (let loop ((n count))
      (cond ((= n 0) #f)
            ((compile--scan forward) (loop (- n 1)))
            (else (goto-char origin)
                  (user-error (compile--past-message (compile--kind (buffer-name)) forward)))))))

(define-command (compilation-next-error . args)
  "Move point to the next error in the compilation buffer.
With a numeric argument N, move N errors (backward if N is negative).
Does not visit the source."
  (if (not (compile--source (buffer-name))) (error "Not in a compilation buffer"))
  (let ((n (simple--arg args)))
    (compile--scan-in-buffer (>= n 0) (abs n))))

(define-command (compilation-previous-error . args)
  "Move point to the previous error in the compilation buffer.
With a numeric argument N, move N errors back (forward if N is negative).
Does not visit the source."
  (if (not (compile--source (buffer-name))) (error "Not in a compilation buffer"))
  (let ((n (simple--arg args)))
    (compile--scan-in-buffer (< n 0) (abs n))))

(define (compile--file-step forward)
  "Move to the first message (going FORWARD or back) about another file than the message at point."
  (let ((origin (point))
        (file (let ((hit (compile--line-hit))) (and hit (car hit)))))
    (let loop ()
      (let ((hit (compile--scan forward)))
        (cond ((not hit)
               (goto-char origin)
               (user-error (compile--past-message (compile--kind (buffer-name)) forward)))
              ((equal? (car hit) file) (loop))
              (else #f))))))

(define-command (compilation-next-file . args)
  "Move point to the next error for a different file than the current one."
  (if (not (compile--source (buffer-name))) (error "Not in a compilation buffer"))
  (let ((n (simple--arg args)))
    (let loop ((i (abs n))) (when (> i 0) (compile--file-step (>= n 0)) (loop (- i 1))))))

(define-command (compilation-previous-file . args)
  "Move point to the previous error for a different file than the current one."
  (if (not (compile--source (buffer-name))) (error "Not in a compilation buffer"))
  (let ((n (simple--arg args)))
    (let loop ((i (abs n))) (when (> i 0) (compile--file-step (< n 0)) (loop (- i 1))))))

(define-command (compilation-display-error . args)
  "Display the source for current error in another window, without selecting it."
  (let ((name (buffer-name)))
    (compile--no-select (lambda () (compile-goto-error)))
    (set! next-error-last-buffer name)))

(define-command (compilation-mode . args)
  "Major mode for compilation log buffers.
RET visits the source of the error message on the line, n and p move to the next and previous
messages and show their source, M-n and M-p (also TAB and S-TAB) only move, M-{ and M-}
move to the messages of another file, g runs the compilation again, and q quits the window."
  (set-major-mode-name 'compilation-mode)
  (for-each (lambda (binding) (local-set-key (car binding) (cadr binding)))
            '(("RET" compile-goto-error) ("C-c C-c" compile-goto-error)
              ("C-o" compilation-display-error)
              ("n" next-error-no-select) ("p" previous-error-no-select)
              ("M-n" compilation-next-error) ("M-p" compilation-previous-error)
              ("TAB" compilation-next-error) ("S-TAB" compilation-previous-error)
              ("M-{" compilation-previous-file) ("M-}" compilation-next-file)
              ("g" recompile) ("q" quit-window)
              ("SPC" scroll-up-command) ("DEL" scroll-down-command)
              ("<" beginning-of-buffer) (">" end-of-buffer))))

(define (grep--home-name path)
  "Return PATH with the home directory written as ~."
  (let ((home (getenv "HOME")))
    (if (and home (string-prefix? (string-append home "/") path))
        (string-append "~" (substring path (string-length home) (string-length path)))
        path)))

(define (grep--files directory)
  "Return the files below DIRECTORY, not those in .git, target, or node_modules."
  (grep-files directory))

(define (grep--time)
  (format-time-string "%a %b %e %H:%M:%S %Y"))

(define (grep--show directory pattern hits)
  "Show HITS ((FILE LINE TEXT) ...) in the *grep* buffer, in another window; return the buffer name."
  (let ((start 1)
        (end 1)
        (count (length hits)))
    (with-current-buffer (get-buffer-create "*grep*")
      (set-buffer-read-only #f)
      (erase-buffer)
      (insert (format "-*- mode: grep; default-directory: \"%s\" -*-\n" (grep--home-name directory)))
      (insert (format "Grep started at %s\n\n" (grep--time)))
      (insert (format "grep (built-in) -r -e %s .\n" pattern))
      (set! start (point))
      (for-each (lambda (hit)
                  (insert (format "%s:%d:%s\n" (car hit) (cadr hit) (caddr hit))))
                hits)
      (set! end (point))
      (insert (format "\nGrep finished with %s found at %s\n"
                      (cond ((= count 0) "no matches") ((= count 1) "1 match") (else (format "%d matches" count)))
                      (grep--time)))
      (goto-char (point-min))
      (set-buffer-modified-p #f)
      (set-buffer-read-only #t)
      (for-each (lambda (binding) (local-set-key (car binding) (cadr binding)))
                '(("RET" compile-goto-error) ("C-o" compilation-display-error)
                  ("n" next-error-no-select) ("p" previous-error-no-select)
                  ("M-n" compilation-next-error) ("M-p" compilation-previous-error)
                  ("TAB" compilation-next-error) ("S-TAB" compilation-previous-error)
                  ("M-{" compilation-previous-file) ("M-}" compilation-next-file)
                  ("g" grep-again) ("q" quit-window)
                  ("SPC" scroll-up-command) ("DEL" scroll-down-command)
                  ("<" beginning-of-buffer) (">" end-of-buffer))))
    (compile--set-source "*grep*" directory start end)
    (display-buffer "*grep*")
    "*grep*"))

(define grep--last #f
  "The arguments of the last search, (PATTERN DIRECTORY GLOB RECURSIVE), for `grep-again'.")

(define (grep--glob->regexp glob)
  "Return a regexp that matches a file name matching the shell wildcard GLOB (* ? and literal characters)."
  (string-append
   "^"
   (let loop ((i 0) (result ""))
     (if (= i (string-length glob))
         result
         (let ((c (substring glob i (+ i 1))))
           (loop (+ i 1)
                 (string-append result
                                (cond ((equal? c "*") "[^/]*")
                                      ((equal? c "?") "[^/]")
                                      (else (regexp-quote c))))))))
   "$"))

(define (grep--glob-match globs name)
  "Return true if NAME matches one of GLOBS (a list of wildcards)."
  (let loop ((globs globs))
    (cond ((null? globs) #f)
          ((regexp-match (grep--glob->regexp (car globs)) name) #t)
          (else (loop (cdr globs))))))

(define (grep--select files directory glob recursive)
  "Return the FILES below DIRECTORY that match GLOB (wildcards separated by spaces); only those directly in it unless RECURSIVE."
  (let ((globs (filter (lambda (g) (not (equal? g ""))) (split-string glob " "))))
    (filter (lambda (file)
              (let ((relative (substring file (string-length directory) (string-length file))))
                (and (or recursive (not (string-search "/" relative)))
                     (grep--glob-match globs (file-name-nondirectory file)))))
            files)))

(define (grep--run pattern dir glob recursive)
  "Search for PATTERN in the files matching GLOB in DIR (and below, if RECURSIVE), and show them in *grep*."
  (let* ((directory (let ((expanded (expand-file-name dir)))
                      (if (string-suffix? "/" expanded) expanded (string-append expanded "/"))))
         (hits (map (lambda (hit)
                      (list (substring (car hit) (string-length directory) (string-length (car hit)))
                            (cadr hit)
                            (caddr hit)))
                    (rust-grep pattern
                               (grep--select (grep--files directory) directory glob recursive)))))
    (set! grep--last (list pattern dir glob recursive))
    (grep--show directory pattern hits)
    (message (format "Grep finished with %s found"
                     (cond ((null? hits) "no matches") ((null? (cdr hits)) "1 match") (else (format "%d matches" (length hits))))))
    #f))

(define-command (grep . args)
  "Search the files below a directory for lines that match a regexp.
Interactively, read the regexp and the directory in the minibuffer.
The matches are listed in the *grep* buffer; \\[next-error] visits them."
  (let* ((pattern (if (pair? args) (car args) (read-string "Run grep (regexp): ")))
         (dir (if (and (pair? args) (pair? (cdr args)))
                  (cadr args)
                  (read-file-name "In directory: " (compile--default-directory)))))
    (grep--run pattern dir "*" #t)))

(define-command (rgrep . args)
  "Search the files matching GLOB below a directory for lines that match a regexp.
Interactively, read the regexp, the file name wildcards (separated by spaces) and the directory."
  (let* ((pattern (if (pair? args) (car args) (read-string "Search for (regexp): ")))
         (glob (if (and (pair? args) (pair? (cdr args)))
                   (cadr args)
                   (read-string "Files (wildcards): " "*")))
         (dir (if (and (pair? args) (pair? (cdr args)) (pair? (cddr args)))
                  (caddr args)
                  (read-file-name "Base directory: " (compile--default-directory)))))
    (grep--run pattern dir glob #t)))

(define-command (lgrep . args)
  "Search the files matching GLOB in a directory (not below it) for lines that match a regexp.
Interactively, read the regexp, the file name wildcards (separated by spaces) and the directory."
  (let* ((pattern (if (pair? args) (car args) (read-string "Search for (regexp): ")))
         (glob (if (and (pair? args) (pair? (cdr args)))
                   (cadr args)
                   (read-string "Files (wildcards): " "*")))
         (dir (if (and (pair? args) (pair? (cdr args)) (pair? (cddr args)))
                  (caddr args)
                  (read-file-name "Directory: " (compile--default-directory)))))
    (grep--run pattern dir glob #f)))

(define-command (grep-again . args)
  "Run the last grep, rgrep or lgrep again."
  (if grep--last
      (apply grep--run grep--last)
      (error "No grep to run again")))

(provide 'compile)

;;; compile.scm ends here
