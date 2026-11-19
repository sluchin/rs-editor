;;; modes.scm --- minor modes of the editor -*- scheme -*-

;; Emacs の副モード (minor mode) のうち, 打った文字と画面の見た目に関わるもの. 起動のときに読む (core/src/scheme/loadup.rs).
;; バッファだけのモード (overwrite, auto-fill, visual-line, display-line-numbers, hl-line) は, バッファが覚える
;; (`set-buffer-minor-mode`). 全てのバッファのモード (line-number, column-number, electric-pair,
;; electric-indent, show-paren と, global-*-mode) は, エディタが覚える (`set-global-minor-mode`).
;; 初めから入っているのは, line-number, column-number, electric-indent, show-paren (core/src/editor.rs).
;; 画面は, 効いているモードの名前 (`BufferFrame.minor_modes`) を見て描く.

(define (modes--command args name global label)
  "Turn the minor mode NAME on or off for ARGS, like a minor mode command, and return whether it is on.
GLOBAL true means that it is a mode of all buffers.  LABEL is the name shown in the message."
  (let* ((interactive (null? args))
         (mode (simple--mode-arg args))
         (now (if global (global-minor-mode-p name) (buffer-minor-mode-p name)))
         (on (if (eq? mode 'toggle) (not now) (= mode 1))))
    (if global (set-global-minor-mode name on) (set-buffer-minor-mode name on))
    (if interactive
        (message (format "%s %s%s" label (if on "enabled" "disabled") (if global "" " in current buffer"))))
    on))

;;; Modes shown on the screen

(define-command (visual-line-mode . args)
  "Toggle visual line mode in the current buffer: long lines wrap at the window edge.
With a positive argument, turn it on; with zero or a negative one, turn it off."
  (modes--command args 'visual-line #f "Visual-Line mode"))

(define-command (global-visual-line-mode . args)
  "Toggle Visual-Line mode in all buffers."
  (modes--command args 'visual-line #t "Global Visual-Line mode"))

(define-command (display-line-numbers-mode . args)
  "Toggle the display of line numbers in the current buffer."
  (modes--command args 'display-line-numbers #f "Display-Line-Numbers mode"))

(define-command (global-display-line-numbers-mode . args)
  "Toggle the display of line numbers in all buffers."
  (modes--command args 'display-line-numbers #t "Global Display-Line-Numbers mode"))

(define-command (hl-line-mode . args)
  "Toggle the highlight of the line at point in the current buffer."
  (modes--command args 'hl-line #f "Hl-Line mode"))

(define-command (global-hl-line-mode . args)
  "Toggle the highlight of the line at point in all buffers."
  (modes--command args 'hl-line #t "Global Hl-Line mode"))

(define-command (line-number-mode . args)
  "Toggle the line number in the mode line."
  (modes--command args 'line-number #t "Line number mode"))

(define-command (column-number-mode . args)
  "Toggle the column number in the mode line."
  (modes--command args 'column-number #t "Column number mode"))

(define-command (show-paren-mode . args)
  "Toggle the highlight of the parenthesis that matches the one at point."
  (modes--command args 'show-paren #t "Show Paren mode"))

;;; Modes that change what typed characters do

(define-command (overwrite-mode . args)
  "Toggle Overwrite mode in the current buffer: a typed character replaces the one after point."
  (modes--command args 'overwrite #f "Overwrite mode"))

(define (overwrite--before-insert)
  "Delete the characters that a typed character replaces, for `overwrite-mode'."
  (when (and (buffer-minor-mode-p 'overwrite) (not (delete-selection--active-p)))
    (let ((end (min (+ (point) (string-length self-insert-text)) (line-end-position))))
      (delete-region (point) end))))

(add-hook 'pre-self-insert-hook 'overwrite--before-insert)

(define-command (electric-indent-mode . args)
  "Toggle indentation of the new line when RET is typed (on by default)."
  (modes--command args 'electric-indent #t "Electric-Indent mode"))

(define electric-pair-pairs '(("(" ")") ("[" "]") ("{" "}") ("\"" "\""))
  "List of the pairs that `electric-pair-mode' inserts: (OPENING CLOSING).")

(define-command (electric-pair-mode . args)
  "Toggle inserting the closing parenthesis or quote when the opening one is typed.
Typing the closing character when it is already after point moves over it."
  (modes--command args 'electric-pair #t "Electric-Pair mode"))

(define (electric-pair--after-insert)
  "Insert the closing character of a typed opening one, or skip over a typed closing one."
  (when (global-minor-mode-p 'electric-pair)
    (let* ((typed self-insert-text)
           (pair (assoc typed electric-pair-pairs))
           (next (char-after)))
      (cond ((and next (equal? typed next)
                  (member typed (map cadr electric-pair-pairs)))
             (delete-char 1))
            ((and pair
                  (or (not next) (member next '(" " "\t" "\n"))
                      (member next (map cadr electric-pair-pairs)))
                  (not (and (equal? typed (cadr pair))
                            (> (point) 2)
                            (equal? (char-syntax (char-before (- (point) 1))) "w"))))
             (insert (cadr pair))
             (backward-char 1))))))

(add-hook 'post-self-insert-hook 'electric-pair--after-insert)

(define-command (auto-fill-mode . args)
  "Toggle Auto Fill mode in the current buffer: a space typed past the fill column breaks the line."
  (modes--command args 'auto-fill #f "Auto-Fill mode"))

(define (auto-fill--break-position here)
  "Return where to break the line that point (HERE, after a typed space) has made too long, or false."
  (save-excursion
    (let* ((bol (line-beginning-position))
           (indent-end (begin (goto-char bol) (skip-chars-forward " \t") (point))))
      (move-to-column (+ fill-column 1))
      (skip-chars-backward "^ \t" bol)
      (skip-chars-backward " \t" bol)
      (if (> (point) indent-end)
          (point)
          (begin
            (goto-char indent-end)
            (skip-chars-forward "^ \t" here)
            (if (< (+ (point) 1) here) (point) #f))))))

(define (do-auto-fill)
  "Break the current line at a space so that it fits in `fill-column', if it is too long."
  (when (> (current-column) fill-column)
    (let* ((here (point-marker))
           (at (auto-fill--break-position (marker-position here))))
      (when at
        (let ((indent (begin (goto-char (line-beginning-position)) (skip-chars-forward " \t") (current-column))))
          (goto-char at)
          (delete-horizontal-space)
          (insert "\n")
          (if fill-prefix (insert fill-prefix) (simple--indent-to indent))))
      (goto-char (marker-position here))
      (marker-release here))))

(define (auto-fill--after-insert)
  "Fill the line after a space is typed, for `auto-fill-mode'."
  (when (and (buffer-minor-mode-p 'auto-fill) (equal? self-insert-text " "))
    (do-auto-fill)))

(add-hook 'post-self-insert-hook 'auto-fill--after-insert)
