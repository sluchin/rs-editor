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

(define (simple--move-line n)
  "Move point N lines, and to the goal column.  Signal an error if point cannot move."
  (if (not (member last-command '(next-line previous-line)))
      (set! temporary-goal-column (current-column)))
  (let ((start (line-beginning-position)))
    (forward-line n)
    (if (and (not (= n 0)) (= start (line-beginning-position)))
        (error (if (> n 0) "End of buffer" "Beginning of buffer"))
        (move-to-column temporary-goal-column))))

(define-command (next-line . args)
  "Move point to the next line, staying in the same column if possible.
Signal an error at the last line."
  (simple--move-line (simple--arg args)))

(define-command (previous-line . args)
  "Move point to the previous line, staying in the same column if possible.
Signal an error at the first line."
  (simple--move-line (- (simple--arg args))))

(define-command (move-beginning-of-line . args)
  "Move point to the beginning of the line."
  (goto-char (line-beginning-position)))

(define-command (move-end-of-line . args)
  "Move point to the end of the line."
  (goto-char (line-end-position)))

(define-command (beginning-of-buffer . args)
  "Move point to the beginning of the buffer, and leave the mark where point was."
  (push-mark (point))
  (goto-char (point-min)))

(define-command (end-of-buffer . args)
  "Move point to the end of the buffer, and leave the mark where point was."
  (push-mark (point))
  (goto-char (point-max)))

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

(define-command (goto-line . args)
  "Go to line N.  Interactively, read N in the minibuffer."
  (let ((n (if (null? args)
               (string->number (read-string "Goto line: "))
               (car args))))
    (push-mark (point))
    (goto-char (point-min))
    (forward-line (- n 1))))

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
