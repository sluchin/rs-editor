;;; kmacro.scm --- the ring of keyboard macros -*- scheme -*-

;; Emacs の kmacro.el の, マクロの環 (kmacro-ring), 数え上げの設定, キーへの割り当て, 行ごとの実行.
;; 起動のときに読む (core/src/scheme/loadup.rs). マクロを覚える・動かす基本は lisp/simple.scm.
;; 今のマクロ (`last-kbd-macro`, `kmacro-counter`, `kmacro-counter-format`) は環の先頭で, 環 (`kmacro-ring`) は,
;; その次からのマクロ (キー, 数え上げ, 書式の 3 つ組) を新しい順に持つ.

(define kmacro-ring '()
  "The keyboard macros before the last one, the newest first.  Each is (KEYS COUNTER FORMAT).")

(define kmacro-ring-max 8
  "Maximum number of keyboard macros to save in the macro ring.")

(define kmacro--bound 0)

(define (kmacro--head)
  "Return the last keyboard macro as (KEYS COUNTER FORMAT)."
  (list last-kbd-macro kmacro-counter kmacro-counter-format))

(define (kmacro--install entry)
  "Make ENTRY, a (KEYS COUNTER FORMAT), the last keyboard macro."
  (set! last-kbd-macro (car entry))
  (set! kmacro-counter (cadr entry))
  (set! kmacro-counter-format (caddr entry)))

(define (kmacro--take items count)
  "Return the first COUNT elements of ITEMS."
  (if (or (null? items) (<= count 0)) '() (cons (car items) (kmacro--take (cdr items) (- count 1)))))

(define (kmacro--push-last)
  "Put the last keyboard macro on the ring before a new one is defined."
  (when last-kbd-macro
    (set! kmacro-ring (kmacro--take (cons (kmacro--head) kmacro-ring) kmacro-ring-max))))

(define (kmacro--check-ring)
  "Signal an error if there is no other keyboard macro."
  (if (null? kmacro-ring) (error "No other keyboard macros")))

(define-command (kmacro-cycle-ring-next . args)
  "Move to the next keyboard macro in the ring: the last macro goes to the end of the ring."
  (kmacro--check-ring)
  (let ((head (kmacro--head)) (next (car kmacro-ring)))
    (set! kmacro-ring (append (cdr kmacro-ring) (list head)))
    (kmacro--install next)
    (message "Keyboard macro: %s" (kmacro--describe)))
  #f)

(define-command (kmacro-cycle-ring-previous . args)
  "Move to the previous keyboard macro in the ring: the last macro goes to the front of the ring."
  (kmacro--check-ring)
  (let* ((head (kmacro--head))
         (items (reverse kmacro-ring))
         (previous (car items)))
    (set! kmacro-ring (cons head (reverse (cdr items))))
    (kmacro--install previous)
    (message "Keyboard macro: %s" (kmacro--describe)))
  #f)

(define-command (kmacro-swap-ring . args)
  "Swap the first two keyboard macros: the last macro and the one after it in the ring."
  (kmacro--check-ring)
  (let ((head (kmacro--head)) (second (car kmacro-ring)))
    (set! kmacro-ring (cons head (cdr kmacro-ring)))
    (kmacro--install second))
  #f)

(define-command (kmacro-delete-ring-head . args)
  "Delete the last keyboard macro; the next one in the ring becomes the last."
  (if (null? kmacro-ring)
      (set! last-kbd-macro #f)
      (begin (kmacro--install (car kmacro-ring))
             (set! kmacro-ring (cdr kmacro-ring))))
  #f)

(define-command (kmacro-call-ring-2nd . args)
  "Call the second keyboard macro of the ring (the one after the last), without changing the ring.
A prefix argument serves as a repeat count; zero means repeat until error."
  (kmacro--check-ring)
  (execute-kbd-macro (car (car kmacro-ring))
                     (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args)))
  #f)

(define (kmacro--describe)
  "Return the keys of the last keyboard macro as one string."
  (if last-kbd-macro (string-join last-kbd-macro " ") "(empty)"))

(define-command (kmacro-view-macro . args)
  "Show the keys of the last keyboard macro in the echo area."
  (if last-kbd-macro
      (message "Macro: %s" (kmacro--describe))
      (error "No keyboard macro defined"))
  #f)

(define-command (kmacro-set-counter . args)
  "Set the keyboard macro counter to the number (the prefix argument, or asked for)."
  (set! kmacro-counter (if (null? args)
                           (let ((arg (current-prefix-arg)))
                             (if arg (prefix-numeric-value arg) (read-number "Macro counter value: " 0)))
                           (car args)))
  #f)

(define-command (kmacro-add-counter . args)
  "Add the prefix argument (or the number asked for) to the keyboard macro counter."
  (set! kmacro-counter (+ kmacro-counter
                          (if (null? args)
                              (let ((arg (current-prefix-arg)))
                                (if arg (prefix-numeric-value arg) (read-number "Add to macro counter: " 1)))
                              (car args))))
  #f)

(define-command (kmacro-set-format . args)
  "Set the format of the keyboard macro counter (a format string with %d; empty means %d)."
  (let ((format-string (if (null? args) (read-string "Macro Counter Format: ") (car args))))
    (set! kmacro-counter-format (if (equal? format-string "") "%d" format-string)))
  #f)

(define-command (kmacro-bind-to-key . args)
  "Bind the last keyboard macro to a key (the argument, or asked for), for all buffers."
  (if (not last-kbd-macro)
      (error "No keyboard macro defined")
      (let ((key (if (null? args) (read-string "Bind last macro to key (for example C-c a): ") (car args)))
            (name (begin (set! kmacro--bound (+ kmacro--bound 1))
                         (string->symbol (format "kmacro-macro-%d" kmacro--bound)))))
        (eval (list 'define-command (list name)
                    "Keyboard macro."
                    (list 'execute-kbd-macro (list 'quote last-kbd-macro) 1)
                    #f))
        (global-set-key key name)
        (message "Bound %s to %s" key key)))
  #f)

(define-command (apply-macro-to-region-lines . args)
  "Run the last keyboard macro on each line of the region (from the beginning of the line).
The macro runs after the command, one line after another, from the first line down."
  (if (not last-kbd-macro)
      (error "No keyboard macro defined")
      (let* ((start (if (null? args) (region-beginning) (car args)))
             (end (if (null? args) (region-end) (cadr args)))
             (lines (max 1 (count-lines start end)))
             (keys (let loop ((n lines) (all '()))
                     (if (= n 0)
                         all
                         (loop (- n 1) (append (list "C-a") last-kbd-macro (list "C-n") all))))))
        (goto-char start)
        (execute-kbd-macro keys 1)))
  #f)

(define-command (kbd-macro-query . args)
  "Query user during kbd macro execution.
While a keyboard macro is being defined, this only records the key; when the macro runs, it asks
whether to go on: `y' or SPC goes on, any other key stops the macro.
With a prefix argument while defining, Emacs enters a recursive edit; that is not supported here."
  (cond ((executing-kbd-macro-p)
         (kbd-macro--ask "Proceed with macro? (y or SPC to go on, anything else stops) "))
        ((kbd-macro-defining-p)
         (if (current-prefix-arg) (error "A recursive edit is not supported")))
        (else (error "Not defining or executing kbd macro")))
  #f)

(define kmacro--edit-header
  ";; Keyboard Macro Editor.  Press C-c C-c to finish; the keys are in the notation of `kbd'.\n;; Comment lines start with `;;'.\n\nMacro:\n\n"
  "The text before the keys in the *Edit Macro* buffer.")

(define-command (kmacro-edit-macro . args)
  "Edit the last keyboard macro in the *Edit Macro* buffer: its keys are text, such as `C-x C-f foo RET'.
Press C-c C-c in the buffer to make the text the last keyboard macro."
  (unless last-kbd-macro (error "No keyboard macro defined"))
  (switch-to-buffer "*Edit Macro*")
  (erase-buffer)
  (insert kmacro--edit-header (string-join last-kbd-macro " ") "\n")
  (goto-char (point-max))
  (local-set-key "C-c C-c" 'kmacro-edit-finish)
  (message "Press C-c C-c to finish editing the macro")
  #f)

(define-command (edit-kbd-macro . args)
  "Edit a keyboard macro: the last one (`kmacro-edit-macro'); named macros are not supported."
  (kmacro-edit-macro))

(define (kmacro--edited-keys)
  "Return the keys of the text of the *Edit Macro* buffer, without the comment lines and the label."
  (let ((lines (filter (lambda (line)
                         (not (or (string-prefix? ";;" line) (equal? line "Macro:"))))
                       (split-string (buffer-string) "\n"))))
    (kbd (string-join lines " "))))

(define-command (kmacro-edit-finish . args)
  "Make the text of the *Edit Macro* buffer the last keyboard macro, and close the buffer."
  (unless (equal? (buffer-name) "*Edit Macro*")
    (error "Not in the *Edit Macro* buffer"))
  (let ((keys (kmacro--edited-keys)))
    (set! last-kbd-macro (if (null? keys) #f keys))
    (kill-buffer "*Edit Macro*")
    (message (if last-kbd-macro "Macro changed" "Macro is empty"))
    #f))

(define (kmacro--commands keys)
  "Split KEYS, a list of keys, into the key sequences of one command each.
A sequence grows while the keymap says that it is only the beginning of a longer one."
  (let loop ((rest keys) (current '()) (all '()))
    (cond ((null? rest)
           (reverse (if (null? current) all (cons current all))))
          (else
           (let* ((next (append current (list (car rest))))
                  (binding (key-binding (string-join next " "))))
             (if (and (equal? binding #t) (not (null? (cdr rest))))
                 (loop (cdr rest) next all)
                 (loop (cdr rest) '() (cons next all))))))))

(define-command (kmacro-step-edit-macro . args)
  "Run the last keyboard macro one command at a time, asking before each one.
SPC or `y' runs the command, `n' skips it, `!' runs all the rest, and `q' or C-g stops."
  (unless last-kbd-macro (error "No keyboard macro defined"))
  (let loop ((steps (kmacro--commands last-kbd-macro)) (all #f))
    (unless (null? steps)
      (let* ((step (car steps))
             (answer (if all
                         "y"
                         (read-key (string-append "Next: " (string-join step " ")
                                                  "  (SPC run, n skip, ! rest, q stop) ")
                                   #t))))
        (cond ((or (equal? answer " ") (equal? answer "y"))
               (execute-kbd-macro step 1)
               (loop (cdr steps) all))
              ((equal? answer "!")
               (execute-kbd-macro step 1)
               (loop (cdr steps) #t))
              ((equal? answer "n")
               (loop (cdr steps) all))
              (else (message "Stepping stopped"))))))
  #f)
