;;; abbrev.scm --- abbrev commands -*- scheme -*-

;; Emacs の abbrev.el に当たるもの (`expand-abbrev`, `add-global-abbrev`, `abbrev-prefix-mark` など).
;; 略語の表は, 全てのバッファで 1 つの `local-abbrev-table` と `global-abbrev-table` (メジャーモードごとの表と
;; abbrev-mode (打つと自動で展開する) は無い). 展開は `expand-abbrev` を呼んだときだけ.

(define (make-abbrev-table)
  "Create a new, empty abbrev table.
An abbrev table is called with an operation (get, put, clear or names) and its arguments."
  (let ((entries '()))
    (lambda (op . args)
      (cond ((eq? op 'get) (assoc (car args) entries))
            ((eq? op 'put)
             (set! entries (cons (list (car args) (cadr args) (caddr args) (list-ref args 3))
                                 (filter (lambda (e) (not (equal? (car e) (car args)))) entries)))
             #f)
            ((eq? op 'clear) (set! entries '()) #f)
            (else (map car entries))))))

(define (clear-abbrev-table table)
  "Undefine all abbrevs in abbrev table TABLE, leaving TABLE empty."
  (table 'clear))

(define global-abbrev-table (make-abbrev-table)
  "The abbrev table whose abbrevs affect all buffers.")

(define local-abbrev-table (make-abbrev-table)
  "Local (mode-specific) abbrev table.")

(define only-global-abbrevs #f
  "Non-nil means user plans to use global abbrevs only.
This makes the commands that normally define mode-specific abbrevs define global abbrevs instead.")

(define abbrev-all-caps #f
  "Non-nil means expand multi-word abbrevs in all caps if the abbrev was so.")

(define abbrev-start-location #f
  "Buffer position for `expand-abbrev' to use as the start of the abbrev.
When nil, use the word before point as the abbrev.
Calling `expand-abbrev' sets this to nil.")

(define last-abbrev #f
  "The abbrev of the last abbrev expanded.")

(define last-abbrev-text #f
  "The exact text of the last abbrev that was expanded.")

(define last-abbrev-location 0
  "The location of the start of the last abbrev that was expanded.")

(define (abbrev--active-tables table)
  "Return the abbrev tables to look in: TABLE only, or the local and the global table."
  (if table (list table) (list local-abbrev-table global-abbrev-table)))

(define (abbrev--entry abbrev table)
  "Return the entry of ABBREV in TABLE (name expansion hook count), folding case, or #f if it has no expansion."
  (let ((entry (or (table 'get abbrev) (table 'get (string-downcase abbrev)))))
    (and entry (or (cadr entry) (caddr entry)) entry)))

(define (abbrev--symbol abbrev table)
  "Return the entry of ABBREV in the TABLE (or the active tables), or #f."
  (let loop ((tables (abbrev--active-tables table)))
    (cond ((null? tables) #f)
          ((abbrev--entry abbrev (car tables)))
          (else (loop (cdr tables))))))

(define (abbrev-expansion abbrev . table)
  "Return the string that ABBREV expands into in the current buffer.
Optionally specify an abbrev TABLE as second arg; then ABBREV is looked up in that table only."
  (let ((entry (abbrev--symbol abbrev (and (pair? table) (car table)))))
    (and entry (cadr entry))))

(define (define-abbrev table abbrev expansion . rest)
  "Define ABBREV in TABLE, to expand into EXPANSION and optionally call HOOK.
ABBREV must be a string, and should be lower-case.
To undefine an abbrev, define it with EXPANSION = nil.
If HOOK is non-nil, it should be a function of no arguments;
it is called after EXPANSION is inserted."
  (table 'put abbrev expansion (and (pair? rest) (car rest)) 0)
  abbrev)

(define (abbrev--add table type arg)
  "Define an abbrev in TABLE whose expansion is ARG words before point; read the abbrev from the minibuffer."
  (let ((exp (cond ((or (and (not arg) (use-region-p))
                        (= (prefix-numeric-value arg) 0))
                    (buffer-substring (region-beginning) (region-end)))
                   ((> (prefix-numeric-value arg) 0)
                    (let ((end (save-excursion (forward-word (- (prefix-numeric-value arg))) (point))))
                      (buffer-substring (min (point) end) (max (point) end))))
                   (else #f))))
    (let ((name (read-string (if exp
                                 (format "%s abbrev that expands into \"%s\": " type exp)
                                 (format "Undefine %s abbrev: " type)))))
      (if (or (not exp)
              (not (abbrev-expansion name table))
              (y-or-n-p (format "%s expands into \"%s\"; redefine? " name (abbrev-expansion name table))))
          (define-abbrev table (string-downcase name) exp))))
  #f)

(define-command (add-mode-abbrev . args)
  "Define a mode-specific abbrev whose expansion is the last word before point.
If there's an active region, use that as the expansion.

Prefix argument ARG says how many words before point to use for the expansion;
zero means the entire region is the expansion.

A negative ARG means to undefine the specified abbrev.

This command reads the abbreviation from the minibuffer."
  (abbrev--add (if only-global-abbrevs global-abbrev-table local-abbrev-table) "Mode"
               (if (null? args) (current-prefix-arg) (car args))))

(define-command (add-global-abbrev . args)
  "Define a global (all modes) abbrev whose expansion is last word before point.
If there's an active region, use that as the expansion.

Prefix argument ARG says how many words before point to use for the expansion;
zero means the entire region is the expansion.

A negative ARG means to undefine the specified abbrev.

This command reads the abbreviation from the minibuffer."
  (abbrev--add global-abbrev-table "Global" (if (null? args) (current-prefix-arg) (car args))))

(define (abbrev--inverse-add table type n)
  "Define the Nth word before point as an abbrev in TABLE, reading the expansion, and expand it."
  (let* ((bounds (save-excursion
                   (forward-word (+ 1 (- n)))
                   (skip-syntax-backward "^w")
                   (let ((end (point)))
                     (backward-word 1)
                     (list (point) end))))
         (start (car bounds))
         (end (cadr bounds))
         (name (buffer-substring start end))
         (exp (read-string (format "Expansion for %s abbrev \"%s\": " type name))))
    (if (or (not (abbrev-expansion name table))
            (y-or-n-p (format "%s expands into \"%s\"; redefine? " name (abbrev-expansion name table))))
        (begin
          (define-abbrev table (string-downcase name) exp)
          (save-excursion
            (goto-char end)
            (abbrev--expand)))))
  #f)

(define-command (inverse-add-mode-abbrev . args)
  "Define the word before point as a mode-specific abbreviation.
With prefix argument N, define the Nth word before point as the
abbreviation.  Negative N means use the Nth word after point.

If `only-global-abbrevs' is non-nil, this command defines a
global (mode-independent) abbrev instead of a mode-specific one.

This command reads the expansion from the minibuffer, defines the
abbrev, and then expands the abbreviation in the current buffer."
  (abbrev--inverse-add (if only-global-abbrevs global-abbrev-table local-abbrev-table) "Mode"
                       (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args))))

(define-command (inverse-add-global-abbrev . args)
  "Define the word before point as a global (mode-independent) abbreviation.
With prefix argument N, define the Nth word before point as the
abbreviation.  Negative N means use the Nth word after point.

This command reads the expansion from the minibuffer, defines the
abbrev, and then expands the abbreviation in the current buffer."
  (abbrev--inverse-add global-abbrev-table "Global"
                       (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args))))

(define-command (abbrev-prefix-mark . args)
  "Mark point as the beginning of an abbreviation.
The abbrev to be expanded starts at point rather than at the
beginning of a word.  This way, you can expand an abbrev with
a prefix: insert the prefix, use this command, then insert the
abbrev.

This command inserts a hyphen after the prefix, and if the abbrev
is subsequently expanded, this hyphen will be removed.

If the prefix is itself an abbrev, this command expands it,
unless ARG is non-nil.  Interactively, ARG is the prefix
argument."
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (if (not arg) (abbrev--expand))
    (set! abbrev-start-location (point))
    (insert "-"))
  #f)

(define (abbrev--before-point)
  "Try and find an abbrev before point.  Return (ENTRY NAME START END) if found, #f otherwise."
  (let ((pos (point)))
    (if abbrev-start-location
        (let ((start abbrev-start-location))
          (set! abbrev-start-location #f)
          ;; Remove the hyphen inserted by `abbrev-prefix-mark'.
          (when (and (< start (point-max)) (equal? (char-after start) "-"))
            (delete-region start (+ start 1))
            (set! pos (- pos 1)))
          (skip-syntax-backward " ")
          (let ((end (point)))
            (goto-char pos)
            (and (> end start)
                 (let ((name (buffer-substring start end)))
                   (let ((entry (abbrev--symbol name #f)))
                     (and entry (list entry name start end)))))))
        (let* ((lim pos)
               (bounds (begin
                         (condition-case e (backward-word 1) (error #f))
                         (let ((start (point)))
                           (condition-case e (forward-word 1) (error #f))
                           (list start (min (point) lim)))))
               (name (buffer-substring (car bounds) (cadr bounds)))
               (entry (and (> (string-length name) 0) (abbrev--symbol name #f))))
          (goto-char pos)
          (and entry (list entry name (car bounds) (cadr bounds)))))))

(define (abbrev--has-lower? name) (pair? (regexp-match-positions "\\p{Ll}" name)))
(define (abbrev--has-upper? name) (pair? (regexp-match-positions "\\p{Lu}" name)))

(define (abbrev--insert entry name wordstart wordend)
  "Replace the text from WORDSTART to WORDEND with the expansion of ENTRY (found as NAME).
Return #t."
  (let ((expansion (cadr entry)))
    (when (string? expansion)
      (goto-char wordstart)
      (insert expansion)
      (delete-char (- wordend wordstart))
      ;; The name differs from the abbrev only in capitalization: adjust the expansion.
      (when (and (not (equal? name (car entry))) (abbrev--has-upper? name))
        (if (not (abbrev--has-lower? name))
            (if (and (not abbrev-all-caps)
                     (save-excursion
                       (> (begin (backward-word 1) (point))
                          (begin (goto-char wordstart) (forward-word 1) (point)))))
                (upcase-initials-region wordstart (point))
                (upcase-region wordstart (point)))
            (let ((end (point)))
              (goto-char wordstart)
              (skip-syntax-forward "^w" (- end 1))
              (upcase-initials-region (point) (+ (point) 1))
              (goto-char end))))))
  (if (caddr entry) ((caddr entry)))
  #t)

(define (abbrev--expand)
  "Expand the abbrev before point, if there is an abbrev there.  Return #t if one was expanded."
  (let ((found (abbrev--before-point)))
    (and found
         (let* ((entry (car found)) (name (cadr found)) (wordstart (caddr found)) (wordend (list-ref found 3))
                (trailing (- (point) wordend)))
           (set! last-abbrev-text name)
           (set! last-abbrev (car entry))
           (set! last-abbrev-location wordstart)
           (let ((done (abbrev--insert entry name wordstart wordend)))
             ;; If point was further than the end of the abbrev, move it back to where it started.
             (when (> trailing 0)
               (goto-char (+ (point) trailing)))
             done)))))

(define-command (expand-abbrev . args)
  "Expand the abbrev before point, if there is an abbrev there.
Effective when explicitly called even when `abbrev-mode' is nil."
  (abbrev--expand)
  #f)
