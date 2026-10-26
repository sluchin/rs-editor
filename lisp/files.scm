;;; files.scm --- file input and output commands -*- scheme -*-

;; Emacs の files.el に当たるもの. 起動のときに読む (src-tauri/src/scheme/loadup.rs).
;; コマンドは, 引数を省くと (キーと M-x から呼ぶと), ミニバッファで聞く (Emacs の interactive に当たる).

(define-command (find-file . args)
  "Edit file FILENAME.
Switch to a buffer visiting file FILENAME, creating one if none already exists.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Find file: ") (car args))))
    (switch-to-buffer (find-file-noselect filename))))

(define-command (find-alternate-file . args)
  "Find file FILENAME, select its buffer, and kill the previous buffer.
If the current buffer visits a file and is modified, ask before killing it.
Interactively, read FILENAME in the minibuffer; the initial input is the current file."
  (let ((filename (if (null? args)
                      (read-file-name "Find alternate file: " (or (buffer-file-name) ""))
                      (car args)))
        (old (buffer-name)))
    (if (and (buffer-file-name)
             (buffer-modified-p)
             (not (yes-or-no-p (format "Kill and replace buffer ‘%s’ without saving it? " old))))
        (message "Aborted")
        (begin
          (set-buffer-modified-p #f)
          (kill-buffer old)
          (find-file filename)))))

(define-command (insert-file . args)
  "Insert the contents of file FILENAME at point.
Point stays before the inserted text.
Interactively, read FILENAME in the minibuffer with file name completion."
  (let ((filename (if (null? args) (read-file-name "Insert file: ") (car args)))
        (pos (point)))
    (insert (read-file-string filename))
    (goto-char pos)))

(define (files--write-file filename)
  "Write the current buffer into FILENAME, and make the buffer visit it.
Ask before overwriting an existing file other than the visited one."
  (if (and (not (equal? (expand-file-name filename) (buffer-file-name)))
           (file-exists? filename)
           (not (y-or-n-p (format "File ‘%s’ exists; overwrite? " filename))))
      (message "Canceled")
      (files--write-buffer filename)))

(define-command (write-file . args)
  "Write the current buffer into file FILENAME, and make the buffer visit it.
The buffer is renamed after the file. Ask before overwriting another existing file.
Interactively, read FILENAME in the minibuffer with file name completion."
  (files--write-file (if (null? args) (read-file-name "Write file: ") (car args))))

(define-command (save-buffer)
  "Save the current buffer in its file, if it is modified.
If the buffer visits no file, read a file name in the minibuffer and write it there.
The first save after visiting a file keeps the old file as a backup (FILE~)."
  (cond ((not (buffer-file-name)) (files--write-file (read-file-name "Save file: ")))
        ((buffer-modified-p) (files--write-buffer (buffer-file-name)))
        (else (message "(No changes need to be saved)"))))

(define-command (save-some-buffers)
  "Save some modified file-visiting buffers, asking about each one.
Answer y or SPC to save it, n or DEL to skip it, ! to save it and all the rest,
. to save it and stop, and q or RET to stop."
  (let ((names (filter (lambda (name) (and (buffer-file-name name) (buffer-modified-p name)))
                       (buffer-list))))
    (if (null? names)
        (message "(No files need saving)")
        (let loop ((names names) (saved 0))
          (if (null? names)
              (when (= saved 0) (message ""))
              (let* ((name (car names))
                     (file (buffer-file-name name))
                     (key (read-key (format "Save file %s? (y, n, !, ., q) " file))))
                (cond ((member key '("y" " "))
                       (files--write-buffer file name)
                       (loop (cdr names) (+ saved 1)))
                      ((member key '("n" "Backspace" "Delete")) (loop (cdr names) saved))
                      ((equal? key "!")
                       (for-each (lambda (name) (files--write-buffer (buffer-file-name name) name))
                                 names))
                      ((equal? key ".") (files--write-buffer file name))
                      ((member key '("q" "Enter")) (loop '() saved))
                      (else (loop names saved)))))))))

(provide 'files)

;;; files.scm ends here
