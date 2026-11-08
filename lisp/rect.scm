;;; rect.scm --- rectangle commands -*- scheme -*-

;; Emacs の rect.el に当たるもの. 矩形は, マークとカーソルを角とする桁の範囲 (Emacs の `apply-on-rectangle`).
;; 桁は, タブと幅の広い文字を数えた桁. 桁がタブの途中なら, タブを空白にして分ける (`move-to-column` の `coerce`).

(define killed-rectangle '()
  "Rectangle for `yank-rectangle' to insert: a list of strings, one for each line.")

(define string-rectangle-history '()
  "History list for `string-rectangle'.")

(define (rect--without-tabs thunk)
  "Call THUNK with indent-tabs-mode turned off."
  (let ((saved indent-tabs-mode))
    (set! indent-tabs-mode #f)
    (unwind-protect (thunk) (set! indent-tabs-mode saved))))

(define (apply-on-rectangle function start end . args)
  "Call FUNCTION for each line of the rectangle with corners at START and END.
FUNCTION is called with the start and end columns of the rectangle, plus ARGS,
and point is at the beginning of the line.  Return point after the last call."
  (save-excursion
    (let* ((start-column (begin (goto-char start) (current-column)))
           (end-column (begin (goto-char end) (current-column)))
           (left (min start-column end-column))
           (right (max start-column end-column))
           (start-point (begin (goto-char start) (line-beginning-position)))
           (limit (begin (goto-char end) (goto-char (line-end-position)) (point-marker)))
           (final-point start-point))
      (goto-char start-point)
      (let loop ()
        (apply function left right args)
        (set! final-point (point))
        (when (and (zero? (forward-line 1)) (bolp) (<= (point) (marker-position limit)))
          (loop)))
      (marker-release limit)
      final-point)))

(define (rect--delete-line left right fill)
  "Delete the columns from LEFT to RIGHT of the current line."
  (when (>= (move-to-column left (if fill #t 'coerce)) left)
    (let ((from (point)))
      (move-to-column right 'coerce)
      (delete-region from (point)))))

(define (rect--delete-extract-line left right fill)
  "Delete the columns from LEFT to RIGHT of the current line, and return the deleted text."
  (if (< (move-to-column left (if fill #t 'coerce)) left)
      (simple--spaces (- right left))
      (let ((from (point)))
        (move-to-column right #t)
        (let ((text (buffer-substring from (point))))
          (delete-region from (point))
          text))))

(define (delete-extract-rectangle start end . fill)
  "Delete the contents of the rectangle with corners at START and END.
Return it as a list of strings, one for each line of the rectangle.
With an optional FILL argument, also fill lines where nothing has to be deleted."
  (let ((lines '()))
    (apply-on-rectangle
     (lambda (left right)
       (set! lines (cons (rect--delete-extract-line left right (and (pair? fill) (car fill))) lines)))
     start end)
    (reverse lines)))

(define (insert-rectangle rectangle)
  "Insert text of RECTANGLE with upper left corner at point.
RECTANGLE's first line is inserted at point, its second line is inserted at a point
vertically under point, etc.  RECTANGLE should be a list of strings.
After this command, the mark is at the upper left corner and point is at the lower right corner."
  (let ((column (current-column)))
    (push-mark)
    (let loop ((lines rectangle) (first #t))
      (when (pair? lines)
        (unless first
          (forward-line 1)
          (unless (bolp) (insert "\n"))
          (move-to-column column #t))
        (insert-for-yank (car lines))
        (loop (cdr lines) #f)))))

(define-command (kill-rectangle . args)
  "Delete the region-rectangle and save it as the last killed one.
Called from a program with the corners START and END, delete that rectangle.
With a prefix argument (or a FILL argument), also fill lines where nothing has to be deleted."
  (let ((start (if (null? args) (region-beginning) (car args)))
        (end (if (null? args) (region-end) (cadr args)))
        (fill (if (and (pair? args) (pair? (cdr args)) (pair? (cddr args)))
                  (caddr args)
                  (and (null? args) (current-prefix-arg)))))
    (rect--without-tabs
     (lambda () (set! killed-rectangle (delete-extract-rectangle start end fill))))
    #f))

(define-command (yank-rectangle . args)
  "Yank the last killed rectangle with upper left corner at point."
  (insert-rectangle killed-rectangle)
  #f)

(define (rect--string-line left right string delete)
  "Replace the columns from LEFT to RIGHT of the current line with STRING."
  (move-to-column left #t)
  (if delete (rect--delete-line left right #f))
  (insert string))

(define-command (string-rectangle . args)
  "Replace rectangle contents with STRING on each line.
The length of STRING need not be the same as the rectangle width.
Called from a program, takes three arguments: START, END and STRING."
  (let* ((start (if (null? args) (region-beginning) (car args)))
         (end (if (null? args) (region-end) (cadr args)))
         (string (if (and (pair? args) (pair? (cdr args)) (pair? (cddr args)))
                     (caddr args)
                     (read-string "String rectangle: "))))
    (goto-char (apply-on-rectangle rect--string-line start end string #t))
    #f))

(define (rect--args args)
  "Return (START END FILL) for a rectangle command: from ARGS (START END [FILL]), or the region and the prefix argument."
  (if (null? args)
      (list (region-beginning) (region-end) (current-prefix-arg))
      (list (car args) (cadr args) (and (pair? (cddr args)) (caddr args)))))

(define (rect--extract-line left right lines)
  "Return the text of the columns from LEFT to RIGHT of the current line, with tabs and missing columns as spaces."
  (move-to-column left)
  (let* ((start (point))
         (begin-extra (- (current-column) left)))
    (move-to-column right)
    (let* ((end (point))
           (end-extra (- right (current-column)))
           (line (buffer-substring start end)))
      (when (< begin-extra 0)
        (set! end-extra (+ end-extra begin-extra))
        (set! begin-extra 0))
      (when (< end-extra 0) (set! end-extra 0))
      (goto-char start)
      (let loop ()
        (when (search-forward "\t" end #t)
          (let ((width (- (current-column) (save-excursion (forward-char -1) (current-column)))))
            (set! line (string-append (substring line 0 (+ (string-length line) (- (point) end 1)))
                                      (simple--spaces width)
                                      (substring line (+ (string-length line) (- (point) end))))))
          (loop)))
      (if (or (> begin-extra 0) (> end-extra 0))
          (string-append (simple--spaces begin-extra) line (simple--spaces end-extra))
          line))))

(define (extract-rectangle start end)
  "Return the contents of the rectangle with corners at START and END.
Return it as a list of strings, one for each line of the rectangle."
  (let ((lines '()))
    (apply-on-rectangle (lambda (left right) (set! lines (cons (rect--extract-line left right lines) lines)))
                        start end)
    (reverse lines)))

;; rectangle-mark-mode は, 領域を矩形として扱う (印を付けて, `C-w` と `M-w` が矩形を切る・写す). 矩形の強調は, まだ出さない.
(define-command (rectangle-mark-mode . args)
  "Toggle the region as a rectangle.
With a positive argument, turn it on; with zero or a negative one, turn it off.
When on, the region is a rectangle between point and the mark: \\[kill-region] kills it
like `kill-rectangle', and \\[kill-ring-save] saves it like `copy-rectangle-as-kill'.
Turning it on without an active region sets the mark at point."
  (let* ((arg (simple--raw-arg args))
         (on (if arg (> (prefix-numeric-value arg) 0) (not (simple--rectangle-region-p)))))
    (cond (on
           (unless (region-active-p)
             (push-mark (point))
             (activate-mark))
           (set! rectangle-mark-mode--on #t)
           (message "Rectangle mark mode enabled"))
          (else
           (set! rectangle-mark-mode--on #f)
           (deactivate-mark)
           (message "Rectangle mark mode disabled")))
    #f))

(define-command (copy-rectangle-as-kill . args)
  "Copy the region-rectangle and save it as the last killed one."
  (let ((bounds (rect--args args)))
    (set! killed-rectangle (extract-rectangle (car bounds) (cadr bounds)))
    (set-deactivate-mark #t)
    #f))

(define-command (delete-rectangle . args)
  "Delete (don't save) text in the region-rectangle.
The same range of columns is deleted in each line starting with the
line where the region begins and ending with the line where the region ends.
With a prefix (or a FILL) argument, also fill lines where nothing has to be deleted."
  (let ((bounds (rect--args args)))
    (rect--without-tabs
     (lambda ()
       (apply-on-rectangle (lambda (left right) (rect--delete-line left right (caddr bounds)))
                           (car bounds) (cadr bounds))))
    #f))

(define (rect--open-line left right fill)
  "Open a blank of the width of the rectangle at column LEFT of the current line."
  (when (= (move-to-column left (if fill #t 'coerce)) left)
    (unless (and (not fill) (= (point) (line-end-position)))
      (simple--indent-to right))))

(define-command (open-rectangle . args)
  "Blank out the region-rectangle, shifting text right.
The text previously in the region is not overwritten by the blanks,
but instead winds up to the right of the rectangle.
With a prefix (or a FILL) argument, fill with blanks even if there is
no text on the right side of the rectangle."
  (let ((bounds (rect--args args)))
    (apply-on-rectangle rect--open-line (car bounds) (cadr bounds) (caddr bounds))
    (goto-char (car bounds))
    #f))

(define (rect--clear-line left right fill)
  "Blank out the columns from LEFT to RIGHT of the current line."
  (let ((line-end (line-end-position)))
    (when (= (move-to-column left (if fill #t 'coerce)) left)
      (if (and (not fill)
               (<= (save-excursion (goto-char line-end) (current-column)) right))
          (delete-region (point) line-end)
          (let ((from (point)))
            (move-to-column right #t)
            (let ((column (current-column)))
              (delete-region from (point))
              (simple--indent-to column)))))))

(define-command (clear-rectangle . args)
  "Blank out the region-rectangle.
The text previously in the region is overwritten with blanks.
With a prefix (or a FILL) argument, also fill with blanks the parts of the
rectangle which were empty."
  (let ((bounds (rect--args args)))
    (apply-on-rectangle rect--clear-line (car bounds) (cadr bounds) (caddr bounds))
    #f))

(define (rect--default-line-number-format start end start-at)
  "Return the default format string for numbering the lines from START-AT."
  (string-append "%"
                 (number->string (string-length (number->string (+ (simple--count-lines start end) start-at))))
                 "d "))

(define-command (rectangle-number-lines . args)
  "Insert numbers in front of the region-rectangle.
START-AT, if given, is the number from which to begin counting.  FORMAT, if given,
is a format string for the line count.  With a prefix argument, read START-AT and FORMAT."
  (let* ((start (if (>= (length args) 2) (car args) (region-beginning)))
         (end (if (>= (length args) 2) (cadr args) (region-end)))
         (start-at (cond ((>= (length args) 3) (caddr args))
                         ((and (null? args) (current-prefix-arg))
                          (string->number (read-string "Number to count from: " "1")))
                         (else 1)))
         (format-string (cond ((>= (length args) 4) (list-ref args 3))
                              ((and (null? args) (current-prefix-arg))
                               (read-string "Format string: " (rect--default-line-number-format start end start-at)))
                              (else #f)))
         (format-string (or format-string (rect--default-line-number-format start end start-at)))
         (counter start-at))
    (apply-on-rectangle (lambda (left right)
                          (move-to-column left #t)
                          (insert (format format-string counter))
                          (set! counter (+ counter 1)))
                        start end)
    #f))
