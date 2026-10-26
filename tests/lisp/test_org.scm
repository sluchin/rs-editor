;;; test_org.scm --- org.scm のテスト (ERT) -*- scheme -*-

;; Emacs の ERT と同じ書き方で書いた, lisp/org.scm のテスト.
;;
;; 実行のしかた:
;; - エディタで: lisp/ を load-path に入れて, このファイルを読み (M-: (load "…/tests/lisp/test_org.scm")),
;;   M-x ert (RET で全て. 名前の正規表現で選べる). 結果は *ert* のバッファに出る.
;; - コマンドラインで: cd src-tauri && cargo test --test lisp_ert_test -- --nocapture
;;   (偽のエディタの上で, tests/lisp/*.scm を全て実行する. Emacs の ert-run-tests-batch-and-exit に当たる)
;;
;; バッファを変えるコマンドと, 入力を聞くもの (read-string) は, ここでは試さない
;; (src-tauri/tests/org_test.rs で試す).

(require 'org)

;;; 文字列とリスト

(ert-deftest org-test-strings-and-lists ()
  (should (equal (org-index-of "b" '("a" "b")) 1))
  (should-not (org-index-of "z" '("a" "b")))
  (should (equal (org-unique '(1 2 1 3 2)) '(1 2 3)))
  (should (equal (org-pad-right "ab" 4) "ab  "))
  (should (equal (org-pad-left "ab" 4) "  ab"))
  ;; 全角は 2 桁
  (should (equal (org-pad-right "日本" 6) "日本  "))
  (should (equal (org-2d 7) "07"))
  (should (equal (org-replace-all "a-b-c" "-" "+") "a+b+c")))

;;; 見出し

(ert-deftest org-test-headings ()
  (should (= (org-level "** a") 2))
  (should (= (org-level "*") 1))
  (should-not (org-level "*a"))
  (should-not (org-level " * a"))
  ;; (星 状態 優先度の印 題 タグ)
  (should (equal (org-parse-heading "** TODO [#A] Title :a:b:")
                 '("**" "TODO" "[#A]" "Title" ":a:b:")))
  (should (equal (org-parse-heading "* Title") '("*" #f #f "Title" #f)))
  (should (equal (org-heading-tags (org-parse-heading "* x :a:b:")) '("a" "b")))
  (should (equal (org-tags-string '("a" "b")) ":a:b:"))
  ;; タグは 77 桁目で終わるように揃える
  (should (equal (string-length (org-make-heading "*" "TODO" #f "x" ":t:")) 77))
  (should (equal (org-make-heading "*" #f "[#B]" "x" #f) "* [#B] x")))

(ert-deftest org-test-todo-keywords ()
  (should (equal (org-todo-sets) '((("TODO") ("DONE")))))
  (let ((saved org-todo-keywords))
    (set! org-todo-keywords '((sequence "TODO(t)" "WAIT(w@/!)" "|" "DONE(d!)" "CANCELED(c@)")))
    (should (equal (org-todo-sets) '((("TODO" "WAIT") ("DONE" "CANCELED")))))
    (should (equal (org-todo-keywords-1) '("TODO" "WAIT" "DONE" "CANCELED")))
    (should (equal (org-done-keywords) '("DONE" "CANCELED")))
    ;; 回す (最後の次は, 状態無し)
    (should (equal (org-next-todo #f #t #f) "TODO"))
    (should (equal (org-next-todo "WAIT" #t #f) "DONE"))
    (should-not (org-next-todo "CANCELED" #t #f))
    (should (equal (org-next-todo #f #f #f) "CANCELED"))
    (set! org-todo-keywords saved)))

;;; 項目

(ert-deftest org-test-items ()
  ;; (字下げ 印 箱 文)
  (should (equal (org-parse-item "  - [X] done") '("  " "-" "[X]" "done")))
  (should (equal (org-parse-item "1. one") '("" "1." #f "one")))
  (should (equal (org-parse-item "- ") '("" "-" #f "")))
  (should-not (org-parse-item "-a"))
  (should-not (org-parse-item "* heading")))

;;; 日付

(ert-deftest org-test-dates ()
  (should (equal (org-date-add "2026-12-31" 1) "2027-01-01"))
  (should (equal (org-date-add "2026-03-01" -1) "2026-02-28"))
  (should (equal (org-weekday "2026-10-26") "Mon"))
  (should (= (org-days-between "2026-10-26" "2026-11-02") 7))
  ;; 月を足して, 日が無ければ, 次の月へあふれる (Emacs の encode-time と同じ)
  (should (equal (org-date-add-months "2026-01-31" 1) "2026-03-03"))
  (should (equal (org-date-add-months "2026-11-15" 2) "2027-01-15"))
  (should (equal (org-make-timestamp "2026-10-30" #f #t) "<2026-10-30 Fri>"))
  (should (equal (org-make-timestamp "2026-10-30" "09:00" #f) "[2026-10-30 Fri 09:00]"))
  (should (equal (org-make-timestamp "2026-10-30" #f #t " +1w") "<2026-10-30 Fri +1w>")))

(ert-deftest org-test-timestamps ()
  ;; (括弧 日付 時刻 終わりの時刻 繰り返しと警告)
  (let ((parsed (org-parse-timestamp "<2026-10-30 Fri 09:00-10:30 +1w -2d>")))
    (should (equal (org-take parsed 4) '("<" "2026-10-30" "09:00" "10:30")))
    (should (equal (org-timestamp-repeater parsed) '("+" 1 "w")))
    (should (= (org-timestamp-warning parsed) 2)))
  (should (equal (org-timestamp-repeater (org-parse-timestamp "<2026-10-30 Fri .+2d>")) '(".+" 2 "d")))
  (should-not (org-timestamp-warning (org-parse-timestamp "[2026-10-30 Fri]")))
  ;; 括弧が合わなければ, 日付でない
  (should-not (org-parse-timestamp "<2026-10-30 Fri]"))
  ;; 時刻を変える (日を越える. 範囲の終わりも動く)
  (should (equal (org-timestamp-string
                  (org-timestamp-change (org-parse-timestamp "<2026-10-30 Fri 23:30-23:45>") "hour" 1))
                 "<2026-10-31 Sat 00:30-00:45>"))
  (should (equal (org-timestamp-string
                  (org-timestamp-change (org-parse-timestamp "<2026-10-30 Fri>") "month" -1))
                 "<2026-09-30 Wed>")))

(ert-deftest org-test-planning ()
  (should (org-planning-line? "DEADLINE: <2026-10-30 Fri> SCHEDULED: <2026-10-29 Thu>"))
  (should-not (org-planning-line? "text"))
  (should (equal (org-planning-items "  CLOSED: [2026-10-30 Fri 10:00] DEADLINE: <2026-10-30 Fri>")
                 '(("CLOSED" "[2026-10-30 Fri 10:00]") ("DEADLINE" "<2026-10-30 Fri>")))))

;;; 表

(ert-deftest org-test-tables ()
  (should (equal (org-table-cells "| a | b  |c|") '("a" "b" "c")))
  (should (org-table-number? "-1.5"))
  (should (org-table-number? "10%"))
  (should-not (org-table-number? "a1"))
  ;; 数の多い列は右に揃える. 横線は列の幅に
  (should (equal (org-table-render "" '(("a" "1") hline ("bb" "22")))
                 '("| a  |  1 |" "|----+----|" "| bb | 22 |"))))

;;; リンク

(ert-deftest org-test-links ()
  (should (equal (org-make-link-string "https://a" #f) "[[https://a]]"))
  (should (equal (org-make-link-string "file:x" "X") "[[file:x][X]]"))
  ;; 題の [1/2] と [50%] を除く
  (should (equal (org-strip-cookies "Head [1/2] [50%]") "Head")))

;;; タグの検索

(ert-deftest org-test-tags-match ()
  (should (org-tags-match? "a+b" '("a" "b")))
  (should-not (org-tags-match? "a+b" '("a")))
  (should (org-tags-match? "a|c" '("c")))
  (should (org-tags-match? "a-b" '("a")))
  (should-not (org-tags-match? "a-b" '("a" "b")))
  (should (org-tags-match? "-b" '())))

;;; test_org.scm ends here
