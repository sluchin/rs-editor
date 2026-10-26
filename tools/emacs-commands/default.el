;;; default.el --- 初期設定で読む機能を, 設定をせずに読む -*- lexical-binding: t -*-

;; 環境変数 DUMP_DIR の custom-features.el (初期設定を読んだあとの load-path と機能) を読み,
;; 同じ機能を require する. 初期設定のキーの割り当ての前 (デフォルト) を, 同じ機能で比べるため.

(let* ((data (with-temp-buffer
               (insert-file-contents (expand-file-name "custom-features.el" (getenv "DUMP_DIR")))
               (read (current-buffer))))
       (n 0) (failed 0))
  (setq load-path (car data))
  (dolist (f (reverse (cadr data)))
    (unless (featurep f)
      (condition-case nil
          (let ((inhibit-message t)) (require f) (setq n (1+ n)))
        ((error quit) (setq failed (1+ failed))))))
  (princ (format "default: required=%d failed=%d\n" n failed)))

;;; default.el ends here
