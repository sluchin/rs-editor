;;; test_howm.scm --- howm.scm のテスト (ERT) -*- scheme -*-

;; Emacs の ERT と同じ書き方で書いた, lisp/howm.scm のテスト.
;;
;; 実行のしかた:
;; - エディタで: lisp/ を load-path に入れて, このファイルを読み (M-: (load "…/tests/test_howm.scm")),
;;   M-x ert (RET で全て. 名前の正規表現で選べる). 結果は *ert* のバッファに出る.
;;   メモのバッファなどを作るので, 試すときは, 保存していないバッファの無いときに.
;; - コマンドラインで: cd src-tauri && cargo test --test lisp_ert_test -- --nocapture
;;   (偽のエディタの上で, tests/*.scm を全て実行する. Emacs の ert-run-tests-batch-and-exit に当たる)
;;
;; 入力を聞くもの (read-string) は, ここでは試さない (src-tauri/tests/howm_test.rs で試す).

(require 'howm)

;;; 準備

;; メモ ((名前 中身) のリスト) を書いた一時ディレクトリを howm-directory にして, thunk を呼ぶ.
;; 終わったら (失敗しても), ディレクトリを消し, 設定を戻す
(define (howm-test-with-memos memos thunk)
  (let ((dir (make-temp-file "howm-test-" #t))
        (saved howm-directory))
    (set! howm-directory dir)
    (set! howm-keyword-cache #f)
    (set! howm-history-list #f)
    (for-each (lambda (memo)
                (write-file-string (string-append dir "/" (car memo)) (cadr memo)))
              memos)
    (unwind-protect
        (thunk dir)
      (set! howm-directory saved)
      (set! howm-keyword-cache #f)
      (set! howm-history-list #f)
      (delete-directory dir #t))))

;; 今日から n 日後の日付
(define (howm-test-day n)
  (howm-date-shift (howm-today-date) n))

;;; 文字列とリスト

(ert-deftest howm-test-strings-and-lists ()
  (should (equal (howm-replace-all "a-b-c" "-" "+") "a+b+c"))
  (should (equal (howm-replace-all "abc" "x" "y") "abc"))
  (should (equal (howm-take '(1 2 3) 2) '(1 2)))
  (should (equal (howm-take '(1) 5) '(1)))
  (should (equal (howm-drop '(1 2 3) 2) '(3)))
  (should (equal (howm-unique '(1 2 1 3 2)) '(1 2 3)))
  (should (equal (howm-range 3) '(0 1 2)))
  (should (equal (howm-index-of "b" '("a" "b")) 1))
  (should-not (howm-index-of "z" '("a" "b")))
  (should (equal (howm-pad2 7) "07"))
  (should (equal (howm-trim-trailing-lines '("a" "" "")) '("a")))
  ;; 小文字だけなら, 大文字と小文字を区別しない
  (should (equal (howm-case-fold "emacs" "emacs") "(?i)emacs"))
  (should (equal (howm-case-fold "Emacs" "Emacs") "Emacs")))

(ert-deftest howm-test-matches ()
  ;; 合った所と括弧の位置 (文字の数). 合わなかった括弧は #f
  (should (equal (howm-matches "あ(い)?" "xあいあ") '(((1 3) (2 3)) ((3 4) #f))))
  (should (equal (howm-matches "z" "abc") '()))
  (should (equal (howm-replace-matches "[0-9]+" "a1b22" (lambda (line found) "#")) "a#b#")))

;;; 日付

(ert-deftest howm-test-dates ()
  (should (equal (howm-date-shift "2026-12-31" 1) "2027-01-01"))
  (should (equal (howm-date-shift "2026-03-01" -1) "2026-02-28"))
  (should (equal (howm-day-of-week "2026-10-25") "日"))
  (should (equal (howm-day-of-week "2026-10-26") "月"))
  (should (= (howm-days-in-month 2024 2) 29))
  (should (= (howm-days-in-month 1900 2) 28))
  (should (= (howm-days-in-month 2000 2) 29))
  (should (= (howm-days-in-month 2026 4) 30))
  ;; 月を足すと, 日はその月の終わりまでにする
  (should (equal (howm-date-add-months "2027-01-31" 1) "2027-02-28"))
  (should (equal (howm-date-add-months "2026-11-15" 2) "2027-01-15"))
  ;; 数字の日付 (省いた所は, 元の日付の年, 月, 日)
  (should (equal (howm-date-expand "5" "2026-10-25") "2026-10-05"))
  (should (equal (howm-date-expand "1101" "2026-10-25") "2026-11-01"))
  (should (equal (howm-date-expand "270102" "2026-10-25") "2027-01-02"))
  (should (equal (howm-date-expand "20280229" "2026-10-25") "2028-02-29")))

;;; ローマ字検索

(ert-deftest howm-test-roma ()
  (should (equal (howm-roma-to-hiragana "kitte") "きって"))
  (should (equal (howm-roma-to-hiragana "konnnichiha") "こんにちは"))
  (should (equal (howm-roma-to-hiragana "shinbun") "しんぶん"))
  (should (equal (howm-roma-to-hiragana "ra-men") "らーめん"))
  (should (equal (howm-hiragana-to-katakana "らーめん") "ラーメン"))
  (should (equal (howm-migemo-regexp "ramen") "(?i)(?:ramen|らめん|ラメン)"))
  ;; かなにならないものは, 1 つだけ
  (should (equal (howm-migemo-regexp "123") "(?i)(?:123)")))

;;; 予定と todo の優先度

(ert-deftest howm-test-priority ()
  ;; 〆切は, 日付を過ぎると一番上. 済みは, いつも一番下
  (should (> (howm-priority "!" 1 7) howm-huge))
  (should (< (howm-priority "." 0 1) (- howm-huge)))
  ;; todo は, 日付の前は沈んでいて, 後は浮かぶ
  (should (< (howm-priority "+" -1 7) (- howm-huge)))
  (should (< (howm-priority "+" 0 7) (howm-priority "+" 14 7)))
  ;; 覚書は, 日付の後, 沈む
  (should (> (howm-priority "-" 0 1) (howm-priority "-" 3 1)))
  ;; 予定は, 過ぎると一番下
  (should (< (howm-priority "@" 1 1) (- howm-huge))))

;;; メモの区切り

(ert-deftest howm-test-sections ()
  (let ((lines '("前書き" "= A" "本文A" "= B" "本文B")))
    (should (equal (howm-section-range lines 1) '(0 1)))
    (should (equal (howm-section-range lines 3) '(1 3)))
    (should (equal (howm-section-range lines 5) '(3 5)))
    (should (equal (howm-lines-in lines '(3 5)) '("= B" "本文B")))
    (should (howm-title-line? "= A"))
    (should (howm-title-line? "="))
    (should-not (howm-title-line? "==A"))
    (should (equal (howm-title-of "=  題  ") "題"))))

;;; RET で動くもの

(ert-deftest howm-test-line-actions ()
  (howm-test-with-memos
   '(("k.txt" "<<< Emacs\n"))
   (lambda (dir)
     (should (equal (howm-action-at ">>> foo bar " 0 #f) '("search" "foo bar")))
     ;; 行の終わりは, リンクの外
     (should-not (howm-action-at ">>> foo" 7 #f))
     (should (equal (howm-action-at ">>> %menu%" 2 #f) '("menu" "%menu%")))
     (should (equal (howm-action-at "<<< foo" 1 #f) '("search" "foo")))
     (should (equal (howm-action-at "x [[言葉]]" 3 #f) '("wiki" "言葉")))
     (should (equal (howm-action-at "file:///tmp/a" 0 #f) '("file" "/tmp/a")))
     ;; 予定と todo: 日付の上は日付, 記号の上は予定と todo
     (should (equal (howm-action-at "- [2026-10-25]+7 買う" 3 #f) '("date" "2026-10-25" 2 14)))
     (should (equal (howm-action-at "- [2026-10-25]+7 買う" 15 #f) '("reminder" 2)))
     ;; キーワード ("<<< 言葉" の言葉) は, どこにあってもリンク
     (should (equal (howm-action-at "いつか Emacs を" 4 #f) '("search" "Emacs")))
     (should-not (howm-action-at "いつか emacs を" 4 #f))
     ;; ボタン, %eval%, %call% は, メニューだけ
     (should (equal (howm-action-at "[一覧]" 1 #t) '("button" "一覧")))
     (should-not (howm-action-at "[一覧]" 1 #f))
     (should-not (howm-action-at "[知らない]" 1 #t))
     (should (equal (howm-action-at "x %eval%(+ 1 2)" 3 #t) '("eval" "(+ 1 2)")))
     (should (equal (howm-action-at "%call%howm-list-all" 0 #t) '("call" "howm-list-all"))))))

;;; メニュー

(ert-deftest howm-test-menu-text ()
  ;; %| で, 見せない所. 行の終わりが見えなければ, 改行も消す
  (should (equal (howm-menu-visible "a%|b%|c") '("ac" #f)))
  (should (equal (howm-menu-visible "a%|b") '("a" #t)))
  ;; ショートカット
  (should (equal (howm-menu-strip-keys "%\"c\"[新規] %\"jump%\"") "c[新規] jump"))
  (should (equal (howm-menu-line-shortcuts "%\"c\"[新規] %\"jump%\" %\"e\"x %eval%(foo)")
                 '(("c" ("button" "新規")) ("j" ("move" "jump")) ("e" ("eval" "(foo)")))))
  ;; %here% (許した手続きと変数だけ)
  (should (equal (howm-menu-expand "数: %here%howm-menu-todo-num 個") '("数: 50 個")))
  (should (equal (howm-menu-expand "%here%(car x)") '("Not allowed: (car x)")))
  (should (equal (howm-menu-render "%\"x\"[一覧] %sdays\n見える%|見えない\n次")
                 '(("x[一覧] 7" "見える次") (("x" ("button" "一覧")))))))

(ert-deftest howm-test-menu-sections ()
  (let ((text "= <<< %menu%\nmain\n= <<< %sub%\nsub\n"))
    (should (equal (howm-section-of text "%menu%") "= <<< %menu%\nmain"))
    (should (equal (howm-section-of text "%sub%") "= <<< %sub%\nsub\n"))
    (should-not (howm-section-of text "%none%"))))

(ert-deftest howm-test-menu-buffer ()
  (howm-test-with-memos
   '(("menu.txt" "= <<< %menu%\n%\"x\"[一覧] %\"!\"呼ぶ%| %eval%(message \"hi\")%|\n%recent\n")
     ("a.txt" "= メモA\n"))
   (lambda (dir)
     (set! howm-menu-file (string-append dir "/menu.txt"))
     (unwind-protect
         (begin
           (howm-menu)
           (should (equal (buffer-name) "*howmM:%menu%*"))
           (should (equal (buffer-string) "= <<< %menu%\nx[一覧] !呼ぶ\na | メモA\n"))
           (should-not (buffer-modified-p))
           ;; キーで, %| の中の %eval% を動かす
           (howm-menu-key-!)
           (should (equal (howm-menu-items) '()))
           (howm-menu)
           (should (equal (howm-menu-items) (list (list 3 (string-append dir "/a.txt") 1)))))
       (set! howm-menu-file #f)))))

;;; ファイルと検索

(ert-deftest howm-test-files ()
  (howm-test-with-memos
   '(("2026/10/2026-10-01-100000.txt" "= 日記\nemacs を使う\n>>> Emacs\n")
     ("2026/09/2026-09-01-100000.txt" "= Emacs\n<<< Emacs\n")
     ("2026/10/2026-10-01-100000.txt~" "バックアップ")
     ("0000-00-00-000000.txt" "メニュー"))
   (lambda (dir)
     ;; バックアップとメニューは, メモではない
     (should (equal (map howm-file-label (howm-files)) '("2026-09-01-100000" "2026-10-01-100000")))
     (should (equal (howm-title (car (howm-files))) "Emacs"))
     (should (equal (howm-keywords) '("Emacs")))
     (should (howm-memo-file? (car (howm-files))))
     (should-not (howm-memo-file? "/elsewhere/a.txt"))
     ;; come-from の行を先に
     (howm-search-keyword "emacs")
     (should (equal (buffer-name) howm-summary-name))
     (should (equal (buffer-string)
                    (string-append "2026-09-01-100000 | <<< Emacs\n"
                                   "2026-10-01-100000 | emacs を使う\n"
                                   "2026-10-01-100000 | >>> Emacs\n"
                                   "2026-09-01-100000 | = Emacs")))
     (should (= (length howm-summary-items) 4))
     ;; 履歴はファイルにも書く
     (howm-history-add "a")
     (howm-history-add "b")
     (howm-history-add "a")
     (should (equal (howm-history-load) '("a" "b")))
     (should (equal (read-file-string (string-append dir "/.howm-history")) "a\nb")))))

(ert-deftest howm-test-reminders ()
  (howm-test-with-memos
   (list (list "r.txt"
               (string-append "[" (howm-test-day 2) "]@ 会議\n"
                              "[" (howm-test-day 30) "]@ 遠い予定\n"
                              "[" (howm-test-day -7) "]+ 買い物\n"
                              "[" (howm-test-day 1) "]! 〆切\n"
                              "[" (howm-test-day -1) "]. 済み\n")))
   (lambda (dir)
     (should (equal (map (lambda (r) (list-ref r 5)) (howm-schedule 7))
                    (list (string-append "[" (howm-test-day 2) "]@ 会議"))))
     (should (= (length (howm-schedule #f)) 2))
     (should (equal (map (lambda (r) (list-ref r 2)) (howm-todo)) '("+" "!"))))))

;;; メモを作る

(ert-deftest howm-test-create-and-dup ()
  (howm-test-with-memos
   '(("m.txt" "= A\n本文A\n= B\n本文B\n"))
   (lambda (dir)
     (howm-create-memo "題")
     (should (string-prefix? (string-append dir "/") (buffer-file-name)))
     (should (string-prefix? "= 題\n[" (buffer-string)))
     ;; 複製: カーソルのメモ (題の行から, 次の題の行の前まで)
     (howm-open (string-append dir "/m.txt") 4)
     (should (equal (howm-current-title) "B"))
     (howm-dup)
     (should (equal (read-file-string (buffer-file-name)) "= B\n本文B\n"))
     (should-not (equal (buffer-file-name) (string-append dir "/m.txt"))))))

(ert-deftest howm-test-remember ()
  (howm-test-with-memos
   '()
   (lambda (dir)
     (switch-to-buffer "*scratch*")
     (howm-remember)
     (should (equal (buffer-name) howm-remember-name))
     (insert "題名\n本文\n")
     (howm-remember-submit)
     (should (equal (buffer-name) "*scratch*"))
     (should-not (member howm-remember-name (buffer-list)))
     (let ((memo (car (howm-files))))
       (should (string-prefix? "= 題名\n[" (read-file-string memo)))
       (should (string-suffix? "]\n\n本文\n" (read-file-string memo)))))))

;;; test_howm.scm ends here
