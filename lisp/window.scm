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
the default is another buffer."
  (let ((name (if (null? args)
                  (let ((default (window--other-buffer)))
                    (completing-read (if default
                                         (format "Switch to buffer (default %s): " default)
                                         "Switch to buffer: ")
                                     (buffer-list) #f #f #f #f default))
                  (car args))))
    (unless (equal? name "")
      (pop-to-buffer-same-window name))))

(provide 'window)

;;; window.scm ends here
