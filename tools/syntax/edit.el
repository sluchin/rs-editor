;;; edit.el --- core/tests/edit.json を作る (emacs -Q --batch -l edit.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, 編集のコマンド (simple.el) を動かし, 本文, カーソル, kill-ring の先頭, マーク, this-command を書く.
;; 形: [[名前, 手順, 種, マーク, 本文, [位置ごとの結果]], ...].
;;   手順: [[コマンド, 引数, 前のコマンド], ...]. 引数: 数, null (nil), true (t), "sym:名前" (記号), {"c":"a"} (文字), {"l":[4]} (リスト).
;;     引数 "region" は, 点とマークの間 (kill-region の BEG END). ["region", 引数...] は, BEG END の後ろに引数を足す. 前のコマンドが "keep" なら, last-command を変えない.
;;   種: 動かす前に kill-new で入れる文字列 (後のものが先頭).
;;   結果: [本文, カーソル, kill-ring の先頭 または null, マーク または null, this-command または null, mark-active, deactivate-mark, 出したメッセージ または null]
;;         または ["error", メッセージ, 本文, カーソル]. 位置は 0 から数える.

(require 'json)
(require 'cl-lib)

;; 対話的に使うときと同じに, マークは, transient-mark-mode で使う. message は, 出した文字を覚える
(transient-mark-mode 1)
(setq dabbrev-check-other-buffers nil)
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
    "one two three"
    "foobar food Foo\nfoo FOOBAZ fo bar_x barx\nbar-y b"
    "abc ABC Abc foo FOO Foo x"))

(defvar abbrev-inputs nil)
(advice-add 'read-string :override (lambda (&rest _) (pop abbrev-inputs)))
(advice-add 'y-or-n-p :override (lambda (&rest _) (equal (pop abbrev-inputs) "y")))

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
   (mapcar (lambda (a) (one "upcase-word" "upcase-word" a)) '([1] [2] [-1] [-2]))
   (mapcar (lambda (a) (one "downcase-word" "downcase-word" a)) '([1] [2] [-1]))
   (mapcar (lambda (a) (one "capitalize-word" "capitalize-word" a)) '([1] [2] [-1] [-2]))
   (list (one "upcase-region" "upcase-region" "region" nil 1)
         (one "downcase-region" "downcase-region" "region" nil 1)
         (one "capitalize-region" "capitalize-region" "region" nil 1))
   ;; 矩形
   (list (one "kill-rectangle" "kill-rectangle" "region" nil 1)
         (one "kill-rectangle" "kill-rectangle" ["region" t] nil 1)
         (one "kill-rectangle" "kill-rectangle" "region" nil 9)
         (one "string-rectangle" "string-rectangle" ["region" "XY"] nil 1)
         (one "string-rectangle" "string-rectangle" ["region" ""] nil 12)
         (list "kill-yank-rectangle"
               (vector (step "kill-rectangle" "region") (step "end-of-buffer" []) (step "yank-rectangle" []))
               (vector) 1)
         (list "kill-yank-rectangle"
               (vector (step "kill-rectangle" "region") (step "beginning-of-buffer" []) (step "yank-rectangle" []))
               (vector) 5))
   ;; 矩形 (そのほか)
   (list (one "delete-rectangle" "delete-rectangle" "region" nil 1)
         (one "delete-rectangle" "delete-rectangle" ["region" t] nil 9)
         (one "clear-rectangle" "clear-rectangle" "region" nil 1)
         (one "clear-rectangle" "clear-rectangle" ["region" t] nil 9)
         (one "open-rectangle" "open-rectangle" "region" nil 1)
         (one "open-rectangle" "open-rectangle" ["region" t] nil 9)
         (one "rectangle-number-lines" "rectangle-number-lines" ["region" 1] nil 9)
         (one "rectangle-number-lines" "rectangle-number-lines" ["region" 8 "<%02d>"] nil 1)
         (list "copy-yank-rectangle"
               (vector (step "copy-rectangle-as-kill" "region") (step "end-of-buffer" []) (step "yank-rectangle" []))
               (vector) 1))
   ;; レジスタ (そのほか)
   (list (list "copy-rectangle-to-register"
               (vector (step "copy-rectangle-to-register" [((c . "a")) 2 9]) (step "end-of-buffer" []) (step "insert-register" [((c . "a"))]))
               (vector) nil)
         (list "copy-rectangle-to-register-delete"
               (vector (step "copy-rectangle-to-register" [((c . "a")) 2 9 t]) (step "insert-register" [((c . "a")) t]))
               (vector) nil)
         (list "number-to-register-prefix"
               (vector (step "number-to-register" [5 ((c . "n"))]) (step "insert-register" [((c . "n")) t]))
               (vector) nil)
         (list "number-to-register-buffer"
               (vector (step "number-to-register" [nil ((c . "n"))]) (step "insert-register" [((c . "n")) t]))
               (vector) nil)
         (list "number-to-register-increment"
               (vector (step "number-to-register" [5 ((c . "n"))]) (step "increment-register" [3 ((c . "n"))])
                       (step "insert-register" [((c . "n")) t]))
               (vector) nil)
         (list "increment-register-text"
               (vector (step "copy-to-register" [((c . "t")) 1 3]) (step "increment-register" [nil ((c . "t"))])
                       (step "end-of-buffer" []) (step "insert-register" [((c . "t")) t]))
               (vector) 3)
         (list "increment-register-empty"
               (vector (step "increment-register" [nil ((c . "e"))]) (step "end-of-buffer" [])
                       (step "insert-register" [((c . "e")) t]))
               (vector) 3)
         (list "append-to-register"
               (vector (step "copy-to-register" [((c . "t")) 1 3]) (step "append-to-register" [((c . "t")) 3 5])
                       (step "end-of-buffer" []) (step "insert-register" [((c . "t")) t]))
               (vector) nil)
         (list "point-jump-register"
               (vector (step "point-to-register" [((c . "p"))]) (step "end-of-buffer" []) (step "jump-to-register" [((c . "p"))]))
               (vector) nil)
         (list "jump-empty-register" (vector (step "jump-to-register" [((c . "z"))])) (vector) nil))
   ;; レジスタ
   (list (list "copy-insert-register"
               (vector (step "copy-to-register" [((c . "a")) 1 4]) (step "end-of-buffer" []) (step "insert-register" [((c . "a"))]))
               (vector) nil)
         (list "copy-insert-register-mark"
               (vector (step "copy-to-register" [((c . "a")) 2 6]) (step "beginning-of-buffer" []) (step "insert-register" [((c . "a")) nil]))
               (vector) nil)
         (list "copy-delete-register"
               (vector (step "copy-to-register" [((c . "a")) 2 5 t]) (step "insert-register" [((c . "a")) t]))
               (vector) nil)
         (list "insert-empty-register" (vector (step "insert-register" [((c . "z"))])) (vector) nil)
         (list "copy-register-region" (vector (step "copy-to-register" [((c . "q")) 1 3])) (vector) 1))
   ;; バッファ
   (list (list "clone-buffer"
               (vector (step "clone-buffer" ["clone-x"]) (step "set-buffer" ["clone-x"]))
               (vector) nil)
         (list "clone-buffer-mark"
               (vector (step "clone-buffer" ["clone-x"]) (step "set-buffer" ["clone-x"]))
               (vector) 2)
         (list "clone-buffer-narrowed"
               (vector (step "narrow-to-region" [2 6]) (step "clone-buffer" ["clone-x"]) (step "set-buffer" ["clone-x"]))
               (vector) nil)
         (list "clone-buffer-edit"
               (vector (step "clone-buffer" ["clone-x"]) (step "set-buffer" ["clone-x"]) (step "insert" ["Z"]))
               (vector) nil)
         (list "clone-buffer-twice"
               (vector (step "clone-buffer" ["clone-x"]) (step "clone-buffer" ["clone-x"]) (step "set-buffer" ["clone-x<2>"]))
               (vector) nil)
         (one "not-modified" "not-modified" [nil])
         (one "not-modified-arg" "not-modified" [t]))
   ;; 読み取り専用
   (let ((ro (lambda (cmd args &optional mark) (list (concat "read-only-" cmd)
                                                       (vector (step "read-only-mode" [1]) (step cmd args))
                                                       (vector) mark))))
     (list (funcall ro "insert" ["x"])
           (funcall ro "delete-char" [1])
           (funcall ro "kill-word" [1])
           (funcall ro "kill-line" [])
           (funcall ro "upcase-word" [1])
           (funcall ro "newline" [])
           (funcall ro "transpose-chars" [1])
           (funcall ro "delete-horizontal-space" [])
           (funcall ro "kill-region" ["region"] 1)
           (funcall ro "upcase-region" ["region"] 1)
           (funcall ro "yank" [])
           (funcall ro "open-line" [1])
           (funcall ro "just-one-space" [])
           (list "read-only-off"
                 (vector (step "read-only-mode" [1]) (step "read-only-mode" [-1]) (step "insert" ["x"]))
                 (vector) nil)))
   ;; シェル
   (list (list "shell-command"
               (vector (step "shell-command" ["echo hi"]) (step "insert-buffer" ["*Shell Command Output*"]))
               (vector) nil)
         (list "shell-command-long"
               (vector (step "shell-command" ["printf 'a\\nb\\nc\\nd\\ne\\nf\\ng\\nh\\n'"])
                       (step "insert-buffer" ["*Shell Command Output*"]))
               (vector) nil)
         (list "shell-command-mixed"
               (vector (step "shell-command" ["echo a; echo b 1>&2; echo c"])
                       (step "insert-buffer" ["*Shell Command Output*"]))
               (vector) nil)
         (list "shell-command-no-output" (vector (step "shell-command" ["true"])) (vector) nil)
         (list "shell-command-fail" (vector (step "shell-command" ["false"])) (vector) nil)
         (list "shell-command-fail-output" (vector (step "shell-command" ["echo x; exit 3"])) (vector) nil)
         (list "shell-command-insert" (vector (step "shell-command" ["echo hi" t])) (vector) nil)
         (list "shell-command-insert-twice"
               (vector (step "shell-command" ["echo hi" t]) (step "shell-command" ["echo ho" t]))
               (vector) nil)
         (list "shell-command-buffer"
               (vector (step "shell-command" ["echo hi" "*foo*"]) (step "insert-buffer" ["*foo*"]))
               (vector) nil)
         (list "shell-command-on-region"
               (vector (step "shell-command-on-region" ["region" "tr a-z A-Z"])
                       (step "insert-buffer" ["*Shell Command Output*"]))
               (vector) 1)
         (list "shell-command-on-region-replace"
               (vector (step "shell-command-on-region" ["region" "tr a-z A-Z" nil t]))
               (vector) 1)
         (list "shell-command-on-region-replace-back"
               (vector (step "goto-char" [9]) (step "shell-command-on-region" ["region" "tr a-z A-Z" nil t]))
               (vector) 1)
         (list "shell-command-on-region-t"
               (vector (step "shell-command-on-region" ["region" "tr a-z A-Z" t]))
               (vector) 1)
         (list "shell-command-on-region-empty"
               (vector (step "shell-command-on-region" ["region" "cat"]))
               (vector) 1))
   ;; 略語
   (let ((defs (vector (step "define-abbrev" '(((v . "global-abbrev-table")) "abc" "alphabet city"))
                       (step "define-abbrev" '(((v . "global-abbrev-table")) "foo" "foo bar baz"))
                       (step "define-abbrev" '(((v . "global-abbrev-table")) "one" "uno"))
                       (step "define-abbrev" '(((v . "global-abbrev-table")) "def" "!def")))))
     (cl-flet ((with (&rest steps) (vconcat defs steps))
               (inputs (&rest xs) (step "abbrev-inputs" (vconcat xs))))
       (list (list "expand-abbrev" (with (step "expand-abbrev" [])) (vector) nil)
             (list "expand-abbrev-twice" (with (step "expand-abbrev" []) (step "expand-abbrev" [])) (vector) nil)
             (list "abbrev-prefix-mark"
                   (with (step "end-of-buffer" []) (step "insert" ["pre"]) (step "abbrev-prefix-mark" [nil])
                         (step "insert" ["abc"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "abbrev-prefix-mark-arg"
                   (with (step "end-of-buffer" []) (step "insert" ["pre"]) (step "abbrev-prefix-mark" [t])
                         (step "insert" ["one"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "abbrev-prefix-mark-expands"
                   (with (step "end-of-buffer" []) (step "insert" [" one"]) (step "abbrev-prefix-mark" [nil])
                         (step "insert" ["def"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-global-abbrev"
                   (with (inputs "zz") (step "add-global-abbrev" [nil]) (step "end-of-buffer" [])
                         (step "insert" [" zz"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-global-abbrev-2"
                   (with (inputs "Zz") (step "add-global-abbrev" [2]) (step "end-of-buffer" [])
                         (step "insert" [" zz"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-global-abbrev-region"
                   (with (inputs "zz") (step "add-global-abbrev" [nil]) (step "end-of-buffer" [])
                         (step "insert" [" zz"]) (step "expand-abbrev" []))
                   (vector) 3)
             (list "add-global-abbrev-0"
                   (with (inputs "zz") (step "add-global-abbrev" [0]) (step "end-of-buffer" [])
                         (step "insert" [" zz"]) (step "expand-abbrev" []))
                   (vector) 3)
             (list "add-global-abbrev-undefine"
                   (with (inputs "abc") (step "add-global-abbrev" [-1]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-global-abbrev-redefine"
                   (with (inputs "abc" "y") (step "add-global-abbrev" [1]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-global-abbrev-keep"
                   (with (inputs "abc" "n") (step "add-global-abbrev" [1]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "add-mode-abbrev"
                   (with (inputs "zz") (step "add-mode-abbrev" [nil]) (step "end-of-buffer" [])
                         (step "insert" [" zz"]) (step "expand-abbrev" []))
                   (vector) nil)
             (list "inverse-add-global-abbrev"
                   (with (inputs "expanded text") (step "inverse-add-global-abbrev" [1]))
                   (vector) nil)
             (list "inverse-add-global-abbrev-2"
                   (with (inputs "expanded text") (step "inverse-add-global-abbrev" [2]))
                   (vector) nil)
             (list "inverse-add-global-abbrev-after"
                   (with (inputs "expanded text") (step "inverse-add-global-abbrev" [-1]))
                   (vector) nil)
             (list "inverse-add-global-abbrev-redefine"
                   (with (inputs "new" "y") (step "inverse-add-global-abbrev" [1]))
                   (vector) nil)
             (list "inverse-add-mode-abbrev"
                   (with (inputs "expanded text") (step "inverse-add-mode-abbrev" [1]))
                   (vector) nil))))
   ;; 動的略語
   (append
    (mapcar (lambda (a) (one "dabbrev-expand" "dabbrev-expand" a)) '([nil] [1] [2] [-1] [-2]))
    (list (one "dabbrev-completion" "dabbrev-completion" [nil])
          (list "dabbrev-completion-twice"
                (vector (step "dabbrev-completion" [nil]) (step "dabbrev-completion" [nil] "dabbrev-completion"))
                (vector) nil)
          (list "dabbrev-expand-twice"
                (vector (step "dabbrev-expand" [nil]) (step "dabbrev-expand" [nil] "dabbrev-expand"))
                (vector) nil)
          (list "dabbrev-expand-thrice"
                (vector (step "dabbrev-expand" [nil]) (step "dabbrev-expand" [nil] "dabbrev-expand")
                        (step "dabbrev-expand" [nil] "dabbrev-expand"))
                (vector) nil)
          (list "dabbrev-expand-all"
                (vector (step "dabbrev-expand" [nil]) (step "dabbrev-expand" [nil] "dabbrev-expand")
                        (step "dabbrev-expand" [nil] "dabbrev-expand") (step "dabbrev-expand" [nil] "dabbrev-expand")
                        (step "dabbrev-expand" [nil] "dabbrev-expand") (step "dabbrev-expand" [nil] "dabbrev-expand")
                        (step "dabbrev-expand" [nil] "dabbrev-expand"))
                (vector) nil)))
   (mapcar (lambda (a) (one "delete-char" "delete-char" a)) '([1] [3] [-2] [0] [2 t] [-1 t] [4 t]))
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
        ((and (consp a) (assq 'v a)) (symbol-value (intern (cdr (assq 'v a)))))
        (t a)))

(defun run-steps (steps seeds mark pos)
  (setq kill-ring nil kill-ring-yank-pointer nil abbrev-inputs nil abbrev-start-location nil)
  (when (get-buffer "clone-x") (kill-buffer "clone-x"))
  (when (get-buffer "clone-x<2>") (kill-buffer "clone-x<2>"))
  (clear-abbrev-table global-abbrev-table)
  (clear-abbrev-table local-abbrev-table)
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
                      (cond ((equal cmd "abbrev-inputs")
                             (setq abbrev-inputs (append abbrev-inputs (append args nil))))
                            ((equal args "region")
                             (funcall (intern cmd) (region-beginning) (region-end)))
                            ((and (vectorp args) (> (length args) 0) (equal (aref args 0) "region"))
                             (apply (intern cmd) (region-beginning) (region-end)
                                    (mapcar #'decode-arg (cdr (append args nil)))))
                            (t (apply (intern cmd) (mapcar #'decode-arg (append args nil)))))
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
    (progn
     (let* ((name (nth 0 case)) (steps (nth 1 case)) (seeds (nth 2 case)) (mark (nth 3 case))
           (per-pos nil))
      (dotimes (i (1+ (length text)))
        (with-temp-buffer
          (insert text)
          (push (run-steps steps seeds mark (1+ i)) per-pos)))
      (push (list name steps (or seeds []) (or mark :json-false) text (vconcat (nreverse per-pos))) results)))))

(with-temp-file (expand-file-name (or (getenv "OUT") "edit.json"))
  (insert (json-encode (vconcat (nreverse results)))))
