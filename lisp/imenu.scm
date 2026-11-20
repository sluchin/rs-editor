;;; imenu.scm --- jump to a definition of the buffer -*- scheme -*-

;; Emacs の imenu.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; 索引は, メジャーモードごとの正規表現で行を読んで作る (構文木は作らない).

(define imenu-expressions
  '(("scheme-mode" "^(def[a-z0-9*-]*[ \t]+(?\\([^ \t()]+\\)" 1)
    ("emacs-lisp-mode" "^(def[a-z0-9*-]*[ \t]+(?\\([^ \t()]+\\)" 1)
    ("lisp-interaction-mode" "^(def[a-z0-9*-]*[ \t]+(?\\([^ \t()]+\\)" 1)
    ("lisp-data-mode" "^(def[a-z0-9*-]*[ \t]+(?\\([^ \t()]+\\)" 1)
    ("c-mode" "^[A-Za-z_][A-Za-z0-9_ \t*]*[ \t*]\\([A-Za-z_][A-Za-z0-9_]*\\)[ \t]*([^;]*$" 1)
    ("c++-mode" "^[A-Za-z_][A-Za-z0-9_:<> \t*&~]*[ \t*&]\\([A-Za-z_~][A-Za-z0-9_:~]*\\)[ \t]*([^;]*$" 1)
    ("rust-mode" "^[ \t]*\\(pub\\(([a-z]+)\\)?[ \t]+\\)?\\(async[ \t]+\\)?\\(unsafe[ \t]+\\)?\\(fn\\|struct\\|enum\\|trait\\|mod\\|type\\)[ \t]+\\([A-Za-z0-9_]+\\)" 6)
    ("typescript-mode" "^[ \t]*\\(export[ \t]+\\)?\\(default[ \t]+\\)?\\(async[ \t]+\\)?\\(function\\|class\\|interface\\|type\\|enum\\)[ \t]+\\([A-Za-z0-9_$]+\\)" 5)
    ("markdown-mode" "^#\\{1,6\\}[ \t]+\\(.*\\)$" 1)
    ("org-mode" "^\\(\\*+\\)[ \t]+\\(.*\\)$" 2))
  "List of (MODE-NAME REGEXP GROUP) that find the definitions of a buffer for `imenu'.
MODE-NAME is the major mode (`major-mode-name'); GROUP is the group of REGEXP that is the name.")

(define (imenu--expression)
  "Return the (REGEXP GROUP) that finds the definitions of the current buffer, or #f."
  (let ((found (assoc (symbol->string (major-mode-name)) imenu-expressions)))
    (and found (cdr found))))

(define (imenu--index)
  "Return the definitions of the current buffer as a list of (NAME POSITION), in buffer order."
  (let ((expression (imenu--expression)))
    (if (not expression)
        (error "No imenu tags in this buffer")
        (save-excursion
          (goto-char (point-min))
          (let loop ((found '()))
            (let* ((start (line-beginning-position))
                   (line (buffer-substring start (line-end-position)))
                   (index (if (string-match (car expression) line)
                              (let ((name (match-string (cadr expression) line)))
                                (if (and name (not (equal? name "")))
                                    (cons (list name start) found)
                                    found))
                              found)))
              (if (zero? (forward-line 1))
                  (if (eobp) (reverse index) (loop index))
                  (reverse index))))))))

(define-command (imenu . args)
  "Jump to a definition in the buffer: choose one of the names found, by completion.
Called from Lisp with a NAME, jump to the first definition of that name."
  (let* ((index (imenu--index))
         (names (map car index)))
    (if (null? index)
        (error "No items suitable for an index found in this buffer")
        (let* ((name (if (null? args) (completing-read "Index item: " names) (car args)))
               (entry (assoc name index)))
          (if (not entry)
              (error (format "No such index entry: %s" name))
              (begin (push-mark)
                     (goto-char (cadr entry))
                     (back-to-indentation)))))
    #f))
