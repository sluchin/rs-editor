;;; occur.el --- core/tests/occur.json を作る (emacs -Q --batch -l occur.el) -*- lexical-binding: t -*-
;; 本文と正規表現と前後の行の数から, `occur` の *Occur* バッファの本文とメッセージを書く.
;; 形: [[本文, 正規表現, 前後の行の数 または null, *Occur* の本文 または null, メッセージ または null], ...].

(require 'json)
(require 'cl-lib)
(defvar last-message nil)
(advice-add 'message :override
            (lambda (fmt &rest args)
              (setq last-message (and fmt (apply #'format-message fmt args)))))

(defconst texts
  '("foo\nbar\nfoo bar\nbaz" "a\nb\nc\nd\ne\nf\ng\nh\n" "x\nFoo\nfoo" "a\n\nb" "aaa aaa\nb" "" "foo" "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten\neleven"
    "Foo foo\nFOO\n日本語\nfoo\n"))
(defconst regexps '("foo" "o+" "[ce]" "zzz" "^$" "a*" "^" "$" "Foo" "\\(o\\)\\1" "e\nf" "日本" "\\<t"))
(defconst contexts '(nil 1 2))

(defvar results nil)
(dolist (text texts)
  (dolist (regexp regexps)
    (dolist (n contexts)
      (when (get-buffer "*Occur*") (kill-buffer "*Occur*"))
      (with-current-buffer (get-buffer-create "src")
        (erase-buffer) (insert text) (goto-char (point-min))
        (setq last-message nil)
        (condition-case err (occur regexp n) (error (setq last-message (error-message-string err))))
        (push (list text regexp n
                    (and (get-buffer "*Occur*") (with-current-buffer "*Occur*" (substring-no-properties (buffer-string))))
                    last-message)
              results)))))

(with-temp-file (expand-file-name (or (getenv "OUT") "occur.json"))
  (insert (json-encode (vconcat (nreverse results)))))
