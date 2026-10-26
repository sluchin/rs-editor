;;; load-init.el --- 初期設定ファイルを, フォームごとに評価する -*- lexical-binding: t -*-

;; 環境変数 INIT_FILE (既定は ~/.emacs.el) を読み, 1 つずつ評価する. 失敗したフォームは, 飛ばして続ける.
;; 評価したフォームと失敗の数を出す. --batch なので, 窓 (window-system) を使う設定は, 動かない.

(defvar load-init-errors nil)
(defvar load-init-forms 0)

(let* ((file (expand-file-name (or (getenv "INIT_FILE") "~/.emacs.el")))
       (src (find-file-noselect file)))
  (setq user-init-file file)
  (with-current-buffer src (goto-char (point-min)))
  (condition-case nil
      (while t
        ;; 評価で今のバッファが変わっても, 続きを読めるように, バッファから読む
        (let ((form (read src)))
          (setq load-init-forms (1+ load-init-forms))
          (condition-case err
              (save-current-buffer
                (let ((inhibit-message t) (debug-on-error nil)) (eval form t)))
            ((error quit) (push (list (car-safe form) err) load-init-errors)))))
    (end-of-file nil)))
(setq debug-on-error nil)
(princ (format "init: forms=%d errors=%d\n" load-init-forms (length load-init-errors)))

;;; load-init.el ends here
