;;; edit.el --- core/tests/edit.json を作る (emacs -Q --batch -l edit.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, 編集のコマンド (simple.el) を動かし, 本文, カーソル, kill-ring の先頭, マーク, this-command を書く.
;; 形: [[名前, 手順, 種, マーク, 本文, [位置ごとの結果]], ...].
;;   手順: [[コマンド, 引数, 前のコマンド], ...]. 引数: 数, null (nil), true (t), "sym:名前" (記号), {"c":"a"} (文字), {"l":[4]} (リスト).
;;     引数 "region" は, 点とマークの間 (kill-region の BEG END). 前のコマンドが "keep" なら, last-command を変えない.
;;   種: 動かす前に kill-new で入れる文字列 (後のものが先頭).
;;   結果: [本文, カーソル, kill-ring の先頭 または null, マーク または null, this-command または null, mark-active, deactivate-mark, 出したメッセージ または null]
;;         または ["error", メッセージ, 本文, カーソル]. 位置は 0 から数える.

(require 'json)
(require 'cl-lib)

;; 対話的に使うときと同じに, マークは, transient-mark-mode で使う. message は, 出した文字を覚える
(transient-mark-mode 1)
(defvar last-message nil)
(advice-add 'message :override
            (lambda (fmt &rest args)
              (setq last-message (and fmt (apply #'format-message fmt args)))))

(defconst texts
  '("abc def\nghi jkl\n\n  mno pqr  \nstu"
    "foo  bar\t baz\n   indented\nlast"
    "あいう えお\nabc"
    ""
    "x"
    "a\n\n\n\nb\n\n"
    "one two three"))

(defun step (cmd args &optional last) (vector cmd args last))

(defun one (name cmd args &optional seeds mark)
  (list name (vector (step cmd args)) (vconcat seeds) mark))

(defconst cases
  (append
   (mapcar (lambda (a) (one "kill-line" "kill-line" a)) '([] [1] [2] [0] [-1] [-2]))
   (mapcar (lambda (a) (one "kill-whole-line" "kill-whole-line" a)) '([] [1] [2] [0] [-1]))
   (mapcar (lambda (a) (one "kill-word" "kill-word" a)) '([1] [2] [-1]))
   (mapcar (lambda (a) (one "backward-kill-word" "backward-kill-word" a)) '([1] [2] [-1]))
   (list (one "delete-horizontal-space" "delete-horizontal-space" [])
         (one "delete-horizontal-space" "delete-horizontal-space" [t])
         (one "delete-blank-lines" "delete-blank-lines" [])
         (one "delete-indentation" "delete-indentation" [])
         (one "delete-indentation" "delete-indentation" [t])
         (one "just-one-space" "just-one-space" [])
         (one "just-one-space" "just-one-space" [2])
         (one "just-one-space" "just-one-space" [-1])
         (one "cycle-spacing" "cycle-spacing" [])
         (one "open-line" "open-line" [1])
         (one "open-line" "open-line" [2])
         (one "split-line" "split-line" [])
         (one "newline" "newline" [])
         (one "newline" "newline" [2]))
   (mapcar (lambda (a) (one "transpose-chars" "transpose-chars" a)) '([1] [-1] [2]))
   (mapcar (lambda (a) (one "transpose-words" "transpose-words" a)) '([1] [-1] [2]))
   (mapcar (lambda (a) (one "transpose-lines" "transpose-lines" a)) '([1] [-1] [2]))
   (list (one "zap-to-char" "zap-to-char" (vector 1 '((c . "e"))))
         (one "zap-to-char" "zap-to-char" (vector 2 '((c . "e"))))
         (one "zap-to-char" "zap-to-char" (vector -1 '((c . "e"))))
         (one "zap-to-char" "zap-to-char" (vector 1 '((c . " ")))))
   (mapcar (lambda (a) (one "yank" "yank" a '("X" "YY"))) (list [] [2] (vector '((l . [4]))) ["sym:-"]))
   ;; yank の後の yank-pop
   (mapcar (lambda (arg)
             (list "yank-pop" (vector (step "yank" []) (step "yank-pop" arg "yank")) (vector "A" "B" "C") nil))
           '([] [2] [-1]))
   ;; リージョン
   (list (one "kill-region" "kill-region" "region" nil 1)
         (one "kill-ring-save" "kill-ring-save" "region" nil 1)
         (list "kill-region-twice"
               (vector (step "kill-word" [1]) (step "kill-word" [1] "kill-region"))
               (vector) nil)
         (list "append-next-kill"
               (vector (step "kill-word" [1]) (step "append-next-kill" [nil] "keep") (step "kill-word" [1] "keep"))
               (vector) nil)
         (list "kill-line-twice"
               (vector (step "kill-line" []) (step "kill-line" [] "kill-region"))
               (vector) nil)
         (one "insert-buffer" "insert-buffer" ["other"]))
   ;; マーク
   (list (one "exchange-point-and-mark" "exchange-point-and-mark" [] nil 1)
         (one "exchange-point-and-mark" "exchange-point-and-mark" [t] nil 1)
         (one "mark-whole-buffer" "mark-whole-buffer" [])
         (one "mark-word" "mark-word" [1])
         (one "mark-word" "mark-word" [2])
         (one "mark-word" "mark-word" [-1])
         (one "set-mark-command" "set-mark-command" [nil])
         (one "set-mark-command" "set-mark-command" [t])
         (one "push-mark-command" "push-mark-command" [nil])
         (one "what-cursor-position" "what-cursor-position" [])
         )))

(defun decode-arg (a)
  (cond ((null a) nil)
        ((eq a t) t)
        ((numberp a) a)
        ((and (stringp a) (string-prefix-p "sym:" a)) (intern (substring a 4)))
        ((stringp a) a)
        ((and (consp a) (assq 'c a)) (string-to-char (cdr (assq 'c a))))
        ((and (consp a) (assq 'l a)) (append (cdr (assq 'l a)) nil))
        (t a)))

(defun run-steps (steps seeds mark pos)
  (setq kill-ring nil kill-ring-yank-pointer nil)
  (dolist (s (append seeds nil)) (kill-new s))
  (goto-char pos)
  (set-mark nil)
  (when mark (set-mark (1+ mark)))
  (setq deactivate-mark nil last-message nil)
  (condition-case err
      (progn
        (cl-loop for step across steps
                 do (let ((cmd (aref step 0)) (args (aref step 1)) (last (aref step 2)))
                      (unless (equal last "keep") (setq last-command (and last (intern last))))
                      (setq this-command (intern cmd))
                      (if (equal args "region")
                          (funcall (intern cmd) (region-beginning) (region-end))
                        (apply (intern cmd) (mapcar #'decode-arg (append args nil))))
                      (unless (equal last "keep") nil)))
        (vector (buffer-string) (1- (point)) (car kill-ring)
                (and (mark t) (1- (mark t)))
                (and (symbolp this-command) this-command (symbol-name this-command))
                (if mark-active t :json-false)
                (if deactivate-mark t :json-false)
                last-message))
    (error (vector "error" (error-message-string err) (buffer-string) (1- (point))))))

(defvar results nil)
(generate-new-buffer "other")
(with-current-buffer "other" (insert "OTHER"))
(dolist (text texts)
  (dolist (case cases)
    (let* ((name (nth 0 case)) (steps (nth 1 case)) (seeds (nth 2 case)) (mark (nth 3 case))
           (per-pos nil))
      (dotimes (i (1+ (length text)))
        (with-temp-buffer
          (insert text)
          (push (run-steps steps seeds mark (1+ i)) per-pos)))
      (push (list name steps (or seeds []) (or mark :json-false) text (vconcat (nreverse per-pos))) results))))

(with-temp-file (expand-file-name (or (getenv "OUT") "edit.json"))
  (insert (json-encode (vconcat (nreverse results)))))
