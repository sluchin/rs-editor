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

(define-command (goto-line . args)
  "Go to line N, counting from line 1 at the beginning of the buffer.
N is the argument or the numeric prefix argument; otherwise read it in the minibuffer.
Push the mark at the previous position, unless the mark is active."
  (let ((n (cond ((pair? args) (car args))
                 ((current-prefix-arg) (prefix-numeric-value (current-prefix-arg)))
                 (else (string->number (read-string "Goto line: "))))))
    (or (region-active-p) (push-mark (point)))
    (goto-char (point-min))
    (forward-line (- n 1))))

(define-command (goto-line-relative . args)
  "Go to line N, counting from the line at the beginning of the accessible part.
The buffer is never narrowed, so this is goto-line."
  (apply goto-line args))

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

(define-command (newline . args)
  "Insert N newlines.  N is the prefix argument, or 1.
Signal an error if N is negative."
  (let ((n (simple--arg args)))
    (if (< n 0) (error "Repetition argument has to be non-negative"))
    (let loop ((n n))
      (when (> n 0)
        (insert "\n")
        (loop (- n 1))))))

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
  "Insert a tab character at point."
  (insert "\t"))

(define-command (default-indent-new-line . args)
  "Break the line at point and delete the spaces around the break.
The new line is not indented yet: indentation comes with the indentation commands."
  (newline 1)
  (save-excursion
    (backward-char 1)
    (simple--delete-space " \t" #f))
  (simple--delete-space " \t" #f))

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
  "Save the text between point and the mark in the kill ring, without killing it."
  (kill-new (buffer-substring (region-beginning) (region-end)))
  (set-deactivate-mark #t))

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
With an argument N, use the Nth previous kill.  Signal an error if the previous
command was not a yank."
  (if (not (equal? last-command 'yank))
      (error "Previous command was not a yank")
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
  (let ((aux (lambda (x)
               (let* ((first (begin (mover x) (point)))
                      (second (begin (mover (- x)) (point))))
                 (list first second)))))
    (cond ((= arg 0)
           (let ((pos1 #f) (pos2 #f))
             (save-excursion
               (set! pos1 (aux 1))
               (goto-char (or (mark) (error "No mark set in this buffer")))
               (set! pos2 (aux 1))
               (simple--transpose-1 pos1 pos2))
             (exchange-point-and-mark)))
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
  "Swap the text of the two regions POS1 and POS2, each a list (start end)."
  (let* ((a (if (> (car pos1) (cadr pos1)) (list (cadr pos1) (car pos1)) pos1))
         (b (if (> (car pos2) (cadr pos2)) (list (cadr pos2) (car pos2)) pos2))
         (first (if (> (car a) (car b)) b a))
         (second (if (> (car a) (car b)) a b)))
    (if (> (cadr first) (car second)) (error "Don’t have two things to transpose"))
    (let ((text1 (buffer-substring (car first) (cadr first)))
          (text2 (buffer-substring (car second) (cadr second))))
      (delete-region (car second) (cadr second))
      (goto-char (car second))
      (insert text1)
      (delete-region (car first) (cadr first))
      (goto-char (car first))
      (insert text2))))

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
the character after point with its codes, the position, and the column."
  (let* ((total (buffer-size))
         (pos (point))
         (percent (simple--round-percent (- pos 1) (max 1 total)))
         (column (current-column))
         (narrowed #f))
    (if (eobp)
        (message "%s" (format "point=%d of %d (EOB) column=%d" pos total column))
        (let* ((char (char-after))
               (code (string-to-char char))
               (shown (if (< code 128) (simple--single-key-description char) char))
               (encoding (if (>= code 128)
                             (format "(%d, #o%o, #x%x, file ...)" code code code)
                             (format "(%d, #o%o, #x%x)" code code code))))
          (message "%s" (format "Char: %s %s point=%d of %d (%d%%) column=%d"
                                shown encoding pos total percent column))))))

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
(define simple--kill-buffer kill-buffer)

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

(define-command (kill-buffer . args)
  "Kill the buffer named by the argument.  Interactively, read the name in the minibuffer.
The default is the current buffer.  Ask before killing a modified file buffer."
  (let* ((asked (if (null? args)
                    (completing-read (format "Kill buffer (default %s): " (buffer-name))
                                     (buffer-list))
                    (car args)))
         (name (if (equal? asked "") (buffer-name) asked)))
    (when (and (buffer-file-name name)
               (buffer-modified-p name)
               (not (yes-or-no-p (format "Buffer %s modified; kill anyway? " name))))
      (error "Quit"))
    (with-current-buffer name (set-buffer-modified-p #f))
    (simple--kill-buffer name)))

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

(define-command (end-of-defun . args)
  "Move forward to the end of a defun.  With argument N, do it N times.
Push the mark first, unless repeated."
  (simple--push-mark-for-defun 'end-of-defun)
  (goto-char (end-of-defun-position (point) (simple--arg args))))

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

(provide 'simple)

;;; simple.scm ends here
