;;; winner.scm --- restore old window configurations -*- scheme -*-

;; Emacs の winner.el に当たるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; Emacs は, 画面を描き直すときにウィンドウの変化に気付く (window-configuration-change-hook).
;; ここでは, コマンドの後 (post-command-hook) に前のコマンドの後と比べて気付き, 次のコマンドの後で覚える.
;; 画面を描かない --batch では, Emacs と同じく気付かない.

(define winner--on #f
  "True if Winner mode is enabled (`winner-mode' of Emacs; here the name is the command).")

(define winner-ring-size 200
  "Maximum number of stored window configurations.")

(define winner-boring-buffers '("*Completions*")
  "List of buffer names whose windows `winner-undo' will not restore.")

(define winner--ring '()
  "The stored configurations, the newest first.  Each is (CONFIGURATION DATA),
where DATA lists the edges and the buffer of each window.")

(define winner--ring-made #f)
(define winner--currents #f)
(define winner--last-command #f)
(define winner--last-saved #f)
(define winner--modified #f)
(define winner--undo-counter 0)
(define winner--undone-data '())

(define (winner--sorted-window-list)
  (sort (window-list)
        (lambda (x y)
          (let loop ((a (window-edges x)) (b (window-edges y)))
            (cond ((null? a) #f)
                  ((= (car a) (car b)) (loop (cdr a) (cdr b)))
                  (else (< (car a) (car b))))))))

(define (winner--win-data)
  (map (lambda (w) (list (window-edges w) (window-buffer w))) (winner--sorted-window-list)))

(define (winner--conf)
  (let ((data (winner--win-data)))
    ;; 同じ形なら, 前に覚えた設定を使う (設定を覚えるたびに, 増やさない)
    (if (and winner--currents (equal? (cadr winner--currents) data))
        winner--currents
        (list (current-window-configuration) data))))

(define (winner--remember)
  (set! winner--currents (winner--conf)))

(define (winner--configuration)
  (or winner--currents (winner--conf)))

(define (winner--ring-get)
  (unless winner--ring-made
    (set! winner--ring-made #t)
    (set! winner--ring (list (winner--configuration))))
  winner--ring)

(define (winner--drop-index items index)
  "Return ITEMS without the element at INDEX."
  (let loop ((rest items) (i 0) (acc '()))
    (cond ((null? rest) (reverse acc))
          ((= i index) (loop (cdr rest) (+ i 1) acc))
          (else (loop (cdr rest) (+ i 1) (cons (car rest) acc))))))

(define (winner--ring-insert conf)
  (let ((ring (cons conf (winner--ring-get))))
    (set! winner--ring (let loop ((rest ring) (n winner-ring-size) (acc '()))
                         (if (or (null? rest) (<= n 0))
                             (reverse acc)
                             (loop (cdr rest) (- n 1) (cons (car rest) acc)))))))

(define (winner--ring-remove index)
  "Remove the configuration at INDEX of the ring and return it."
  (let ((item (list-ref winner--ring index)))
    (set! winner--ring (winner--drop-index winner--ring index))
    item))

(define (winner--insert-if-new)
  (unless (or winner--last-saved (eq? this-command 'winner-redo))
    (let ((conf (winner--configuration)))
      (winner--ring-get)
      (if (and (pair? winner--ring) (equal? (cadr conf) (cadr (car winner--ring))))
          (winner--ring-remove 0))
      (winner--ring-insert conf)
      (set! winner--last-saved #t))))

(define (winner--note-command)
  (unless (eq? this-command winner--last-command)
    (set! winner--last-saved #f)
    (set! winner--last-command this-command)))

(define (winner-save-old-configurations)
  "Store the window configuration before the last change (in post-command-hook)."
  (winner--note-command)
  (if winner--modified (winner--insert-if-new))
  ;; 描き直すときに気付く変化 (次のコマンドの後で覚える)
  (set! winner--modified
        (and (not noninteractive)
             winner--currents
             (not (equal? (cadr winner--currents) (winner--win-data)))))
  (winner--remember))

(define (winner--save-conditionally)
  (winner--note-command)
  (winner--insert-if-new)
  (winner--remember))

(define (winner--set conf)
  "Restore the configuration CONF, except windows of boring buffers.  The selected window stays if it is in CONF."
  (let ((chosen (selected-window)))
    (set-window-configuration (car conf))
    (if (member chosen (window-list)) (select-window chosen))
    (let ((boring (filter (lambda (w) (member (window-buffer w) winner-boring-buffers))
                          (winner--sorted-window-list))))
      (for-each (lambda (w) (if (pair? (cdr (window-list))) (delete-window w))) boring)
      #t)))

(define-command (winner-mode . args)
  "Toggle Winner mode on or off.
Winner mode is a global minor mode that records the changes in
the window configuration (i.e. how the frames are partitioned
into windows) so that the changes can be \"undone\" using the
command `winner-undo'.  By default this one is bound to the key
sequence C-c <left>.  If you change your mind (while undoing),
you can press C-c <right> (calling `winner-redo')."
  (let* ((interactive (null? args))
         (mode (simple--mode-arg args))
         (on (if (eq? mode 'toggle) (not winner--on) (= mode 1))))
    (set! winner--on on)
    (cond (on
           (add-hook 'post-command-hook 'winner-save-old-configurations)
           (global-set-key "C-c <left>" 'winner-undo)
           (global-set-key "C-c <right>" 'winner-redo)
           (set! winner--modified #t)
           (winner-save-old-configurations))
          (else
           (remove-hook 'post-command-hook 'winner-save-old-configurations)))
    (if interactive (message (if on "Winner mode enabled" "Winner mode disabled")))
    on))

(define (winner--undo-this)
  (let loop ()
    (cond ((>= winner--undo-counter (length winner--ring))
           (message "No further window configuration undo information")
           #f)
          ((and (winner--set (list-ref winner--ring winner--undo-counter))
                (let ((data (winner--win-data)))
                  (and (not (member data winner--undone-data))
                       (begin (set! winner--undone-data (cons data winner--undone-data)) #t))))
           #t)
          (else
           (winner--ring-remove winner--undo-counter)
           (loop)))))

(define-command (winner-undo . args)
  "Switch back to an earlier window configuration saved by Winner mode.
In other words, \"undo\" changes in window configuration."
  (cond ((not winner--on) (error "Winner mode is turned off"))
        (else
         (unless (eq? last-command 'winner-undo)
           (winner--save-conditionally)
           (winner--ring-get)
           (set! winner--undo-counter 0)
           (set! winner--undone-data (list (winner--win-data))))
         (set! winner--undo-counter (+ winner--undo-counter 1))
         (if (winner--undo-this)
             (message "Winner undo (%d / %d)" winner--undo-counter (- (length winner--ring) 1))))))

(define-command (winner-redo . args)
  "Restore a more recent window configuration saved by Winner mode."
  (cond ((eq? last-command 'winner-undo)
         (winner--set (winner--ring-remove 0))
         (message "Winner undid undo"))
        (else (user-error "Previous command was not a `winner-undo'"))))

(provide 'winner)

;;; winner.scm ends here
