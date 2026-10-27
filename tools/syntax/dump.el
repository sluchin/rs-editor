;;; dump.el --- core/tests/syntax.json を作る (emacs -Q --batch -l dump.el) -*- lexical-binding: t -*-
;; 本文の全ての位置で, Emacs の構文の走査 (parse-partial-sexp, scan-lists, scan-sexps, forward-comment,
;; backward-prefix-chars) を動かし, 結果を JSON に書く. 位置は 0 から数える.
;; 形: [[モード, 本文, 問い, [位置ごとの結果]], ...]

(require 'json)
(require 'cl-lib)
(require 'pcase)

(defconst texts
  '("(a (b c) d)"
    "(a [b {c}] d)"
    "foo bar-baz"
    "'(a . b) `(c ,d ,@e) #'f"
    "(a \"str ) (\" b)"
    "(a \"es\\\"c\" b)"
    "(a ; comment ) (\n b)\n"
    "; only comment"
    "(a ;; c1\n   b ;; c2"
    "a\\ b\\(c"
    "(a b"
    "a b)"
    "((a) (b (c)))"
    "(a #| block ( |# b)"
    "#| outer #| inner |# still |# x"
    "#| unterminated"
    "(a \"unterminated"
    "\"s1\" \"s2\"\n(x)"
    "(a | b ) | c|"
    "x ## y ,@z ,@ w"
    "(define (f x)\n  ;; doc\n  (+ x 1))\n"
    "（あ）「い」 う　え"
    "a\u00a0b → ★ 😀 、。 x"
    "a\tb\fc\rd"
    "\\\\ \\"
    ""
    "()"
    ")("
    "(((" ")))"))

(defvar results nil)

(defun call (fn)
  "FN の結果を, JSON にする値にする (位置は 0 から). エラーは [\"error\", メッセージ, 始め, 終わり]."
  (condition-case e
      (let ((v (funcall fn)))
        (cond ((null v) nil)
              ((integerp v) (1- v))
              (t v)))
    (scan-error (vector "error" (nth 1 e) (1- (nth 2 e)) (1- (nth 3 e))))))

(defun ppss (pos)
  (let ((s (parse-partial-sexp 1 pos)))
    (vector (nth 0 s)
            (vconcat (mapcar #'1- (nth 9 s)))
            (if (nth 3 s) t :json-false)
            (if (nth 4 s) t :json-false)
            (if (integerp (nth 4 s)) t :json-false)
            (let ((st (nth 8 s))) (and st (1- st))))))

(defun query (kind pos)
  (goto-char pos)
  (pcase kind
    ("ppss" (ppss pos))
    ("scan-lists 1 0" (call (lambda () (scan-lists pos 1 0))))
    ("scan-lists -1 0" (call (lambda () (scan-lists pos -1 0))))
    ("scan-lists 1 1" (call (lambda () (scan-lists pos 1 1))))
    ("scan-lists -1 1" (call (lambda () (scan-lists pos -1 1))))
    ("scan-lists 1 -1" (call (lambda () (scan-lists pos 1 -1))))
    ("scan-lists -1 -1" (call (lambda () (scan-lists pos -1 -1))))
    ("scan-sexps 1" (call (lambda () (scan-sexps pos 1))))
    ("scan-sexps -1" (call (lambda () (scan-sexps pos -1))))
    ("scan-sexps 2" (call (lambda () (scan-sexps pos 2))))
    ("scan-sexps -2" (call (lambda () (scan-sexps pos -2))))
    ("forward-comment 1" (call (lambda () (forward-comment 1) (point))))
    ("forward-comment 99" (call (lambda () (forward-comment 99) (point))))
    ("backward-prefix-chars" (call (lambda () (backward-prefix-chars) (point))))))

(defconst kinds
  '("ppss" "scan-lists 1 0" "scan-lists -1 0" "scan-lists 1 1" "scan-lists -1 1" "scan-lists 1 -1"
    "scan-lists -1 -1" "scan-sexps 1" "scan-sexps -1" "scan-sexps 2" "scan-sexps -2"
    "forward-comment 1" "forward-comment 99" "backward-prefix-chars"))

(dolist (mode '(("fundamental" . fundamental-mode) ("elisp" . emacs-lisp-mode) ("scheme" . scheme-mode)))
  (dolist (text texts)
    (with-temp-buffer
      (insert text)
      (funcall (cdr mode))
      (syntax-propertize (point-max))
      (dolist (kind kinds)
        (let ((per-pos (cl-loop for pos from 1 to (1+ (length text))
                                collect (query kind pos))))
          (push (list (car mode) text kind (vconcat per-pos)) results))))))

(with-temp-file (expand-file-name (or (getenv "OUT") "syntax.json"))
  (insert (json-encode (vconcat (nreverse results)))))
