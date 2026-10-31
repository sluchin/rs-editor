;;; isearch.el --- core/tests/isearch.json を作る (emacs -Q --batch -l isearch.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, isearch を, キーの並びで動かし, 終わったときのカーソル, マーク, メッセージを書く.
;; Emacs はコマンドループで動くので, キーを, 同じ名前のコマンド (isearch-printing-char など) に直して, 順に呼ぶ.
;; 形: [[開始のコマンド, 本文, 位置, キー, 結果], ...]. キーの名前は, " ", "C-s", "C-r", "Backspace", "Enter", "C-g" と 1 文字.
;;   結果: [カーソル, マーク または null, メッセージ または null] または ["quit", カーソル, マーク または null].  位置は 0 から数える.
;;   キーの並びは, Enter か C-g の quit で終わる.

(require 'json)
(require 'cl-lib)
(transient-mark-mode 1)
(defvar last-message nil)
(advice-add 'message :override
            (lambda (fmt &rest args) (setq last-message (and fmt (apply #'format-message fmt args)))))

(defconst texts '("foo bar foo baz" "aXbxAxB" "a\nb\nfoo\nfoo" "" "oooo ooo"))
(defconst starts '(isearch-forward isearch-backward isearch-forward-regexp isearch-backward-regexp))
(defconst key-lists
  '(("f" "o" "o") ("f" "o" "o" "C-s") ("f" "o" "o" "C-s" "C-s" "C-s") ("z") ("f" "x") ("o" "o" "Backspace")
    ("f" "x" "C-g") ("f" "o" "C-r") ("o" "C-s" "C-r") ("b" "a" "r") (" ") ("a" "Backspace" "Backspace")
    ("f" "o" "o" "C-g" "C-g") ("o" "o" "C-r" "C-r" "C-s") ("x" "C-s" "C-s" "C-s" "C-s") ("a" "x" "C-s" "C-s")
    ("o" "C-r" "C-r" "C-r" "C-r") ("." "*") ("o" "+") ("\\" "(") ("^" "f") ("o" "$")))

(defun key-cmd (k)
  (cond ((equal k "C-s") (lambda () (isearch-repeat-forward)))
        ((equal k "C-r") (lambda () (isearch-repeat-backward)))
        ((equal k "Backspace") (lambda () (isearch-delete-char)))
        ((equal k "Enter") (lambda () (isearch-exit)))
        ((equal k "C-g") (lambda () (isearch-abort)))
        (t (lambda () (let ((last-command-event (aref k 0))) (isearch-printing-char))))))

(defvar results nil)
(dolist (start starts)
  (dolist (text texts)
    (dolist (keys key-lists)
      (let ((per-pos nil) (full (append keys nil)))
        (dotimes (i (1+ (length text)))
          (with-temp-buffer
            (switch-to-buffer (current-buffer))
            (insert text) (goto-char (1+ i)) (set-mark nil) (setq last-message nil)
            (setq search-ring nil regexp-search-ring nil)
            (let ((used nil) (quit nil) (res nil))
              (condition-case err
                  (progn
                    (call-interactively start)
                    (dolist (k keys) (push k used) (funcall (key-cmd k)))
                    (when isearch-mode
                      (if (equal isearch-string "")
                          (progn (isearch-done t) (setq res (vector "skip")))
                        (push "Enter" used) (isearch-exit))))
                (quit (setq quit t) (when isearch-mode (isearch-done t)))
                (error (setq res (vector "error" (error-message-string err)))))
              (when (and isearch-mode (not res)) (isearch-done t))
              (when (and (null res) (not (equal (car used) "Enter")) (not (and quit (equal (car used) "C-g"))))
                (setq res nil))
              (setq full (reverse used))
              (push (or res
                        (if quit
                            (vector "quit" (1- (point)) (and (mark t) (1- (mark t))))
                          (vector (1- (point)) (and (mark t) (1- (mark t))) last-message)))
                    per-pos))))
        (push (list (symbol-name start) text (vconcat keys) (vconcat (nreverse per-pos))) results)))))

(with-temp-file (expand-file-name (or (getenv "OUT") "isearch.json"))
  (insert (json-encode (vconcat (nreverse results)))))
