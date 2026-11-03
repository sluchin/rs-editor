;;; simple.scm --- basic editing commands -*- scheme -*-

;; Emacs の simple.el に当たるもの. ターミナルのアプリ (tui/) が読む.
;; コマンドは, 前置引数 (C-u, M-数字) があれば N に使う (Emacs の interactive "p" に当たる).
;; コマンドループは, 動かす前に変数 last-command (前のコマンド) と this-command を決める.

(define last-command #f
  "The name (a symbol) of the previous command.
A command can look at it, for example to append a kill to the previous one.")

(define this-command #f
  "The name (a symbol) of the running command.
A command can set it, so that the next command sees it as last-command.")

(define temporary-goal-column 0
  "The column that next-line and previous-line try to stay in.")

(define pending-undo-list #f
  "The undo records that are not undone yet, while undo is repeated.")

(define simple--interactive #f
  "True while a command is run from the keyboard (the terminal app sets it), like called-interactively-p.")

(define (simple--allow-extend args)
  "Return the ALLOW-EXTEND argument of a mark command: the second of ARGS, or true when called from the keyboard."
  (if (and (pair? args) (pair? (cdr args))) (cadr args) simple--interactive))

(define (simple--arg args)
  "Return the first of ARGS, or the numeric value of the prefix argument if ARGS is empty."
  (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args)))

;;; Moving

(define goal-column #f
  "The goal column of next-line and previous-line, or #f.
If it is a number, those commands move to that column of the line they move to.")

(define (simple--raw-arg args)
  "Return the first of ARGS, or the raw prefix argument (#f, a number, - or a list like (4)) if ARGS is empty."
  (if (null? args) (current-prefix-arg) (car args)))

(define simple--forward-char forward-char)
(define simple--backward-char backward-char)

(define-command (forward-char . args)
  "Move point N characters forward (backward if N is negative).
N is the prefix argument, or 1.  Signal an error at the end of the buffer."
  (simple--forward-char (simple--arg args)))

(define-command (backward-char . args)
  "Move point N characters backward (forward if N is negative).
N is the prefix argument, or 1.  Signal an error at the beginning of the buffer."
  (simple--backward-char (simple--arg args)))

(define-command (left-char . args)
  "Move point N characters to the left (to the right if N is negative).
The text is written from left to right, so this is backward-char."
  (backward-char (simple--arg args)))

(define-command (right-char . args)
  "Move point N characters to the right (to the left if N is negative).
The text is written from left to right, so this is forward-char."
  (forward-char (simple--arg args)))

(define (simple--move-line n)
  "Move point N lines, one line at a time, and to the goal column.
Signal an error if there are not N lines: point is then at the end of the buffer
(moving down) or at the beginning of the buffer (moving up)."
  (if (not (member last-command '(next-line previous-line)))
      (set! temporary-goal-column (current-column)))
  (let loop ((left (abs n)))
    (cond ((= left 0) (move-to-column (or goal-column temporary-goal-column)))
          ((> n 0)
           (when (= (line-end-position) (point-max))
             (goto-char (point-max))
             (error "End of buffer"))
           (forward-line 1)
           (loop (- left 1)))
          (else
           (when (= (line-beginning-position) (point-min))
             (goto-char (point-min))
             (error "Beginning of buffer"))
           (forward-line -1)
           (loop (- left 1))))))

(define-command (next-line . args)
  "Move point to the next line, staying in the same column if possible.
If goal-column is a number, move to that column instead.  Signal an error at the last line."
  (simple--move-line (simple--arg args)))

(define-command (previous-line . args)
  "Move point to the previous line, staying in the same column if possible.
If goal-column is a number, move to that column instead.  Signal an error at the first line."
  (simple--move-line (- (simple--arg args))))

(define-command (set-goal-column . args)
  "Set the current column as the goal column of next-line and previous-line.
With an argument, clear the goal column."
  (if (simple--raw-arg args)
      (begin
        (set! goal-column #f)
        (message "No goal column"))
      (begin
        (set! goal-column (current-column))
        (message (format "Goal column %d (use set-goal-column with an arg to unset it)" goal-column)))))

(define-command (move-beginning-of-line . args)
  "Move point to the beginning of the line.
With argument N other than 1, move forward N - 1 lines first."
  (let ((n (simple--arg args)))
    (if (not (= n 1)) (forward-line (- n 1)))
    (goto-char (line-beginning-position))))

(define-command (move-end-of-line . args)
  "Move point to the end of the line.
With argument N other than 1, move forward N - 1 lines first.
If there are not that many lines above, go to the beginning of the buffer."
  (let ((n (simple--arg args)))
    (if (and (not (= n 1)) (< (forward-line (- n 1)) 0))
        (goto-char (point-min))
        (goto-char (line-end-position)))))

(define-command (back-to-indentation . args)
  "Move point to the first non-whitespace character on this line."
  (goto-char (line-beginning-position))
  (looking-at "\\s-*")
  (goto-char (min (match-end 0) (line-end-position)))
  (backward-prefix-chars))

(define-command (beginning-of-buffer . args)
  "Move point to the beginning of the buffer.
With numeric argument N, put point N/10 of the way from the beginning.
Push the mark at the previous position, unless the argument is a list like (4)
or the mark is active."
  (let ((arg (simple--raw-arg args)))
    (or (pair? arg) (region-active-p) (push-mark (point)))
    (let ((size (- (point-max) (point-min))))
      (goto-char (if (and arg (not (pair? arg)))
                     (+ (point-min) 1 (quotient (* size (prefix-numeric-value arg)) 10))
                     (point-min))))
    (if (and arg (not (pair? arg))) (forward-line 1))))

(define-command (end-of-buffer . args)
  "Move point to the end of the buffer.
With numeric argument N, put point N/10 of the way from the end.
Push the mark at the previous position, unless the argument is a list like (4)
or the mark is active."
  (let ((arg (simple--raw-arg args)))
    (or (pair? arg) (region-active-p) (push-mark (point)))
    (let ((size (- (point-max) (point-min))))
      (goto-char (if (and arg (not (pair? arg)))
                     (- (point-max) (quotient (* size (prefix-numeric-value arg)) 10))
                     (point-max))))
    (if (and arg (not (pair? arg))) (forward-line 1))))

(define-command (forward-word . args)
  "Move point forward over N words (backward if N is negative)."
  (let ((n (simple--arg args)))
    (cond ((> n 0)
           (re-search-forward "\\W*\\w+" #f 'move)
           (forward-word (- n 1)))
          ((< n 0) (backward-word (- n))))))

(define-command (backward-word . args)
  "Move point backward over N words (forward if N is negative)."
  (let ((n (simple--arg args)))
    (cond ((> n 0)
           (when (not (bobp))
             (backward-char 1)
             (re-search-backward "\\<" #f 'move))
           (backward-word (- n 1)))
          ((< n 0) (forward-word (- n))))))

(define-command (left-word . args)
  "Move point N words to the left (to the right if N is negative).
The text is written from left to right, so this is backward-word."
  (backward-word (simple--arg args)))

(define-command (right-word . args)
  "Move point N words to the right (to the left if N is negative).
The text is written from left to right, so this is forward-word."
  (forward-word (simple--arg args)))

(define (simple--goto-line args prompt relative)
  "Go to the line given by ARGS (or the prefix argument, or read with PROMPT).
If RELATIVE, count from the beginning of the accessible part; otherwise widen first, then
widen the buffer for good if the line is outside the accessible part."
  (let ((n (cond ((pair? args) (car args))
                 ((current-prefix-arg) (prefix-numeric-value (current-prefix-arg)))
                 (else (string->number (read-string prompt))))))
    (or (region-active-p) (push-mark (point)))
    (let ((pos (save-restriction
                 (if (not relative) (widen))
                 (goto-char (point-min))
                 (forward-line (- n 1))
                 (point))))
      (if (and (not relative) (buffer-narrowed-p)
               (not (and (<= (point-min) pos) (<= pos (point-max)))))
          (widen))
      (goto-char pos))))

(define-command (goto-line . args)
  "Go to line N, counting from line 1 at the beginning of the buffer.
N is the argument or the numeric prefix argument; otherwise read it in the minibuffer.
Push the mark at the previous position, unless the mark is active.
If the line is outside the narrowed part, widen the buffer."
  (simple--goto-line args "Goto line: " #f))

(define-command (goto-line-relative . args)
  "Go to line N, counting from the line at the beginning of the accessible part.
N is the argument or the numeric prefix argument; otherwise read it in the minibuffer."
  (simple--goto-line args "Goto line relative: " #t))

(define-command (scroll-up-command . args)
  "Scroll the text of the window up by almost one window height.
Signal an error at the end of the buffer."
  (if (eobp) (error "End of buffer"))
  (forward-line (- (window-body-height) 2)))

(define-command (scroll-down-command . args)
  "Scroll the text of the window down by almost one window height.
Signal an error at the beginning of the buffer."
  (if (bobp) (error "Beginning of buffer"))
  (forward-line (- 2 (window-body-height))))

(define-command (recenter-top-bottom . args)
  "Put the line of point in the middle of the window."
  (recenter))

;;; Inserting and deleting

(define indent-tabs-mode #t
  "Non-nil means indentation can use tab characters.")

(define fill-prefix #f
  "The string that is inserted after a line break in filled text, or #f.")

(define (simple--spaces n)
  "Return a string of N spaces (the empty string if N is not positive)."
  (let loop ((n n) (result ""))
    (if (> n 0) (loop (- n 1) (string-append result " ")) result)))

(define (simple--indent-to column)
  "Insert tabs (if indent-tabs-mode) and spaces so that point gets to COLUMN."
  (let loop ()
    (let* ((current (current-column))
           (next-tab (* (+ (quotient current 8) 1) 8)))
      (cond ((and indent-tabs-mode (<= next-tab column))
             (insert "\t")
             (loop))
            ((< current column)
             (insert (simple--spaces (- column current))))))))

(define simple--move-to-column move-to-column)

(define (move-to-column column . rest)
  "Move point to column COLUMN in the current line, and return the column where it ends up.
The column of a tab or a wide character can be beyond COLUMN.
If the optional second argument FORCE is #t and the line is too short, add spaces (and tabs if
indent-tabs-mode) at the end of the line to reach COLUMN.  If FORCE is another true value and COLUMN
is in the middle of a tab, change the tab into spaces."
  (let* ((force (and (pair? rest) (car rest)))
         (reached (simple--move-to-column column)))
    (when (and force (> reached column) (equal? (char-before) "\t"))
      (backward-char 1)
      (let ((previous (current-column)))
        (if (< previous column)
            (begin
              ;; タブの前に桁までの空白を入れ, タブを消して, 元の桁まで字下げし直す
              (insert (simple--spaces (- column previous)))
              (delete-char 1)
              (let ((goal (point)))
                (simple--indent-to reached)
                (goto-char goal)
                (set! reached column)))
            (forward-char 1))))
    (when (and (< reached column) (eq? force #t))
      (simple--indent-to column)
      (set! reached column))
    reached))

(define-command (newline . args)
  "Insert N newlines.  N is the prefix argument, or 1.
Signal an error if N is negative."
  (let ((n (simple--arg args)))
    (if (< n 0) (error "Repetition argument has to be non-negative"))
    (if (and simple--interactive (null? args) (= n 1))
        ;; キーから 1 つ入れるときは, electric-indent-mode のように字下げする
        (newline-and-indent-op indent-tabs-mode)
        (let loop ((n n))
          (when (> n 0)
            (insert "\n")
            (loop (- n 1)))))))

(define-command (open-line . args)
  "Insert N newlines and leave point before them.  N is the prefix argument, or 1."
  (let ((n (simple--arg args))
        (location (point)))
    (newline n)
    (goto-char location)))

(define-command (split-line . args)
  "Split the current line, moving the part beyond point to a new line below.
The new line gets the same indentation as the column of point."
  (skip-chars-forward " \t")
  (let ((column (current-column))
        (here (point))
        (prefix (cond ((string? (simple--raw-arg args)) (simple--raw-arg args))
                      ((simple--raw-arg args) #f)
                      (else fill-prefix))))
    (let ((has-prefix (and prefix
                           (save-excursion
                             (goto-char (line-beginning-position))
                             (looking-at (regexp-quote prefix))))))
      (newline 1)
      (if has-prefix (insert prefix))
      (simple--indent-to column)
      (goto-char here))))

(define-command (indent-for-tab-command . args)
  "Indent the current line or the region, or insert whitespace at point.
In a Lisp buffer, fix the indentation of the line; with an active region, indent the region.
Elsewhere, move to the next indent point of the previous line (indent-relative)."
  (if (use-region-p)
      (indent-region (region-beginning) (region-end))
      (indent-for-tab-op indent-tabs-mode)))

(define-command (indent-region . args)
  "Indent each nonblank line in the region (the lines from START to END).
Without arguments, the region is used.  With a third argument COLUMN, indent every nonblank line to COLUMN."
  (cond ((null? args) (indent-region (region-beginning) (region-end)))
        ((and (pair? (cddr args)) (caddr args))
         ;; COLUMN: 空でない行を, 全て COLUMN に字下げする
         (let ((column (prefix-numeric-value (caddr args)))
               (tail (- (point-max) (max (car args) (cadr args)))))
           (save-excursion
             (goto-char (min (car args) (cadr args)))
             (unless (bolp) (forward-line 1))
             (let loop ()
               (when (< (point) (- (point-max) tail))
                 (delete-region (point) (begin (skip-chars-forward " \t") (point)))
                 (unless (eolp) (indent-to column 0))
                 (forward-line 1)
                 (loop))))
           (set-deactivate-mark #t)))
        (else
         (indent-region-op (car args) (cadr args) indent-tabs-mode)
         (set-deactivate-mark #t)
         (message "Indenting region...done"))))

(define-command (indent-sexp . args)
  "Indent each line of the list starting just after point (Lisp buffers only)."
  (indent-sexp-op indent-tabs-mode))

(define-command (indent-rigidly . args)
  "Indent all lines starting in the region by ARG columns (the prefix argument, default 1).
Without START and END, the region is used."
  (let* ((raw (simple--raw-arg (if (> (length args) 2) (cddr args) '())))
         (n (prefix-numeric-value raw))
         (start (if (null? args) (region-beginning) (car args)))
         (end (if (null? args) (region-end) (cadr args))))
    ;; 点は, 字下げの中なら, 新しい字下げの終わりへ. そうでなければ, 本文と一緒に動かす
    (let ((target (point)))
      (goto-char start)
      (if (not (bolp)) (forward-line 1))
      (let loop ((end end))
        (when (< (point) end)
          (let ((size (buffer-size))
                (bol (point)))
            (looking-at "[ \t]*")
            (let* ((space-end (match-end 0))
                   (blank (begin (goto-char space-end) (eolp)))
                   (indent (current-column)))
              (delete-region bol space-end)
              (goto-char bol)
              (if (not blank)
                  (simple--indent-to (max 0 (+ indent n))))
              (let ((delta (- (- (point) bol) (- space-end bol))))
                (cond ((> target space-end) (set! target (+ target delta)))
                      ((>= target bol) (set! target (point))))))
            (forward-line 1)
            (loop (+ end (- (buffer-size) size))))))
      (goto-char target))))

(define-command (tab-to-tab-stop . args)
  "Insert whitespace at point, then move to the next tab stop (a multiple of 8 columns)."
  (let ((next (* (+ (quotient (current-column) 8) 1) 8)))
    (simple--delete-space " \t" #t)
    (simple--indent-to next)))

(define-command (set-fill-prefix . args)
  "Set the fill prefix to the current line up to point.
At the beginning of a line, cancel the fill prefix."
  (let ((prefix (buffer-substring (line-beginning-position) (point))))
    (set! fill-prefix (if (equal? prefix "") #f prefix))
    (if fill-prefix
        (message (format "fill-prefix: \"%s\"" fill-prefix))
        (message "fill-prefix canceled"))))

(define-command (fill-paragraph . args)
  "Fill the paragraph at or after point (the comment paragraph in a Lisp buffer)."
  (fill-paragraph-op fill-column fill-prefix indent-tabs-mode))

(define-command (fill-region . args)
  "Fill each of the paragraphs in the region.
Called from Lisp with FROM and TO, fill the paragraphs between them."
  (let* ((from (min (if (null? args) (region-beginning) (car args))
                    (if (null? args) (region-end) (cadr args))))
         (to (max (if (null? args) (region-beginning) (car args))
                  (if (null? args) (region-end) (cadr args))))
         (tail (- (point-max) to)))
    (save-excursion
      (goto-char from)
      (let loop ()
        (skip-chars-forward "\n \t")
        (when (< (point) (- (point-max) tail))
          (fill-paragraph-op fill-column fill-prefix indent-tabs-mode)
          (forward-paragraph)
          (loop))))
    #f))

(define comment-column 40
  "Column to indent a comment to.")

(define-command (comment-dwim . args)
  "Comment or uncomment the region, or insert a comment or move to the one on this line."
  (let ((region (use-region-p)))
    (comment-dwim-op (and region (mark)) comment-column indent-tabs-mode)
    (if region (set-deactivate-mark #t))))

(define-command (comment-line . args)
  "Comment or uncomment the current line, then move to the next line.
With an active region, comment or uncomment the lines it covers instead."
  (if (use-region-p)
      (let ((from (save-excursion (goto-char (region-beginning)) (line-beginning-position)))
            (to (region-end)))
        (comment-region-op from to indent-tabs-mode)
        (set-deactivate-mark #t))
      (let ((from (line-beginning-position))
            (to (min (+ (line-end-position) 1) (+ (buffer-size) 1))))
        (comment-region-op from to indent-tabs-mode)
        (forward-line 1)
        (back-to-indentation))))

(define-command (comment-set-column . args)
  "Set the comment column to the argument, or to the current column."
  (let ((arg (simple--raw-arg args)))
    (set! comment-column (if (number? arg) arg (current-column)))
    (message (format "Comment column set to %d" comment-column))))

(define-command (default-indent-new-line . args)
  "Break the line at point and delete the spaces around the break.
The new line is not indented yet: indentation comes with the indentation commands."
  (newline 1)
  (save-excursion
    (backward-char 1)
    (simple--delete-space " \t" #f))
  (simple--delete-space " \t" #f))

(define simple--delete-char delete-char)

(define-command (delete-char . args)
  "Delete the following N characters (previous if N is negative).
Optional second arg KILLFLAG non-nil means to kill (save in kill ring) instead of delete.
Interactively, N is the prefix argument, and KILLFLAG is true if there is a prefix argument."
  (let ((n (simple--arg args))
        (kill (if (and (pair? args) (pair? (cdr args))) (cadr args) (and (null? args) (current-prefix-arg)))))
    (if kill
        (let ((target (+ (point) n)))
          (if (or (< target (point-min)) (> target (point-max)))
              (error "Args out of range")
              (simple--kill (point) target)))
        (simple--delete-char n))
    #f))

(define-command (delete-forward-char . args)
  "Delete the N characters after point (the N characters before point if N is negative)."
  (delete-char (simple--arg args)))

(define-command (delete-backward-char . args)
  "Delete the N characters before point (the N characters after point if N is negative)."
  (delete-char (- (simple--arg args))))

(define (simple--delete-space chars backward-only)
  "Delete the characters of CHARS around point (only before point if BACKWARD-ONLY is true)."
  (let* ((original (point))
         (end (if backward-only
                  original
                  (begin (skip-chars-forward chars) (point))))
         (start (begin (skip-chars-backward chars) (point))))
    (delete-region start end)))

(define-command (delete-horizontal-space . args)
  "Delete all spaces and tabs around point.
With an argument, delete them only before point."
  (simple--delete-space " \t" (simple--raw-arg args)))

(define (simple--fixup-whitespace)
  "Fix up white space between objects around point: leave no space or one space."
  (save-excursion
    (simple--delete-space " \t" #f)
    (if (or (looking-at "^\\|$\\|\\s)")
            (save-excursion
              (backward-char 1)
              (looking-at "$\\|\\s(\\|\\s'")))
        #f
        (insert " "))))

(define-command (just-one-space . args)
  "Delete all spaces and tabs around point, leaving one space (or N spaces).
If N is negative, delete newlines as well, leaving -N spaces."
  (let* ((n (simple--arg args))
         (chars (if (< n 0) " \t\n\r" " \t")))
    (skip-chars-backward chars)
    (let* ((count (- (abs n) (skip-chars-forward " " (+ (abs n) (point)))))
           (middle (point))
           (end (begin (skip-chars-forward chars) (point))))
      (delete-region middle end)
      (insert (simple--spaces count)))))

(define simple--cycle-context #f
  "The state of cycle-spacing: (original-point whitespace-string n last-action).")

(define (simple--cycle-restore)
  "Put back the white space cycle-spacing found."
  (simple--delete-space " \t\n\r" #f)
  (insert (list-ref simple--cycle-context 1))
  (goto-char (list-ref simple--cycle-context 0)))

(define-command (cycle-spacing . args)
  "Manipulate the white space around point in a cycle.
The first call leaves one space (just-one-space), the second deletes all the
white space, and the third restores it.  Repeat the command to cycle."
  (let ((n (simple--arg args)))
    (when (or (not (equal? last-command 'cycle-spacing)) (not simple--cycle-context))
      (let ((original (point)))
        (save-excursion
          (skip-chars-backward " \t\n\r")
          (let ((start (point)))
            (skip-chars-forward " \t\n\r")
            (set! simple--cycle-context
                  (list original (buffer-substring start (point)) n #f))))))
    (let* ((actions '(just-one-space delete-all-space restore))
           (last (list-ref simple--cycle-context 3))
           (action (cond ((not last) (car actions))
                         ((eq? last 'just-one-space) 'delete-all-space)
                         ((eq? last 'delete-all-space) 'restore)
                         (else 'just-one-space))))
      (simple--cycle-restore)
      (cond ((eq? action 'just-one-space) (just-one-space n))
            ((eq? action 'delete-all-space) (simple--delete-space " \t\n\r" #f)))
      (set! simple--cycle-context (list (list-ref simple--cycle-context 0)
                                        (list-ref simple--cycle-context 1)
                                        (list-ref simple--cycle-context 2)
                                        action)))))

(define-command (delete-indentation . args)
  "Join this line to the previous one and fix up the white space at the join.
With an argument, join the current line to the following line."
  (set-deactivate-mark #t)
  (let* ((arg (simple--raw-arg args))
         (beg (- (line-beginning-position (if arg 2 1)) 1)))
    (if arg (forward-line 1))
    (let loop ()
      (when (and (> (line-beginning-position) beg)
                 (forward-line 0)
                 (equal? (char-before) "\n"))
        (delete-char -1)
        (simple--fixup-whitespace)
        (loop)))))

(define-command (delete-blank-lines . args)
  "On a blank line, delete all the surrounding blank lines, leaving just one.
On an isolated blank line, delete that one.
On a nonblank line, delete any blank lines that follow it."
  (let ((this-blank #f) (single-blank #f))
    (save-excursion
      (goto-char (line-beginning-position))
      (set! this-blank (looking-at "[ \t]*$"))
      (set! single-blank
            (and this-blank
                 (not (looking-at "[ \t]*\n[ \t]*$"))
                 (or (bobp)
                     (begin (forward-line -1)
                            (not (looking-at "[ \t]*$")))))))
    (when this-blank
      (goto-char (line-beginning-position))
      (if single-blank (forward-line 1))
      (let ((from (point)))
        (delete-region from
                       (if (re-search-backward "[^ \t\n]" #f #t)
                           (begin (forward-line 1) (point))
                           (point-min)))))
    (when (not (and this-blank single-blank))
      (save-excursion
        (goto-char (line-end-position))
        (forward-line 1)
        (let ((from (point)))
          (delete-region from
                         (if (re-search-forward "[^ \t\n]" #f #t)
                             (begin (goto-char (line-beginning-position)) (point))
                             (point-max))))))
    (when (looking-at "^[ \t]*\n\\'")
      (delete-region (point) (point-max)))))

;;; Killing and yanking

(define (simple--kill from to)
  "Delete the text between FROM and TO and save it in the kill ring.
If the previous command was a kill, append the text to its kill."
  (let ((text (buffer-substring from to)))
    (delete-region from to)
    (if (equal? last-command 'kill-region)
        (kill-append text (< to from))
        (kill-new text))
    (set! this-command 'kill-region)
    (set-deactivate-mark #t)))

(define-command (kill-region . args)
  "Kill the text between point and the mark, or between the two positions FROM and TO.
The killed text is saved in the kill ring."
  (if (null? args)
      (simple--kill (region-beginning) (region-end))
      (simple--kill (car args) (cadr args))))

(define-command (kill-ring-save . args)
  "Save the region in the kill ring, without killing it.
Called from Lisp with START and END, save the text between them."
  (let ((start (if (null? args) (region-beginning) (car args)))
        (end (if (null? args) (region-end) (cadr args))))
    (kill-new (buffer-substring (min start end) (max start end)))
    (set-deactivate-mark #t)))

(define-command (kill-line . args)
  "Kill the rest of the line; if there are no nonblanks there, kill through the newline.
With an argument N, kill that many lines from point; a negative N kills backward,
and 0 kills the text before point on the line.  Signal an error at the end of the buffer."
  (let ((arg (simple--raw-arg args))
        (start (point)))
    (if arg
        (forward-line (prefix-numeric-value arg))
        (begin
          (if (eobp) (error "End of buffer"))
          (let ((end (line-end-position)))
            (if (save-excursion (skip-chars-forward " \t" end) (= (point) end))
                (forward-line 1)
                (goto-char end)))))
    (simple--kill start (point))))

(define (simple--marker position from to)
  "Return where POSITION is after the text from FROM to TO was deleted."
  (let ((low (min from to)) (high (max from to)))
    (cond ((<= position low) position)
          ((>= position high) (- position (- high low)))
          (else low))))

(define-command (kill-whole-line . args)
  "Kill the current line, with its newline.
With an argument N, kill that many lines from the current line; a negative N kills
backward, also killing the preceding newline.  With 0, kill the line without its newline."
  (let ((arg (if (null? args) (prefix-numeric-value (current-prefix-arg)) (or (car args) 1))))
    (if (and (> arg 0) (eobp) (save-excursion (forward-line 0) (eobp)))
        (error "End of buffer"))
    (if (and (< arg 0) (bobp) (save-excursion (goto-char (line-end-position)) (bobp)))
        (error "Beginning of buffer"))
    (when (not (equal? last-command 'kill-region))
      (kill-new "")
      (set! last-command 'kill-region))
    (let* ((begin-position (point))
           (end-position
            (cond ((= arg 0)
                   (let ((p (save-excursion (forward-line 0) (point))))
                     (goto-char (line-end-position))
                     p))
                  ((< arg 0)
                   (let ((p (save-excursion (goto-char (line-end-position)) (point))))
                     (forward-line (+ 1 arg))
                     (if (not (bobp)) (backward-char 1))
                     p))
                  (else
                   (let ((p (save-excursion (forward-line 0) (point))))
                     (forward-line arg)
                     p)))))
      (kill-region begin-position end-position)
      (kill-region (simple--marker begin-position begin-position end-position) (point)))))

;;; Keyboard macros

(define simple--start-kbd-macro start-kbd-macro)
(define last-kbd-macro #f
  "Last kbd macro defined, as a list of key names (nil if none).")

(define executing-kbd-macro #f
  "Non-nil while a keyboard macro is executing.")

(define simple--end-kbd-macro-op end-kbd-macro)

(define (simple--end-kbd-macro)
  "Finish defining a keyboard macro and remember it in `last-kbd-macro'."
  (simple--end-kbd-macro-op)
  (let ((keys (kbd-macro-last)))
    (set! last-kbd-macro (if (null? keys) #f keys))))

(define-command (start-kbd-macro . args)
  "Record subsequent keyboard input, defining a keyboard macro.
The commands are recorded even as they are executed.
With a prefix argument (or APPEND), append to the last macro instead of starting a new one."
  (simple--start-kbd-macro (if (null? args) (current-prefix-arg) (car args)))
  #f)

(define-command (end-kbd-macro . args)
  "Finish defining a keyboard macro.
The definition was started by start-kbd-macro.  The macro is now available for use with
call-last-kbd-macro.  With numeric arg, repeat the macro now that many times, counting the
definition just completed as the first repetition.  An argument of zero means repeat until error."
  (simple--end-kbd-macro)
  (let ((n (cond ((null? args) (prefix-numeric-value (current-prefix-arg)))
                 ((car args) (prefix-numeric-value (car args)))
                 (else 1))))
    (cond ((= n 0) (execute-kbd-macro last-kbd-macro 0))
          ((> n 1) (execute-kbd-macro last-kbd-macro (- n 1)))))
  #f)

(define-command (call-last-kbd-macro . args)
  "Call the last keyboard macro that you defined.
A prefix argument serves as a repeat count.  Zero means repeat until error."
  (if (kbd-macro-defining-p)
      (begin (simple--end-kbd-macro) (execute-kbd-macro last-kbd-macro 1))
      (if (not last-kbd-macro)
          (error "No kbd macro has been defined")
          (execute-kbd-macro last-kbd-macro
                             (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args)))))
  #f)

;;; Keyboard macros (kmacro)

(define kmacro-counter 0 "Current keyboard macro counter.")
(define kmacro-last-counter 0 "Last counter inserted by `kmacro-insert-counter'.")
(define kmacro-counter-format "%d" "Format to use when inserting the keyboard macro counter.")

(define-command (kmacro-start-macro . args)
  "Record subsequent keyboard input, defining a keyboard macro.
Without a prefix argument, the counter is reset to 0.  With a numeric prefix argument, the
counter is set to it.  With C-u, append to the last macro and keep the counter."
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (if (kbd-macro-defining-p)
        (message "Already defining keyboard macro.")
        (begin
          (if (not (pair? arg))
              (begin (set! kmacro-last-counter kmacro-counter)
                     (set! kmacro-counter (if arg (prefix-numeric-value arg) 0))))
          (simple--start-kbd-macro (pair? arg)))))
  #f)

(define-command (kmacro-insert-counter . args)
  "Insert the next value of the keyboard macro counter and increment it.
With a numeric prefix argument, increment by that amount.  With C-u, insert the previous
value without incrementing."
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (if (pair? arg)
        (insert (format kmacro-counter-format kmacro-last-counter))
        (begin
          (insert (format kmacro-counter-format kmacro-counter))
          (set! kmacro-last-counter kmacro-counter)
          (set! kmacro-counter (+ kmacro-counter (prefix-numeric-value arg)))
          ;; 実行中のキーボードマクロでなければ, 新しい値を知らせる
          (message "New macro counter value: %s (%d)"
                   (format kmacro-counter-format kmacro-counter) kmacro-counter))))
  #f)

(define-command (kmacro-start-macro-or-insert-counter . args)
  "Start defining a keyboard macro, or insert the counter if one is being defined."
  (if (or (kbd-macro-defining-p) (executing-kbd-macro-p))
      (kmacro-insert-counter)
      (kmacro-start-macro))
  #f)

(define-command (kmacro-end-macro . args)
  "Finish defining a keyboard macro.
With numeric arg, repeat the macro now that many times, counting the definition just
completed as the first repetition.  An argument of zero means repeat until error."
  ;; マクロを動かしている間は, マクロを終えるキーを無視する (Emacs と同じ)
  (when (not executing-kbd-macro)
    (if (not (kbd-macro-defining-p))
        (error "Not defining keyboard macro")
        (begin
          (end-kbd-macro (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args)))
          (if (not last-kbd-macro) (message "Ignore empty macro")))))
  #f)

(define-command (kmacro-end-and-call-macro . args)
  "Call the last keyboard macro, ending the definition first if one is in progress.
A prefix argument serves as a repeat count; zero means repeat until error."
  (if (kbd-macro-defining-p) (kmacro-end-macro 1))
  (if (not last-kbd-macro)
      (error "No kbd macro has been defined")
      (execute-kbd-macro last-kbd-macro
                         (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args))))
  #f)

(define-command (kmacro-end-or-call-macro . args)
  "End the keyboard macro being defined, or call the last one."
  (if (kbd-macro-defining-p)
      (kmacro-end-macro)
      (call-last-kbd-macro))
  #f)

(define-command (kmacro-name-last-macro . args)
  "Assign a name to the last keyboard macro defined."
  (if (not last-kbd-macro)
      (error "No keyboard macro defined")
      (let ((name (if (null? args) (read-string "Name for last kbd macro: ") (car args))))
        (if (equal? name "")
            (error "No command name given")
            (eval (list 'define-command (list (string->symbol name))
                        "Keyboard macro."
                        (list 'execute-kbd-macro (list 'quote last-kbd-macro) 1)
                        #f)))))
  #f)

;;; Case

(define (simple--casify-region mode args)
  "Convert the text between point and the mark, or between the two positions in ARGS, with MODE."
  (if (null? args)
      (casify-region mode (region-beginning) (region-end))
      (casify-region mode (car args) (cadr args)))
  #f)

(define (simple--casify-word mode args)
  "Convert the N words after point (before point if N is negative) with MODE.
Point moves to the end of the converted text, except for a negative N."
  (let* ((start (point))
         (far (save-excursion (forward-word (simple--arg args)) (point))))
    (goto-char (casify-region mode start far))
    #f))

(define-command (upcase-region . args)
  "Convert the region to upper case.
Called from Lisp with the two positions BEG and END, convert the text between them."
  (simple--casify-region "upcase" args))

(define-command (downcase-region . args)
  "Convert the region to lower case.
Called from Lisp with the two positions BEG and END, convert the text between them."
  (simple--casify-region "downcase" args))

(define-command (upcase-initials-region . args)
  "Upcase the initial of each word in the region.
Subsequent letters of each word are not changed.
Called from Lisp with the two positions BEG and END, convert the text between them."
  (simple--casify-region "upcase-initials" args))

(define-command (capitalize-region . args)
  "Convert the region to capitalized form.
Capitalized form means each word's first character is upper case and the rest lower case.
Called from Lisp with the two positions BEG and END, convert the text between them."
  (simple--casify-region "capitalize" args))

(define-command (upcase-word . args)
  "Convert to upper case from point to end of word, moving over.
With argument N, do this to that many words; a negative N converts the words before point
and leaves point where it is."
  (simple--casify-word "upcase" args))

(define-command (downcase-word . args)
  "Convert to lower case from point to end of word, moving over.
With argument N, do this to that many words; a negative N converts the words before point
and leaves point where it is."
  (simple--casify-word "downcase" args))

(define-command (capitalize-word . args)
  "Capitalize from point to the end of word, moving over.
With argument N, capitalize that many words; a negative N converts the words before point
and leaves point where it is."
  (simple--casify-word "capitalize" args))

(define-command (kill-word . args)
  "Kill the characters forward until the end of a word.
With argument N, kill that many words."
  (let ((start (point)))
    (forward-word (simple--arg args))
    (simple--kill start (point))))

(define-command (backward-kill-word . args)
  "Kill the characters backward until the beginning of a word.
With argument N, kill that many words."
  (let ((start (point)))
    (backward-word (simple--arg args))
    (simple--kill start (point))))

(define-command (zap-to-char . args)
  "Kill up to and including the Nth occurrence of CHAR.
Interactively, N is the prefix argument and CHAR is read as a key."
  (let ((n (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args)))
        (char (if (or (null? args) (null? (cdr args)))
                  (let ((key (read-key "Zap to char: ")))
                    (if (equal? key "Enter") "\n" key))
                  (cadr args)))
        (start (point)))
    (simple--kill start (search-forward char #f #f n))))

(define-command (append-next-kill . args)
  "Cause the following command, if it is a kill, to append to the previous kill.
Interactively, it takes effect after this command."
  (if (or (null? args) (car args))
      (begin
        (set! this-command 'kill-region)
        (message "If the next command is a kill, it will append"))
      (set! last-command 'kill-region)))

(define-command (ignore-preserving-kill-region . args)
  "Do nothing, but keep the kill that the previous command made for the next kill."
  (if (equal? last-command 'kill-region) (set! this-command 'kill-region)))

(define-command (yank . args)
  "Insert the most recent kill at point, and leave the mark at the start of the text.
With a plain prefix argument (C-u), leave point at the start and the mark at the end.
With a number N, insert the Nth most recent kill; with -, the second most recent."
  (let ((arg (simple--raw-arg args)))
    (set! this-command #t)
    (push-mark (point))
    (insert-for-yank (current-kill (cond ((or (not arg) (pair? arg)) 0)
                                         ((eq? arg '-) -2)
                                         (else (- arg 1)))))
    (when (pair? arg)
      (let ((old (mark)))
        (set-mark-position (point))
        (goto-char old)))
    (if (eq? this-command #t) (set! this-command 'yank))))

(define-command (yank-pop . args)
  "Replace the text just yanked with the previous kill in the kill ring.
With an argument N, use the Nth previous kill.  If the previous
command was not a yank, read a kill from the kill ring in the minibuffer."
  (if (not (equal? last-command 'yank))
      ;; 直前が yank でなければ, kill-ring から選んで入れる (Emacs 28 以降と同じ)
      (let ((count (kill-ring-length)))
        (if (= count 0) (error "Kill ring is empty"))
        (let ((choice (completing-read "Yank from kill-ring: "
                                       (let loop ((i 0) (items '()))
                                         (if (< i count)
                                             (loop (+ i 1) (cons (current-kill i #t) items))
                                             (reverse items))))))
          (push-mark (point))
          (insert-for-yank choice)
          #f))
      (let ((n (simple--arg args)))
        (set! this-command 'yank)
        (let ((before (< (point) (mark))))
          (if before
              (delete-region (point) (mark))
              (delete-region (mark) (point)))
          (set-mark-position (point))
          (insert-for-yank (current-kill n))
          (when before
            (let ((old (mark)))
              (set-mark-position (point))
              (goto-char old)))))))

(define-command (insert-buffer . args)
  "Insert after point the contents of BUFFER, and leave point before it and the mark after it.
Interactively, read the name of BUFFER in the minibuffer."
  (let* ((name (if (null? args)
                   (completing-read "Insert buffer: " (buffer-list) #f #t)
                   (car args)))
         (text (with-current-buffer name (buffer-string))))
    (push-mark (save-excursion (insert text) (point)))))

(define (simple--transpose mover arg)
  "Transpose the objects that MOVER moves over, as transpose-chars, transpose-words and transpose-lines do."
  (simple--transpose-pairs
   (lambda (x)
     (let* ((first (begin (mover x) (point)))
            (second (begin (mover (- x)) (point))))
       (list first second)))
   arg))

(define (simple--transpose-pairs aux arg)
  "Transpose the objects whose start and end AUX returns (a list of two positions) for a direction."
  (let ()
    (cond ((= arg 0)
           (let ((pos1 #f) (pos2 #f))
             (save-excursion
               (set! pos1 (aux 1))
               (goto-char (or (mark) (error "No mark set in this buffer")))
               (set! pos2 (aux 1))
               (simple--transpose-1 pos1 pos2))
             (exchange-point-and-mark #f)))
          ((> arg 0)
           (let* ((pos1 (aux -1)) (pos2 (aux arg)))
             (simple--transpose-1 pos1 pos2)
             (goto-char (car pos2))))
          (else
           (let ((pos1 (aux -1)))
             (goto-char (car pos1))
             (let ((pos2 (aux arg)))
               (simple--transpose-1 pos1 pos2)
               (goto-char (+ (car pos2) (- (cadr pos1) (car pos1))))))))))

(define (simple--transpose-1 pos1 pos2)
  "Swap the text of the two regions POS1 and POS2, each a list (start end), keeping the markers at their ends."
  (let* ((a (if (> (car pos1) (cadr pos1)) (list (cadr pos1) (car pos1)) pos1))
         (b (if (> (car pos2) (cadr pos2)) (list (cadr pos2) (car pos2)) pos2))
         (first (if (> (car a) (car b)) b a))
         (second (if (> (car a) (car b)) a b)))
    (if (> (cadr first) (car second)) (error "Don’t have two things to transpose"))
    (let* ((word (buffer-substring (car second) (cadr second)))
           (len1 (- (cadr first) (car first)))
           (len2 (string-length word))
           (boundary (save-excursion (goto-char (car second)) (point-marker))))
      (goto-char (cadr first))
      (insert-before-markers word)
      (let ((moved (buffer-substring (car first) (+ (car first) len1))))
        (delete-region (car first) (+ (car first) len1))
        (goto-char (marker-position boundary))
        (insert moved))
      (goto-char (+ (marker-position boundary) len1))
      (delete-region (point) (+ (point) len2))
      (marker-release boundary))))

(define-command (transpose-chars . args)
  "Interchange the characters around point, and move point forward over them.
At the end of a line, the previous two characters are interchanged.
With argument N, drag the character before point forward over N other characters."
  (when (and (eolp) (not (bobp)))
    (backward-char 1))
  (simple--transpose forward-char (simple--arg args)))

(define-command (transpose-words . args)
  "Interchange the words around point, and move point over the second.
With argument N, drag the word before or around point forward over N other words."
  (simple--transpose forward-word (simple--arg args)))

(define-command (transpose-lines . args)
  "Exchange the current line and the previous line, and leave point after both.
With argument N, move the previous line past N lines."
  (simple--transpose
   (lambda (n)
     (if (> n 0)
         (let ((short (forward-line n)))
           (if (not (equal? (char-before) "\n")) (set! short (+ short 1)))
           (if (> short 0) (newline short)))
         (forward-line n)))
   (simple--arg args)))

(define-command (quoted-insert . args)
  "Read a character and insert it as it is, N times.
Only the keys that stand for one character can be inserted."
  (let* ((key (read-key "C-q-"))
         (char (cond ((= (string-length key) 1) key)
                     ((equal? key "Enter") "\r")
                     ((equal? key "TAB") "\t")
                     ((equal? key "C-j") "\n")
                     ((equal? key "C-m") "\r")
                     ((equal? key "C-i") "\t")
                     (else (error (format "Cannot insert %s" key))))))
    (let loop ((n (simple--arg args)))
      (when (> n 0)
        (insert char)
        (loop (- n 1))))))

;;; The mark

(define set-mark-command-repeat-pop #f
  "Non-nil means that repeating set-mark-command after popping a mark pops another one.")

(define-command (push-mark-command . args)
  "Set the mark at point; with an argument, set it even if it is already there.
If the mark is already at point, activate it instead.  The second argument
NOMSG, if true, suppresses the message."
  (let ((arg (simple--raw-arg args))
        (nomsg (and (pair? args) (pair? (cdr args)) (cadr args)))
        (old (mark)))
    (if (or arg (not old) (not (= old (point))))
        (push-mark (point) nomsg #t)
        (begin
          (activate-mark)
          (if (not nomsg) (message "Mark activated"))))))

(define-command (pop-to-mark-command . args)
  "Jump to the mark, and pop a new position for the mark off the mark ring."
  (if (not (mark))
      (error "No mark set in this buffer")
      (begin
        (if (= (point) (mark)) (message "Mark popped"))
        (goto-char (mark))
        (pop-mark))))

(define-command (pop-global-mark . args)
  "Pop off the global mark ring and jump to the top location."
  (let* ((entry (global-mark-pop))
         (name (car entry))
         (position (cadr entry)))
    (switch-to-buffer name)
    (goto-char position)))

(define-command (set-mark-command . args)
  "Set the mark where point is, and activate it; or jump to the mark.
With a plain prefix argument (C-u), jump to the mark and pop the mark ring.
With C-u C-u, set the mark and activate it without the special cases.
Repeated, it activates or deactivates the mark."
  (let ((arg (simple--raw-arg args)))
    (cond
     ((and (pair? arg) (> (prefix-numeric-value arg) 4))
      (push-mark-command #f))
     ((not (eq? this-command 'set-mark-command))
      (if arg
          (pop-to-mark-command)
          (push-mark-command #t)))
     ((and set-mark-command-repeat-pop (eq? last-command 'pop-global-mark) (not arg))
      (set! this-command 'pop-global-mark)
      (pop-global-mark))
     ((or (and set-mark-command-repeat-pop (eq? last-command 'pop-to-mark-command)) arg)
      (set! this-command 'pop-to-mark-command)
      (pop-to-mark-command))
     ((eq? last-command 'set-mark-command)
      (if (region-active-p)
          (begin (deactivate-mark) (message "Mark deactivated"))
          (begin (activate-mark) (message "Mark activated"))))
     (else (push-mark-command #f)))))

(define-command (exchange-point-and-mark . args)
  "Put the mark where point is now, and point where the mark is now.
The mark is activated, unless there is an argument (which deactivates it)."
  (let ((arg (simple--raw-arg args))
        (old (mark)))
    (if (not old) (error "No mark set in this buffer"))
    (set-mark (point))
    (goto-char old)
    (if (not (eq? (and arg #t) (not (region-active-p))))
        (deactivate-mark)
        (activate-mark))))

(define-command (mark-whole-buffer . args)
  "Put point at the beginning and the mark at the end of the buffer."
  (push-mark (point))
  (push-mark (point-max) #f #t)
  (goto-char (point-min)))

(define-command (mark-word . args)
  "Set the mark at the end of the next word, or extend the region by a word.
With argument N, do it for N words (backward if N is negative)."
  (let ((arg (simple--raw-arg args))
        (allow-extend (simple--allow-extend args)))
    (cond ((and allow-extend
                (or (and (equal? last-command this-command) (mark))
                    (region-active-p)))
           (let ((n (if arg
                        (prefix-numeric-value arg)
                        (if (< (mark) (point)) -1 1))))
             (set-mark (save-excursion
                         (goto-char (mark))
                         (forward-word n)
                         (point)))))
          (else
           (push-mark (save-excursion
                        (forward-word (prefix-numeric-value arg))
                        (point))
                      #f #t)))))

(define-command (keyboard-quit . args)
  "Deactivate the mark and cancel the running command by signaling a quit."
  (deactivate-mark)
  (error "Quit"))

(define-command (keyboard-escape-quit . args)
  "Exit the current mode in a general sense: deactivate the region, clear a prefix
argument, or go back to just one window."
  (cond ((equal? last-command 'mode-exited) #f)
        ((region-active-p) (deactivate-mark))
        ((current-prefix-arg) #f)
        ((> (length (window-list)) 1) (delete-other-windows))
        (else #f)))

;;; Prefix arguments

(define prefix-arg #f
  "The prefix argument for the next command: #f, a number, the symbol -, or a list like (4).")

(define last-command-event #f
  "The key (a string such as \"5\" or \"M-5\") that ran the command.")

(define universal-argument-mode #f
  "True while the keys 0-9, - and C-u make up a prefix argument (the universal-argument-map).")

(define (simple--prefix-state)
  "Pass the prefix command state to the next command: it will see the previous last-command."
  (set! this-command last-command))

(define (simple--event-digit)
  "Return the digit of the key that ran the command (the last character of last-command-event)."
  (let ((key (format "%s" last-command-event)))
    (string->number (substring key (- (string-length key) 1)))))

(define-command (universal-argument . args)
  "Begin a numeric argument for the following command.
Digits or minus sign following it make up the numeric argument.
C-u without digits or minus sign provides 4 as argument."
  (simple--prefix-state)
  (set! prefix-arg (list 4))
  (set! universal-argument-mode #t))

(define-command (universal-argument-more . args)
  "Multiply the prefix argument by 4 (C-u after C-u), or end the argument after digits."
  (let ((arg (simple--raw-arg args)))
    (simple--prefix-state)
    (set! prefix-arg (cond ((pair? arg) (list (* 4 (car arg))))
                           ((eq? arg '-) (list -4))
                           (else arg)))
    (if (pair? prefix-arg) (set! universal-argument-mode #t))))

(define-command (digit-argument . args)
  "Part of the numeric argument for the next command: add the digit of the key."
  (let ((arg (simple--raw-arg args))
        (digit (simple--event-digit)))
    (simple--prefix-state)
    (set! prefix-arg (cond ((number? arg)
                            (+ (* arg 10) (if (< arg 0) (- digit) digit)))
                           ((eq? arg '-) (if (= digit 0) '- (- digit)))
                           (else digit)))
    (set! universal-argument-mode #t)))

(define-command (negative-argument . args)
  "Begin a negative numeric argument for the next command, or negate it."
  (let ((arg (simple--raw-arg args)))
    (simple--prefix-state)
    (set! prefix-arg (cond ((number? arg) (- arg))
                           ((eq? arg '-) #f)
                           (else '-)))
    (set! universal-argument-mode #t)))

;;; Information

(define (simple--single-key-description char)
  "Return how Emacs writes the ASCII character CHAR as a key, such as C-a or SPC."
  (let ((code (string-to-char char)))
    (cond ((= code 32) "SPC")
          ((= code 9) "TAB")
          ((= code 13) "RET")
          ((= code 27) "ESC")
          ((= code 127) "DEL")
          ((< code 32) (string-append "C-" (char-to-string (+ code 96))))
          (else char))))

(define-command (what-cursor-position . args)
  "Print information about the cursor position in the echo area:
the character after point with its codes, the position, and the column.
In a narrowed buffer, also show the accessible range as <BEG-END>."
  (let* ((total (buffer-size))
         (pos (point))
         (percent (simple--round-percent (- pos 1) (max 1 total)))
         (column (current-column))
         (range (if (and (= (point-min) 1) (= (point-max) (+ total 1)))
                    ""
                    (format " <%d-%d>" (point-min) (point-max)))))
    (if (eobp)
        (if (equal? range "")
            (message "%s" (format "point=%d of %d (EOB) column=%d" pos total column))
            (message "%s" (format "point=%d of %d (%d%%)%s column=%d" pos total percent range column)))
        (let* ((char (char-after))
               (code (string-to-char char))
               (shown (if (< code 128) (simple--single-key-description char) char))
               (encoding (if (>= code 128)
                             (format "(%d, #o%o, #x%x, file ...)" code code code)
                             (format "(%d, #o%o, #x%x)" code code code))))
          (message "%s" (format "Char: %s %s point=%d of %d (%d%%)%s column=%d"
                                shown encoding pos total percent range column))))))

(define (simple--round-percent n total)
  "Return 100 * N / TOTAL rounded to the nearest integer, ties to even (like Emacs round)."
  (let* ((numerator (* 100 n))
         (q (quotient numerator total))
         (r (remainder numerator total)))
    (cond ((> (* 2 r) total) (+ q 1))
          ((= (* 2 r) total) (if (even? q) q (+ q 1)))
          (else q))))

(define (simple--count-lines start end)
  "Return the number of lines between START and END."
  (let ((text (buffer-substring start end)))
    (if (equal? text "")
        0
        (let loop ((i 0) (n 0))
          (cond ((>= i (string-length text))
                 (if (equal? (substring text (- (string-length text) 1) (string-length text)) "\n") n (+ n 1)))
                ((equal? (substring text i (+ i 1)) "\n") (loop (+ i 1) (+ n 1)))
                (else (loop (+ i 1) n)))))))

(define (simple--count-words start end)
  "Return the number of words between START and END."
  (save-excursion
    (goto-char (min start end))
    (let ((limit (max start end)))
      (let loop ((words 0))
        (if (and (re-search-forward "\\W*\\w+" limit #t))
            (loop (+ words 1))
            words)))))

(define (simple--count-sentences start end)
  "Return the number of sentences between START and END."
  (save-excursion
    (goto-char (min start end))
    (let ((limit (max start end)))
      (let loop ((sentences 0))
        (if (re-search-forward "[.?!…‽][]\"'”’)}»›]*\\($\\|[ \t]\\| \\)[ \t\n]*" limit #t)
            (loop (+ sentences 1))
            (let ((rest (buffer-substring (point) limit)))
              (if (string-search "\\w" rest) (+ sentences 1) sentences)))))))

(define (simple--count-words-format label start end)
  "Return the message about the lines, sentences, words and characters between START and END."
  (let ((lines (simple--count-lines start end))
        (sentences (simple--count-sentences start end))
        (words (simple--count-words start end))
        (chars (- end start)))
    (format "%s has %d line%s, %d sentence%s, %d word%s, and %d character%s"
            label
            lines (if (= lines 1) "" "s")
            sentences (if (= sentences 1) "" "s")
            words (if (= words 1) "" "s")
            chars (if (= chars 1) "" "s"))))

(define-command (count-words-region . args)
  "Print the number of lines, sentences, words and characters in the region.
With a prefix argument, report on the whole buffer."
  (if (and (pair? args) (pair? (cdr args)))
      (simple--count-words (car args) (cadr args))
      (if (current-prefix-arg)
          (message (simple--count-words-format "Buffer" (point-min) (point-max)))
          (message (simple--count-words-format "Region" (region-beginning) (region-end))))))

(define fill-column 70
  "The column beyond which automatic line breaking should happen.")

(define truncate-lines #f
  "Non-nil means do not display continuation lines: give each line one row, cut at the window edge.")

(define selective-display #f
  "A number N means that lines indented by N columns or more are not displayed.")

(define-command (set-fill-column . args)
  "Set fill-column to the argument.  With C-u, use the current column."
  (let ((arg (if (null? args) (current-prefix-arg) (car args))))
    (if (pair? arg) (set! arg (current-column)))
    (if (not (number? arg))
        (error "set-fill-column requires an explicit argument")
        (begin
          (message (format "Fill column set to %d (was %d)" arg fill-column))
          (set! fill-column arg)))))

(define-command (toggle-truncate-lines . args)
  "Toggle truncating of long lines for the current buffer.
With a positive argument, truncate; with zero or a negative one, wrap."
  (let ((arg (simple--raw-arg args)))
    (set! truncate-lines (if (not arg) (not truncate-lines) (> (prefix-numeric-value arg) 0)))
    (message (format "Truncate long lines %s" (if truncate-lines "enabled" "disabled")))))

(define-command (set-selective-display . args)
  "Set selective-display to the argument; clear it if there is none.
When it is a number, lines indented by that many columns or more are not displayed."
  (let ((arg (simple--raw-arg args)))
    (set! selective-display (and arg (prefix-numeric-value arg)))
    (message (format "selective-display set to %s." (if selective-display selective-display "nil")))))

;;; Undoing

(define simple--undo-equiv '()
  "Which undo lists were made by undo: a list of (length value).
The value is what is left to undo (a list, #t or empty).  It is keyed by the length of the undo list.")

(define (simple--skip-boundaries ul)
  "Return the undo list UL without the boundaries (#f) at its front."
  (if (and (pair? ul) (not (car ul)))
      (simple--skip-boundaries (cdr ul))
      ul))

(define (simple--equiv-ref ul)
  "Return what undo recorded for the undo list UL, or #f."
  (let ((entry (assoc (length ul) simple--undo-equiv)))
    (and entry (cadr entry))))

(define (simple--equiv-set ul value)
  "Record VALUE for the undo list UL."
  (set! simple--undo-equiv
        (cons (list (length ul) value)
              (filter (lambda (entry) (not (equal? (car entry) (length ul))))
                      simple--undo-equiv))))

(define (simple--last-change-was-undo-p undo-list)
  "Return what undo recorded if the last change in UNDO-LIST was made by undo, else #f."
  (simple--equiv-ref (simple--skip-boundaries undo-list)))

(define (simple--undo-more n)
  "Undo N more groups of changes from the pending undo list."
  (if (not (list? pending-undo-list))
      (error "No further undo information"))
  (set! pending-undo-list (primitive-undo n pending-undo-list))
  (if (null? pending-undo-list) (set! pending-undo-list #t)))

(define (simple--drop-point-records ul)
  "Return the undo list UL without the position records that undo made in its newest group."
  (let scan ((rest ul) (kept '()))
    (cond ((or (null? rest) (not (car rest))) ul)
          ((number? (car rest))
           (let ((position (car rest)))
             (let drop ((tail (cdr rest)) (kept kept))
               (cond ((or (null? tail) (not (car tail))) (append (reverse kept) tail))
                     ((equal? (car tail) position) (drop (cdr tail) kept))
                     (else (drop (cdr tail) (cons (car tail) kept)))))))
          (else (scan (cdr rest) (cons (car rest) kept))))))

(define-command (undo . args)
  "Undo some previous changes.  Repeat the command to undo more.
With a number N, undo N groups of changes.  After the other commands, the
undo starts again from the newest change, so undoing an undo redoes the change."
  (let ((arg (simple--raw-arg args)))
    (set! this-command 'undo-start)
    (when (not (and (equal? last-command 'undo)
                    (or (eq? pending-undo-list #t)
                        (simple--last-change-was-undo-p (buffer-undo-list)))))
      (set! pending-undo-list (buffer-undo-list))
      (simple--undo-more 1))
    (set! this-command 'undo)
    (let ((redo (and (list? pending-undo-list) (simple--equiv-ref pending-undo-list))))
      (simple--undo-more (if (number? arg) (prefix-numeric-value arg) 1))
      (let* ((ul (simple--skip-boundaries (buffer-undo-list)))
             (value (cond ((and (list? pending-undo-list)
                                (= (length ul) (length pending-undo-list)))
                           (or (simple--equiv-ref ul) 'empty))
                          (else pending-undo-list))))
        ;; the position records made by the undo are dropped, so the key is the list without them
        (set-buffer-undo-list (simple--drop-point-records (buffer-undo-list)))
        (simple--equiv-set (simple--skip-boundaries (buffer-undo-list)) value))
      (message (if redo "Redo" "Undo")))))

(define-command (undo-redo . args)
  "Undo the last N undos, that is redo the last N changes that were undone."
  (let ((n (simple--arg args)))
    (if (not (simple--last-change-was-undo-p (buffer-undo-list)))
        (error "No undone changes to redo")
        (let* ((undo-list (simple--skip-boundaries (buffer-undo-list)))
               (new-list (primitive-undo n undo-list))
               (new-pending (simple--last-change-was-undo-p new-list)))
          (message "Redo")
          (set! this-command 'undo)
          (set! pending-undo-list (or new-pending '()))
          (set-buffer-undo-list new-list)))))

;;; Windows and buffers

(define simple--delete-window delete-window)
(define simple--delete-other-windows delete-other-windows)
(define simple--other-window other-window)

(define-command (split-window-below . args)
  "Split the selected window into two windows, one above the other."
  (split-window 'below))

(define-command (split-window-right . args)
  "Split the selected window into two windows side by side."
  (split-window 'right))

(define-command (delete-window . args)
  "Delete the selected window, or the window given as the argument."
  (apply simple--delete-window args))

(define-command (delete-other-windows . args)
  "Make the selected window fill the screen."
  (simple--delete-other-windows))

(define-command (other-window . args)
  "Select another window.  With argument N, select the Nth window after this one."
  (simple--other-window (simple--arg args)))

(define (simple--modified-file-buffers)
  "Return the names of the buffers that visit a file and are modified."
  (filter (lambda (name) (and (buffer-file-name name) (buffer-modified-p name)))
          (buffer-list)))

(define-command (save-buffers-kill-terminal . args)
  "Offer to save the modified buffers, and exit the editor.
Ask once more if modified buffers remain."
  (save-some-buffers)
  (when (or (null? (simple--modified-file-buffers))
            (yes-or-no-p "Modified buffers exist; exit anyway? "))
    (kill-emacs)))

;;; Transposing sexps, sentences and paragraphs

(define (simple--symbol-constituent-at pos)
  "True if the character at POS has word or symbol syntax (Emacs \\sw\\|\\s_)."
  (and (< pos (point-max)) (member (syntax-class-at pos) (list "w" "_")) #t))

(define (simple--transpose-sexps-pair arg)
  "Return the start and end of the sexp to transpose in the direction ARG (Emacs transpose-sexps-default-function)."
  (if (if (> arg 0)
          (simple--symbol-constituent-at (point))
          (and (not (bobp)) (simple--symbol-constituent-at (- (point) 1))))
      (begin
        (if (> arg 0) (skip-syntax-backward "w_") (skip-syntax-forward "w_"))
        (list (save-excursion (simple--forward-sexp arg) (point)) (point)))
      (begin
        (if (> arg 0) (skip-syntax-backward " .") (skip-syntax-forward " ."))
        (list (save-excursion (simple--forward-sexp arg) (point))
              (begin
                (let loop ()
                  (when (or (forward-comment (if (> arg 0) 1 -1))
                            (not (= 0 (if (> arg 0) (skip-syntax-forward ".") (skip-syntax-backward ".")))))
                    (loop)))
                (point))))))

(define-command (transpose-sexps . args)
  "Like transpose-chars, but applies to sexps.  Point must be between the two sexps.
With argument N, drag the sexp before point forward past N other sexps (backward if negative).
With zero, interchange the sexps ending at or after point and at or after mark."
  (let ((n (simple--arg args)))
    (if simple--interactive
        (condition-case e
            (simple--transpose-pairs simple--transpose-sexps-pair n)
          (scan-error (user-error "Not between two complete sexps")))
        (simple--transpose-pairs simple--transpose-sexps-pair n))))

(define-command (transpose-sentences . args)
  "Interchange the current sentence with the next one.
With argument N, move the current sentence past N sentences."
  (simple--transpose forward-sentence (simple--arg args)))

(define-command (transpose-paragraphs . args)
  "Interchange the current paragraph with the next one.
With argument N, move the current paragraph past N paragraphs."
  (simple--transpose forward-paragraph (simple--arg args)))

;;; Pages

(define simple--page-mark (char-to-string 12)
  "The form feed character that starts a page boundary line.")

(define (simple--page-forward-search)
  "Move point past the next page delimiter (a form feed at the start of a line); return false if none."
  (let loop ()
    (if (search-forward simple--page-mark #f #t)
        (if (= (line-beginning-position) (- (point) 1)) #t (loop))
        #f)))

(define (simple--page-backward-search)
  "Move point to the start of the previous page delimiter; return false if none."
  (let loop ()
    (if (search-backward simple--page-mark #f #t)
        (if (= (line-beginning-position) (point)) #t (loop))
        #f)))

(define-command (forward-page . args)
  "Move forward to page boundary.  With argument N, repeat, or go back if negative.
A page boundary is a line that starts with a form feed."
  (let loop ((count (simple--arg args)))
    (cond ((and (> count 0) (not (eobp)))
           (if (and (bolp) (equal? (char-after) simple--page-mark))
               (begin
                 (set-match-data (list (point) (+ (point) 1)))
                 (forward-char 1))
               (begin
                 (if (bolp) (forward-char 1))
                 (if (not (simple--page-forward-search)) (goto-char (point-max)))))
           (loop (- count 1)))
          ((and (< count 0) (not (bobp)))
           (let ((here (point)))
             (if (and (simple--page-backward-search) (= (+ (point) 1) here))
                 #t
                 (goto-char here)))
           (when (not (bobp))
             (forward-char -1)
             (if (simple--page-backward-search)
                 (goto-char (+ (point) 1))
                 (goto-char (point-min))))
           (loop (+ count 1))))))

(define-command (backward-page . args)
  "Move backward to page boundary.  With argument N, repeat, or go forward if negative."
  (forward-page (- (simple--arg args))))

(define-command (mark-page . args)
  "Put the mark at the end of the page and point at the beginning.
With argument N, mark the page N pages forward (backward if negative)."
  (let ((arg (let ((raw (simple--raw-arg args))) (if raw (prefix-numeric-value raw) 0))))
    (cond ((> arg 0) (forward-page arg))
          ((< arg 0) (forward-page (- arg 1))))
    (forward-page 1)
    (push-mark (point) #t #t)
    (forward-page -1)))

;;; Narrowing

(define simple--narrow-to-region narrow-to-region)
(define simple--widen widen)

(define-command (narrow-to-region . args)
  "Restrict editing in this buffer to the current region.
The rest of the text becomes temporarily invisible and unchangeable.
Called from Lisp with START and END, restrict to that range instead."
  (if (and (pair? args) (pair? (cdr args)))
      (simple--narrow-to-region (car args) (cadr args))
      (let ((from (region-beginning))
            (to (region-end)))
        (simple--narrow-to-region from to))))

(define-command (widen . args)
  "Remove restrictions (narrowing) from the current buffer.
This allows the buffer's full text to be seen and edited."
  (simple--widen))

(define (simple--page-delimiter-p)
  "Return true if point is at a page delimiter: a form feed at the start of a line."
  (and (bolp) (looking-at (regexp-quote simple--page-mark))))

(define-command (narrow-to-page . args)
  "Make text outside current page invisible.
A numeric argument specifies to move forward or backward by that many pages,
thus showing a page other than the one point was originally in."
  (let ((arg (let ((raw (simple--raw-arg args))) (if raw (prefix-numeric-value raw) 0))))
    (save-excursion
      (widen)
      (cond ((> arg 0) (forward-page arg))
            ((< arg 0)
             (let* ((opoint (point))
                    (adjust (save-excursion
                              (goto-char (line-beginning-position))
                              (if (and (simple--page-delimiter-p) (= (match-end 0) opoint)) 0 1))))
               (forward-page (- arg adjust)))))
      (set-match-data #f)
      (forward-page 1)
      ;; Stop at the end of the buffer, or at the start of the delimiter line.
      (if (and (match-beginning 0)
               (save-excursion (goto-char (match-beginning 0)) (simple--page-delimiter-p)))
          (goto-char (match-beginning 0)))
      (let* ((end (point))
             (start (begin
                      (forward-page -1)
                      (if (and (eolp) (not (bobp))) (forward-line 1))
                      (point))))
        (narrow-to-region start end)))))

(define narrow-to-defun-include-comments #f
  "Whether narrow-to-defun includes the comments before a defun.")

(define-command (narrow-to-defun . args)
  "Make text outside current defun invisible.
The current defun is the one that contains point or follows point.
Preceding comments are included if INCLUDE-COMMENTS is true.
Called from the keyboard, use the value of narrow-to-defun-include-comments."
  (let ((include-comments (if (null? args) narrow-to-defun-include-comments (car args))))
    (save-excursion
      (widen)
      (let ((opoint (point))
            (start #f)
            (end #f))
        ;; Move one character forward first, since beginning-of-defun on the first
        ;; character of a defun goes to the previous one.
        (let ((here (point)))
          (unless (eolp) (forward-char 1))
          (beginning-of-defun)
          (when (< (point) here)
            (goto-char here)
            (beginning-of-defun)))
        (set! start (point))
        (simple--end-of-defun 1 #t)
        (set! end (point))
        (let loop ()
          (when (looking-at "^\n")
            (forward-line 1)
            (loop)))
        (unless (> (point) opoint)
          ;; beginning-of-defun moved back one defun, so this is the wrong one.
          (goto-char opoint)
          (simple--end-of-defun 1 #t)
          (set! end (point))
          (beginning-of-defun)
          (set! start (point)))
        (when include-comments
          (goto-char start)
          ;; Move back past all preceding comments (and whitespace).
          (when (forward-comment -1)
            (let loop ()
              (when (forward-comment -1) (loop)))
            ;; Move forwards past any page breaks within these comments.
            (let loop ()
              (when (re-search-forward (string-append "^" simple--page-mark) start #t) (loop)))
            ;; Lastly, move past any empty lines.
            (skip-chars-forward "[:space:]\n")
            (goto-char (line-beginning-position))
            (set! start (point))))
        (goto-char end)
        (re-search-backward "^\n" (- (point) 1) #t)
        (narrow-to-region start end)))))

;;; Pages

(define (simple--page-line-counts)
  "Return (TOTAL BEFORE AFTER): the number of lines on the current page, before point and after point."
  (save-excursion
    (let ((opoint (point)))
      (forward-page 1)
      (forward-line 0)
      (unless (looking-at (regexp-quote simple--page-mark)) (goto-char (line-end-position)))
      (let ((end (point)))
        (forward-page -1)
        (list (simple--count-lines (point) end)
              (simple--count-lines (point) opoint)
              (simple--count-lines opoint end))))))

(define-command (count-lines-page . args)
  "Report number of lines on current page, and how many are before or after point."
  (let* ((counts (simple--page-line-counts))
         (total (car counts)))
    (message (format (if (= total 1) "Page has %d line (%d + %d)" "Page has %d lines (%d + %d)")
                     total (cadr counts) (caddr counts)))))

;;; S-expressions, defuns, paragraphs and sentences

(define (simple--forward-sexp n)
  "Move forward across N sexps, signalling the scan error as is."
  (goto-char (or (scan-sexps (point) n) (if (> n 0) (point-max) (point-min))))
  (if (< n 0) (backward-prefix-chars)))

(define-command (forward-sexp . args)
  "Move forward across one balanced expression (sexp).
With argument N, do it N times; a negative N moves backward.
From the keyboard, report a failure as \"No next sexp\" or \"No previous sexp\"."
  (let ((n (simple--arg args)))
    (if simple--interactive
        (condition-case e
            (simple--forward-sexp n)
          (scan-error (user-error (if (> n 0) "No next sexp" "No previous sexp"))))
        (simple--forward-sexp n))))

(define-command (backward-sexp . args)
  "Move backward across one balanced expression (sexp).
With argument N, do it N times; a negative N moves forward."
  (forward-sexp (- (simple--arg args))))

(define-command (forward-list . args)
  "Move forward across one balanced group of parentheses.
With argument N, do it N times; a negative N moves backward.
From the keyboard, report a failure as \"No next group\" or \"No previous group\"."
  (let ((n (simple--arg args)))
    (define (move)
      (goto-char (or (scan-lists (point) n 0) (if (> n 0) (point-max) (point-min)))))
    (if simple--interactive
        (condition-case e
            (move)
          (scan-error (user-error (if (> n 0) "No next group" "No previous group"))))
        (move))))

(define-command (backward-list . args)
  "Move backward across one balanced group of parentheses.
With argument N, do it N times; a negative N moves forward."
  (forward-list (- (simple--arg args))))

(define-command (down-list . args)
  "Move forward down one level of parentheses.
With argument N, do this N times; a negative N moves backward but still goes down a level.
From the keyboard, report a failure as \"At bottom level\"."
  (let ((n (simple--arg args)))
    (when (list-ref (syntax-ppss) 8)
      (user-error "This command doesn’t work in strings or comments"))
    (define (move)
      (let loop ((arg n))
        (let ((inc (if (> arg 0) 1 -1)))
          (when (not (= arg 0))
            (goto-char (or (scan-lists (point) inc -1) (if (> arg 0) (point-max) (point-min))))
            (loop (- arg inc))))))
    (if simple--interactive
        (condition-case e
            (move)
          (scan-error (user-error "At bottom level")))
        (move))))

(define-command (up-list . args)
  "Move forward out of one level of parentheses.
With argument N, do this N times; a negative N moves backward.
Inside a string, move out of the string first."
  (let loop ((n (simple--arg args)))
    (cond ((> n 0) (goto-char (up-list-position (point) 1)) (loop (- n 1)))
          ((< n 0) (goto-char (up-list-position (point) -1)) (loop (+ n 1))))))

(define-command (backward-up-list . args)
  "Move backward out of one level of parentheses.
With argument N, do this N times; a negative N moves forward."
  (up-list (- (simple--arg args))))

(define-command (kill-sexp . args)
  "Kill the sexp (balanced expression) following point.
With argument N, kill N sexps; a negative N kills backward."
  (let ((start (point)))
    (forward-sexp (simple--arg args))
    (simple--kill start (point))))

(define-command (backward-kill-sexp . args)
  "Kill the sexp (balanced expression) preceding point.
With argument N, kill N sexps; a negative N kills forward."
  (kill-sexp (- (simple--arg args))))

(define (simple--forward-sexp-to-select n none)
  "Return the end of N sexps from point; signal NONE if there is no sexp to select."
  (condition-case e
      (save-excursion (simple--forward-sexp n) (point))
    (scan-error
     (let ((reason (scan-error-reason e)))
       (user-error (if (equal? reason "Containing expression ends prematurely") none reason))))))

(define-command (mark-sexp . args)
  "Set the mark at the end of the next sexp, or extend the region by one sexp.
With argument N, do it for N sexps (backward if N is negative)."
  (let ((arg (simple--raw-arg args))
        (allow-extend (simple--allow-extend args)))
    (cond ((and allow-extend
                (or (and (equal? last-command this-command) (mark))
                    (region-active-p)))
           (let ((n (if arg
                        (prefix-numeric-value arg)
                        (if (< (mark) (point)) -1 1))))
             (set-mark (save-excursion
                         (goto-char (mark))
                         (simple--forward-sexp-to-select n "No more sexp to select")))))
          (else
           (push-mark (simple--forward-sexp-to-select (prefix-numeric-value arg) "No sexp to select")
                      #f #t)))))

(define (simple--push-mark-for-defun name)
  "Push the mark before a defun motion NAME, unless it repeats NAME or the region is active."
  (if (and (eq? this-command name) (not (eq? last-command name)) (not (region-active-p)))
      (push-mark)))

(define-command (beginning-of-defun . args)
  "Move backward to the beginning of a defun (an open paren in column 0).
With argument N, do it N times.  Push the mark first, unless repeated."
  (simple--push-mark-for-defun 'beginning-of-defun)
  (let loop ((n (simple--arg args)))
    (when (> n 0)
      (goto-char (beginning-of-defun-position (point)))
      (loop (- n 1)))))

(define (simple--end-of-defun n scan-errors)
  "Move forward N defuns.  If SCAN-ERRORS, an unbalanced defun signals the scan error that Lisp code sees."
  (goto-char (end-of-defun-position (point) n scan-errors)))

(define-command (end-of-defun . args)
  "Move forward to the end of a defun.  With argument N, do it N times.
Push the mark first, unless repeated."
  (simple--push-mark-for-defun 'end-of-defun)
  (simple--end-of-defun (simple--arg args) #f))

(define-command (mark-defun . args)
  "Put the region around the current defun, including the comments before it.
If the region is active, extend it to the end of the next defun."
  (let ((extend (and (use-region-p) (mark)))
        (opoint (point)))
    (if (not extend) (push-mark opoint))
    (let ((range (mark-defun-range opoint extend)))
      (push-mark (cadr range) #t #t)
      (goto-char (car range)))))

(define-command (forward-paragraph . args)
  "Move forward to the end of the paragraph.  With argument N, do it N times;
a negative N moves backward to the start of the paragraph."
  (goto-char (forward-paragraph-position (point) (simple--arg args))))

(define-command (backward-paragraph . args)
  "Move backward to the start of the paragraph.  With argument N, do it N times."
  (forward-paragraph (- (simple--arg args))))

(define-command (mark-paragraph . args)
  "Put the mark at the end of this paragraph, point at the beginning.
If the region is active, extend it by one more paragraph."
  (let ((arg (if (null? args) (prefix-numeric-value (current-prefix-arg)) (or (car args) 1)))
        (allow-extend (simple--allow-extend args)))
    (when (= arg 0) (error "Cannot mark zero paragraphs"))
    (cond ((and allow-extend
                (or (and (equal? last-command this-command) (mark))
                    (region-active-p)))
           (set-mark (save-excursion
                       (goto-char (mark))
                       (forward-paragraph arg)
                       (point))))
          (else
           (forward-paragraph arg)
           (push-mark (point) #t #t)
           (backward-paragraph arg)))))

(define-command (kill-paragraph . args)
  "Kill forward to the end of the paragraph.  With argument N, kill N paragraphs."
  (let ((start (point)))
    (forward-paragraph (simple--arg args))
    (simple--kill start (point))))

(define-command (backward-kill-paragraph . args)
  "Kill back to the start of the paragraph.  With argument N, kill N paragraphs."
  (kill-paragraph (- (simple--arg args))))

(define-command (forward-sentence . args)
  "Move forward to next end of sentence.  With argument N, do it N times;
a negative N moves backward to the start of the sentence."
  (goto-char (forward-sentence-position (point) (simple--arg args))))

(define-command (backward-sentence . args)
  "Move backward to the start of the sentence.  With argument N, do it N times."
  (forward-sentence (- (simple--arg args))))

(define-command (kill-sentence . args)
  "Kill from point to the end of the current sentence.  With argument N, kill N sentences."
  (let ((start (point)))
    (forward-sentence (simple--arg args))
    (simple--kill start (point))))

(define-command (backward-kill-sentence . args)
  "Kill back from point to the start of the sentence.  With argument N, kill N sentences."
  (kill-sentence (- (simple--arg args))))

(define-command (mark-end-of-sentence . args)
  "Put the mark at the end of this sentence.  With argument N, N sentences forward."
  (push-mark (save-excursion (forward-sentence (simple--arg args)) (point)) #f #t))

;;; Running commands

(define (simple--command-names)
  "Return the names of all commands, sorted."
  (filter (lambda (name) (commandp (string->symbol name))) (help-function-names)))

(define command-history '()
  "The commands run with M-x, newest first, each written as an expression such as \"(forward-char)\".")

(define (simple--remember-command name)
  "Put the command NAME on the front of command-history, unless it is already there."
  (let ((entry (format "(%s)" name)))
    (when (or (null? command-history) (not (equal? (car command-history) entry)))
      (set! command-history (cons entry command-history)))))

(define (simple--run-named-command prompt)
  "Read a command name with PROMPT, remember it, and run it."
  (let ((name (completing-read prompt (simple--command-names) #f #t)))
    (when (not (equal? name ""))
      (simple--remember-command name)
      (set! this-command (string->symbol name))
      (call-interactively (string->symbol name)))))

(define-command (execute-extended-command . args)
  "Read a command name in the minibuffer, with completion, and run it."
  (simple--run-named-command "M-x "))

(define-command (execute-extended-command-for-buffer . args)
  "Read a command name in the minibuffer, with completion, and run it.
Unlike Emacs, every command is offered, not only those that fit the major mode."
  (simple--run-named-command "M-X "))

(define-command (repeat-complex-command . args)
  "Edit and re-evaluate the last command run with M-x.
With the argument N, use the Nth previous command (1 is the last)."
  (when (null? command-history)
    (error "There are no previous complex commands to repeat"))
  (let* ((n (simple--arg args))
         (initial (list-ref command-history (min (max (- n 1) 0) (- (length command-history) 1))))
         (expression (read-string "Redo: " initial)))
    (simple--remember-command-expression expression)
    (set! this-command 'repeat-complex-command)
    (eval (read expression))))

(define (simple--remember-command-expression expression)
  "Put EXPRESSION on the front of command-history, unless it is already there."
  (when (or (null? command-history) (not (equal? (car command-history) expression)))
    (set! command-history (cons expression command-history))))

(define-command (eval-expression . args)
  "Read an expression in the minibuffer, evaluate it, and show the value."
  (let ((expression (if (null? args) (read-string "Eval: ") (car args))))
    (message (format "%s" (eval (read expression))))))

(define-command (eval-last-sexp . args)
  "Evaluate the sexp before point and show its value in the echo area."
  (let* ((end (point))
         (start (save-excursion (simple--forward-sexp -1) (point))))
    (if (>= start end) (user-error "No sexp before point"))
    (message (format "%S" (eval (read (buffer-substring start end)))))))

(define-command (electric-newline-and-maybe-indent . args)
  "Insert a newline without indenting it, as `electric-indent-mode' makes C-j do.
With argument N, insert N newlines."
  (let loop ((n (simple--arg args)))
    (when (> n 0)
      (insert "\n")
      (loop (- n 1)))))

(define-command (abort-recursive-edit . args)
  "Abort the command that requested this recursive edit or minibuffer input."
  (user-error "No recursive edit is in progress"))

(provide 'simple)

;;; simple.scm ends here

;;; Parentheses

(define parens-require-spaces #t
  "If true, add whitespace as needed when inserting parentheses.
This affects `insert-parentheses'.")

(define (simple--insert-pair raw open close)
  "Enclose following sexps in OPEN and CLOSE as `insert-parentheses' does for the prefix argument RAW."
  (if (region-active-p)
      (begin
        (save-excursion (goto-char (region-end)) (insert close))
        (goto-char (region-beginning))
        (insert open))
      (let ((arg (if raw (prefix-numeric-value raw) 0)))
        (cond ((> arg 0) (skip-chars-forward " \t"))
              ((< arg 0) (simple--forward-sexp arg) (set! arg (- arg))))
        (when (and parens-require-spaces
                   (not (bobp))
                   (member (char-syntax (char-before)) (list "w" "_" (char-syntax close))))
          (insert " "))
        (insert open)
        (save-excursion
          (unless (= arg 0) (simple--forward-sexp arg))
          (insert close)
          (when (and parens-require-spaces
                     (not (eobp))
                     (member (char-syntax (char-after)) (list "w" "_" (char-syntax open))))
            (insert " "))))))

(define-command (insert-parentheses . args)
  "Enclose following ARG sexps in parentheses.
Leave point after open-paren.
A negative ARG encloses the preceding ARG sexps instead.
No argument is equivalent to zero: just insert `()' and leave point between.
If `parens-require-spaces' is true, this command also inserts a space
before and after, depending on the surrounding characters.
If region is active, insert enclosing characters at region boundaries.

This command assumes point is not in a string or comment."
  (simple--insert-pair (simple--raw-arg args) "(" ")"))

(define (simple--newline-and-indent)
  "Delete the horizontal space before point, insert a newline, and indent the new line (not the previous one)."
  (plain-newline-and-indent-op indent-tabs-mode))

(define-command (move-past-close-and-reindent . args)
  "Move past next `)', delete indentation before it, then indent after it."
  ;; Called from Lisp, up-list neither leaves a string first nor says "At top level": it reports the scan error
  (goto-char (scan-lists (point) 1 1))
  (forward-char -1)
  (let loop ()
    (when (save-excursion
            (let ((before-paren (point)))
              (back-to-indentation)
              (and (= (point) before-paren)
                   (begin
                     (forward-line 0)
                     (forward-char -1)
                     (let ((state (syntax-ppss (point))))
                       (and (not (list-ref state 3)) (not (list-ref state 4))))))))
      (delete-indentation)
      (loop)))
  (forward-char 1)
  (simple--newline-and-indent)
  #f)

;;; Shell commands

(define shell-command-buffer-name "*Shell Command Output*"
  "Name of the output buffer for shell commands.")

(define shell-command-buffer-name-async "*Async Shell Command*"
  "Name of the output buffer for asynchronous shell commands.")

(define (simple--max-mini-window-lines)
  "Return the number of lines the echo area may use before a buffer is displayed instead."
  (condition-case e (* 0.25 (+ 1 (window-body-height))) (error 6.25)))

(define (simple--nth args n)
  "Return the Nth element of ARGS (from 0), or #f if there are not that many."
  (cond ((null? args) #f) ((= n 0) (car args)) (else (simple--nth (cdr args) (- n 1)))))

(define (simple--screen-lines text width)
  "Return the number of screen lines TEXT takes in WIDTH columns."
  (apply + (map (lambda (line) (max 1 (quotient (+ (string-width line) (- width 1)) width)))
                (split-string text "\n"))))

(define (simple--display-message-or-buffer name)
  "Show the text of the buffer NAME in the echo area if it is short, otherwise display the buffer."
  (with-current-buffer name
    (let* ((text (buffer-string))
           (trimmed (if (and (> (string-length text) 0)
                             (equal? (substring text (- (string-length text) 1)) "\n"))
                        (substring text 0 (- (string-length text) 1))
                        text))
           (lines (if (equal? text "") 0 (simple--screen-lines trimmed 80))))
      (cond ((= lines 0) #f)
            ;; Not in the echo area if the buffer is already displayed in a window.
            ((and (<= lines (max 1 (simple--max-mini-window-lines)))
                  (not (member name (map window-buffer (window-list)))))
             (goto-char (point-max))
             (if (bolp) (backward-char 1))
             (message trimmed))
            (else
             (goto-char (point-min))
             (display-buffer name))))))

(define (simple--call-shell-region start end command delete)
  "Run COMMAND with the text from START to END as input, and insert the output at point.
If DELETE is true, delete that text first.  Return the exit code."
  (let* ((from (min start end))
         (to (max start end))
         (result (call-shell-command command (buffer-substring from to) #f)))
    (if delete (delete-region from to))
    (insert (cadr result))
    (car result)))

(define (simple--shell-report exit-status)
  "Report an EXIT-STATUS of a shell command that printed no output."
  (message (if (equal? exit-status 0)
               "(Shell command succeeded with no output)"
               (format "(Shell command failed with code %d and no output)" exit-status))))

(define-command (shell-command-on-region . args)
  "Execute string COMMAND in inferior shell with region as input.
Normally display output (if any) in temp buffer `*Shell Command Output*';
Prefix arg means replace the region with it.  Return the exit code of COMMAND.

Called from Lisp with START, END, COMMAND, OUTPUT-BUFFER and REPLACE (and the unused
ERROR-BUFFER and DISPLAY-ERROR-BUFFER).  If OUTPUT-BUFFER is a buffer name, erase that buffer and
insert the output there; if it is another non-nil value, replace the region with the output.
If REPLACE is non-nil, delete the region and insert the output in its place."
  (let* ((interactive (null? args))
         (_ (if (and interactive (not (mark)))
                (user-error "The mark is not set now, so there is no region")))
         (command (if interactive (read-string "Shell command on region: ") (simple--nth args 2)))
         (start (if interactive (region-beginning) (car args)))
         (end (if interactive (region-end) (cadr args)))
         (output-buffer (if interactive (current-prefix-arg) (simple--nth args 3)))
         (replace (if interactive (current-prefix-arg) (simple--nth args 4)))
         (exit-status #f))
    (if (or replace (and output-buffer (not (string? output-buffer))))
        ;; Replace the region with the output of the command.
        (let ((swap (and replace (< start end))))
          (goto-char start)
          (if (and replace (not (eq? replace 'no-mark)))
              (push-mark (point) #t))
          (set! exit-status (simple--call-shell-region start end command replace))
          (if (and replace swap (not (eq? replace 'no-mark)))
              (exchange-point-and-mark)))
        ;; Put the output in a buffer, replacing its entire contents.
        (let ((name (or output-buffer shell-command-buffer-name)))
          (get-buffer-create name)
          (if (equal? name (buffer-name))
              ;; The input is the output buffer: keep only the region, and replace it.
              (begin
                (delete-region (max start end) (point-max))
                (delete-region (point-min) (min start end))
                (set! exit-status (simple--call-shell-region (point-min) (point-max) command #t)))
              (let ((input (buffer-substring (min start end) (max start end))))
                (with-current-buffer name
                  (set-buffer-read-only #f)
                  (erase-buffer)
                  (let ((result (call-shell-command command input #f)))
                    (insert (cadr result))
                    (set! exit-status (car result))))))
          (if (> (with-current-buffer name (buffer-size)) 0)
              (simple--display-message-or-buffer name)
              (simple--shell-report exit-status))))
    exit-status))

(define-command (shell-command . args)
  "Execute string COMMAND in inferior shell; display output, if any.
With prefix argument, insert the COMMAND's output at point.
If COMMAND ends in `&', execute it asynchronously (this editor runs it to the end first, and
shows its output in `*Async Shell Command*').
The output appears in the buffer `*Shell Command Output*'.  If the output is short enough to
display in the echo area, it is shown there.

The optional second argument OUTPUT-BUFFER, if non-nil, says to put the output in some other
buffer.  If OUTPUT-BUFFER is a buffer name, erase that buffer and insert the output there.  If it
is not a buffer name and not nil (which happens interactively when the prefix argument is
given), insert the output in the current buffer after point leaving mark after it."
  (let* ((interactive (null? args))
         (command (if interactive (read-string "Shell command: ") (car args)))
         (output-buffer (if interactive (current-prefix-arg) (simple--nth args 1))))
    (cond ((pair? (regexp-match-positions "[ \t]*&[ \t]*$" command))
           (let* ((start (car (car (regexp-match-positions "[ \t]*&[ \t]*$" command))))
                  (command (substring command 0 start))
                  (name (if (string? output-buffer) output-buffer shell-command-buffer-name-async))
                  (result (call-shell-command command #f #f)))
             (get-buffer-create name)
             (with-current-buffer name
               (set-buffer-read-only #f)
               (erase-buffer)
               (insert (cadr result)))
             (display-buffer name)
             (message (format "%s: %s." command
                              (if (equal? (car result) 0)
                                  "finished"
                                  (format "exited abnormally with code %d" (car result)))))
             #f))
          ((and output-buffer (or (not (string? output-buffer)) (equal? output-buffer (buffer-name))))
           ;; Synchronous command with output in the current buffer.
           (barf-if-buffer-read-only)
           (push-mark (point) #t)
           (let ((result (call-shell-command command #f #f)))
             (insert (cadr result)))
           ;; Like `exchange-point-and-mark', but without activating the mark.
           (let ((end (point)))
             (goto-char (mark))
             (set-mark end)
             (deactivate-mark))
           #f)
          (else
           (shell-command-on-region (point) (point) command output-buffer #f)
           #f))))

(define-command (async-shell-command . args)
  "Execute string COMMAND asynchronously in background.
Like `shell-command', but adds `&' at the end of COMMAND to execute it asynchronously.
The output appears in the buffer `*Async Shell Command*'.
In this editor the command is run to the end before the next command is read."
  (let ((command (if (null? args) (read-string "Async shell command: ") (car args))))
    (shell-command (string-append command " &")
                   (if (and (pair? args) (pair? (cdr args))) (cadr args) shell-command-buffer-name-async))))

;;; Buffers

(define simple--rename-buffer rename-buffer)

(define-command (rename-buffer . args)
  "Change current buffer's name to NEWNAME (a string).
If second arg UNIQUE is nil or omitted, it is an error if a buffer named NEWNAME already exists.
If UNIQUE is non-nil, come up with a new name using `generate-new-buffer-name'.
Interactively, you can set UNIQUE with a prefix argument.
Return the name actually given to the buffer."
  (if (null? args)
      (simple--rename-buffer (read-string "Rename buffer (to new name): ") (and (current-prefix-arg) #t))
      (simple--rename-buffer (car args) (and (pair? (cdr args)) (cadr args)))))

(define (simple--clone-name name)
  "Return NAME without a trailing <N>."
  (let ((match (regexp-match-positions "<[0-9]+>$" name)))
    (if (pair? match) (substring name 0 (car (car match))) name)))

(define (simple--clone-buffer newname)
  "Create a twin copy of the current buffer named after NEWNAME, and return its name."
  (if (buffer-file-name) (error "Cannot clone a file-visiting buffer"))
  (let* ((newname (simple--clone-name (or newname (buffer-name))))
         (text (save-restriction (widen) (buffer-string)))
         (begv (point-min))
         (zv (point-max))
         (pt (point))
         (mk (and (region-active-p) (mark)))
         (modified (buffer-modified-p))
         (new (generate-new-buffer newname)))
    (with-current-buffer new
      (insert text)
      (narrow-to-region begv zv)
      (goto-char pt)
      (if mk (set-mark mk))
      (set-buffer-modified-p modified)
      ;; Inserting the text of the twin must not ask to deactivate the mark.
      (set-deactivate-mark #f))
    new))

(define-command (clone-buffer . args)
  "Create and return a twin copy of the current buffer.
Unlike an indirect buffer, the new buffer can be edited independently of the old one.
NEWNAME is the name of the new buffer.  It may be modified by adding or incrementing <N> at the
end as necessary to create a unique buffer name.  If nil, it defaults to the name of the current
buffer, with the proper suffix.  If DISPLAY-FLAG is non-nil, the new buffer is shown in another
window.  Trying to clone a file-visiting buffer results in an error.
Interactively, DISPLAY-FLAG is t and NEWNAME is the name of the current buffer with appropriate
suffix.  However, if a prefix argument is given, then the command prompts for NEWNAME."
  (let* ((interactive (null? args))
         (_ (if (and interactive (buffer-file-name)) (error "Cannot clone a file-visiting buffer")))
         (newname (if interactive
                      (and (current-prefix-arg) (read-string "Name of new cloned buffer: " (buffer-name)))
                      (car args)))
         (display (if interactive #t (and (pair? (cdr args)) (cadr args))))
         (new (simple--clone-buffer newname)))
    (if display (display-buffer new))
    new))

(define-command (clone-indirect-buffer-other-window . args)
  "Like `clone-buffer' but display in another window and select it.
This editor has no indirect buffers: the new buffer is a twin copy whose text is not shared."
  (let* ((interactive (null? args))
         (newname (if interactive
                      (and (current-prefix-arg) (read-string "Name of indirect buffer: " (buffer-name)))
                      (car args)))
         (new (simple--clone-buffer newname)))
    (display-buffer new)
    (other-window 1)
    new))

(define (simple--mode-arg args)
  "Return what a minor mode command does for ARGS: 1 to enable, 0 to disable, or toggle."
  (cond ((null? args) (let ((arg (current-prefix-arg)))
                        (cond ((not arg) 'toggle)
                              ((eq? arg '-) 0)
                              (else (if (> (prefix-numeric-value arg) 0) 1 0)))))
        ((or (not (car args)) (eq? (car args) 'toggle))
         (if (eq? (car args) 'toggle) 'toggle 1))
        ((and (number? (car args)) (<= (car args) 0)) 0)
        (else 1)))

(define-command (read-only-mode . args)
  "Change whether the current buffer is read-only.
With no argument (or `toggle'), toggle it.  With a positive argument, make the buffer read-only;
with zero or a negative argument, make it writable."
  (let* ((interactive (null? args))
         (mode (simple--mode-arg args))
         (on (if (eq? mode 'toggle) (not (buffer-read-only)) (= mode 1))))
    (set-buffer-read-only on)
    (if interactive
        (message (if on "Read-Only mode enabled in current buffer" "Read-Only mode disabled in current buffer")))
    on))

;;; Builtins of Emacs (C) and small helpers

(define-command (beginning-of-line . args)
  "Move point to beginning of current line.
With argument N not nil or 1, move forward N - 1 lines first.
If point reaches the beginning or end of buffer, it stops there."
  (forward-line (- (simple--arg args) 1))
  (goto-char (line-beginning-position))
  #f)

(define-command (end-of-line . args)
  "Move point to end of current line.
With argument N not nil or 1, move forward N - 1 lines first.
If point reaches the beginning or end of buffer, it stops there."
  (forward-line (- (simple--arg args) 1))
  (goto-char (line-end-position))
  #f)

(define (count-lines start end . rest)
  "Return number of lines between START and END.
This is usually the number of newlines between them, but can be one more if START is not equal
to END and the greater of them is not at the start of a line."
  (save-excursion
    (save-restriction
      (narrow-to-region start end)
      (goto-char (point-min))
      (- (buffer-size) (forward-line (buffer-size))))))

(define-command (copy-region-as-kill . args)
  "Save the region as if killed, but don't kill it.
Called from Lisp with START and END, it saves the text between them."
  (let ((start (if (null? args) (region-beginning) (car args)))
        (end (if (null? args) (region-end) (cadr args))))
    (let ((text (buffer-substring (min start end) (max start end))))
      ;; 前のコマンドが kill なら, その kill につなぐ
      (if (equal? last-command 'kill-region)
          (kill-append text (< end start))
          (kill-new text)))
    #f))

(define-command (insert-char . args)
  "Insert COUNT copies of CHARACTER (a character code or a string of one character).
COUNT defaults to 1.  Interactively, read the character in the minibuffer; COUNT is the prefix argument."
  (let* ((char (if (null? args) (substring (read-string "Insert character: ") 0 1) (car args)))
         (count (cond ((and (pair? args) (pair? (cdr args))) (cadr args))
                      ((null? args) (prefix-numeric-value (current-prefix-arg)))
                      (else 1)))
         (text (if (number? char) (char-to-string char) char)))
    (let loop ((i 0))
      (when (< i count)
        (insert text)
        (loop (+ i 1))))
    #f))

(define (indent-to column . rest)
  "Indent from point with tabs and spaces until COLUMN is reached.
Optional second argument MINIMUM says always do at least MINIMUM spaces even if that goes past COLUMN.
Return the column where point ends up."
  (let ((target (max column (+ (current-column) (if (pair? rest) (car rest) 0)))))
    (simple--indent-to target)
    target))

(define (current-indentation)
  "Return the indentation of the current line.
This is the horizontal position of the character following any initial whitespace."
  (save-excursion
    (back-to-indentation)
    (current-column)))

(define (indent-line-to column)
  "Indent current line to COLUMN.
This function removes or adds spaces and tabs at beginning of line only if necessary."
  (back-to-indentation)
  (let ((current (current-column)))
    (cond ((< current column)
           (when (>= (- column (* (quotient current 8) 8)) 8)
             (delete-region (point) (begin (skip-chars-backward " ") (point))))
           (indent-to column))
          ((> current column)
           (delete-region (begin (move-to-column column #t) (point))
                          (begin (back-to-indentation) (point)))))
    #f))

(define (looking-back regexp . rest)
  "Return true if text before point matches regular expression REGEXP.
Optional second argument LIMIT bounds the search; the match may not start before it."
  (let ((end (point))
        (start (max (if (pair? rest) (car rest) (point-min)) (point-min))))
    (save-excursion
      (save-restriction
        (narrow-to-region start end)
        (goto-char start)
        (and (re-search-forward (string-append "\\(?:" regexp "\\)\\'") end #t) #t)))))

(define (copy-marker position . rest)
  "Return a new marker pointing at the same place as POSITION (a buffer position)."
  (save-excursion
    (goto-char position)
    (point-marker)))

(define (set-marker marker position . rest)
  "Position MARKER before character number POSITION in the current buffer.
Return MARKER."
  (marker-set marker position)
  marker)

(define (buffer-enable-undo . rest)
  "Start keeping undo information for the buffer.  Every buffer keeps it here, so this does nothing."
  #f)

(define-command (delete-trailing-whitespace . args)
  "Delete trailing whitespace between START and END.
If called interactively, START and END are the start/end of the region if the mark is active,
or of the buffer's accessible portion if the mark is inactive.
When END is not given, also delete trailing empty lines at the end of the buffer."
  (let* ((region (and (null? args) (use-region-p)))
         (start (cond ((pair? args) (car args)) (region (region-beginning)) (else #f)))
         (end (cond ((and (pair? args) (pair? (cdr args))) (cadr args)) (region (region-end)) (else #f)))
         (tail (and end (- (point-max) end))))
    (save-excursion
      (goto-char (or start (point-min)))
      (let loop ()
        (when (re-search-forward "[ \t]+$" (and tail (- (point-max) tail)) #t)
          (replace-match "")
          (loop)))
      (unless end
        (goto-char (point-max))
        (when (and (> (point) (point-min)) (equal? (buffer-substring (- (point) 1) (point)) "\n"))
          (skip-chars-backward "\n")
          (when (< (point) (- (point-max) 1))
            (delete-region (+ (point) 1) (point-max))))))
    #f))

(define (replace-regexp-in-string regexp rep string . rest)
  "Replace all matches for REGEXP with REP in STRING.
REP is a string (which may use \\& and \\N), or a function called with the matched text.
Return a new string containing the replacements."
  (let ((work (generate-new-buffer " *replace*")))
    (unwind-protect
        (with-current-buffer work
          (insert string)
          (goto-char (point-min))
          (let loop ()
            (when (and (<= (point) (point-max)) (re-search-forward regexp #f #t))
              (let ((empty (= (match-beginning 0) (match-end 0))))
                (if (procedure? rep)
                    (replace-match (rep (match-string 0)) #t #t)
                    (replace-match rep))
                (if (and empty (not (eobp))) (forward-char 1))
                (unless (and empty (eobp)) (loop)))))
          (buffer-string))
      (kill-buffer work))))

(define (search-forward-regexp . args)
  "Search forward from point for a regular expression.  Same as `re-search-forward'."
  (apply re-search-forward args))

(define (search-backward-regexp . args)
  "Search backward from point for a regular expression.  Same as `re-search-backward'."
  (apply re-search-backward args))

(define (word-search-regexp string . lax)
  "Return a regexp which matches the words of STRING, whatever the punctuation between them.
Unless LAX is non-nil, the regexp also needs word boundaries at its ends."
  (let ((words (split-string (replace-regexp-in-string "[^[:alnum:]]" " " string))))
    (if (null? words)
        ""
        (string-append
         (if (null? lax) "\\b" "")
         (apply string-append
                (cons (regexp-quote (car words))
                      (map (lambda (w) (string-append "\\W+" (regexp-quote w))) (cdr words))))
         (if (null? lax) "\\b" "")))))

(define (word-search-forward string . args)
  "Search forward for STRING as a sequence of words, whatever the punctuation between them.
The other arguments are those of `search-forward'."
  (apply re-search-forward (word-search-regexp string) args))

(define (word-search-backward string . args)
  "Search backward for STRING as a sequence of words, whatever the punctuation between them.
The other arguments are those of `search-backward'."
  (apply re-search-backward (word-search-regexp string) args))

(define (line-number-at-pos . args)
  "Return the line number of POSITION (default: the point), counting from the start of the
accessible part of the buffer.  If the second argument ABSOLUTE is non-nil, count from the
start of the whole buffer."
  (let ((pos (if (and (pair? args) (car args)) (car args) (point)))
        (absolute (and (pair? args) (pair? (cdr args)) (cadr args))))
    (save-excursion
      (save-restriction
        (if absolute (widen))
        (goto-char pos)
        (+ 1 (count-lines (point-min) (line-beginning-position)))))))

(define (kbd--key-p word)
  "Return true if WORD is one key name such as C-x, M-x, RET or <f1>."
  (regexp-match "^(?:[CMSsHA]-)*(?:RET|TAB|SPC|ESC|DEL|LFD|NUL|<[^>]+>|.)$" word))

(define (kbd keys)
  "Convert KEYS (a string of key names, such as \"C-x C-f foo RET\") to a list of keys.
A word that is not a key name stands for the keys of its characters.  The result is what
`execute-kbd-macro' takes."
  (apply append
         (map (lambda (word)
                (if (kbd--key-p word)
                    (list word)
                    (let loop ((i 0) (acc '()))
                      (if (< i (string-length word))
                          (loop (+ i 1) (cons (substring word i (+ i 1)) acc))
                          (reverse acc)))))
              (split-string keys))))
