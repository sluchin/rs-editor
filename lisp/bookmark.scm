;;; bookmark.scm --- set bookmarks and jump to them -*- scheme -*-

;; Emacs の bookmark.el に当たるもの (`bookmark-set`, `bookmark-jump`).
;; ブックマークは, 名前と, ファイルと, 位置の前後の文字を覚える. 位置は, Emacs と同じく 1 から数える.
;; 保存は, Lisp の形 (ドット対が無いので `(name (filename F) ...)` の形) で, 付けるたびに書く.

(define bookmark-default-file "~/.rseditor.d/bookmarks"
  "File in which to save bookmarks by default.")

(define bookmark-search-size 16
  "Length of the context strings recorded around a bookmark's position.")

(define bookmark-alist '()
  "Association list of bookmarks.
Each element is (NAME (filename FILE) (front-context-string STRING)
(rear-context-string STRING) (position POINT)).  The newest bookmark comes first.")

(define bookmark-alist-loaded #f
  "Non-nil once the bookmarks in `bookmark-default-file' have been read.")

(define bookmark-current-bookmark #f
  "Name of the bookmark set or jumped to last, or #f.")

(define (bookmark--get record key)
  "Return the value of KEY in the bookmark RECORD, or #f."
  (let ((entry (assoc key (cdr record))))
    (and entry (cadr entry))))

(define (bookmark-maybe-load-default-file)
  "Read `bookmark-default-file' into `bookmark-alist' the first time."
  (unless bookmark-alist-loaded
    (set! bookmark-alist-loaded #t)
    (let ((file (expand-file-name bookmark-default-file)))
      (when (file-exists? file)
        (set! bookmark-alist (read (read-file-string file)))))))

(define (bookmark-save)
  "Write the bookmarks to `bookmark-default-file'."
  (let ((file (expand-file-name bookmark-default-file)))
    (make-directory (file-name-directory file))
    (write-file-string file (format "%S\n" bookmark-alist))))

(define (bookmark--store record)
  "Add RECORD to `bookmark-alist', replacing the bookmark of the same name in place."
  (let ((name (car record)))
    (set! bookmark-alist
          (if (assoc name bookmark-alist)
              (map (lambda (old) (if (equal? (car old) name) record old)) bookmark-alist)
              (cons record bookmark-alist)))
    (set! bookmark-current-bookmark name)
    (bookmark-save)))

(define (bookmark--make-record name)
  "Return the bookmark NAME for the point of the current buffer."
  (let ((pos (point)))
    (list name
          (list 'filename (buffer-file-name))
          (list 'front-context-string
                (buffer-substring pos (min (point-max) (+ pos bookmark-search-size))))
          (list 'rear-context-string
                (buffer-substring (max (point-min) (- pos bookmark-search-size)) pos))
          (list 'position pos))))

(define (bookmark--default-name)
  "Return the name offered by default: the file name of the buffer, or the buffer name."
  (let ((file (buffer-file-name)))
    (if file
        (let ((m (regexp-match "([^/]+)$" file))) (if m (cadr m) file))
        (buffer-name))))

(define-command (bookmark-set . args)
  "Set a bookmark named NAME, at point of the current buffer, and save the bookmarks.
Interactively, read NAME in the minibuffer; an empty input means the default,
the last bookmark or the file name.  A bookmark of the same name is replaced.
The bookmark records the file, the position and the text around it."
  (unless (buffer-file-name)
    (error "Buffer not visiting a file"))
  (bookmark-maybe-load-default-file)
  (let* ((default (or bookmark-current-bookmark (bookmark--default-name)))
         (name (if (null? args)
                   (read-string (format "Set bookmark (default %s): " default))
                   (car args)))
         (name (if (equal? name "") default name)))
    (bookmark--store (bookmark--make-record name))
    #f))

(define (bookmark--search-back text pattern from)
  "Return the last index of PATTERN in TEXT that starts at FROM or before, or #f."
  (let loop ((i from))
    (cond ((< i 0) #f)
          ((equal? (substring text i (+ i (string-length pattern))) pattern) i)
          (else (loop (- i 1))))))

(define (bookmark--position record)
  "Return the point of the bookmark RECORD in the current buffer.
Search the text after the recorded position for the front context, then the text before for the rear context,
so that the position follows the edits of the file."
  (let* ((text (buffer-string))
         (front (or (bookmark--get record 'front-context-string) ""))
         (rear (or (bookmark--get record 'rear-context-string) ""))
         (pos (- (min (bookmark--get record 'position) (point-max)) 1))
         (pos (let ((found (and (not (equal? front "")) (string-search front text pos))))
                (or found pos)))
         (rear-start (- pos (string-length rear)))
         (found (and (not (equal? rear "")) (>= rear-start 0)
                     (bookmark--search-back text rear rear-start))))
    (+ 1 (if found (+ found (string-length rear)) pos))))

(define-command (bookmark-jump . args)
  "Jump to the bookmark BOOKMARK, visiting its file.
Interactively, read the name in the minibuffer with completion; the default is the last bookmark.
If the text of the file has changed, find the position again from the text around it."
  (bookmark-maybe-load-default-file)
  (when (null? bookmark-alist)
    (error "No bookmarks"))
  (let* ((default (or (and bookmark-current-bookmark
                           (assoc bookmark-current-bookmark bookmark-alist)
                           bookmark-current-bookmark)
                      (car (car bookmark-alist))))
         (name (if (null? args)
                   (completing-read (format "Jump to bookmark (default %s): " default)
                                    (map car bookmark-alist) #f #t #f #f default)
                   (car args)))
         (name (if (equal? name "") default name))
         (record (assoc name bookmark-alist)))
    (unless record
      (error "Invalid bookmark %s" name))
    (let ((file (bookmark--get record 'filename)))
      (unless file
        (error "Bookmark %s does not contain a file" name))
      (unless (file-exists? file)
        (error "Bookmark %s: file not found: %s" name file))
      (switch-to-buffer (find-file-noselect file))
      (goto-char (bookmark--position record))
      (set! bookmark-current-bookmark name)
      #f)))

(provide 'bookmark)

;;; bookmark.scm ends here
