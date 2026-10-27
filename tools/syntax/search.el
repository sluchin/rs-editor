;;; search.el --- core/tests/search.json を作る (emacs -Q --batch -l search.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, re-search-forward, re-search-backward, looking-at を動かし, 合った所 (match-data) を書く.
;; 形: [[パターン, 本文, 問い, [位置ごとの結果]], ...]. 結果は, 合わなければ null,
;; 合えば [動いた先の位置, [始め, 終わり] または null, ...]. 位置は 0 から数える.

(require 'json)
(require 'cl-lib)

(defconst patterns
  '("a" "ab" "a\\|b" "\\(a\\)\\(b\\)" "\\(?:ab\\)+" "a*" "a+" "a?" "a*?" "a+?" "x\\{2\\}" "a\\{1,2\\}" "a\\{2,\\}"
    "[a-c]+" "[^a-c]" "[[:alpha:]]+" "[[:digit:]]+" "[[:space:]]" "[]a]" "[^]a]" "[a\\]" "."
    "^a" "a$" "^$" "^" "$" "a^b" "a$b" "\\`a" "a\\'" "\\<a" "a\\>" "\\bfoo\\b" "\\Bo" "\\w+" "\\W+" "\\s-+" "\\sw+"
    "(a)" "a{2}" "a|b" "*a" "\\(*a\\)" "\\(a\\|b\\)c" "\\(a\\)?b" "foo\\|foobar" "\\(foo\\|foobar\\)x"
    "\\." "\\*" "\\+" "\\?" "\\\\" "[.]" "\\(\\)" "" "\\n" "a.b" "\\_<foo\\_>" "\\(a\\)\\1" "\\(.\\)\\1" "\\(a\\|b\\)\\1+" "[[:upper:]]" "[[:lower:]]+" "[[:word:]]+" "[[:punct:]]" "[[:blank:]]+"))

(defconst texts
  '("abab aab  ab" "foo foobar x_foo\nfoo" "a\nab\n\nb" "aaa" "a^b a$b ab" "x(a){2} a|b *a" "A a É é" "  a1b22 \t3" "\n" ""))

(defun match-vec ()
  (vconcat (mapcar (lambda (i)
                     (let ((s (match-beginning i)))
                       (and s (vector (1- s) (1- (match-end i))))))
                   (number-sequence 0 (1- (/ (length (match-data)) 2))))))

(defvar results nil)

(dolist (pat patterns)
  (dolist (text texts)
    (dolist (kind '("re-search-forward" "re-search-backward" "looking-at" "re-search-forward 2"
                    "re-search-forward fold" "re-search-backward fold" "looking-at fold"))
      (with-temp-buffer
        (insert text)
        (let* ((case-fold-search (string-suffix-p " fold" kind))
               (per-pos
               (cl-loop for pos from 1 to (1+ (length text))
                        collect
                        (progn
                          (goto-char pos)
                          (condition-case err
                              (let ((ok (pcase (string-remove-suffix " fold" kind)
                                          ("re-search-forward" (re-search-forward pat nil t))
                                          ("re-search-forward 2" (re-search-forward pat nil t 2))
                                          ("re-search-backward" (re-search-backward pat nil t))
                                          ("looking-at" (looking-at pat)))))
                                (and ok (vconcat (vector (1- (point))) (match-vec))))
                            (error (vector "error")))))))
          (push (list pat text kind (vconcat per-pos)) results))))))

(with-temp-file (expand-file-name (or (getenv "OUT") "search.json"))
  (insert (json-encode (vconcat (nreverse results)))))
