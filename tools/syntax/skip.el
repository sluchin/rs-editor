;;; skip.el --- core/tests/skip.json を作る (emacs -Q --batch -l skip.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から skip-chars-forward / skip-chars-backward を動かし, 動いた数と動いた先を書く.
;; 形: [[集まり, 向き, 限り, 本文, [位置ごとの [数, カーソル] か ["error", メッセージ]]], ...]. 位置は 0 から.

(require 'json)
(require 'cl-lib)

(defconst sets '("a-c" "^a-c" " \t" "^\n" "[:alpha:]" "[:digit:]a" "\\^a" "a\\-c" "-a" "a-" "^ " "b" "^" "" "あ-ん" "[:space:]" "[:upper:]x" "\\\\"))
(defconst texts '("abc def\n  ghi\tjkl\nABC 123 xyz" "aabbcc  ^-a\\" "あいう abc\nえお" "" "  "))
(defvar results nil)

(dolist (set sets)
  (dolist (dir '("forward" "backward"))
    (dolist (lim '(nil -3 3))
      (dolist (text texts)
        (with-temp-buffer
          (insert text)
          (let ((per-pos
                 (cl-loop for pos from 1 to (1+ (length text))
                          collect
                          (progn
                            (goto-char pos)
                            (condition-case err
                                (let* ((l (and lim (if (> lim 0) (min (1+ (length text)) (+ pos lim))
                                                      (max 1 (+ pos lim)))))
                                       (n (if (equal dir "forward")
                                              (skip-chars-forward set l)
                                            (skip-chars-backward set l))))
                                  (vector n (1- (point))))
                              (error (vector "error" (error-message-string err))))))))
            (push (list set dir (or lim :json-false) text (vconcat per-pos)) results)))))))

(with-temp-file (expand-file-name (or (getenv "OUT") "skip.json"))
  (insert (json-encode (vconcat (nreverse results)))))
