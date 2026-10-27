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

(define-command (newline . args)
  "Insert N newlines.  N is the prefix argument, or 1."
  (let loop ((n (simple--arg args)))
    (when (> n 0)
      (insert "\n")
      (loop (- n 1)))))

(define-command (indent-for-tab-command . args)
  "Insert a tab character at point."
  (insert "\t"))

(define-command (delete-forward-char . args)
  "Delete the N characters after point (the N characters before point if N is negative)."
  (delete-char (simple--arg args)))

(define-command (delete-backward-char . args)
  "Delete the N characters before point (the N characters after point if N is negative)."
  (delete-char (- (simple--arg args))))

;;; Killing and yanking

(define (simple--kill from to)
  "Delete the text between FROM and TO and save it in the kill ring.
If the previous command was a kill, append the text to its kill."
  (let ((text (buffer-substring from to)))
    (delete-region from to)
    (if (equal? last-command 'kill-region)
        (kill-append text (< to from))
        (kill-new text))
    (set! this-command 'kill-region)))

(define-command (kill-region . args)
  "Kill the text between point and the mark, or between the two positions FROM and TO.
The killed text is saved in the kill ring."
  (if (null? args)
      (begin
        (simple--kill (region-beginning) (region-end))
        (deactivate-mark))
      (simple--kill (car args) (cadr args))))

(define-command (kill-ring-save . args)
  "Save the text between point and the mark in the kill ring, without killing it."
  (kill-new (buffer-substring (region-beginning) (region-end)))
  (deactivate-mark))

(define-command (kill-line . args)
  "Kill the rest of the line, or the newline if point is at the end of the line.
Signal an error at the end of the buffer."
  (let ((start (point)))
    (cond ((eobp) (error "End of buffer"))
          ((eolp) (simple--kill start (+ start 1)))
          (else (simple--kill start (line-end-position))))))

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

(define-command (yank . args)
  "Insert the most recent kill at point, and leave the mark at the start of the text."
  (push-mark (point))
  (insert-for-yank (current-kill 0))
  (set! this-command 'yank))

(define-command (yank-pop . args)
  "Replace the text just yanked with the previous kill in the kill ring.
Signal an error if the previous command was not a yank."
  (if (not (equal? last-command 'yank))
      (error "Previous command was not a yank")
      (begin
        (delete-region (mark) (point))
        (insert-for-yank (current-kill 1))
        (set! this-command 'yank))))

;;; The mark

(define-command (set-mark-command . args)
  "Set the mark at point, and activate it."
  (push-mark (point) #t #t)
  (message "Mark set"))

(define-command (exchange-point-and-mark . args)
  "Put the mark where point is now, and point where the mark is now."
  (let ((old (mark)))
    (if (not old) (error "No mark set in this buffer"))
    (set-mark (point))
    (goto-char old)))

(define-command (mark-whole-buffer . args)
  "Put point at the beginning and the mark at the end of the buffer."
  (push-mark (point-max) #t #t)
  (goto-char (point-min)))

;;; Undoing

(define-command (undo . args)
  "Undo some previous changes.  Repeat the command to undo more.
With argument N, undo N groups of changes."
  (let ((n (simple--arg args)))
    (when (not (equal? last-command 'undo))
      ;; get rid of the boundary that the command loop put at the front
      (set! pending-undo-list (primitive-undo 1 (buffer-undo-list))))
    (if (null? pending-undo-list)
        (error "No further undo information")
        (set! pending-undo-list (primitive-undo n pending-undo-list)))
    (set! this-command 'undo)
    (message "Undo")))

(define-command (keyboard-quit . args)
  "Cancel the running command by signaling a quit."
  (error "Quit"))

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

;;; Running commands

(define (simple--command-names)
  "Return the names of all commands, sorted."
  (filter (lambda (name) (commandp (string->symbol name))) (help-function-names)))

(define-command (execute-extended-command . args)
  "Read a command name in the minibuffer, with completion, and run it."
  (let ((name (completing-read "M-x " (simple--command-names) #f #t)))
    (when (not (equal? name ""))
      (set! this-command (string->symbol name))
      (call-interactively (string->symbol name)))))

(define-command (eval-expression . args)
  "Read an expression in the minibuffer, evaluate it, and show the value."
  (let ((expression (if (null? args) (read-string "Eval: ") (car args))))
    (message (format "%s" (eval (read expression))))))

(provide 'simple)

;;; simple.scm ends here
