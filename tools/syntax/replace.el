;;; replace.el --- core/tests/replace.json を作る (emacs -Q --batch -l replace.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, 置換のコマンドを fundamental-mode で動かし, 結果を書く.
;; 形: [[名前, 本文, 位置, マーク または null, Scheme の式, [キー], 結果], ...].
;;   結果: [本文, カーソル, マーク または null, 出したメッセージ または null, mark-active] または ["error", メッセージ, 本文, カーソル].
;;   キーは, 問い (y-or-n など) に答えるキーの名前 (" ", "y", "n", "Enter", "Backspace", "!", ".", ",", "q"). 位置は 0 から数える.

(require 'json)
(require 'cl-lib)
(transient-mark-mode 1)
(defvar last-message nil)
(advice-add 'message :override
            (lambda (fmt &rest args)
              (setq last-message (and fmt (apply #'format-message fmt args)))))

(defconst texts
  '("foo bar foo\nFoo FOO baz foo" "aaa" "a b a b\n" "xaxbx" "The cat. The Cat THE CAT the cat"
    "foo-bar foobar bar foo" "" "ab\nab\n" "aXbxAxB"))

;; (名前 Emacs の式 Scheme の式 キー マークの位置 または nil)
(defconst cases
  '(("rs/foo" "(replace-string \"foo\" \"baz\")" "(replace-string \"foo\" \"baz\")" nil nil)
    ("rs/Foo" "(replace-string \"Foo\" \"x\")" "(replace-string \"Foo\" \"x\")" nil nil)
    ("rs/amp" "(replace-string \"a\" \"\\\\&b\")" "(replace-string \"a\" \"\\\\&b\")" nil nil)
    ("rs/delimited" "(replace-string \"foo\" \"baz\" t)" "(replace-string \"foo\" \"baz\" #t)" nil nil)
    ("rs/region" "(replace-string \"a\" \"b\" nil 2 7)" "(replace-string \"a\" \"b\" #f 2 7)" nil nil)
    ("rs/empty-from-case" "(replace-string \"the cat\" \"a dog\")" "(replace-string \"the cat\" \"a dog\")" nil nil)
    ("rr/groups" "(replace-regexp \"\\\\(a\\\\)\\\\(b\\\\)?\" \"[\\\\2\\\\1\\\\&]\")" "(replace-regexp \"\\\\(a\\\\)\\\\(b\\\\)?\" \"[\\\\2\\\\1\\\\&]\")" nil nil)
    ("rr/star" "(replace-regexp \"x*\" \"-\")" "(replace-regexp \"x*\" \"-\")" nil nil)
    ("rr/bol" "(replace-regexp \"^\" \"> \")" "(replace-regexp \"^\" \"> \")" nil nil)
    ("rr/eol" "(replace-regexp \"$\" \";\")" "(replace-regexp \"$\" \";\")" nil nil)
    ("rr/plus" "(replace-regexp \"a+\" \"<\\\\&>\")" "(replace-regexp \"a+\" \"<\\\\&>\")" nil nil)
    ("rr/word" "(replace-regexp \"\\\\<.\" \"_\")" "(replace-regexp \"\\\\<.\" \"_\")" nil nil)
    ("qr/yn" "(query-replace \"a\" \"b\")" "(query-replace \"a\" \"b\")" ("y" "n" "y" "n") nil)
    ("qr/all" "(query-replace \"a\" \"b\")" "(query-replace \"a\" \"b\")" ("n" "!") nil)
    ("qr/quit" "(query-replace \"a\" \"b\")" "(query-replace \"a\" \"b\")" ("y" "q") nil)
    ("qr/dot" "(query-replace \"a\" \"b\")" "(query-replace \"a\" \"b\")" ("n" ".") nil)
    ("qr/comma" "(query-replace \"a\" \"b\")" "(query-replace \"a\" \"b\")" ("," "n") nil)
    ("qr/space-del" "(query-replace \"foo\" \"X\")" "(query-replace \"foo\" \"X\")" (" " "Backspace" "Enter") nil)
    ("qrr/yn" "(query-replace-regexp \"a+\" \"<\\\\&>\")" "(query-replace-regexp \"a+\" \"<\\\\&>\")" ("n" "y" "y") nil)))

(defun key-event (key)
  (cond ((equal key "Enter") ?\r) ((equal key "Backspace") 127) ((equal key " ") ?\s) (t (aref key 0))))

(defun body (text) (let ((b (buffer-string))) (if (equal b text) nil b)))

(defvar results nil)
(dolist (case cases)
  (dolist (text texts)
    (let ((per-pos nil))
      (dotimes (i (1+ (length text)))
        (with-temp-buffer
          (insert text) (goto-char (1+ i)) (set-mark nil)
          (setq deactivate-mark nil last-message nil)
          (let ((unread-command-events (mapcar #'key-event (if (nth 3 case) (append (nth 3 case) '("q")) nil))))
            (push (condition-case err
                      (progn (eval (car (read-from-string (nth 1 case))) t)
                             (vector (body text) (1- (point)) (and (mark t) (1- (mark t))) last-message (if mark-active t :json-false)))
                    (error (vector "error" (error-message-string err) (body text) (1- (point)))))
                  per-pos))))
      (push (list (nth 0 case) text (nth 2 case) (if (nth 3 case) (vconcat (nth 3 case) '("q")) []) (vconcat (nreverse per-pos))) results))))

(with-temp-file (expand-file-name (or (getenv "OUT") "replace.json"))
  (insert (json-encode (vconcat (nreverse results)))))
