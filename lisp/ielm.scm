;;; ielm.scm --- interaction with Lisp in a buffer -*- scheme -*-

;; Emacs の ielm.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; `M-x ielm` で `*ielm*` を出す. 最後の行の `ELISP> ` の後に式を書いて RET を押すと, 評価して値を書く.

(define ielm-prompt "ELISP> "
  "Prompt used in the *ielm* buffer.")

(define ielm-history '()
  "The expressions that were sent, the newest first.")

(define ielm--history-index -1)

(define (ielm--insert-prompt)
  "Insert the prompt at the end of the buffer."
  (goto-char (point-max))
  (insert ielm-prompt))

(define (ielm--input)
  "Return the text after the last prompt."
  (let ((start (begin (goto-char (point-max))
                      (if (search-backward ielm-prompt (point-min) #t)
                          (+ (point) (string-length ielm-prompt))
                          (error "No prompt in this buffer")))))
    (goto-char (point-max))
    (buffer-substring start (point-max))))

(define (ielm--evaluate input)
  "Evaluate the expression INPUT, and return its value as a string (or the message of the error)."
  (condition-case e
      (format "%S" (eval (read input)))
    (error (format "*** %s" (error-message-string e)))))

(define-command (ielm-return . args)
  "Evaluate the expression after the last prompt, print the value, and show a new prompt.
Ignores an empty input."
  (let ((input (ielm--input)))
    (cond ((equal? (string-trim input) "")
           (goto-char (point-max))
           (insert "\n")
           (ielm--insert-prompt))
          (else
           (set! ielm-history (cons input ielm-history))
           (set! ielm--history-index -1)
           (let ((result (ielm--evaluate input)))
             (goto-char (point-max))
             (insert "\n" result "\n")
             (ielm--insert-prompt)))))
  #f)

(define (ielm--replace-input text)
  "Replace the text after the last prompt with TEXT."
  (let ((start (begin (goto-char (point-max))
                      (search-backward ielm-prompt (point-min) #t)
                      (+ (point) (string-length ielm-prompt)))))
    (delete-region start (point-max))
    (goto-char (point-max))
    (insert text)))

(define-command (ielm-previous-input . args)
  "Replace the input with the previous expression of the history."
  (when (< (+ ielm--history-index 1) (length ielm-history))
    (set! ielm--history-index (+ ielm--history-index 1))
    (ielm--replace-input (list-ref ielm-history ielm--history-index)))
  #f)

(define-command (ielm-next-input . args)
  "Replace the input with the next expression of the history."
  (cond ((> ielm--history-index 0)
         (set! ielm--history-index (- ielm--history-index 1))
         (ielm--replace-input (list-ref ielm-history ielm--history-index)))
        ((= ielm--history-index 0)
         (set! ielm--history-index -1)
         (ielm--replace-input "")))
  #f)

(define-command (ielm . args)
  "Interactively evaluate Lisp expressions in the *ielm* buffer.
Type an expression after the prompt and RET to evaluate it.  M-p and M-n go through the previous inputs."
  (switch-to-buffer "*ielm*")
  (local-set-key "RET" 'ielm-return)
  (local-set-key "M-p" 'ielm-previous-input)
  (local-set-key "M-n" 'ielm-next-input)
  (when (= (buffer-size) 0)
    (insert "*** Welcome to IELM ***  Type (describe-mode) or press C-h m for help.\n")
    (ielm--insert-prompt))
  (goto-char (point-max))
  #f)
