;;; tab-bar.scm --- frame-local tabs with named window configurations -*- scheme -*-

;; Emacs の tab-bar.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; タブごとに, ウィンドウの設定 (current-window-configuration) を持つ. 2 つ以上のタブがあるとき,
;; 画面がタブの行を出す (Emacs の tab-bar-show の 1 と同じ). タブの名前は, 付けなければ, 選んでいるウィンドウのバッファの名前.

(define tab-bar--tabs '()
  "The tabs, in order.  Each is (NAME CONFIGURATION CAPTION): NAME is the name given
by `tab-rename' or #f, CONFIGURATION the window configuration saved when the tab
was left, and CAPTION the buffer name at that time.")

(define tab-bar--current 0
  "The index (from 0) of the current tab.")

(define (tab-bar--list-ref items index)
  "Return the element INDEX of ITEMS."
  (if (= index 0) (car items) (tab-bar--list-ref (cdr items) (- index 1))))

(define (tab-bar--list-set items index value)
  "Return ITEMS with the element INDEX replaced by VALUE."
  (if (= index 0)
      (cons value (cdr items))
      (cons (car items) (tab-bar--list-set (cdr items) (- index 1) value))))

(define (tab-bar--list-remove items index)
  "Return ITEMS without the element INDEX."
  (if (= index 0)
      (cdr items)
      (cons (car items) (tab-bar--list-remove (cdr items) (- index 1)))))

(define (tab-bar--list-insert items index value)
  "Return ITEMS with VALUE inserted before the element INDEX."
  (if (or (= index 0) (null? items))
      (cons value items)
      (cons (car items) (tab-bar--list-insert (cdr items) (- index 1) value))))

(define (tab-bar--ensure)
  "Make the first tab, for the current window configuration, if there is none."
  (when (null? tab-bar--tabs)
    (set! tab-bar--tabs (list (list #f #f (buffer-name))))
    (set! tab-bar--current 0)))

(define (tab-bar--save-current)
  "Store the window configuration of the current tab."
  (let ((tab (tab-bar--list-ref tab-bar--tabs tab-bar--current)))
    (set! tab-bar--tabs
          (tab-bar--list-set tab-bar--tabs tab-bar--current
                             (list (car tab) (current-window-configuration) (buffer-name))))))

(define (tab-bar--names)
  "Return the names shown for the tabs."
  (let loop ((tabs tab-bar--tabs) (index 0) (names '()))
    (if (null? tabs)
        (reverse names)
        (loop (cdr tabs) (+ index 1)
              (cons (or (car (car tabs))
                        (if (= index tab-bar--current) (buffer-name) (caddr (car tabs))))
                    names)))))

(define (tab-bar--update)
  "Tell the screen the names of the tabs and the current one."
  (set-tab-bar (if (pair? (cdr tab-bar--tabs)) (tab-bar--names) '()) tab-bar--current))

(define (tab-bar--select index)
  "Make the tab INDEX (from 0) the current one."
  (unless (= index tab-bar--current)
    (tab-bar--save-current)
    (let ((tab (tab-bar--list-ref tab-bar--tabs index)))
      (set! tab-bar--current index)
      (set-window-configuration (cadr tab))))
  (tab-bar--update))

(define (tab-bar--count-arg args)
  "Return the count given in ARGS or by the prefix argument (1 if none)."
  (cond ((pair? args) (car args))
        (else (prefix-numeric-value (current-prefix-arg)))))

(define (tab-bar--wrap index)
  "Return INDEX wrapped into the range of the tabs."
  (modulo index (length tab-bar--tabs)))

(define-command (tab-new . args)
  "Create a new tab, right after the current one, that shows the current buffer in one window."
  (tab-bar--ensure)
  (tab-bar--save-current)
  (delete-other-windows)
  (let ((index (+ tab-bar--current 1)))
    (set! tab-bar--tabs
          (tab-bar--list-insert tab-bar--tabs index (list #f (current-window-configuration) (buffer-name))))
    (set! tab-bar--current index))
  (tab-bar--update)
  #f)

(define-command (tab-close . args)
  "Close the tab N (the current one if none; N counts from 1) and show the next one.
It is an error to close the only tab."
  (tab-bar--ensure)
  (let ((index (if (pair? args) (- (car args) 1) tab-bar--current)))
    (when (or (< index 0) (>= index (length tab-bar--tabs)))
      (error "No such tab"))
    (when (null? (cdr tab-bar--tabs))
      (error "Attempt to delete the sole tab in a frame"))
    (let ((was-current (= index tab-bar--current)))
      (set! tab-bar--tabs (tab-bar--list-remove tab-bar--tabs index))
      (cond (was-current
             (set! tab-bar--current (min index (- (length tab-bar--tabs) 1)))
             (set-window-configuration (cadr (tab-bar--list-ref tab-bar--tabs tab-bar--current))))
            ((< index tab-bar--current)
             (set! tab-bar--current (- tab-bar--current 1)))))
    (tab-bar--update)
    #f))

(define-command (tab-close-other . args)
  "Close all the tabs but the current one."
  (tab-bar--ensure)
  (tab-bar--save-current)
  (set! tab-bar--tabs (list (tab-bar--list-ref tab-bar--tabs tab-bar--current)))
  (set! tab-bar--current 0)
  (tab-bar--update)
  #f)

(define-command (tab-next . args)
  "Switch to the next tab, or N tabs ahead (back if N is negative), wrapping around."
  (tab-bar--ensure)
  (tab-bar--select (tab-bar--wrap (+ tab-bar--current (tab-bar--count-arg args))))
  #f)

(define-command (tab-previous . args)
  "Switch to the previous tab, or N tabs back (ahead if N is negative), wrapping around."
  (tab-bar--ensure)
  (tab-bar--select (tab-bar--wrap (- tab-bar--current (tab-bar--count-arg args))))
  #f)

(define (tab-bar-select-tab number)
  "Switch to the tab NUMBER (from 1)."
  (tab-bar--ensure)
  (if (or (< number 1) (> number (length tab-bar--tabs)))
      (error "No such tab"))
  (tab-bar--select (- number 1))
  #f)

(define-command (tab-switch . args)
  "Switch to the tab named NAME.  Interactively, read the name in the minibuffer with completion."
  (tab-bar--ensure)
  (let* ((names (tab-bar--names))
         (name (if (pair? args) (car args) (completing-read "Switch to tab by name: " names #f #t)))
         (found (let loop ((names names) (index 0))
                  (cond ((null? names) #f)
                        ((equal? (car names) name) index)
                        (else (loop (cdr names) (+ index 1)))))))
    (if found
        (tab-bar--select found)
        (error (format "No tab with name `%s'" name)))
    #f))

(define-command (tab-rename . args)
  "Give the current tab the name NAME.  An empty name gives it the buffer name again."
  (tab-bar--ensure)
  (let* ((name (if (pair? args) (car args) (read-string "Rename tab: ")))
         (tab (tab-bar--list-ref tab-bar--tabs tab-bar--current)))
    (set! tab-bar--tabs
          (tab-bar--list-set tab-bar--tabs tab-bar--current
                             (list (if (equal? name "") #f name) (cadr tab) (caddr tab))))
    (tab-bar--update)
    #f))

(define-command (tab-move . args)
  "Move the current tab N places ahead (back if N is negative), wrapping around."
  (tab-bar--ensure)
  (let* ((count (length tab-bar--tabs))
         (tab (tab-bar--list-ref tab-bar--tabs tab-bar--current))
         (target (modulo (+ tab-bar--current (tab-bar--count-arg args)) count)))
    (set! tab-bar--tabs
          (tab-bar--list-insert (tab-bar--list-remove tab-bar--tabs tab-bar--current) target tab))
    (set! tab-bar--current target)
    (tab-bar--update)
    #f))

;; バッファを切り替えると, 名前が変わるので, コマンドの後に, 画面へ知らせる
(add-hook 'post-command-hook
          (lambda () (if (and (pair? tab-bar--tabs) (pair? (cdr tab-bar--tabs))) (tab-bar--update))))

(global-set-key "C-x t 2" 'tab-new)
(global-set-key "C-x t 0" 'tab-close)
(global-set-key "C-x t 1" 'tab-close-other)
(global-set-key "C-x t o" 'tab-next)
(global-set-key "C-x t O" 'tab-previous)
(global-set-key "C-x t RET" 'tab-switch)
(global-set-key "C-x t r" 'tab-rename)
(global-set-key "C-x t m" 'tab-move)

(provide 'tab-bar)

;;; tab-bar.scm ends here
