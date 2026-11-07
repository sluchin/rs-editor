;;; uniquify.scm --- unique buffer names dependent on file name -*- scheme -*-

;; Emacs の uniquify.el に当たるもの. 同じ名前のファイルを訪れるバッファの名前を, ディレクトリの名前で分ける
;; (`a.txt<src>` など). バッファを作るとき (`create-file-buffer`), 名前を変えるとき (`rename-buffer`),
;; 消すとき (`kill-buffer-hook`) に, 名前を付け直す. dired のバッファは扱わない.
;; バッファは名前で呼ぶので, 名前を変えても同じバッファを指す「箱」(`uniquify--box`) で覚える.

(define uniquify-buffer-name-style 'post-forward-angle-brackets
  "How to construct unique buffer names for files with the same base name.
The value can be one of: `forward', `reverse', `post-forward',
`post-forward-angle-brackets', or nil.")

(define uniquify-after-kill-buffer-p #t
  "If non-nil, rerationalize buffer names after a buffer has been killed.")

(define uniquify-ignore-buffers-re #f
  "Regular expression matching buffer names that should not be uniquified.")

(define uniquify-min-dir-content 0
  "Minimum number of directory name components included in buffer name.")

(define uniquify-separator #f
  "String separator for buffer name components.
When `uniquify-buffer-name-style' is `post-forward', separates
base file name from directory part in buffer names (default \"|\").
When `uniquify-buffer-name-style' is `reverse', separates all
file name components (default \"\\\").")

(define uniquify-strip-common-suffix #t
  "If non-nil, strip common directory suffixes of conflicting files.
E.g. if you open /a1/b/c/d and /a2/b/c/d, the buffer names will say
\"d|a1\" and \"d|a2\" instead of \"d|a1/b/c\" and \"d|a2/b/c\".")

(define uniquify-possibly-resolvable #f)

;;; Buffers and items

(define uniquify--boxes '())

(define (uniquify--box name)
  "Return the box of the buffer NAME: a procedure returning the buffer's current name."
  (let ((entry (assoc name uniquify--boxes)))
    (if entry
        (cadr entry)
        (let* ((current name)
               (box (lambda args (if (pair? args) (set! current (car args)) current))))
          (set! uniquify--boxes (cons (list name box) uniquify--boxes))
          box))))

(define (uniquify--renamed old new)
  "Note that the buffer OLD is now called NEW."
  (let ((entry (assoc old uniquify--boxes)))
    (when entry
      ((cadr entry) new)
      (set! uniquify--boxes (cons (list new (cadr entry))
                                  (filter (lambda (e) (not (equal? (car e) old))) uniquify--boxes))))))

(define (uniquify--forget name)
  "Forget the buffer NAME (it was killed)."
  (set! uniquify--boxes (filter (lambda (e) (not (equal? (car e) name))) uniquify--boxes)))

(define uniquify--managed-table '())

(define (uniquify--managed box)
  "Return the conflicting items of the buffer BOX (Emacs's buffer-local `uniquify-managed')."
  (let loop ((table uniquify--managed-table))
    (cond ((null? table) '())
          ((eq? (car (car table)) box) (cadr (car table)))
          (else (loop (cdr table))))))

(define (uniquify--set-managed box items)
  (set! uniquify--managed-table
        (cons (list box items) (filter (lambda (e) (not (eq? (car e) box))) uniquify--managed-table))))

(define (uniquify--make-item base dirname box proposed)
  "Return an item: a procedure taking a field name (and a new value for dirname and proposed)."
  (lambda (field . value)
    (case field
      ((base) base)
      ((box) box)
      ((dirname) (if (pair? value) (set! dirname (car value)) dirname))
      (else (if (pair? value) (set! proposed (car value)) proposed)))))

(define (uniquify--memq item items)
  (and (pair? items) (or (eq? item (car items)) (uniquify--memq item (cdr items)))))

(define (uniquify--live? item)
  (and (get-buffer ((item 'box))) #t))

(define (uniquify--directory-file-name dir)
  "Return DIR without its trailing slash (but \"/\" stays)."
  (if (and (> (string-length dir) 1) (string-suffix? "/" dir))
      (uniquify--directory-file-name (substring dir 0 (- (string-length dir) 1)))
      dir))

(define (uniquify-buffer-file-name buffer)
  "Return the directory of the file visited by BUFFER, or #f."
  (let ((filename (buffer-file-name buffer)))
    (and filename
         (uniquify--directory-file-name
          (file-name-directory (expand-file-name (uniquify--directory-file-name filename)))))))

;;; Rationalizing

(define (uniquify-rationalize-file-buffer-names base dirname newbuf)
  "Make file buffer names unique by adding segments from file name.
Arguments BASE, DIRNAME, and NEWBUF specify the new buffer that causes
this rationalization."
  (let ((newbox (uniquify--box newbuf)))
    (uniquify--set-managed newbox '())
    (when dirname
      (let ((fix-list (list (uniquify--make-item base (uniquify--directory-file-name (expand-file-name dirname))
                                                 newbox #f))))
        (for-each
         (lambda (buffer)
           (let* ((box (uniquify--box buffer))
                  (items (uniquify--managed box)))
             (when (and (not (and uniquify-ignore-buffers-re (string-match uniquify-ignore-buffers-re buffer)))
                        (pair? items)
                        (equal? base ((car items) 'base))
                        (not (uniquify--memq (car items) fix-list)))
               (when (null? (cdr items))
                 ((car items) 'dirname (uniquify-buffer-file-name (((car items) 'box))))
                 (unless ((car items) 'dirname)
                   (uniquify--set-managed (((car items) 'box)) '())
                   (set! items '())))
               (set! fix-list
                     (append fix-list
                             (filter (lambda (item) (not (uniquify--memq item fix-list)))
                                     (reverse (filter uniquify--live? items))))))))
         (buffer-list))
        (uniquify-rationalize fix-list)))))

(define (uniquify--file-name-nondirectory dir)
  (if (equal? dir "/") "" (file-name-nondirectory dir)))

(define (uniquify-rationalize fix-list)
  (for-each (lambda (item)
              (item 'proposed (uniquify-get-proposed-name (item 'base) (item 'dirname) #f))
              (uniquify--set-managed (item 'box) fix-list))
            fix-list)
  (when (and (pair? (cdr fix-list)) uniquify-strip-common-suffix)
    (let loop ()
      (let ((base (uniquify--file-name-nondirectory ((car fix-list) 'dirname))))
        (when (and (> (string-length base) 0)
                   (null? (filter (lambda (item)
                                    (not (equal? base (uniquify--file-name-nondirectory (item 'dirname)))))
                                  fix-list)))
          (set! fix-list
                (reverse
                 (map (lambda (item)
                        (let ((f (file-name-directory (item 'dirname))))
                          (uniquify--make-item (item 'base) (and f (uniquify--directory-file-name f))
                                               (item 'box) (item 'proposed))))
                      fix-list)))
          (loop)))))
  (uniquify-rationalize-a-list fix-list #f))

(define (uniquify-rationalize-a-list fix-list depth)
  (let ((depth (or depth uniquify-min-dir-content))
        (conflicting-sublist '())
        (old-proposed ""))
    (for-each (lambda (item)
                (let ((proposed (item 'proposed)))
                  (unless (equal? proposed old-proposed)
                    (uniquify-rationalize-conflicting-sublist conflicting-sublist old-proposed depth)
                    (set! conflicting-sublist '()))
                  (set! conflicting-sublist (cons item conflicting-sublist))
                  (set! old-proposed proposed)))
              (sort fix-list (lambda (item1 item2) (string<? (item2 'proposed) (item1 'proposed)))))
    (uniquify-rationalize-conflicting-sublist conflicting-sublist old-proposed depth)))

(define (uniquify-get-proposed-name base dirname depth)
  "Return the name proposed for the buffer visiting BASE in DIRNAME, with DEPTH directories."
  (let ((extra-string '())
        (n (or depth uniquify-min-dir-content)))
    (let loop ()
      (when (and (> n 0) dirname)
        (let ((file (uniquify--file-name-nondirectory dirname)))
          (set! dirname (let ((d (file-name-directory dirname)))
                          (and d (not (equal? dirname "/")) (uniquify--directory-file-name d))))
          (set! n (- n 1))
          (set! extra-string
                (cons (if (equal? file "") (begin (set! dirname #f) "") file) extra-string))
          (loop))))
    (when (= n 0)
      (if (and dirname (pair? extra-string) (equal? dirname "/"))
          (set! extra-string (cons "" extra-string)))
      (set! uniquify-possibly-resolvable #t))
    (cond ((null? extra-string) base)
          ((equal? base "") (string-join extra-string "/"))
          ((eq? uniquify-buffer-name-style 'reverse)
           (string-join (cons base (reverse extra-string)) (or uniquify-separator "\\")))
          ((eq? uniquify-buffer-name-style 'forward)
           (string-join (append extra-string (list base)) "/"))
          ((eq? uniquify-buffer-name-style 'post-forward)
           (string-append base (or uniquify-separator "|") (string-join extra-string "/")))
          ((eq? uniquify-buffer-name-style 'post-forward-angle-brackets)
           (string-append base "<" (string-join extra-string "/") ">"))
          (else (error (format "Bad value for uniquify-buffer-name-style: %s" uniquify-buffer-name-style))))))

(define (uniquify-rationalize-conflicting-sublist conf-list old-name depth)
  (when (pair? conf-list)
    (if (or (pair? (cdr conf-list))
            (let ((buf (get-buffer old-name)))
              (and buf (not (equal? old-name (((car conf-list) 'box)))))))
        (when uniquify-possibly-resolvable
          (set! uniquify-possibly-resolvable #f)
          (for-each (lambda (item)
                      (item 'proposed (uniquify-get-proposed-name (item 'base) (item 'dirname) (+ depth 1))))
                    conf-list)
          (uniquify-rationalize-a-list conf-list (+ depth 1)))
        (unless (equal? old-name "")
          (uniquify-rename-buffer (car conf-list) old-name)))))

(define (uniquify-rename-buffer item newname)
  "Rename the buffer of ITEM to NEWNAME (a unique name), without rationalizing again."
  (let ((name ((item 'box))))
    (unless (equal? newname name)
      (with-current-buffer name
        (uniquify--renamed name (simple--rename-buffer newname #t))))))

(define (uniquify-maybe-rerationalize-w/o-cb)
  "Re-rationalize the buffers conflicting with the current one, without it."
  (let ((items (uniquify--managed (uniquify--box (buffer-name)))))
    (and (pair? items)
         (pair? (cdr items))
         uniquify-buffer-name-style
         (let ((new-fix-list (filter (lambda (item)
                                       (and (not (equal? ((item 'box)) (buffer-name))) (uniquify--live? item)))
                                     (reverse items))))
           (if (pair? new-fix-list)
               (uniquify-rationalize new-fix-list))))))

;;; Hooks

(define (uniquify-kill-buffer-function)
  "Re-rationalize buffer names, ignoring current buffer.
For use on `kill-buffer-hook'."
  (and uniquify-after-kill-buffer-p
       (uniquify-maybe-rerationalize-w/o-cb))
  (uniquify--set-managed (uniquify--box (buffer-name)) '())
  (uniquify--forget (buffer-name)))

(add-hook 'kill-buffer-hook uniquify-kill-buffer-function)

(define (uniquify--rename-buffer-advice old newname unique)
  "Rationalize after the current buffer was renamed from OLD to NEWNAME (UNIQUE as `rename-buffer')."
  (uniquify--renamed old (buffer-name))
  (uniquify-maybe-rerationalize-w/o-cb)
  (if (not unique)
      (uniquify--set-managed (uniquify--box (buffer-name)) '())
      (when uniquify-buffer-name-style
        (uniquify-rationalize-file-buffer-names newname (uniquify-buffer-file-name (buffer-name)) (buffer-name)))))

(define (uniquify--create-file-buffer-advice buf filename basename)
  (when uniquify-buffer-name-style
    (uniquify-rationalize-file-buffer-names
     basename
     (file-name-directory (expand-file-name (uniquify--directory-file-name filename)))
     buf)))
