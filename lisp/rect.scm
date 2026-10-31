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
