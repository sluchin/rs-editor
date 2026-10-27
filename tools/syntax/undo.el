;;; undo.el --- core/tests/undo.json を作る (emacs -Q --batch -l undo.el) -*- lexical-binding: t -*-
;; 編集と undo の手順を動かし, 動作ごとに本文とカーソルを書く.
;; 形: [[名前, 初めの本文, 初めのカーソル, 手順, [動作ごとの結果]], ...].
;;   手順: ["ins", 文字列] ["del", 始め, 終わり] ["goto", 位置] ["undo"] ["undo", n] ["undo-redo"]. 位置は 0 から数える.
;;   結果: [本文, カーソル] または ["error", メッセージ, 本文, カーソル]. 動作の前に, undo-boundary を入れる.

(require 'json)
(require 'cl-lib)

(defconst scenarios
  (list
   (list "basic" "hello" 5
         [["ins" " world"] ["del" 0 2] ["goto" 3] ["ins" "XYZ"] ["undo"] ["undo"] ["undo"] ["undo"] ["undo"]])
   (list "redo" "abc" 3
         [["ins" "d"] ["ins" "e"] ["undo"] ["undo"] ["undo-redo"] ["undo-redo"] ["undo-redo"]])
   (list "undo-n" "abcdef" 6
         [["ins" "1"] ["ins" "2"] ["ins" "3"] ["undo" 2] ["undo"] ["undo"] ["undo"]])
   (list "delete-middle" "0123456789" 4
         [["del" 2 5] ["goto" 0] ["del" 0 1] ["undo"] ["undo"] ["undo"]])
   (list "insert-away" "abc\ndef" 0
         [["ins" "X"] ["goto" 6] ["ins" "Y"] ["goto" 0] ["undo"] ["undo"] ["undo"]])
   (list "empty" "" 0
         [["undo"] ["ins" "a"] ["undo"] ["undo"] ["undo-redo"]])
   (list "undo-redo-undo" "xyz" 3
         [["ins" "a"] ["ins" "b"] ["undo"] ["undo"] ["undo-redo"] ["undo"] ["undo-redo"] ["undo-redo"] ["undo"]])
   (list "words" "one two three" 0
         [["del" 0 4] ["goto" 3] ["ins" "ZZ"] ["goto" 0] ["del" 0 2] ["undo" 2] ["undo"] ["undo"] ["undo"] ["undo"]])
   (list "undo-arg-redo" "ab" 2
         [["ins" "1"] ["ins" "2"] ["ins" "3"] ["undo" 2] ["undo-redo"] ["undo" 3] ["undo-redo"]])
   (list "interleaved" "line1\nline2" 11
         [["ins" "!"] ["goto" 0] ["ins" "#"] ["undo"] ["goto" 3] ["ins" "?"] ["undo"] ["undo"] ["undo"]])))

(defvar results nil)

(defun act (step)
  (pcase (aref step 0)
    ("ins" (insert (aref step 1)) nil)
    ("del" (delete-region (1+ (aref step 1)) (1+ (aref step 2))) nil)
    ("goto" (goto-char (1+ (aref step 1))) nil)
    ("undo" (if (> (length step) 1) (undo (aref step 1)) (undo)) t)
    ("undo-redo" (undo-redo) t)))

(dolist (sc scenarios)
  (let ((buf (generate-new-buffer "u")) (out nil))
    (with-current-buffer buf
      (buffer-enable-undo)
      (insert (nth 1 sc))
      (setq buffer-undo-list nil)
      (goto-char (1+ (nth 2 sc)))
      (setq last-command nil)
      (cl-loop for step across (nth 3 sc)
               do (undo-boundary)
                  (let ((undoing (member (aref step 0) '("undo" "undo-redo"))))
                    (setq this-command (and undoing (intern (aref step 0))))
                    (condition-case err
                        (progn (act step)
                               (push (vector (buffer-string) (1- (point))) out))
                      (error (push (vector "error" (error-message-string err) (buffer-string) (1- (point))) out)))
                    (setq last-command (and undoing this-command)))))
    (kill-buffer buf)
    (push (list (nth 0 sc) (nth 1 sc) (nth 2 sc) (nth 3 sc) (vconcat (nreverse out))) results)))

(with-temp-file (expand-file-name (or (getenv "OUT") "undo.json"))
  (insert (json-encode (vconcat (nreverse results)))))
