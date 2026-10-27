;;; motion.el --- core/tests/motion.json を作る (emacs -Q --batch -l motion.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, 動きのコマンド (simple.el) を動かし, 動いた先のカーソルとマークを書く.
;; 形: [[コマンド, 引数, 本文, [位置ごとの結果]], ...].
;; 結果は [カーソル, マーク または null] か, 失敗なら ["error", メッセージ, カーソル]. 位置は 0 から数える.

(require 'json)
(require 'cl-lib)

(defconst texts
  '("abc\n  def ghi\n\n\tjk\nlmnopq"
    "あいう\nabc\n日本語\n"
    ""
    "x"
    "\n\n"
    "a b  c\n   \n\t\n  x(y)"
    "one two\nthree  four  five\n  six\n\nseven"))

;; (コマンド . 引数のリスト). 引数 nil は, 引数を渡さない.
(defconst cases
  '((back-to-indentation)
    (beginning-of-buffer) (beginning-of-buffer 3) (beginning-of-buffer 0) (beginning-of-buffer 10)
    (end-of-buffer) (end-of-buffer 3) (end-of-buffer 0) (end-of-buffer 10)
    (move-beginning-of-line 1) (move-beginning-of-line 2) (move-beginning-of-line 0) (move-beginning-of-line -1)
    (move-beginning-of-line 3)
    (move-end-of-line 1) (move-end-of-line 2) (move-end-of-line 0) (move-end-of-line -1) (move-end-of-line 3)
    (next-line) (next-line 1) (next-line 2) (next-line -1) (next-line 3)
    (previous-line) (previous-line 1) (previous-line 2) (previous-line -1)
    (goto-line 1) (goto-line 2) (goto-line 3) (goto-line 9)
    (goto-line-relative 1) (goto-line-relative 3)
    (forward-word 1) (forward-word 2) (forward-word -1) (backward-word 1) (backward-word 2) (backward-word -1)
    (left-char 1) (left-char 3) (left-char -1) (right-char 1) (right-char 3) (right-char -1)
    (left-word 1) (right-word 1) (left-word 2) (right-word 2)
    (forward-char 1) (forward-char 3) (forward-char -2) (backward-char 1) (backward-char 3)))

(defvar results nil)

(defun mark-or-nil () (and (mark t) (1- (mark t))))

(defun run-case (case pos)
  (goto-char pos)
  (set-mark nil)
  (setq last-command nil)
  (condition-case err
      (progn (apply (car case) (cdr case))
             (vector (1- (point)) (mark-or-nil)))
    (error (vector "error" (error-message-string err) (1- (point))))))

(dolist (text texts)
  (dolist (case cases)
    (with-temp-buffer
      (insert text)
      (setq line-move-visual nil)
      (let ((per-pos (cl-loop for pos from 1 to (1+ (length text)) collect (run-case case pos))))
        (push (list (symbol-name (car case)) (or (cadr case) :json-false) text (vconcat per-pos)) results)))))

;; 目標の桁: set-goal-column をした後の next-line と previous-line (どの位置で決めても)
(dolist (text texts)
  (with-temp-buffer
    (insert text)
    (setq line-move-visual nil)
    (let ((per-pos
           (cl-loop for pos from 1 to (1+ (length text))
                    collect
                    (progn
                      (goto-char pos)
                      (set-goal-column nil)
                      (goto-char (point-min))
                      (setq last-command nil)
                      (condition-case err
                          (let ((first (progn (next-line 1) (1- (point))))
                                (second (progn (next-line 1) (1- (point))))
                                (back (progn (previous-line 1) (1- (point)))))
                            (setq goal-column nil)
                            (vector first second back))
                        (error (setq goal-column nil)
                               (vector "error" (error-message-string err) (1- (point)))))))))
      (push (list "set-goal-column" :json-false text (vconcat per-pos)) results))))

(with-temp-file (expand-file-name (or (getenv "OUT") "motion.json"))
  (insert (json-encode (vconcat (nreverse results)))))
