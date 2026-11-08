;;; bookmark.scm --- set bookmarks and jump to them -*- scheme -*-

;; Emacs の bookmark.el に当たるもの (`bookmark-set`, `bookmark-jump`).
;; ブックマークは, 名前と, ファイルと, 位置の前後の文字を覚える. 位置は, Emacs と同じく 1 から数える.
;; 保存は, Lisp の形 (ドット対が無いので `(name (filename F) ...)` の形) で, 付けるたびに書く.

(define bookmark-default-file "~/.rseditor.d/bookmarks"
  "File in which to save bookmarks by default.")

(define bookmark-search-size 16
  "Length of the context strings recorded around a bookmark's position.")

(define bookmark-save-flag 1
  "If non-nil, `bookmark-set' and the other changes save the bookmarks to `bookmark-default-file'.")

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

(define (bookmark--store record mode)
  "Add RECORD to `bookmark-alist'.
MODE is `overwrite' to replace the bookmark of the same name in place,
`push' to put RECORD in front of the others of the same name,
and anything else to signal an error if the name is taken."
  (let ((name (car record)))
    (when (and (assoc name bookmark-alist) (not (member mode '(overwrite push))))
      (error "A bookmark named \"%s\" already exists" name))
    (set! bookmark-alist
          (if (and (assoc name bookmark-alist) (eq? mode 'overwrite))
              (map (lambda (old) (if (equal? (car old) name) record old)) bookmark-alist)
              (cons record bookmark-alist)))
    (set! bookmark-current-bookmark name)
    (if bookmark-save-flag (bookmark-save))))

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

(define (bookmark--set args prompt mode)
  "Set a bookmark for `bookmark-set' and `bookmark-set-no-overwrite'.
ARGS is the command's arguments, PROMPT the minibuffer prompt, and MODE as in `bookmark--store'."
  (unless (buffer-file-name)
    (error "Buffer not visiting a file"))
  (bookmark-maybe-load-default-file)
  (let* ((default (or bookmark-current-bookmark (bookmark--default-name)))
         (name (if (null? args)
                   (read-string (format "%s (default %s): " prompt default))
                   (car args)))
         (name (if (equal? name "") default name)))
    (bookmark--store (bookmark--make-record name) mode)
    #f))

(define-command (bookmark-set . args)
  "Set a bookmark named NAME, at point of the current buffer, and save the bookmarks.
Interactively, read NAME in the minibuffer; an empty input means the default,
the last bookmark or the file name.  A bookmark of the same name is replaced.
The bookmark records the file, the position and the text around it."
  (bookmark--set args "Set bookmark" 'overwrite))

(define-command (bookmark-set-no-overwrite . args)
  "Set a bookmark named NAME at the current location, like `bookmark-set'.
If a bookmark named NAME already exists, signal an error, unless the second argument
PUSH-BOOKMARK is true (interactively, a prefix argument).  Then the new bookmark is put in front,
so it is the one in effect, and the others of the same name stay behind it."
  (bookmark--set (if (null? args) '() (list (car args)))
                 "Set bookmark"
                 (if (if (null? args) (current-prefix-arg) (and (pair? (cdr args)) (cadr args)))
                     'push
                     'no-overwrite)))

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

(define (bookmark--read-name prompt)
  "Read the name of an existing bookmark in the minibuffer, with PROMPT."
  (bookmark-maybe-load-default-file)
  (when (null? bookmark-alist)
    (error "No bookmarks"))
  (completing-read prompt (map car bookmark-alist) #f #t))

(define-command (bookmark-delete . args)
  "Delete the bookmark named BOOKMARK-NAME, and save the bookmarks.
Interactively, read the name in the minibuffer with completion.
Every bookmark of that name is deleted."
  (let ((name (if (null? args) (bookmark--read-name "Delete bookmark: ") (car args))))
    (bookmark-maybe-load-default-file)
    (unless (assoc name bookmark-alist)
      (error "Invalid bookmark %s" name))
    (set! bookmark-alist (filter (lambda (b) (not (equal? (car b) name))) bookmark-alist))
    (when (equal? bookmark-current-bookmark name)
      (set! bookmark-current-bookmark #f))
    (if bookmark-save-flag (bookmark-save))
    #f))

(define-command (bookmark-rename . args)
  "Change the name of the bookmark OLD to NEW, and save the bookmarks.
Interactively, read both names in the minibuffer.  An error is signaled if NEW is empty."
  (let* ((old (if (null? args) (bookmark--read-name "Old bookmark name: ") (car args)))
         (new (if (and (pair? args) (pair? (cdr args)))
                  (cadr args)
                  (read-string (format "New name (default %s): " old)))))
    (bookmark-maybe-load-default-file)
    (unless (assoc old bookmark-alist)
      (error "Invalid bookmark %s" old))
    (when (equal? new "")
      (error "A bookmark name cannot be empty"))
    (set! bookmark-alist
          (map (lambda (b) (if (equal? (car b) old) (cons new (cdr b)) b)) bookmark-alist))
    (when (equal? bookmark-current-bookmark old)
      (set! bookmark-current-bookmark new))
    (if bookmark-save-flag (bookmark-save))
    #f))

(define bookmark-sort-flag #t
  "Non-nil means that bookmarks are sorted by name in the bookmark list.")

(define bookmark-bmenu-buffer "*Bookmark List*"
  "Name of the buffer of the bookmark list.")

(define (bookmark--spaces n)
  (if (<= n 0) "" (string-append " " (bookmark--spaces (- n 1)))))

(define (bookmark--row record)
  "Return the line of the bookmark list for RECORD: two columns of marks, the annotation mark,
the name padded to 30 columns, the width of the empty type column, then the file."
  (let ((name (car record))
        (file (bookmark--get record 'filename))
        (note (bookmark--get record 'annotation)))
    (string-append "  "
                   (if (and note (not (equal? note ""))) "* " "  ")
                   name
                   (bookmark--spaces (- 30 (string-width name)))
                   (bookmark--spaces 10)
                   (or file "-- Unknown location --")
                   "\n")))

(define (bookmark--sorted)
  "Return `bookmark-alist', sorted by name ignoring case if `bookmark-sort-flag' is true."
  (if bookmark-sort-flag
      (sort bookmark-alist (lambda (a b) (string<? (string-downcase (car a)) (string-downcase (car b)))))
      bookmark-alist))

(define-command (bookmark-bmenu-list . args)
  "Display a list of existing bookmarks.
The list is displayed in a buffer named *Bookmark List*.
Each line has a * in front of the name if the bookmark has an annotation, then the name and the file.
The bookmarks are sorted by name, ignoring case, unless `bookmark-sort-flag' is false."
  (bookmark-maybe-load-default-file)
  (switch-to-buffer bookmark-bmenu-buffer)
  (delete-region (point-min) (point-max))
  (insert (apply string-append (map bookmark--row (bookmark--sorted))))
  (goto-char (point-min))
  (set-buffer-modified-p #f)
  #f)

(provide 'bookmark)

;;; bookmark.scm ends here
