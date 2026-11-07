;;; windmove.scm --- directional window-selection routines -*- scheme -*-

;; Emacs の windmove.el と, window.el の window-in-direction に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; 窓の位置は, 画面のマス (window-edges) で比べる. 基準の位置は, 窓の上端か左端 (Emacs は, ポイントの位置).

(define windmove-wrap-around #f
  "Whether movement off the edge of the frame wraps around.
If true, windmove-up from the top window selects the bottom window, and so on.")

(define (windmove--edges window)
  "Return the edges (left top right bottom) of WINDOW, or those of the minibuffer if WINDOW is minibuffer."
  (if (eq? window 'minibuffer)
      (let ((bottom (windmove--frame-bottom)))
        (list 0 bottom (windmove--frame-right) (+ bottom 1)))
      (window-edges window)))

(define (windmove--frame-bottom)
  (apply max (map (lambda (w) (list-ref (window-edges w) 3)) (window-list))))

(define (windmove--frame-right)
  (apply max (map (lambda (w) (list-ref (window-edges w) 2)) (window-list))))

(define (windmove--at-side-p window side)
  "Return #t if WINDOW is at SIDE (left, top, right or bottom) of the frame."
  (let ((edges (windmove--edges window)))
    (cond ((eq? side 'left) (= (car edges) 0))
          ((eq? side 'top) (= (cadr edges) 0))
          ((eq? side 'right) (= (caddr edges) (windmove--frame-right)))
          (else (= (list-ref edges 3) (windmove--frame-bottom))))))

(define (windmove--in-direction-2 window posn hor)
  "The distance of WINDOW from POSN, across the direction (rows if HOR, columns otherwise)."
  (let* ((edges (windmove--edges window))
         (start (if hor (cadr edges) (car edges)))
         (size (if hor (- (list-ref edges 3) (cadr edges)) (- (caddr edges) (car edges)))))
    (if (> start posn) (- start posn) (- posn start size))))

(define (window-in-direction direction . args)
  "Return the window in DIRECTION (above, below, left or right; up and down too) from WINDOW.
The optional arguments are WINDOW (default: the selected one), IGNORE, SIGN, WRAP and MINIBUF.
SIGN negative means to look from the bottom or right edge of WINDOW, otherwise from the top or left edge.
WRAP true means to wrap around the edges of the frame.  MINIBUF true means the result may be the
symbol minibuffer, for the minibuffer below the windows.  Return #f if there is no such window."
  (let* ((arg (lambda (i) (and (> (length args) i) (list-ref args i))))
         (window (or (arg 0) (selected-window)))
         (sign (arg 2))
         (wrap (arg 3))
         (minibuf (arg 4))
         (direction (cond ((eq? direction 'up) 'above)
                          ((eq? direction 'down) 'below)
                          ((member direction '(above below left right)) direction)
                          (else (error (format "Wrong direction %s" direction)))))
         (hor (member direction '(left right)))
         (edges (windmove--edges window))
         (first (if hor (car edges) (cadr edges)))
         (last (if hor (caddr edges) (list-ref edges 3)))
         (posn (if (and (number? sign) (< sign 0))
                   (- (if hor (list-ref edges 3) (caddr edges)) 1)
                   (if hor (cadr edges) (car edges))))
         (best-edge (cond ((eq? direction 'below) (+ (windmove--frame-bottom) 1))
                          ((eq? direction 'right) (windmove--frame-right))
                          (else -1)))
         (best-edge-2 best-edge)
         (best-diff-2 (if hor (windmove--frame-bottom) (windmove--frame-right)))
         (best #f)
         (best-2 #f))
    (for-each
     (lambda (w)
       (let* ((w-edges (windmove--edges w))
              (w-left (car w-edges))
              (w-top (cadr w-edges))
              (w-width (- (caddr w-edges) w-left))
              (w-height (- (list-ref w-edges 3) w-top)))
         (cond
          ((equal? w window))
          (hor
           (cond
            ((and (<= w-top posn) (< posn (+ w-top w-height)))
             (when (or (and (eq? direction 'left)
                            (or (and (<= w-left first) (> w-left best-edge))
                                (and wrap
                                     (windmove--at-side-p window 'left)
                                     (windmove--at-side-p w 'right))))
                       (and (eq? direction 'right)
                            (or (and (>= w-left last) (< w-left best-edge))
                                (and wrap
                                     (windmove--at-side-p window 'right)
                                     (windmove--at-side-p w 'left)))))
               (set! best-edge w-left)
               (set! best w)))
            ((or (and (eq? direction 'left) (<= (+ w-left w-width) first))
                 (and (eq? direction 'right) (<= last w-left)))
             (let ((diff (windmove--in-direction-2 w posn hor)))
               (when (or (< diff best-diff-2)
                         (and (= diff best-diff-2)
                              (if (eq? direction 'left) (> w-left best-edge-2) (< w-left best-edge-2))))
                 (set! best-edge-2 w-left)
                 (set! best-diff-2 diff)
                 (set! best-2 w))))))
          ((and (<= w-left posn) (< posn (+ w-left w-width)))
           (when (or (and (eq? direction 'above)
                          (or (and (<= w-top first) (> w-top best-edge))
                              (and wrap
                                   (windmove--at-side-p window 'top)
                                   (not (eq? w 'minibuffer))
                                   (windmove--at-side-p w 'bottom))))
                     (and (eq? direction 'below)
                          (or (and (>= w-top first) (< w-top best-edge))
                              (and wrap
                                   (not (eq? window 'minibuffer))
                                   (windmove--at-side-p window 'bottom)
                                   (not (eq? w 'minibuffer))
                                   (windmove--at-side-p w 'top)))))
             (set! best-edge w-top)
             (set! best w)))
          ((or (and (eq? direction 'above) (<= (+ w-top w-height) first))
               (and (eq? direction 'below) (<= last w-top)))
           (let ((diff (windmove--in-direction-2 w posn hor)))
             (when (or (< diff best-diff-2)
                       (and (= diff best-diff-2)
                            (if (eq? direction 'above) (> w-top best-edge-2) (< w-top best-edge-2))))
               (set! best-edge-2 w-top)
               (set! best-diff-2 diff)
               (set! best-2 w)))))))
     (append (window-list) (if minibuf '(minibuffer) '())))
    (or best best-2)))

(define (windmove-find-other-window dir . args)
  "Return the window in direction DIR (left, up, right or down) from WINDOW (default: the selected one).
ARG, if negative, means to look from the right or bottom edge of WINDOW."
  (window-in-direction dir (and (> (length args) 1) (cadr args)) #f
                       (and (pair? args) (car args)) windmove-wrap-around #t))

(define (windmove-do-window-select dir . args)
  "Select the window in direction DIR from WINDOW; signal an error if there is none."
  (let ((other (apply windmove-find-other-window dir args)))
    (cond ((not other) (user-error (format "No window %s from selected window" dir)))
          ((eq? other 'minibuffer) (user-error "Minibuffer is inactive"))
          (else (select-window other)))))

(define (windmove--arg args)
  (let ((arg (simple--raw-arg args)))
    (and arg (prefix-numeric-value arg))))

(define-command (windmove-left . args)
  "Select the window to the left of the current one.
With a positive prefix argument, \"left\" is relative to the top edge of the current
window; with a negative one, to the bottom edge.
If no window is at the desired location, an error is signaled."
  (windmove-do-window-select 'left (windmove--arg args)))

(define-command (windmove-up . args)
  "Select the window above the current one.
With a positive prefix argument, \"up\" is relative to the left edge of the current
window; with a negative one, to the right edge.
If no window is at the desired location, an error is signaled."
  (windmove-do-window-select 'up (windmove--arg args)))

(define-command (windmove-right . args)
  "Select the window to the right of the current one.
With a positive prefix argument, \"right\" is relative to the top edge of the current
window; with a negative one, to the bottom edge.
If no window is at the desired location, an error is signaled."
  (windmove-do-window-select 'right (windmove--arg args)))

(define-command (windmove-down . args)
  "Select the window below the current one.
With a positive prefix argument, \"down\" is relative to the left edge of the current
window; with a negative one, to the right edge.
If no window is at the desired location, an error is signaled."
  (windmove-do-window-select 'down (windmove--arg args)))

(define-command (windmove-default-keybindings . args)
  "Set up keybindings for windmove: MODIFIERS-{left,right,up,down}.
MODIFIERS is a modifier (shift, control, meta, super or hyper) or a list of them;
the default is shift.  If MODIFIERS is none, bind the arrow keys themselves."
  (let* ((modifiers (if (and (pair? args) (car args)) (car args) 'shift))
         (modifiers (cond ((eq? modifiers 'none) '())
                          ((list? modifiers) modifiers)
                          (else (list modifiers))))
         (prefix (apply string-append
                        (map (lambda (m)
                               (cond ((eq? m 'shift) "S-")
                                     ((eq? m 'control) "C-")
                                     ((eq? m 'meta) "M-")
                                     ((eq? m 'super) "s-")
                                     ((eq? m 'hyper) "H-")
                                     (else (error (format "Unknown modifier: %s" m)))))
                             modifiers))))
    (for-each (lambda (binding)
                (global-set-key (string-append prefix "<" (car binding) ">") (cadr binding)))
              '(("left" "windmove-left") ("right" "windmove-right")
                ("up" "windmove-up") ("down" "windmove-down")))))

(provide 'windmove)

;;; windmove.scm ends here
