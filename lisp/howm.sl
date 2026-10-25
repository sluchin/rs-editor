;;; howm.sl --- howm (Hitori Otegaru Wiki Modoki) の Scheme 版 -*- scheme -*-

;; Emacs の howm (howm.el) の主な機能を, rs-editor の Scheme で書き直したもの.
;; 初期設定ファイル (~/.rseditor) で (load "howm.sl のパス") と読み込むと, M-x howm で使える.
;;
;; できること:
;; - メニュー (M-x howm, M-x howm-menu): ボタン ([新規] など) と, 予定・todo, 最近のメモ, ランダムなメモ.
;;   メニューのキー (c, s, g など) か, ボタンの上の RET で動かす. メニューは, メニューのファイル
;;   (howm-menu-file) から作る. ファイルが無ければ, 既定のメニューを書く ([menu 編集] で直せる).
;; - メモを作る (howm-create): howm-directory の下に, 2026/10/2026-10-25-184012.txt のような名前で作る.
;; - 一覧 (howm-list-all), 最近のメモ (howm-list-recent), 検索 (howm-search-fixed, howm-search-regexp),
;;   今日と昨日のメモ (howm-today, howm-yesterday), 予定 (howm-list-schedule), todo (howm-list-todo).
;;   一覧のバッファ (*howmS*) で, RET を押すと, その行のメモを開く.
;; - リンク: メモの中の ">>> 言葉" (goto link) と "<<< 言葉" (come-from link) の上で RET を押すと,
;;   その言葉を探す. come-from のある行を, 一覧の先に出す.
;; - 予定と todo の書き方は howm と同じ: [2026-10-25]@ 予定, [2026-10-25]+ todo, [2026-10-25]! 〆切,
;;   [2026-10-25]- 覚書, [2026-10-25]~ 保留, [2026-10-25]. 済み (記号の後の数は, 猶予の日数).
;;
;; howm.el から移していないもの: 下線 (action-lock の色付け), 一覧の窓 (contents の表示), 複製,
;; 速記, ローマ字検索 (migemo), 前後, 履歴, 酔歩, キーワードの補完, 日付の入力 ([日↓] など),
;; メニューの %eval% と %call%, 別のメニュー (%○○%).

;;; 設定 (初期設定ファイルで, (set! howm-directory "~/memo") のように変える)

;; メモを置くディレクトリ
(define howm-directory "~/howm")
;; メモのファイルの名前 (format-time-string の書式. howm-directory からのパス)
(define howm-file-name-format "%Y/%m/%Y-%m-%d-%H%M%S.txt")
;; メモのファイルの名前 (正規表現. これに合うファイルを, メモとして扱う)
(define howm-file-name-regexp "\\.txt$")
;; メモとして扱わないファイル (パスの正規表現. メニューのファイル, バックアップなど)
(define howm-excluded-file-regexp
  "(^|/)([.]|(menu(_edit)?|0000-00-00-0+))|[~#]$|\\.bak$|/CVS/")
;; メニューのファイル (無ければ, 既定のメニューを書く)
(define howm-menu-file #f)
;; メニューに出す, 予定の日数 (今日から), todo の数, 最近のメモの数, ランダムなメモの数
(define howm-menu-schedule-days 7)
(define howm-menu-todo-num 50)
(define howm-menu-recent-num 20)
(define howm-menu-random-num 10)
;; メモの題の行の始め
(define howm-view-title-header "=")
;; 日付の書式 (メモを作るときに入れる)
(define howm-dtime-format "[%Y-%m-%d %H:%M]")
;; メニューと一覧のバッファの名前
(define howm-menu-name "*howmM:%menu%*")
(define howm-summary-name "*howmS*")

;; 既定のメニュー (howm の ja/0000-00-00-000000.txt から, この版で使えるものを残した)
(define howm-default-menu
  "= <<< %menu%
 %\"c\"[新規] 検索(%\"s\"[固定] %\"g\"[正規]) %\":\"[昨日] %\".\"[今日]
 %\"a\"[一覧] %\"l\"[最近] %\"y\"[予定] %\"t\"[Todo] %\"R\"[menu 更新] [menu 編集]
-------------------------------------
[予定・Todo] … 予定@ を %sdays 日先まで表示.
〆切!, todo+, 覚書-, 保留~ を上位 %tnum 個まで表示.
%reminder
-------------------------------------
最近のメモ
%recent
-------------------------------------
ランダムセレクト (不要なら[menu 編集]で該当箇所を削除してください)
%random
-------------------------------------

予定・todo の書式 (実際は半角で)
[2002-10-21]＠1  予定 (予定表に表示。もし＠3なら3日間の予定、つまり23日までの意味)
[2002-10-21]＋7  todo 指定日になってから7日間かけて徐々に浮かび続ける
[2002-10-21]！7  〆切 指定日の7日前から徐々に浮かび、指定日以降浮きっぱなし
[2002-10-21]−1  覚書 指定日に浮かび上がり、以降1日かけて単位量だけ徐々に沈む
[2002-10-21]〜30 保留 指定日に浮かび上がり、以降30日周期で浮き沈みをくりかえす
[2002-10-21]．   済み 常に底
(記号の後の数字はデフォルト値)

メモの中の >>> 言葉 と <<< 言葉 の上で RET を押すと, その言葉を探します.
")

;; ボタンの名前と, コマンド
(define howm-menu-buttons
  '(("新規" howm-create)
    ("固定" howm-search-fixed)
    ("正規" howm-search-regexp)
    ("昨日" howm-yesterday)
    ("今日" howm-today)
    ("一覧" howm-list-all)
    ("最近" howm-list-recent)
    ("予定" howm-list-schedule)
    ("Todo" howm-list-todo)
    ("todo" howm-list-todo)
    ("更新" howm-menu-refresh)
    ("menu 更新" howm-menu-refresh)
    ("menu 編集" howm-menu-edit)))

;; 今のメニューと一覧の, 行ごとの項目 (行の番号と, (ファイル 行の番号))
(define howm-menu-items '())
(define howm-summary-items '())

;;; 文字列とリスト

;; 文字列の中の from を, 全て to にする
(define (howm-replace-all text from to)
  (let loop ((start 0) (acc '()))
    (let ((found (string-search from text start)))
      (if found
          (loop (+ found (string-length from))
                (cons to (cons (substring text start found) acc)))
          (apply string-append (reverse (cons (substring text start) acc)))))))

;; リストの始めの n 個
(define (howm-take items n)
  (if (or (null? items) (<= n 0))
      '()
      (cons (car items) (howm-take (cdr items) (- n 1)))))

;; 数の切り捨て
(define (howm-floor x)
  (- x (modulo x 1)))

;; 大文字を含まなければ, 大文字と小文字を区別しない正規表現にする (howm と同じ)
(define (howm-case-fold regexp keyword)
  (if (string=? keyword (string-downcase keyword))
      (string-append "(?i)" regexp)
      regexp))

;;; ファイル

;; メモのディレクトリ (絶対パス)
(define (howm-dir)
  (expand-file-name howm-directory))

;; メモのファイル (名前の順). 除くファイルは, howm-directory からのパスで決める
(define (howm-files)
  (let ((skip (+ (string-length (howm-dir)) 1)))
    (filter (lambda (file)
              (not (regexp-match howm-excluded-file-regexp (substring file skip))))
            (directory-files-recursively (howm-dir) howm-file-name-regexp))))

;; ファイルの名前 (ディレクトリと .txt を除く. 一覧に出す)
(define (howm-file-label file)
  (cadr (regexp-match "([^/]*?)(\\.txt)?$" file)))

;; ファイルのディレクトリ
(define (howm-file-directory file)
  (cadr (regexp-match "^(.*)/[^/]*$" file)))

;; メモの題 (最初の "= " の行. 無ければ空)
(define (howm-title file)
  (let ((found (regexp-match
                (string-append "(?m)^" (regexp-quote howm-view-title-header) " ?(.*)$")
                (read-file-string file))))
    (if found (string-trim (cadr found)) "")))

;; 題 (無ければ, ファイルの名前)
(define (howm-title-or-label file)
  (let ((title (howm-title file)))
    (if (string=? title "") (howm-file-label file) title)))

;; 新しい順 (更新の時刻)
(define (howm-recent-files)
  (map cadr
       (sort (map (lambda (file) (list (file-modification-time file) file)) (howm-files))
             (lambda (a b) (> (car a) (car b))))))

;; ランダムに n 個
(define (howm-random-files n)
  (let loop ((files (howm-files)) (n n) (acc '()))
    (if (or (null? files) (<= n 0))
        (reverse acc)
        (let ((picked (list-ref files (random (length files)))))
          (loop (filter (lambda (f) (not (equal? f picked))) files)
                (- n 1)
                (cons picked acc))))))

;;; バッファ

;; バッファの本文を, 全て置き換える
(define (howm-set-buffer-text text)
  (delete-region (point-min) (point-max))
  (insert text)
  (goto-char (point-min))
  (set-buffer-modified-p #f))

;; カーソルの行の番号 (1 から)
(define (howm-current-line)
  (length (split-string (buffer-substring (point-min) (point)) "\n")))

;; 行の始めの位置 (1 から)
(define (howm-line-start text line)
  (let loop ((lines (split-string text "\n")) (n 1) (pos 1))
    (if (or (null? lines) (>= n line))
        pos
        (loop (cdr lines) (+ n 1) (+ pos (string-length (car lines)) 1)))))

;; カーソルの行の文字と, 行の中の位置 (0 から)
(define (howm-current-line-and-column)
  (let* ((text (buffer-string))
         (start (howm-line-start text (howm-current-line)))
         (rest (substring text (- start 1)))
         (end (string-search "\n" rest)))
    (list (if end (substring rest 0 end) rest) (- (point) start))))

;; ファイルを開いて, 行へ動く
(define (howm-open file line)
  (find-file file)
  (goto-char (howm-line-start (buffer-string) line))
  (howm-mode))

;; メモのバッファのキー (リンクの上の RET)
(define (howm-mode)
  (local-set-key "RET" 'howm-action-or-newline))

;;; 一覧 (*howmS*)

;; 項目 ((ファイル 行の番号 文字) のリスト) を, 一覧のバッファに出す
(define (howm-show-summary items empty-message)
  (if (null? items)
      (message empty-message)
      (begin
        (switch-to-buffer howm-summary-name)
        (howm-set-buffer-text
         (string-join
          (map (lambda (item)
                 (format "%s | %s" (howm-file-label (car item)) (list-ref item 2)))
               items)
          "\n"))
        (set! howm-summary-items items)
        (local-set-key "RET" 'howm-action)
        (local-set-key "n" 'next-line)
        (local-set-key "p" 'previous-line)
        (message "%d items" (length items)))))

;; ファイルごとに, 題の行を項目にする
(define (howm-file-items files)
  (map (lambda (file) (list file 1 (howm-title-or-label file))) files))

;;; 検索

;; 正規表現で探して, come-from (<<< 言葉) の行を先に, あとは新しいファイルの順に並べる
(define (howm-search regexp keyword)
  (let* ((found (grep regexp (howm-files)))
         (come-from (string-append (howm-case-fold "<<<\\s*" keyword) (regexp-quote keyword)))
         (rank (lambda (item) (if (regexp-match come-from (list-ref item 2)) 0 1))))
    (howm-show-summary
     (sort found
           (lambda (a b)
             (or (< (rank a) (rank b))
                 (and (= (rank a) (rank b))
                      (or (string<? (car b) (car a))
                          (and (string=? (car a) (car b)) (< (cadr a) (cadr b))))))))
     (format "No match: %s" keyword))))

;; 言葉 (そのままの文字) で探す
(define (howm-search-keyword keyword)
  (howm-search (howm-case-fold (regexp-quote keyword) keyword) keyword))

(define-command (howm-search-fixed)
  (let ((keyword (read-string "Search: ")))
    (unless (string=? keyword "")
      (howm-search-keyword keyword))))

(define-command (howm-search-regexp)
  (let ((regexp (read-string "Search (regexp): ")))
    (unless (string=? regexp "")
      (howm-search (howm-case-fold regexp regexp) regexp))))

;; 今日から n 日後の日付の, メモ
(define (howm-search-day days)
  (howm-search-keyword
   (format-time-string "[%Y-%m-%d" (+ (current-time) (* days 24 60 60)))))

(define-command (howm-today) (howm-search-day 0))
(define-command (howm-yesterday) (howm-search-day -1))

(define-command (howm-list-all)
  (howm-show-summary (howm-file-items (reverse (howm-files))) "No memo"))

(define-command (howm-list-recent)
  (howm-show-summary (howm-file-items (howm-take (howm-recent-files) howm-menu-recent-num))
                     "No memo"))

;;; 予定と todo

;; 予定と todo の正規表現 ([2026-10-25]+7 の, 日付, 種類, 猶予, 残り)
(define howm-reminder-regexp
  "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\]([-+@!.~])([0-9]*)(.*)$")

;; 種類ごとの, 猶予の既定の日数
(define howm-reminder-laziness '(("-" 1) ("+" 7) ("~" 30) ("!" 7) ("@" 1) ("." 1)))

;; 底と天井 (howm の howm-huge)
(define howm-huge 77777)
(define howm-huge+ 88888)
(define howm-huge++ 99999)

;; 優先度 (howm の howm-todo-priority-*). late は, 日付から今までの日数
(define (howm-priority type late laziness)
  (let ((r (/ late laziness)))
    (cond ((string=? type "+")
           (if (< r 0) (- r howm-huge) (* 7 (- r 1))))
          ((string=? type "~")
           (if (< r 0) (- r howm-huge) (- (* 14 (* 2 (abs (- (modulo r 1) 0.5)))))))
          ((string=? type "!")
           (cond ((> r 0) (+ r howm-huge))
                 ((< r -1) (- r howm-huge))
                 (else (* 2 r))))
          ((string=? type "@")
           (if (> r 0) (- r howm-huge++) r))
          ((string=? type ".")
           (- late howm-huge+))
          (else
           (if (< r 0) (- r howm-huge) (- r))))))

;; 予定と todo の項目 ((優先度 日付 種類 ファイル 行の番号 文字) のリスト)
(define (howm-reminders)
  (let ((now (current-time)))
    (map (lambda (found)
           (let* ((parts (regexp-match howm-reminder-regexp (list-ref found 2)))
                  (date (list-ref parts 1))
                  (type (list-ref parts 2))
                  (laziness (if (string=? (list-ref parts 3) "")
                                (cadr (assoc type howm-reminder-laziness))
                                (max 1 (string->number (list-ref parts 3)))))
                  (late (/ (- now (date-to-time date)) (* 24 60 60))))
             (list (howm-priority type late laziness) date type
                   (car found) (cadr found) (car parts))))
         (grep howm-reminder-regexp (howm-files)))))

;; 項目を, 一覧の項目 ((ファイル 行の番号 文字)) にする
(define (howm-reminder-item reminder)
  (list (list-ref reminder 3) (list-ref reminder 4) (list-ref reminder 5)))

;; 予定 (@) を日付の順に. days が数なら, 今日からその日数先まで
(define (howm-schedule days)
  (let ((today (format-time-string "%Y-%m-%d"))
        (last (format-time-string "%Y-%m-%d" (+ (current-time) (* (or days 0) 24 60 60)))))
    (sort (filter (lambda (r)
                    (and (string=? (list-ref r 2) "@")
                         (or (not days)
                             (and (not (string<? (list-ref r 1) today))
                                  (not (string<? last (list-ref r 1)))))))
                  (howm-reminders))
          (lambda (a b) (string<? (list-ref a 1) (list-ref b 1))))))

;; todo (@ と . のほか) を優先度の順に (底に沈んだものは除く)
(define (howm-todo)
  (sort (filter (lambda (r)
                  (and (not (member (list-ref r 2) '("@" ".")))
                       (> (car r) (- howm-huge+))))
                (howm-reminders))
        (lambda (a b) (> (car a) (car b)))))

(define-command (howm-list-schedule)
  (howm-show-summary (map howm-reminder-item (howm-schedule #f)) "No schedule"))

(define-command (howm-list-todo)
  (howm-show-summary (map howm-reminder-item (howm-todo)) "No todo"))

;;; メモを作る

(define-command (howm-create)
  (let ((file (string-append (howm-dir) "/" (format-time-string howm-file-name-format))))
    (make-directory (howm-file-directory file))
    (find-file file)
    (insert howm-view-title-header " \n" (format-time-string howm-dtime-format) "\n\n")
    (goto-char (+ (string-length howm-view-title-header) 2))
    (howm-mode)))

;;; メニュー

;; メニューのファイル (無ければ, 既定のメニューを書く)
(define (howm-menu-path)
  (let ((file (or howm-menu-file (string-append (howm-dir) "/0000-00-00-000000.txt"))))
    (unless (file-exists? file)
      (write-file-string file howm-default-menu))
    (expand-file-name file)))

;; メニューの行 (%reminder などを, 項目の行にする). 行は, 文字か (文字 ファイル 行の番号)
(define (howm-menu-lines template)
  (apply append
         (map (lambda (line)
                (cond ((string=? line "%reminder")
                       (map (lambda (r)
                              (list (list-ref r 5) (list-ref r 3) (list-ref r 4)))
                            (append (howm-schedule howm-menu-schedule-days)
                                    (howm-take (howm-todo) howm-menu-todo-num))))
                      ((or (string=? line "%recent") (string=? line "%random"))
                       (map (lambda (file)
                              (list (format "%s | %s" (howm-file-label file) (howm-title-or-label file))
                                    file 1))
                            (if (string=? line "%recent")
                                (howm-take (howm-recent-files) howm-menu-recent-num)
                                (howm-random-files howm-menu-random-num))))
                      (else (list line))))
              (split-string template "\n"))))

;; メニューのキー (%"c"[新規] の c と, ボタンの名前)
(define (howm-menu-keys template)
  (let loop ((start 0) (acc '()))
    (let ((found (regexp-match "%\"([^\"]+)\"\\[([^\\]]+)\\]" template start)))
      (if found
          (loop (+ (string-search (car found) template start) (string-length (car found)))
                (cons (list (list-ref found 1) (list-ref found 2)) acc))
          (reverse acc)))))

;; ボタンのコマンド (使えないボタンは #f)
(define (howm-button-command label)
  (let ((found (assoc label howm-menu-buttons)))
    (and found (cadr found))))

;; メニューを出す
(define (howm-menu-show)
  (let* ((template (read-file-string (howm-menu-path)))
         (keys (howm-menu-keys template))
         (shown (howm-replace-all
                 (howm-replace-all
                  (howm-replace-all template "%sdays" (number->string howm-menu-schedule-days))
                  "%tnum" (number->string howm-menu-todo-num))
                 "%\"" ""))
         ;; %"c"[新規] は, c[新規] にする (閉じの " を消す)
         (lines (howm-menu-lines (howm-replace-all shown "\"[" "[")))
         (numbered (let loop ((lines lines) (n 1) (acc '()))
                     (if (null? lines)
                         (reverse acc)
                         (loop (cdr lines) (+ n 1) (cons (list n (car lines)) acc))))))
    (switch-to-buffer howm-menu-name)
    (howm-set-buffer-text
     (string-join (map (lambda (line) (if (string? line) line (car line))) lines) "\n"))
    (set! howm-menu-items
          (map (lambda (entry) (list (car entry) (list-ref (cadr entry) 1) (list-ref (cadr entry) 2)))
               (filter (lambda (entry) (pair? (cadr entry))) numbered)))
    (for-each (lambda (key)
                (let ((command (howm-button-command (cadr key))))
                  (when command (local-set-key (car key) command))))
              keys)
    (local-set-key "RET" 'howm-action)))

(define-command (howm-menu) (howm-menu-show))
(define-command (howm) (howm-menu-show))
(define-command (howm-menu-refresh) (howm-menu-show))
(define-command (howm-menu-edit)
  (find-file (howm-menu-path)))

;;; RET (ボタン, 項目, リンク)

;; 行の中の, 位置にある [ボタン] の名前 (無ければ #f)
(define (howm-button-at line column)
  (let loop ((start 0))
    (let ((open (string-search "[" line start)))
      (and open
           (let ((close (string-search "]" line open)))
             (and close
                  (if (and (<= open column) (<= column close))
                      (substring line (+ open 1) close)
                      (loop (+ close 1)))))))))

;; 行の中の, リンク (>>> 言葉, <<< 言葉) の言葉 (無ければ #f)
(define (howm-link-at line)
  (let ((found (regexp-match "(>>>|<<<)\\s*(.+?)\\s*$" line)))
    (and found (list-ref found 2))))

;; カーソルの所の, ボタンかリンクを動かす. 動かしたら #t
(define (howm-action-at-point)
  (let* ((here (howm-current-line-and-column))
         (button (howm-button-at (car here) (cadr here)))
         (link (howm-link-at (car here))))
    (cond ((and button (howm-button-command button))
           (call-interactively (howm-button-command button))
           #t)
          (link (howm-search-keyword link) #t)
          (else #f))))

;; 一覧かメニューの項目を開く. 開いたら #t
(define (howm-open-item items)
  (let ((item (assoc (howm-current-line) items)))
    (and item
         (begin (howm-open (cadr item) (list-ref item 2)) #t))))

(define-command (howm-action)
  (cond ((string=? (buffer-name) howm-summary-name)
         (let ((n (howm-current-line)))
           (when (<= n (length howm-summary-items))
             (let ((item (list-ref howm-summary-items (- n 1))))
               (howm-open (car item) (cadr item))))))
        ((howm-action-at-point) #t)
        ((and (string=? (buffer-name) howm-menu-name) (howm-open-item howm-menu-items)) #t)
        (else (message "No action here"))))

(define-command (howm-action-or-newline)
  (unless (howm-action-at-point)
    (call-interactively 'newline)))

;;; キー (howm と同じく C-c , に続けて押す)

(global-set-key "C-c , ," 'howm-menu)
(global-set-key "C-c , c" 'howm-create)
(global-set-key "C-c , a" 'howm-list-all)
(global-set-key "C-c , l" 'howm-list-recent)
(global-set-key "C-c , s" 'howm-search-fixed)
(global-set-key "C-c , g" 'howm-search-regexp)
(global-set-key "C-c , y" 'howm-list-schedule)
(global-set-key "C-c , t" 'howm-list-todo)
(global-set-key "C-c , ." 'howm-today)
(global-set-key "C-c , :" 'howm-yesterday)
(global-set-key "C-c , RET" 'howm-action)

;;; howm.sl ends here
