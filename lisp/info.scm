;;; info.scm --- read Info manuals -*- scheme -*-

;; Emacs の info.el に当たるものの, 簡単な版. マニュアル (`.info` か `.info.gz`) の全ての節を, 1 つのバッファ `*info*` に出す.
;; 節をたどる命令 (`n`, `p`, `u`, `m`, `f` など) と, 索引の検索 (`i`) は無い. 探すときは, isearch を使う.

(define Info-directory-list '("/usr/share/info" "/usr/local/share/info")
  "List of directories to search for Info files.")

(define (info--file manual)
  "Return the file of MANUAL in `Info-directory-list', or #f."
  (let loop ((dirs Info-directory-list))
    (if (null? dirs)
        #f
        (let ((found (let try ((suffixes '(".info.gz" ".info" ".gz" "")))
                       (cond ((null? suffixes) #f)
                             ((file-exists? (string-append (car dirs) "/" manual (car suffixes)))
                              (string-append (car dirs) "/" manual (car suffixes)))
                             (else (try (cdr suffixes)))))))
          (or found (loop (cdr dirs)))))))

(define (info--read file)
  "Return the text of the Info FILE (decompressed, if it ends in .gz)."
  (if (regexp-match "\\.gz$" file)
      (let ((result (call-shell-command (string-append "gzip -dc " (shell-quote-argument file)) #f #f)))
        (if (not (= (car result) 0))
            (error (format "Cannot read %s" file)))
        (cadr result))
      (read-file-string file)))

(define (info--show manual other-window)
  "Show MANUAL in the *info* buffer and return the buffer name."
  (let ((file (info--file manual)))
    (if (not file)
        (user-error (format "Info file %s not found" manual)))
    (let ((text (info--read file)))
      (with-current-buffer (get-buffer-create "*info*")
        (set-buffer-read-only #f)
        (erase-buffer)
        (insert (string-join (split-string text (char-to-string 31)) ""))
        (goto-char (point-min))
        (set-buffer-modified-p #f)
        (set-buffer-read-only #t))
      (if other-window
          (window--pop-to-buffer-other-window "*info*")
          (pop-to-buffer-same-window "*info*"))
      "*info*")))

(define (info--manual-argument args prompt)
  "Return the manual name in ARGS, or read one with PROMPT."
  (if (pair? args) (car args) (read-string prompt)))

(define-command (info . args)
  "Enter Info, the documentation browser.
Show the Info directory, the list of the manuals.  With a file name argument,
show that manual."
  (info--show (if (pair? args) (car args) "dir") #f)
  #f)

(define-command (info-other-window . args)
  "Like `info' but show the manual in another window."
  (info--show (if (pair? args) (car args) "dir") #t)
  #f)

(define-command (info-display-manual . args)
  "Display an Info buffer showing MANUAL, a manual name such as \"elisp\"."
  (info--show (info--manual-argument args "Manual name: ") #f)
  #f)

(define-command (info-emacs-manual . args)
  "Display the Emacs manual in Info mode."
  (info--show "emacs" #f)
  #f)

(define (info--goto-index-entry name)
  "In the Emacs manual, go to the node of the index entry NAME."
  (info--show "emacs" #f)
  (goto-char (point-min))
  (if (not (re-search-forward (string-append "^\\* " (regexp-quote name) ":[ \t]+\\([^.\n]+\\)\\.") #f #t))
      (user-error (format "Couldn't find documentation for %s" name)))
  (let ((node (match-string 1)))
    (goto-char (point-min))
    (if (re-search-forward (string-append "Node: " (regexp-quote node) ",") #f #t)
        (goto-char (line-beginning-position))))
  #f)

(define-command (Info-goto-emacs-command-node . args)
  "Go to the Info node in the Emacs manual for command COMMAND."
  (info--goto-index-entry
   (symbol->string (if (pair? args)
                       (car args)
                       (help--read-symbol "Find documentation for command: " (help--command-names))))))

(define-command (Info-goto-emacs-key-command-node . args)
  "Go to the node in the Emacs manual which describes the command bound to KEY.
KEY is a string such as \"C-x C-f\"."
  (let* ((keys (if (pair? args) (car args) (help--read-key-sequence "Find documentation for key: ")))
         (command (help--key-command keys)))
    (if (not command)
        (user-error (format "%s is undefined" keys)))
    (info--goto-index-entry command)))

(provide 'info)

;;; info.scm ends here
