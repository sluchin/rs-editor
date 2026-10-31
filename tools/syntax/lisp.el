;;; lisp.el --- core/tests/lisp.json を作る (emacs -Q --batch -l lisp.el) -*- lexical-binding: t -*-
;; 本文の全ての位置から, S 式, defun, 段落, 文, ページのコマンドを, モードごとに動かし, 本文, カーソル, kill-ring の先頭, マーク, this-command を書く.
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

;; 本文が動かす前と同じなら, 結果の本文は null.
;; コマンドは, 対話的に呼ぶ (引数は, 前置引数).
;; 名前は "モード:コマンド". 本文は lisp-texts.json (tests/frontend/lib/motion.json にあった本文).
(defconst base-texts (append (json-read-file (expand-file-name "lisp-texts.json" (file-name-directory load-file-name))) nil))
(defconst indent-texts (append (json-read-file (expand-file-name "lisp-indent-texts.json" (file-name-directory load-file-name))) nil))
(defconst texts (append base-texts indent-texts))
(defconst modes '(fundamental-mode emacs-lisp-mode scheme-mode))

(defun step (cmd args &optional last) (vector cmd args last))
(defun one (cmd args &optional mark modes kind exclude tabs)
  (list cmd (vector (step cmd args)) [] mark modes kind exclude tabs))

;; transpose-sexps は, バックスラッシュで quote した文字と, `##` `,@` の前後で, まだ Emacs と同じではない
(defconst transpose-excluded "\\\\\\|##")

;; 段落と文は, emacs-lisp-mode と fundamental-mode で同じなので, fundamental-mode と scheme-mode だけ
(defconst text-modes '(fundamental-mode scheme-mode))
(defconst lisp-modes '(emacs-lisp-mode scheme-mode))
(defconst all-modes '(fundamental-mode emacs-lisp-mode scheme-mode))

;; 本文の種類: 文章 (段落と文の命令用. 先頭の 12 個と, 空白の多い最後の 1 つ) と, 式 (それ以外)
(defun text-kind (index)
  (cond ((>= index (length base-texts)) 'indent)
        ((or (< index 12) (= index (1- (length base-texts)))) 'prose)
        (t 'code)))

(defconst cases
  (append
   (list (one "forward-sexp" [] nil all-modes 'code) (one "forward-sexp" [-1] nil all-modes 'code)
         (one "forward-sexp" [2] nil lisp-modes 'code)
         (one "backward-sexp" [] nil lisp-modes 'code) (one "forward-list" [] nil lisp-modes 'code)
         (one "forward-list" [-1] nil lisp-modes 'code) (one "backward-list" [] nil lisp-modes 'code)
         (one "down-list" [] nil all-modes 'code) (one "down-list" [-1] nil lisp-modes 'code)
         (one "up-list" [] nil all-modes 'code) (one "up-list" [2] nil lisp-modes 'code)
         (one "backward-up-list" [] nil all-modes 'code) (one "backward-up-list" [-1] nil lisp-modes 'code)
         (one "kill-sexp" [] nil lisp-modes 'code) (one "kill-sexp" [-1] nil lisp-modes 'code)
         (one "backward-kill-sexp" [] nil lisp-modes 'code)
         (one "mark-sexp" [] nil all-modes 'code) (one "mark-sexp" [] 1 lisp-modes 'code)
         (one "mark-sexp" [-1] nil lisp-modes 'code)
         (one "beginning-of-defun" [] nil all-modes 'code) (one "beginning-of-defun" [2] nil lisp-modes 'code)
         (one "end-of-defun" [] nil all-modes 'code) (one "end-of-defun" [2] nil lisp-modes 'code)
         (one "mark-defun" [] nil all-modes 'code) (one "mark-defun" [] 1 lisp-modes 'code)
         (one "forward-paragraph" [] nil text-modes 'prose) (one "forward-paragraph" [2] nil text-modes 'prose)
         (one "forward-paragraph" [-1] nil text-modes 'prose) (one "backward-paragraph" [] nil text-modes 'prose)
         (one "mark-paragraph" [] nil text-modes 'prose) (one "mark-paragraph" [] 1 text-modes 'prose)
         (one "kill-paragraph" [] nil text-modes 'prose) (one "backward-kill-paragraph" [] nil text-modes 'prose)
         (one "forward-sentence" [] nil text-modes 'prose) (one "forward-sentence" [2] nil text-modes 'prose)
         (one "forward-sentence" [-1] nil text-modes 'prose) (one "backward-sentence" [] nil text-modes 'prose)
         (one "kill-sentence" [] nil text-modes 'prose) (one "backward-kill-sentence" [] nil text-modes 'prose)
         (one "mark-end-of-sentence" [] nil text-modes 'prose)
         (one "transpose-sexps" [] nil lisp-modes 'code transpose-excluded) (one "transpose-sexps" [-1] nil lisp-modes 'code transpose-excluded)
         (one "transpose-sexps" [2] nil lisp-modes 'code transpose-excluded) (one "transpose-sexps" [0] 2 lisp-modes 'code transpose-excluded)
         (one "transpose-sentences" [] nil text-modes 'prose) (one "transpose-sentences" [-1] nil text-modes 'prose)
         (one "transpose-paragraphs" [] nil text-modes 'prose) (one "transpose-paragraphs" [-1] nil text-modes 'prose)
         (one "indent-for-tab-command" [] nil all-modes 'indent) (one "indent-for-tab-command" [] nil all-modes 'indent nil 'notabs)
         (one "indent-region" [] 0 all-modes 'indent) (one "indent-region" [] 0 all-modes 'indent nil 'notabs)
         (one "indent-sexp" [] nil lisp-modes 'indent) (one "newline" [] nil all-modes 'indent)
         (one "newline" [] nil all-modes 'indent nil 'notabs)
         (one "comment-dwim" [] nil lisp-modes 'indent "#|\\|—") (one "comment-dwim" [] nil lisp-modes 'code "#|\\|—")
         (one "comment-line" [] nil lisp-modes 'indent "#|\\|\t")
         (one "indent-rigidly" [2] 0 all-modes 'indent) (one "indent-rigidly" [-3] 0 all-modes 'indent nil 'notabs)
         (one "tab-to-tab-stop" [] nil all-modes 'indent) (one "tab-to-tab-stop" [] nil all-modes 'indent nil 'notabs)
         (one "set-fill-prefix" [] nil all-modes 'indent)
         (one "fill-paragraph" [] nil text-modes 'prose "\f") (one "fill-paragraph" [] nil lisp-modes 'indent "\f\\|#|\\|line doc\\|more\"")
         (one "forward-page" [] nil '(fundamental-mode) 'prose) (one "forward-page" [2] nil '(fundamental-mode) 'prose)
         (one "forward-page" [-1] nil '(fundamental-mode) 'prose) (one "backward-page" [] nil '(fundamental-mode) 'prose)
         (one "mark-page" [] nil '(fundamental-mode) 'prose) (one "mark-page" [1] nil '(fundamental-mode) 'prose)
         (one "mark-page" [-1] nil '(fundamental-mode) 'prose))))

(defun decode-arg (a)
  (cond ((null a) nil)
        ((eq a t) t)
        ((numberp a) a)
        ((and (stringp a) (string-prefix-p "sym:" a)) (intern (substring a 4)))
        ((stringp a) a)
        ((and (consp a) (assq 'c a)) (string-to-char (cdr (assq 'c a))))
        ((and (consp a) (assq 'l a)) (append (cdr (assq 'l a)) nil))
        (t a)))

(defun body (text) (let ((b (buffer-string))) (if (equal b text) nil b)))

(defun run-steps (steps seeds mark pos text)
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
                      ;; キーから動かしたときと同じに, 対話的に呼ぶ (前置引数は, 手順の引数)
                      (let ((current-prefix-arg (and (> (length args) 0) (aref args 0))))
                        (call-interactively (intern cmd)))
                      (unless (equal last "keep") nil)))
        (vector (body text) (1- (point)) (car kill-ring)
                (and (mark t) (1- (mark t)))
                (and (symbolp this-command) this-command (symbol-name this-command))
                (if mark-active t :json-false)
                (if deactivate-mark t :json-false)
                last-message))
    (error (vector "error" (error-message-string err) (body text) (1- (point))))))

;; 動かす前に, モードを決めてから本文を入れる
(defvar results nil)
(dolist (mode all-modes)
  (cl-loop for text in texts for index from 0
           do (dolist (case cases)
                (when (and (memq mode (nth 4 case)) (eq (nth 5 case) (text-kind index))
                           (not (and (nth 6 case) (string-match-p (nth 6 case) text))))
                  (let* ((steps (nth 1 case)) (mark (nth 3 case)) (per-pos nil))
                    (dotimes (i (1+ (length text)))
                      (with-temp-buffer
                        (funcall mode)
                        (setq indent-tabs-mode (not (nth 7 case)))
                        (insert text)
                        (push (run-steps steps [] mark (1+ i) text) per-pos)))
                    (push (list (format "%s:%s%s" mode (nth 0 case) (if (nth 7 case) "/notabs" "")) steps [] (or mark :json-false) text (vconcat (nreverse per-pos))) results))))))

(with-temp-file (expand-file-name (or (getenv "OUT") "lisp.json"))
  (insert (json-encode (vconcat (nreverse results)))))
