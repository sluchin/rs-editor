;;; window.scm --- buffer and window commands -*- scheme -*-

;; Emacs の window.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).

(define (window--other-buffer)
  "Return the name of a buffer other than the current one, or #f if there is none."
  (let loop ((names (buffer-list)))
    (cond ((null? names) #f)
          ((equal? (car names) (buffer-name)) (loop (cdr names)))
          (else (car names)))))

(define-command (switch-to-buffer . args)
  "Display buffer BUFFER-OR-NAME in the selected window.
If there is no buffer named BUFFER-OR-NAME, create one.
Interactively, read the name in the minibuffer with completion;
the default is another buffer.
If the selected window is strongly dedicated to its buffer, signal an error;
if it is dedicated in another way, stop dedicating it."
  (if (and (null? args) (not window--override-next))
      (cond ((eq? (window-dedicated-p) #t)
             (user-error "Cannot switch buffers in a dedicated window"))
            ((window-dedicated-p) (set-window-dedicated-p (selected-window) #f))))
  (let ((name (if (null? args)
                  (let ((default (window--other-buffer)))
                    (completing-read (if default
                                         (format "Switch to buffer (default %s): " default)
                                         "Switch to buffer: ")
                                     (buffer-list) #f #f #f #f default))
                  (car args))))
    (unless (equal? name "")
      (pop-to-buffer-same-window name))))

(define window--display-buffer display-buffer)
(define window--pop-to-buffer-same-window pop-to-buffer-same-window)

;;; Displaying the buffer of the next command in another or the same window

(define window--override-next #f
  "How the next command displays a buffer: other (in another window), same (in the selected window), or #f.")

(define window--override-command #f
  "The command that set `window--override-next'.")

(define (window--override-post-command)
  "Forget the override of `other-window-prefix' or `same-window-prefix' after the next command."
  (unless (eq? this-command window--override-command)
    (window--override-clear)))

(define (window--override-clear)
  (set! window--override-next #f)
  (remove-hook 'post-command-hook 'window--override-post-command))

(define (window--override-next-command how)
  (set! window--override-next how)
  (set! window--override-command this-command)
  (add-hook 'post-command-hook 'window--override-post-command))

(define (window--display-overridden name)
  "Display buffer NAME as `other-window-prefix' or `same-window-prefix' asked, and forget the request.
Return #f if there is no such request."
  (let ((how window--override-next))
    (cond ((eq? how 'other)
           (window--override-clear)
           (get-buffer-create name)
           (window--pop-to-buffer-other-window-1 name)
           #t)
          ((eq? how 'same)
           (window--override-clear)
           (window--pop-to-buffer-same-window name)
           #t)
          (else #f))))

(define (pop-to-buffer-same-window name . args)
  "Display buffer NAME in the selected window and make it current.
After `other-window-prefix' or `same-window-prefix', display it as they asked."
  (or (window--display-overridden name)
      (window--pop-to-buffer-same-window name)))

(define-command (other-window-prefix . args)
  "Display the buffer of the next command in a new window.
The next buffer is the buffer displayed by the next command invoked
immediately after this command (ignoring reading from the minibuffer)."
  (window--override-next-command 'other)
  (message "Display next command buffer in a new window..."))

(define-command (same-window-prefix . args)
  "Display the buffer of the next command in the same window.
The next buffer is the buffer displayed by the next command invoked
immediately after this command (ignoring reading from the minibuffer)."
  (window--override-next-command 'same)
  (message "Display next command buffer in the same window..."))

;;; Dedicated windows

(define toggle-window-dedicated-flag 'interactive
  "What `toggle-window-dedicated' passes to `set-window-dedicated-p' by default.")

(define-command (toggle-window-dedicated . args)
  "Toggle whether WINDOW is dedicated to its current buffer.
WINDOW defaults to the selected one.  If FLAG is t (interactively, the
prefix argument), make the window \"strongly\" dedicated to its buffer.
If INTERACTIVE is true, print a message describing the dedication
status of WINDOW, after toggling it."
  (let* ((window (if (and (pair? args) (car args)) (car args) (selected-window)))
         (flag (cond ((and (pair? args) (pair? (cdr args))) (cadr args))
                     ((null? args) (current-prefix-arg))
                     (else #f)))
         (interactive (if (and (pair? args) (pair? (cdr args)) (pair? (cddr args)))
                          (caddr args)
                          (null? args)))
         (flag (cond ((pair? flag) (if (eq? toggle-window-dedicated-flag #t) 'interactive #t))
                     ((not flag) toggle-window-dedicated-flag)
                     (else flag))))
    (if (window-dedicated-p window)
        (set-window-dedicated-p window #f)
        (set-window-dedicated-p window flag))
    (if interactive
        (message "Window is %s dedicated to buffer %s"
                 (let ((status (window-dedicated-p window)))
                   (cond ((not status) "no longer")
                         ((eq? status #t) "now strongly")
                         (else "now")))
                 (window-buffer window)))))

(define (window--other-window-for-scrolling)
  "Return the window after the selected one, the one the scrolling commands act on."
  (let ((windows (window-list)) (selected (selected-window)))
    (if (null? (cdr windows)) (error "There is no other window"))
    (let loop ((rest windows))
      (cond ((null? rest) (car windows))
            ((= (car rest) selected) (if (null? (cdr rest)) (car windows) (cadr rest)))
            (else (loop (cdr rest)))))))

(define (window--call-in-window window thunk)
  "Call THUNK with WINDOW selected, then select the window that was selected before."
  (let ((old (selected-window)))
    (select-window window)
    (unwind-protect (thunk)
      (if (member old (window-list)) (select-window old)))))

(define (window--pop-to-buffer-other-window name)
  "Show buffer NAME in another window (splitting if there is only one) and select that window.
After `other-window-prefix' or `same-window-prefix', display it as they asked."
  (or (window--display-overridden name)
      (window--pop-to-buffer-other-window-1 name)))

(define (window--pop-to-buffer-other-window-1 name)
  (window--display-buffer name)
  (let ((selected (selected-window)))
    (let loop ((windows (window-list)))
      (cond ((null? windows) #f)
            ((and (not (= (car windows) selected)) (equal? (window-buffer (car windows)) name))
             (select-window (car windows)))
            (else (loop (cdr windows)))))))

(define (window--default-buffer-name args prompt)
  "Return the buffer name in ARGS, or read one (default: another buffer) with PROMPT."
  (if (pair? args)
      (car args)
      (let ((default (window--other-buffer)))
        (completing-read (if default (format "%s (default %s): " prompt default) (format "%s: " prompt))
                         (buffer-list) #f #f #f #f default))))

(define-command (switch-to-buffer-other-window . args)
  "Select the buffer BUFFER-OR-NAME in another window.
If there is only one window, split it.  If there is no buffer named BUFFER-OR-NAME, create one."
  (let ((name (window--default-buffer-name args "Switch to buffer in other window")))
    (unless (equal? name "")
      (get-buffer-create name)
      (window--pop-to-buffer-other-window name))))

(define-command (display-buffer . args)
  "Show buffer BUFFER-OR-NAME in some window, without selecting it.
If there is only one window, split it.  Interactively, read the name in the minibuffer."
  (let ((name (window--default-buffer-name args "Display buffer")))
    (unless (equal? name "")
      (let ((how window--override-next))
        (cond ((eq? how 'other)
               (window--override-clear)
               (get-buffer-create name)
               (window--call-in-window (selected-window)
                                       (lambda () (window--pop-to-buffer-other-window-1 name))))
              ((eq? how 'same)
               (window--override-clear)
               (get-buffer-create name)
               (set-window-buffer (selected-window) name))
              (else
               (apply window--display-buffer name (if (pair? args) (cdr args) '()))))))))
(define (window--buffer-names)
  "Return the names of the buffers that `next-buffer' and `previous-buffer' may show."
  (filter (lambda (name) (not (and (> (string-length name) 0) (equal? (substring name 0 1) " "))))
          (buffer-list)))

(define (window--cycle-buffer step)
  (let* ((names (window--buffer-names))
         (count (length names)))
    (let loop ((i 0) (rest names))
      (cond ((null? rest) (error "No other buffer to show"))
            ((equal? (car rest) (buffer-name))
             (if (= count 1)
                 (error "No other buffer to show")
                 (window--pop-to-buffer-same-window (list-ref names (modulo (+ i step) count)))))
            (else (loop (+ i 1) (cdr rest)))))))

(define-command (next-buffer . args)
  "In the selected window, switch to the next buffer in the buffer list.
With a numeric prefix argument N, switch N buffers forward."
  (let ((n (simple--arg args)))
    (window--cycle-buffer n)))

(define-command (previous-buffer . args)
  "In the selected window, switch to the previous buffer in the buffer list.
With a numeric prefix argument N, switch N buffers back."
  (let ((n (simple--arg args)))
    (window--cycle-buffer (- n))))

(define-command (delete-windows-on . args)
  "Delete all windows showing the buffer BUFFER-OR-NAME (default: the current buffer).
If it is the only window, show another buffer in it instead."
  (let ((name (if (pair? args) (car args)
                  (let ((read (completing-read (format "Delete windows on (default %s): " (buffer-name))
                                               (buffer-list))))
                    (if (equal? read "") (buffer-name) read)))))
    (for-each (lambda (window)
                (if (and (member window (window-list))
                         (equal? (window-buffer window) name))
                    (if (null? (cdr (window-list)))
                        (let ((other (window--other-buffer-than name)))
                          (if other (window--show-buffer window other)))
                        (delete-window window))))
              (window-list))))

(define (window--show-buffer window name)
  "Show buffer NAME in WINDOW; if WINDOW is the selected one, make the buffer current too."
  (if (= window (selected-window))
      (window--pop-to-buffer-same-window name)
      (set-window-buffer window name)))

(define (window--other-buffer-than name)
  (let loop ((names (window--buffer-names)))
    (cond ((null? names) #f)
          ((equal? (car names) name) (loop (cdr names)))
          (else (car names)))))

(define window--kill-buffer kill-buffer)

(define (window--kill-buffer-with-hook name)
  "Kill the buffer NAME, running `kill-buffer-hook' in it first (unless it visits a modified file)."
  (when (and (string? name) (member name (buffer-list)) (not (and (buffer-file-name name) (buffer-modified-p name))))
    (with-current-buffer name (run-hooks 'kill-buffer-hook)))
  (window--kill-buffer name))

(define-command (kill-buffer . args)
  "Kill the buffer named by the argument.  Interactively, read the name in the minibuffer.
The default is the current buffer.  Ask before killing a modified file buffer.
Called from Lisp with a name, kill the buffer without asking (an error if it visits a modified file)."
  (if (pair? args)
      (window--kill-buffer-with-hook (car args))
      (let* ((asked (completing-read (format "Kill buffer (default %s): " (buffer-name))
                                     (buffer-list)))
             (name (if (equal? asked "") (buffer-name) asked)))
        (when (and (buffer-file-name name)
                   (buffer-modified-p name)
                   (not (yes-or-no-p (format "Buffer %s modified; kill anyway? " name))))
          (signal 'quit '()))
        (with-current-buffer name (set-buffer-modified-p #f))
        (window--kill-buffer-with-hook name))))

(define-command (kill-buffer-and-window . args)
  "Kill the current buffer and delete the selected window."
  (let ((window (selected-window)))
    (kill-buffer (buffer-name))
    (if (member window (window-list))
        (delete-window window))))

(define-command (quit-window . args)
  "Quit the selected window and bury its buffer.
If there are other windows, delete the window; otherwise show another buffer in it.
With a prefix argument, kill the buffer instead of burying it."
  (let ((name (buffer-name))
        (kill (simple--raw-arg args)))
    (if (null? (cdr (window-list)))
        (let ((other (window--other-buffer-than name)))
          (if other (window--pop-to-buffer-same-window other)))
        (delete-window (selected-window)))
    (if (and kill (get-buffer name)) (kill-buffer name))))

(define-command (scroll-other-window . args)
  "Scroll next window upward ARG lines; or near full screen if no ARG.
A negative ARG scrolls downward, and `-' means a screenful downward."
  (let ((arg (simple--raw-arg args)))
    (window--call-in-window
     (window--other-window-for-scrolling)
     (lambda ()
       (cond ((not arg) (scroll-up-command))
             ((eq? arg '-) (scroll-down-command))
             (else (let ((n (prefix-numeric-value arg)))
                     (if (> n 0)
                         (if (eobp) (signal 'end-of-buffer '()) (forward-line n))
                         (if (bobp) (signal 'beginning-of-buffer '()) (forward-line n))))))))))

(define-command (scroll-other-window-down . args)
  "Scroll the next window downward ARG lines; or near full screen if no ARG."
  (let ((arg (simple--raw-arg args)))
    (window--call-in-window
     (window--other-window-for-scrolling)
     (lambda ()
       (cond ((not arg) (scroll-down-command))
             ((eq? arg '-) (scroll-up-command))
             (else (let ((n (prefix-numeric-value arg)))
                     (if (> n 0)
                         (if (bobp) (signal 'beginning-of-buffer '()) (forward-line (- n)))
                         (if (eobp) (signal 'end-of-buffer '()) (forward-line (- n)))))))))))

(define-command (beginning-of-buffer-other-window . args)
  "Move point to the beginning of the buffer in the next window.
With numeric argument N, put point N/10 of the way from the beginning."
  (let ((arg (simple--raw-arg args)))
    (window--call-in-window (window--other-window-for-scrolling)
                            (lambda () (beginning-of-buffer arg)))))

(define-command (end-of-buffer-other-window . args)
  "Move point to the end of the buffer in the next window.
With numeric argument N, put point N/10 of the way from the end."
  (let ((arg (simple--raw-arg args)))
    (window--call-in-window (window--other-window-for-scrolling)
                            (lambda () (end-of-buffer arg)))))

(define-command (recenter-other-window . args)
  "Call `recenter-top-bottom' in the next window."
  (window--call-in-window (window--other-window-for-scrolling)
                          (lambda () (recenter))))

;;; Window sizes

(define window-min-height 4
  "The minimum total height, in lines, of any window (the mode line included).")

(define window--balance-windows window-balance)

(define-command (split-root-window-below . args)
  "Split the whole screen into two windows, one above the other; the new one is at the bottom."
  (split-root-window 'below)
  #f)

(define-command (split-root-window-right . args)
  "Split the whole screen into two windows side by side; the new one is at the right."
  (split-root-window 'right)
  #f)

(define-command (balance-windows . args)
  "Make all windows the same height and width."
  (window--balance-windows)
  #f)

(define-command (enlarge-window . args)
  "Make the selected window ARG lines taller (negative: shorter).
Take the lines from the neighbouring window."
  (window-resize (selected-window) (simple--arg args))
  #f)

(define-command (shrink-window . args)
  "Make the selected window ARG lines shorter (negative: taller)."
  (window-resize (selected-window) (- (simple--arg args)))
  #f)

(define-command (enlarge-window-horizontally . args)
  "Make the selected window ARG columns wider (negative: narrower)."
  (window-resize (selected-window) (simple--arg args) #t)
  #f)

(define-command (shrink-window-horizontally . args)
  "Make the selected window ARG columns narrower (negative: wider)."
  (window-resize (selected-window) (- (simple--arg args)) #t)
  #f)

(define (window--buffer-lines)
  "Return the number of lines of text in the current buffer."
  (let* ((text (buffer-string))
         (n (length (split-string text "\n"))))
    (if (and (> (string-length text) 0)
             (equal? (substring text (- (string-length text) 1)) "\n"))
        (- n 1)
        n)))

(define (window--fit-height)
  "Return the total height, in lines, that shows all the text of the current buffer."
  (max window-min-height (+ (window--buffer-lines) 1)))

(define-command (shrink-window-if-larger-than-buffer . args)
  "Shrink the selected window if it is taller than its buffer's text.
Do nothing if there is only one window."
  (let ((height (window-total-height))
        (fit (window--fit-height)))
    (if (and (pair? (cdr (window-list))) (> height fit))
        (window-resize (selected-window) (- fit height)))
    #f))

(define-command (fit-window-to-buffer . args)
  "Adjust the height of the selected window to show all the text of its buffer.
Do nothing if there is only one window."
  (let ((height (window-total-height))
        (fit (window--fit-height)))
    (if (and (pair? (cdr (window-list))) (not (= height fit)))
        (window-resize (selected-window) (- fit height)))
    #f))

(define window--line-position 0
  "Where `move-to-window-line-top-bottom' put point last: 0 middle, 1 top, 2 bottom.")

(define-command (move-to-window-line-top-bottom . args)
  "Position point relative to the window: the middle, then the top, then the bottom.
With a numeric argument N, put point at the beginning of line N of the window
(negative: counted from the bottom)."
  (let ((arg (simple--raw-arg args)))
    (cond (arg (move-to-window-line (prefix-numeric-value arg)))
          (else
           (set! window--line-position
                 (if (eq? last-command this-command) (modulo (+ window--line-position 1) 3) 0))
           (cond ((= window--line-position 0) (move-to-window-line))
                 ((= window--line-position 1) (move-to-window-line 0))
                 (else (move-to-window-line -1)))))
    #f))

(define-command (project-tree . args)
  "Show or hide the tree of the files of the project in the side bar."
  (toggle-project-tree))

(provide 'window)

;;; window.scm ends here
