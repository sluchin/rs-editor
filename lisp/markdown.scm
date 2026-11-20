;;; markdown.scm --- major mode for Markdown -*- scheme -*-

;; markdown-mode (Jason Blevins) の, 基本の部分. 起動のときに読む (core/src/scheme/loadup.rs).
;; .md, .markdown, .mdown のファイルを開くと, Markdown モードになる (find-file-hook).
;; 色は `highlight-regexp' の face (src/styles/CodeEditor.css の cm-face-markdown-*). 行ごとに見るので,
;; コードのブロックの中は, 色を付けないだけでなく, 見出しの移動でも, 見出しとして数える.

(define markdown-font-lock-keywords
  '(("^#[ \\t]+.*$" "markdown-header-1" 0)
    ("^##[ \\t]+.*$" "markdown-header-2" 0)
    ("^###[ \\t]+.*$" "markdown-header-3" 0)
    ("^####[ \\t]+.*$" "markdown-header-4" 0)
    ("^#####[ \\t]+.*$" "markdown-header-5" 0)
    ("^######[ \\t]+.*$" "markdown-header-6" 0)
    ("^(?:`{3,}|~{3,}).*$" "markdown-code-fence" 0)
    ("^(?:-{3,}|\\*{3,}|_{3,})[ \\t]*$" "markdown-hr" 0)
    ("^>.*$" "markdown-blockquote" 0)
    ("^[ \\t]*([-+*]|[0-9]+[.)])[ \\t]" "markdown-list" 1)
    ("\\*\\*[^*\\n]+\\*\\*|__[^_\\n]+__" "bold" 0)
    ("(?:^|[^*\\w])(\\*[^*\\s](?:[^*\\n]*[^*\\s])?\\*)(?:[^*\\w]|$)" "italic" 1)
    ("(?:^|[^_\\w])(_[^_\\s](?:[^_\\n]*[^_\\s])?_)(?:[^_\\w]|$)" "italic" 1)
    ("`[^`\\n]+`" "markdown-inline-code" 0)
    ("!?\\[[^\\]\\n]*\\]\\([^)\\n]*\\)" "markdown-link" 0)
    ("<(?:https?|ftp|mailto):[^>\\s]+>|(?:https?|ftp)://[^\\s)>\\]]+" "markdown-link" 0))
  "List of (REGEXP FACE GROUP) that `markdown-mode' highlights; REGEXP is in Rust syntax.")

(define markdown-mode-keys
  '(("C-c C-n" markdown-outline-next) ("C-c C-p" markdown-outline-previous)
    ("C-c C-f" markdown-forward-same-level) ("C-c C-b" markdown-backward-same-level)
    ("C-c C-u" markdown-outline-up) ("C-c C--" markdown-promote) ("C-c C-=" markdown-demote))
  "List of (KEY COMMAND) that `markdown-mode' binds in the buffer.")

(define (markdown--level-at-line)
  "Return the level (1 to 6) of the heading on the current line, or #f if the line is not a heading."
  (let ((m (regexp-match "^(#{1,6})[ \\t]" (buffer-substring (line-beginning-position) (line-end-position)))))
    (and m (string-length (cadr m)))))

(define (markdown--find-heading step accept)
  "Move by lines (STEP is 1 or -1) to the first heading whose level ACCEPT takes, and return its level.
Signal an error, and keep point, if there is no such heading."
  (let ((start (point)))
    (let loop ()
      (if (not (zero? (forward-line step)))
          (begin (goto-char start) (user-error "No more headings"))
          (let ((level (markdown--level-at-line)))
            (cond ((and level (accept level)) level)
                  ((and (= step 1) (eobp)) (goto-char start) (user-error "No more headings"))
                  (else (loop))))))))

(define (markdown--current-level)
  "Return the level of the heading that holds point (on its line or above), or #f."
  (save-excursion
    (let loop ()
      (or (markdown--level-at-line)
          (and (zero? (forward-line -1)) (loop))))))

(define-command (markdown-outline-next . args)
  "Move to the next heading line."
  (end-of-line)
  (markdown--find-heading 1 (lambda (level) #t))
  (beginning-of-line)
  #f)

(define-command (markdown-outline-previous . args)
  "Move to the previous heading line."
  (beginning-of-line)
  (markdown--find-heading -1 (lambda (level) #t))
  #f)

(define-command (markdown-outline-up . args)
  "Move to the heading of the section that holds this one (a heading of a smaller level)."
  (let ((level (markdown--current-level)))
    (if (not level) (user-error "Not in a section"))
    (if (= level 1) (user-error "Already at top level"))
    (if (not (markdown--level-at-line)) (markdown--find-heading -1 (lambda (l) #t)))
    (markdown--find-heading -1 (lambda (l) (< l level)))
    #f))

(define (markdown--same-level step)
  "Move to the next (STEP 1) or previous (-1) heading of the same level, within the same section."
  (let ((level (markdown--current-level)))
    (if (not level) (user-error "Not in a section"))
    (if (not (markdown--level-at-line)) (markdown--find-heading -1 (lambda (l) #t)))
    (let ((here (point)))
      (let ((found (markdown--find-heading step (lambda (l) (<= l level)))))
        (when (< found level)
          (goto-char here)
          (user-error "No more headings at this level"))
        (when (> found level) (goto-char here))))
    #f))

(define-command (markdown-forward-same-level . args)
  "Move to the next heading of the same level, within the same section."
  (markdown--same-level 1))

(define-command (markdown-backward-same-level . args)
  "Move to the previous heading of the same level, within the same section."
  (markdown--same-level -1))

(define-command (markdown-promote . args)
  "Make the heading on this line one level higher (one `#' fewer)."
  (let ((level (markdown--level-at-line)))
    (if (not level) (user-error "Not on a heading"))
    (if (= level 1) (user-error "Already at the top level"))
    (save-excursion (beginning-of-line) (delete-char 1))
    #f))

(define-command (markdown-demote . args)
  "Make the heading on this line one level lower (one `#' more)."
  (let ((level (markdown--level-at-line)))
    (if (not level) (user-error "Not on a heading"))
    (if (= level 6) (user-error "Already at the lowest level"))
    (save-excursion (beginning-of-line) (insert "#"))
    #f))

(define (markdown-fontify)
  "Highlight the buffer."
  (unhighlight-regexp #t)
  (for-each (lambda (k) (highlight-regexp (car k) (cadr k) (caddr k) #t)) markdown-font-lock-keywords))

(define-command (markdown-mode)
  "Major mode for editing Markdown: headings, emphasis, code, links, lists and quotes are highlighted.
C-c C-n and C-c C-p move between headings, C-c C-f and C-c C-b between headings of the same level,
C-c C-u up to the parent heading, and C-c C-- and C-c C-= change the level of a heading."
  (set-syntax-dialect 'fundamental)
  (set-major-mode-name 'markdown-mode)
  (for-each (lambda (key) (local-set-key (car key) (cadr key))) markdown-mode-keys)
  (markdown-fontify)
  (run-hooks 'markdown-mode-hook)
  #f)

(define (markdown--find-file-hook)
  "Turn on Markdown mode if the visited file is a Markdown file.  This is called from `find-file-hook'."
  (let ((file (buffer-file-name)))
    (when (and file (regexp-match "\\.(?:md|markdown|mdown)$" file))
      (markdown-mode))))

(add-hook 'find-file-hook 'markdown--find-file-hook)
