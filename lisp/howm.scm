;;; howm.scm --- howm (Hitori Otegaru Wiki Modoki) の Scheme 版 -*- scheme -*-

;; Emacs の howm (howm.el) を, rs-editor の Scheme で書き直したもの.
;;
;; 使い方: このファイルを ~/.rseditor.d/lisp/howm.scm に置き, 初期設定ファイル
;; (~/.rseditor.d/init.scm) に (require 'howm) と書く. M-x howm か C-c , , でメニューを出す.
;; C-x C-f で開いたメモも howm のバッファにし, 保存したらメニューを出し直す (find-file-hook, after-save-hook).
;; メモのディレクトリは, (set! howm-directory "~/memo") のように変える (既定は ~/howm).
;;
;; できること (キーは C-c , に続けて押す. メニューでは, " の前の文字のキーだけで動く):
;; - メニュー (,): ボタン ([新規] など) と, 予定・todo, 最近のメモ, ランダムなメモ.
;;   メニューは, メニューのファイル (howm-menu-file) から作る. 無ければ, 既定のメニューを書く.
;;   %"c"[新規] (キー), %eval%(式), %call%コマンド, %here%(手続き), %| (見せない所),
;;   %reminder, %recent, %random, %schedule, %todo, %last-update, >>> %名前% (別のメニュー) が使える.
;; - 作る: 新規 (c), 速記 (e. C-c C-c で保存, C-c C-k で取り消し), 複製 (D).
;; - 探す: 固定 (s. キーワードを補完する), 正規 (g), ローマ字 (m. ひらがなとカタカナも探す),
;;   バッファの中 (o), 履歴 (h), 今日 (.), 昨日 (:).
;; - 一覧: 全て (a), 最近 (l), 前後 (A), 予定 (y), todo (t), 酔歩 (w. 1 回に 1 歩).
;;   一覧のバッファ (*howmS*) で n と p を押すと, その行のメモを, もう 1 つの窓に出す. RET で開く.
;; - バッファ: 一覧 (b), 下線の引き直し (r), howm のバッファを全て消す (Q. 書き換えたものは残す).
;; - コピー: メモの題 (K). 入れる: 日付 (d), 日時 (T), キーワード (i).
;; - メモの中の RET: ">>> 言葉" と "<<< 言葉" と "[[言葉]]" と, "<<< 言葉" の言葉 (どこにあっても),
;;   日付 ([2026-10-25]. 一覧, ずらす, 変える, 繰り返す), 予定・todo の記号 (済み, 取り消し, 種類),
;;   file://パス. それぞれに下線を引く.
;; - 予定と todo の書き方は howm と同じ: [2026-10-25]@ 予定, [2026-10-25]+ todo, [2026-10-25]! 〆切,
;;   [2026-10-25]- 覚書, [2026-10-25]~ 保留, [2026-10-25]. 済み (記号の後の数は, 猶予の日数).
;;
;; howm.el から移していないもの: [mark] (mark ring の一覧), 自動で続ける酔歩 (タイマー),
;; メニューの分類した予定 (howm-menu-categorized-reminder), 色 (下線だけ), 一覧の並べ替えと絞り込み.

;;; 設定 (初期設定ファイルで, (set! howm-directory "~/memo") のように変える)

(define howm-directory "~/howm" "Directory where memos are stored.")
(define howm-file-name-format "%Y/%m/%Y-%m-%d-%H%M%S.txt"
  "Format of a new memo file name, relative to `howm-directory'.
It is passed to `format-time-string'.")
(define howm-file-name-regexp "\\.txt$" "Regexp of the file names treated as memos.")
(define howm-excluded-file-regexp
  "(^|/)([.]|(menu(_edit)?|0000-00-00-0+))|[~#]$|\\.bak$|/CVS/"
  "Regexp of the paths not treated as memos.
The menu file, the history, backups and so on match it.")
(define howm-menu-file #f
  "Menu file, or #f to use the default one in `howm-directory'.
The default menu is written if the file does not exist.")
(define howm-menu-schedule-days 7
  "Number of days of the schedule shown in the menu, counted from today.")
(define howm-menu-todo-num 50 "Number of todo items shown in the menu.")
(define howm-menu-recent-num 20 "Number of recent memos shown in the menu.")
(define howm-menu-random-num 10 "Number of random memos shown in the menu.")
(define howm-menu-allow
  '(howm-menu-legend howm-menu-schedule howm-menu-todo howm-menu-reminder
    howm-menu-recent howm-menu-random howm-menu-search howm-menu-last-update)
  "Procedures that %here% in the menu may call.")
(define howm-view-title-header "=" "String that starts the title line of a memo.")
(define howm-date-format "[%Y-%m-%d]" "Format of a date inserted into a memo.")
(define howm-dtime-format "[%Y-%m-%d %H:%M]" "Format of a date and time inserted into a memo.")
(define howm-view-contents #t
  "If true, show the memo of the current line in the other window.
This is used when moving in the summary buffer.")
(define howm-history-file #f
  "File of the search history.
If #f, use .howm-history in `howm-directory'.")
(define howm-history-limit 50 "Number of keywords kept in the search history.")
(define howm-date-repeat-max 200 "Maximum number of lines added when repeating a date (~yymmdd).")
(define howm-menu-name "*howmM:%menu%*" "Name of the menu buffer.")
(define howm-summary-name "*howmS*" "Name of the summary buffer.")
(define howm-remember-name "*howm-remember*"
  "Name of the buffer for quick notes (`howm-remember').")
(define howm-set-mode-on-find-file #t
  "If true, turn on `howm-mode' for memos opened by \\[find-file].
If #f, only memos opened from howm use it.")
(define howm-refresh-after-save #t "If true, underline the memo again after saving it.")
(define howm-menu-refresh-after-save #t "If true, refresh the menu after saving a memo.")
(define howm-after-save-hook '() "Hook run after saving a memo.")

(define howm-default-menu
  "= <<< %menu%
%last-update %\"R\"[menu 更新]

 作成:     %\"e\"[速記]      %\"c\"[新規] %\"D\"[複製]
 検索:     %\"s\"[固定]      %\"g\"[正規] %\"m\"[roma] %\"o\"[バ内] %\"h\"[履歴]
 一覧:     %\"a\"[一覧]      %\"l\"[最近] %\"A\"[前後] %\"w\"[酔歩]
 日時:     %\"y\"[予定]      %\"t\"[Todo] %\".\"[今日] %\":\"[昨日]
 バッファ: %\"b\"[全バ]      %\"r\"[更新] %\"Q\"[全消]
 コピー:   %\"K\"[題↑]       [名↑]
 挿入:     %\"d\"[日↓]      %\"T\"[時↓] %\"i\"[鍵↓]
 設定:      [menu 編集]  [設定]

-------------------------------------
%here%(howm-menu-legend)
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

覚え方
・予定はその日に結び付けられてるから@(at)
・覚書は沈んでいくから-
・todoは浮かび上がってくるから+
・〆切はその日に注意しなければならないから!
・保留は波打つから~
・済みは終わりだから.

-------------------------------------

[○○]は何?                    >>> %ボタンの説明%
このメニュー自体を編集できます >>> %メニューの編集について%

※ このメニューは「一時的に表示されるダイアログ」のようにイメージして
ください。多くのコマンドは、「このメニューを開く直前に表示されていたバッ
ファ」が対象となります。

= <<< %ボタンの説明%

◆ 作成

[新規] → 新規メモ作成
[速記] → 書いて C-c C-c で保存 (1 行目がタイトル). C-c C-k で取り消し
[複製] → 現メモを複製 (住所録テンプレートなどの用途を想定)

◆ 一覧

[一覧] → 全メモの一覧
[最近] → 最近のメモの一覧
[前後] → 前後のメモ (見てたメモを中心に全メモの日付順一覧)
* 対象ファイルを開いた状態からメニューを呼ぶこと
[履歴] → 検索履歴
[予定] → 予定表
[Todo] → todo 一覧
一覧で n と p を押すと, その行のメモを, もう 1 つの窓に表示

◆ 検索

[正規] → 正規表現の検索
* 基本的には大文字小文字の区別なし
  * 「Wiki」のように明示的に大文字を指定したときは区別
[固定] → キーワードを補完入力して固定文字列の検索
[roma] → ローマ字検索 (ローマ字, ひらがな, カタカナで探す)
[バ内] → 対象バッファの中を正規表現で検索
[今日] → 今日のメモ
[昨日] → 昨日のメモ

◆ 編集: 対象ファイルを開いた状態からメニューを呼ぶこと

[更新] → 下線を引きなおす
[鍵↓] → キーワードを補完入力して貼りつけ
[日↓] → 今日の日付 [yyyy-mm-dd] を貼りつけ
[時↓] → 今日の日時 [yyyy-mm-dd HH:MM] を貼りつけ
[題↑] → 現メモのタイトルを kill ring へ (C-y で貼りつけ)
* タイトルがみつからなかったときはファイル名
[名↑] → ファイル名を kill ring へ

◆ 特別

[menu 更新] → メニューの予定表などを更新
[menu 編集] → メニューを編集
[全バ] → バッファの一覧
[全消] → howm のバッファをすべて消す (未保存は除く)
[酔歩] → ランダムにリンクをたどる. 1 回に 1 歩なので, 続けるときは C-c , w
[設定] → 初期設定ファイルを開く

= <<< %メニューの編集について%
[menu 編集] でこのメニューを編集
[menu 更新] でこのメニューを更新
--------------------------------------------------------

メニューの書き方
([menu 編集] 上でリターンを叩き, 元ファイルを読んでください)

◆ 基本
見ればわかるでしょうが,
・[○○] と書いた所がボタン
・％reminder と(半角で)書いた所が予定表と todo list
・％recent と(半角で)書いた所が, 最近のメモ一覧
・％random と(半角で)書いた所が, ランダムなメモ一覧
になります.
どこに書いても機能しますから, 並べかえるなりなんなり, お好きにどうぞ.
よく使うファイルや goto link も書いておけば便利かも.
(例) file:///etc/services   >>> wiki

◆ ショートカット
%\"hoge\"[一覧]
↑こう書いておくと, 「hoge[一覧]」と表示され, キー h で一覧を起動
(正確には, 閉じ「\"」の後の, 最初のボタンなどを動かす)
%\"fuga%\" ← こんなふうに最後に % をつけると, キー f でそこにカーソル移動

◆ lisper 向け

・表示
%here%howm-menu-recent-num  ← 変数 howm-menu-recent-num の値を埋め込み
%here%(howm-menu-search \"検索\")
↑(…) の結果(「検索」の検索結果)を埋め込み
※ ただし, 使う手続きは登録が必要 (おっかないから)
(set! howm-menu-allow (append '(foo bar) howm-menu-allow)) ;; foo と bar を許可

・動作
S 式を eval: %eval%(message (buffer-name))
コマンドを call-interactively: %call%find-file
※ いずれも, メニューを呼ぶ前のバッファで評価されます

◆ 不可視
「％|」と(半角で)書くと, 可視・不可視をトグル.
こんなふうに → 見える%|見えない%|可視%|不可視…行末まで
(行末が不可視だと, 改行も消えます)

◆ 複数メニュー
「%○○%」へのリンクは, 「＜＜＜ %○○%」をメニューモードで開きます: >>> %menu%
指定ファイルをメニューモードで開く: %eval%(howm-menu-open \"00000000-000000.txt\")
"
  "Default menu, written when the menu file does not exist.
It is taken from ja/0000-00-00-000000.txt of howm.")

(define howm-menu-buttons
  '(("速記" howm-remember)
    ("新規" howm-create)
    ("複製" howm-dup)
    ("更新" howm-refresh)
    ("固定" howm-search-fixed)
    ("正規" howm-search-regexp)
    ("roma" howm-search-migemo)
    ("バ内" howm-occur)
    ("履歴" howm-history)
    ("今日" howm-today)
    ("昨日" howm-yesterday)
    ("一覧" howm-list-all)
    ("最近" howm-list-recent)
    ("前後" howm-list-around)
    ("酔歩" howm-random-walk)
    ("予定" howm-list-schedule)
    ("Todo" howm-list-todo)
    ("todo" howm-list-todo)
    ("全バ" howm-list-buffers)
    ("全消" howm-kill-all)
    ("題↑" howm-title-to-kill-ring)
    ("名↑" howm-name-to-kill-ring)
    ("日↓" howm-insert-date)
    ("時↓" howm-insert-dtime)
    ("鍵↓" howm-insert-keyword)
    ("設定" howm-settings)
    ("menu 更新" howm-menu-refresh menu)
    ("menu 編集" howm-menu-edit menu))
  "Alist of button labels and commands.
A command marked menu runs in the menu; the others run in the buffer
shown before the menu.")

(define howm-summary-items '()
  "Items of the current summary, a list of (FILE LINE TEXT).
If FILE is #f, TEXT is searched for.")
(define howm-menu-data '()
  "Data of each menu buffer.
A list of (BUFFER-NAME REFRESH SHORTCUTS ITEMS FILE).")
(define howm-menu-previous-buffer #f "Name of the buffer shown before the menu.")
(define howm-remember-previous-buffer #f "Name of the buffer shown before `howm-remember'.")
(define howm-history-list #f "Search history, newest first, or #f before it is loaded.")
(define howm-keyword-cache #f "Keywords (words of \"<<< word\"), or #f before they are collected.")

;;; 文字列とリスト

(define (howm-replace-all text from to)
  "Return TEXT with every FROM replaced by TO."
  (let loop ((start 0) (acc '()))
    (let ((found (string-search from text start)))
      (if found
          (loop (+ found (string-length from))
                (cons to (cons (substring text start found) acc)))
          (apply string-append (reverse (cons (substring text start) acc)))))))

(define (howm-take items n)
  "Return the first N elements of ITEMS."
  (if (or (null? items) (<= n 0))
      '()
      (cons (car items) (howm-take (cdr items) (- n 1)))))

(define (howm-drop items n)
  "Return ITEMS without the first N elements."
  (if (or (null? items) (<= n 0))
      items
      (howm-drop (cdr items) (- n 1))))

(define (howm-unique items)
  "Return ITEMS without duplicates, keeping the first ones."
  (let loop ((items items) (acc '()))
    (cond ((null? items) (reverse acc))
          ((member (car items) acc) (loop (cdr items) acc))
          (else (loop (cdr items) (cons (car items) acc))))))

(define (howm-range n)
  "Return the list of numbers from 0 to N - 1."
  (let loop ((i (- n 1)) (acc '()))
    (if (< i 0) acc (loop (- i 1) (cons i acc)))))

(define (howm-index-of item items)
  "Return the position of ITEM in ITEMS, counted from 0, or #f."
  (let loop ((items items) (i 0))
    (cond ((null? items) #f)
          ((equal? (car items) item) i)
          (else (loop (cdr items) (+ i 1))))))

(define (howm-pick items)
  "Return a random element of ITEMS."
  (list-ref items (random (length items))))

(define (howm-floor x)
  "Return X rounded down."
  (- x (modulo x 1)))

(define (howm-case-fold regexp keyword)
  "Make REGEXP ignore case if KEYWORD has no upper case letters.
This is the same as howm."
  (if (string=? keyword (string-downcase keyword))
      (string-append "(?i)" regexp)
      regexp))

(define (howm-matches regexp line)
  "Return the positions of the matches of REGEXP in LINE.
Each element is the list of (START END) of the match and of each group,
counted in characters.  A group that did not match is #f."
  (let loop ((start 0) (acc '()))
    (let ((found (and (<= start (string-length line))
                      (regexp-match-positions regexp line start))))
      (if found
          (loop (max (cadr (car found)) (+ (car (car found)) 1)) (cons found acc))
          (reverse acc)))))

(define (howm-group line position)
  "Return the text of LINE at POSITION, a list of (START END)."
  (substring line (car position) (cadr position)))

(define (howm-replace-matches regexp line proc)
  "Replace each match of REGEXP in LINE with (PROC LINE POSITION)."
  (let loop ((matches (howm-matches regexp line)) (pos 0) (acc '()))
    (if (null? matches)
        (apply string-append (reverse (cons (substring line pos) acc)))
        (let ((span (car (car matches))))
          (loop (cdr matches)
                (cadr span)
                (cons (proc line (car matches)) (cons (substring line pos (car span)) acc)))))))

(define (howm-trim-trailing-lines lines)
  "Return LINES without the empty lines at the end."
  (let loop ((rest (reverse lines)))
    (if (and (pair? rest) (string=? (car rest) ""))
        (loop (cdr rest))
        (reverse rest))))

(define (howm-pad2 n)
  "Return N as a string of 2 digits, such as \"01\"."
  (if (< n 10) (string-append "0" (number->string n)) (number->string n)))

;;; 日付 (2026-10-25 の形)

(define howm-day (* 24 60 60) "Number of seconds in a day.")
(define howm-day-of-week-names '("日" "月" "火" "水" "木" "金" "土")
  "Names of the days of the week in Japanese, from Sunday.")

(define (howm-date-time date)
  "Return the time of DATE at noon.
Noon keeps the date right across daylight saving time."
  (date-to-time (string-append date " 12:00")))

(define (howm-today-date)
  "Return the date of today, such as \"2026-10-25\"."
  (format-time-string "%Y-%m-%d"))

(define (howm-date-shift date days)
  "Return the date DAYS days after DATE."
  (format-time-string "%Y-%m-%d" (+ (howm-date-time date) (* days howm-day))))

(define (howm-day-of-week date)
  "Return the day of the week of DATE in Japanese."
  (list-ref howm-day-of-week-names
            (string->number (format-time-string "%w" (howm-date-time date)))))

(define (howm-date-parts date)
  "Return the year, the month and the day of DATE as numbers."
  (map string->number (split-string date "-")))

(define (howm-make-date y m d)
  "Return the date of year Y, month M and day D."
  (string-append (number->string y) "-" (howm-pad2 m) "-" (howm-pad2 d)))

(define (howm-days-in-month y m)
  "Return the number of days of month M of year Y."
  (cond ((= m 2)
         (if (and (= (modulo y 4) 0) (or (not (= (modulo y 100) 0)) (= (modulo y 400) 0)))
             29
             28))
        ((member m '(4 6 9 11)) 30)
        (else 31)))

(define (howm-date-add-months date months)
  "Return the date MONTHS months after DATE.
The day is limited to the end of the month."
  (let* ((parts (howm-date-parts date))
         (index (+ (* (car parts) 12) (- (cadr parts) 1) months))
         (y (quotient index 12))
         (m (+ (modulo index 12) 1)))
    (howm-make-date y m (min (caddr parts) (howm-days-in-month y m)))))

(define (howm-date-expand digits base)
  "Return the date written as DIGITS (yyyymmdd, yymmdd, mmdd or dd).
Omitted parts and parts of 0 are taken from BASE."
  (let* ((digits (if (> (string-length digits) 8)
                     (substring digits (- (string-length digits) 8))
                     digits))
         (padded (string-append (substring "        " 0 (- 8 (string-length digits))) digits))
         (given (map (lambda (r) (or (string->number (substring padded (car r) (cadr r))) 0))
                     '((0 4) (4 6) (6 8))))
         (parts (howm-date-parts base))
         (pick (lambda (g b) (if (= g 0) b g)))
         (y (pick (car given) (car parts))))
    (howm-make-date (if (<= y 99) (+ y 2000) y)
                    (pick (cadr given) (cadr parts))
                    (pick (caddr given) (caddr parts)))))

;;; ファイル

(define (howm-dir)
  "Return the absolute path of `howm-directory'."
  (expand-file-name howm-directory))

(define (howm-files)
  "Return the memo files, sorted by name.
Files matching `howm-excluded-file-regexp' are excluded."
  (let ((skip (+ (string-length (howm-dir)) 1)))
    (filter (lambda (file)
              (not (regexp-match howm-excluded-file-regexp (substring file skip))))
            (directory-files-recursively (howm-dir) howm-file-name-regexp))))

(define (howm-memo-file? file)
  "Return #t if FILE is under `howm-directory'."
  (and (string? file) (string-prefix? (string-append (howm-dir) "/") file)))

(define (howm-file-label file)
  "Return the name of FILE without its directory and .txt."
  (cadr (regexp-match "([^/]*?)(\\.txt)?$" file)))

(define (howm-file-directory file)
  "Return the directory of FILE."
  (cadr (regexp-match "^(.*)/[^/]*$" file)))

(define (howm-title-line? line)
  "Return #t if LINE is a title line, which starts with \"= \"."
  (or (string=? line howm-view-title-header)
      (string-prefix? (string-append howm-view-title-header " ") line)))

(define (howm-title-of line)
  "Return the title of the title LINE."
  (string-trim (substring line (string-length howm-view-title-header))))

(define (howm-title file)
  "Return the title of the memo FILE, or \"\" if it has none.
The title is the first line starting with \"= \"."
  (let ((found (regexp-match
                (string-append "(?m)^" (regexp-quote howm-view-title-header) " ?(.*)$")
                (read-file-string file))))
    (if found (string-trim (cadr found)) "")))

(define (howm-title-or-label file)
  "Return the title of FILE, or its name if it has no title."
  (let ((title (howm-title file)))
    (if (string=? title "") (howm-file-label file) title)))

(define (howm-recent-files)
  "Return the memo files, newest first by modification time."
  (map cadr
       (sort (map (lambda (file) (list (file-modification-time file) file)) (howm-files))
             (lambda (a b) (> (car a) (car b))))))

(define (howm-random-files n)
  "Return N random memo files."
  (let loop ((files (howm-files)) (n n) (acc '()))
    (if (or (null? files) (<= n 0))
        (reverse acc)
        (let ((picked (howm-pick files)))
          (loop (filter (lambda (f) (not (equal? f picked))) files)
                (- n 1)
                (cons picked acc))))))

(define (howm-new-file)
  "Return the path of a new memo file.
If the file exists, use a time one second later."
  (let loop ((time (current-time)))
    (let ((file (string-append (howm-dir) "/" (format-time-string howm-file-name-format time))))
      (if (file-exists? file) (loop (+ time 1)) file))))

(define (howm-write-memo title lines)
  "Write a new memo with TITLE, the date and time, and LINES."
  (let ((file (howm-new-file)))
    (write-file-string file
                       (string-append howm-view-title-header " " title "\n"
                                      (format-time-string howm-dtime-format) "\n"
                                      (string-join lines "\n") "\n"))
    (set! howm-keyword-cache #f)
    file))

(define howm-come-from-regexp "<<<\\s*(.*\\S)" "Regexp of a keyword definition, \"<<< word\".")

(define (howm-keywords)
  "Return the keywords, the words of \"<<< word\" in the memos.
The collected keywords are reused."
  (unless howm-keyword-cache
    (set! howm-keyword-cache
          (howm-unique
           (map (lambda (found) (cadr (regexp-match howm-come-from-regexp (list-ref found 2))))
                (grep howm-come-from-regexp (howm-files))))))
  howm-keyword-cache)

(define (howm-keyword-regexp keywords)
  "Return a regexp matching any of KEYWORDS, longer ones first."
  (string-join (map regexp-quote
                    (sort keywords (lambda (a b) (> (string-length a) (string-length b)))))
               "|"))

;;; バッファ

(define (howm-set-buffer-text text)
  "Replace the whole text of the current buffer with TEXT."
  (delete-region (point-min) (point-max))
  (insert text)
  (goto-char (point-min))
  (set-buffer-modified-p #f))

(define (howm-current-line)
  "Return the line number of point, counted from 1."
  (length (split-string (buffer-substring (point-min) (point)) "\n")))

(define (howm-line-start text line)
  "Return the position of the start of LINE in TEXT, counted from 1."
  (let loop ((lines (split-string text "\n")) (n 1) (pos 1))
    (if (or (null? lines) (>= n line))
        pos
        (loop (cdr lines) (+ n 1) (+ pos (string-length (car lines)) 1)))))

(define (howm-current-line-start)
  "Return the position of the start of the current line."
  (howm-line-start (buffer-string) (howm-current-line)))

(define (howm-current-line-and-column)
  "Return the current line and the column of point in it, counted from 0."
  (let* ((text (buffer-string))
         (start (howm-line-start text (howm-current-line)))
         (rest (substring text (- start 1)))
         (end (string-search "\n" rest)))
    (list (if end (substring rest 0 end) rest) (- (point) start))))

(define (howm-replace-in-line from to text)
  "Replace the text from FROM to TO in the current line with TEXT.
FROM and TO are columns.  Point does not move."
  (let ((start (howm-current-line-start))
        (here (point)))
    (delete-region (+ start from) (+ start to))
    (goto-char (+ start from))
    (insert text)
    (goto-char here)))

(define (howm-section-range lines line)
  "Return the range of the memo containing LINE in LINES.
A memo runs from a title line to the line before the next title line.
LINE counts from 1; the range is (START END), counted from 0, END excluded."
  (let loop ((rest lines) (i 0) (start 0))
    (cond ((null? rest) (list start i))
          ((and (howm-title-line? (car rest)) (> i start))
           (if (< i line) (loop (cdr rest) (+ i 1) i) (list start i)))
          (else (loop (cdr rest) (+ i 1) start)))))

(define (howm-lines-in lines range)
  "Return the lines of LINES in RANGE."
  (howm-take (howm-drop lines (car range)) (- (cadr range) (car range))))

(define (howm-current-section)
  "Return the lines of the memo at point."
  (let ((lines (split-string (buffer-string) "\n")))
    (howm-lines-in lines (howm-section-range lines (howm-current-line)))))

(define (howm-current-title)
  "Return the title of the memo at point, or #f."
  (let ((titles (filter howm-title-line? (howm-current-section))))
    (and (pair? titles)
         (not (string=? (howm-title-of (car titles)) ""))
         (howm-title-of (car titles)))))

(define (howm-switch-back name)
  "Switch to the buffer NAME if it exists."
  (when (and name (member name (buffer-list)))
    (switch-to-buffer name)))

(define (howm-open file line)
  "Open FILE and go to LINE."
  (find-file file)
  (goto-char (howm-line-start (buffer-string) line))
  (howm-mode))

(define (howm-mode)
  "Make the current buffer a memo buffer.
RET runs actions and links are underlined.  The keywords are collected again."
  (set! howm-keyword-cache #f)
  (local-set-key "RET" 'howm-action-or-newline)
  (howm-highlight '()))

(define (howm-set-mode-p)
  "Return #t if the current buffer visits a memo file.
The file must be under `howm-directory' and match `howm-file-name-regexp'."
  (let ((file (buffer-file-name)))
    (and (howm-memo-file? file)
         (regexp-match howm-file-name-regexp file)
         #t)))

(define (howm-find-file-hook)
  "Turn on `howm-mode' when a memo is opened.
This is added to `find-file-hook'."
  (when (and howm-set-mode-on-find-file (howm-set-mode-p))
    (howm-mode)))

(define (howm-after-save)
  "Refresh keywords, underlines and the menu after saving a memo.
This is added to `after-save-hook'."
  (when (howm-set-mode-p)
    (set! howm-keyword-cache #f)
    (when howm-refresh-after-save
      (howm-highlight '()))
    (when howm-menu-refresh-after-save
      (howm-menu-refresh-background))
    (run-hooks 'howm-after-save-hook)))

(define (howm-menu-refresh-background)
  "Refresh the menu if it is shown, and return to the current buffer."
  (when (member howm-menu-name (buffer-list))
    (let ((buffer (buffer-name))
          (previous howm-menu-previous-buffer))
      (howm-menu)
      (set! howm-menu-previous-buffer previous)
      (switch-to-buffer buffer))))

(define howm-highlight-regexps
  '(">>>[ \\t]*\\S.*"
    "<<<[ \\t]*\\S.*"
    "\\[[0-9]{4}-[0-9]{2}-[0-9]{2}\\]([-+@!.~][0-9]*)?"
    "\\[\\[[^\\]\\n]+\\]\\]"
    "file://\\S+")
  "Regexps to underline, written as JavaScript regexps for the editor.")

(define (howm-highlight extra)
  "Underline links, dates, keywords and the regexps in EXTRA again."
  (let ((keywords (howm-keywords)))
    (unhighlight-regexp #t)
    (for-each highlight-regexp (append howm-highlight-regexps extra))
    (unless (null? keywords)
      (highlight-regexp (howm-keyword-regexp keywords)))))

;;; 一覧 (*howmS*)

(define (howm-summary-line item)
  "Return the line of the summary for ITEM."
  (if (car item)
      (format "%s | %s" (howm-file-label (car item)) (list-ref item 2))
      (list-ref item 2)))

(define (howm-show-summary items empty-message)
  "Show ITEMS, a list of (FILE LINE TEXT), in the summary buffer.
If there are no items, show EMPTY-MESSAGE."
  (if (null? items)
      (message empty-message)
      (begin
        (switch-to-buffer howm-summary-name)
        (howm-set-buffer-text (string-join (map howm-summary-line items) "\n"))
        (set! howm-summary-items items)
        (local-set-key "RET" 'howm-action)
        (local-set-key "n" 'howm-summary-next)
        (local-set-key "p" 'howm-summary-previous)
        (howm-summary-contents)
        (message "%d items" (length items)))))

(define (howm-summary-item)
  "Return the item of the current line of the summary, or #f."
  (let ((n (howm-current-line)))
    (and (<= n (length howm-summary-items))
         (list-ref howm-summary-items (- n 1)))))

(define (howm-summary-contents)
  "Show the memo of the current line in the other window."
  (let ((item (howm-summary-item)))
    (when (and howm-view-contents item (car item))
      (display-buffer (find-file-noselect (car item)) (cadr item)))))

(define (howm-summary-open)
  "Open the item of the current line.
If the item has no file, search for its text."
  (let ((item (howm-summary-item)))
    (cond ((not item) #f)
          ((car item) (howm-open (car item) (cadr item)))
          (else (howm-search-keyword-with-history (list-ref item 2))))))

(define-command (howm-summary-next)
  "Move to the next line and show its memo."
  (call-interactively 'next-line)
  (howm-summary-contents))

(define-command (howm-summary-previous)
  "Move to the previous line and show its memo."
  (call-interactively 'previous-line)
  (howm-summary-contents))

(define (howm-file-items files)
  "Return the title lines of FILES as summary items."
  (map (lambda (file) (list file 1 (howm-title-or-label file))) files))

;;; 検索

(define (howm-search regexp keyword)
  "Search the memos for REGEXP.
Lines of \"<<< KEYWORD\" come first, then newer files first."
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

(define (howm-search-keyword keyword)
  "Search the memos for KEYWORD as a fixed string."
  (howm-search (howm-case-fold (regexp-quote keyword) keyword) keyword))

(define (howm-search-keyword-with-history keyword)
  "Search the memos for KEYWORD and add it to the history."
  (howm-history-add keyword)
  (howm-search-keyword keyword))

(define-command (howm-search-fixed)
  "Read a keyword with completion and search the memos for it."
  (let ((keyword (completing-read "Search: " (howm-keywords))))
    (unless (string=? keyword "")
      (howm-search-keyword-with-history keyword))))

(define-command (howm-search-regexp)
  "Read a regexp and search the memos for it."
  (let ((regexp (read-string "Search (regexp): ")))
    (unless (string=? regexp "")
      (howm-history-add regexp)
      (howm-search (howm-case-fold regexp regexp) regexp))))

(define (howm-search-day days)
  "Search the memos for the date DAYS days after today."
  (howm-search-keyword (string-append "[" (howm-date-shift (howm-today-date) days))))

(define-command (howm-today)
  "Search the memos for the date of today."
  (howm-search-day 0))
(define-command (howm-yesterday)
  "Search the memos for the date of yesterday."
  (howm-search-day -1))

(define-command (howm-occur)
  "Read a regexp and list its matches in the current buffer."
  (let ((file (buffer-file-name)))
    (if (not file)
        (message "Not a file buffer")
        (let ((regexp (read-string "Occur (regexp): ")))
          (unless (string=? regexp "")
            (let loop ((lines (split-string (buffer-string) "\n")) (n 1) (acc '()))
              (cond ((null? lines)
                     (howm-show-summary (reverse acc) (format "No match: %s" regexp)))
                    ((regexp-match (howm-case-fold regexp regexp) (car lines))
                     (loop (cdr lines) (+ n 1) (cons (list file n (car lines)) acc)))
                    (else (loop (cdr lines) (+ n 1) acc)))))))))

;;; ローマ字検索 (migemo の代わり. ローマ字を, ひらがなとカタカナにして探す)

(define howm-roma-table
  '(("a" "あ") ("i" "い") ("u" "う") ("e" "え") ("o" "お")
    ("ka" "か") ("ki" "き") ("ku" "く") ("ke" "け") ("ko" "こ")
    ("kya" "きゃ") ("kyu" "きゅ") ("kyo" "きょ")
    ("sa" "さ") ("si" "し") ("shi" "し") ("su" "す") ("se" "せ") ("so" "そ")
    ("sha" "しゃ") ("shu" "しゅ") ("sho" "しょ") ("she" "しぇ")
    ("sya" "しゃ") ("syu" "しゅ") ("syo" "しょ")
    ("ta" "た") ("ti" "ち") ("chi" "ち") ("tu" "つ") ("tsu" "つ") ("te" "て") ("to" "と")
    ("cha" "ちゃ") ("chu" "ちゅ") ("cho" "ちょ") ("che" "ちぇ")
    ("tya" "ちゃ") ("tyu" "ちゅ") ("tyo" "ちょ")
    ("na" "な") ("ni" "に") ("nu" "ぬ") ("ne" "ね") ("no" "の")
    ("nya" "にゃ") ("nyu" "にゅ") ("nyo" "にょ") ("nn" "ん") ("n'" "ん")
    ("ha" "は") ("hi" "ひ") ("hu" "ふ") ("fu" "ふ") ("he" "へ") ("ho" "ほ")
    ("hya" "ひゃ") ("hyu" "ひゅ") ("hyo" "ひょ")
    ("fa" "ふぁ") ("fi" "ふぃ") ("fe" "ふぇ") ("fo" "ふぉ")
    ("ma" "ま") ("mi" "み") ("mu" "む") ("me" "め") ("mo" "も")
    ("mya" "みゃ") ("myu" "みゅ") ("myo" "みょ")
    ("ya" "や") ("yu" "ゆ") ("yo" "よ")
    ("ra" "ら") ("ri" "り") ("ru" "る") ("re" "れ") ("ro" "ろ")
    ("rya" "りゃ") ("ryu" "りゅ") ("ryo" "りょ")
    ("wa" "わ") ("wo" "を")
    ("ga" "が") ("gi" "ぎ") ("gu" "ぐ") ("ge" "げ") ("go" "ご")
    ("gya" "ぎゃ") ("gyu" "ぎゅ") ("gyo" "ぎょ")
    ("za" "ざ") ("zi" "じ") ("ji" "じ") ("zu" "ず") ("ze" "ぜ") ("zo" "ぞ")
    ("ja" "じゃ") ("ju" "じゅ") ("jo" "じょ") ("je" "じぇ")
    ("zya" "じゃ") ("zyu" "じゅ") ("zyo" "じょ")
    ("da" "だ") ("di" "ぢ") ("du" "づ") ("de" "で") ("do" "ど")
    ("ba" "ば") ("bi" "び") ("bu" "ぶ") ("be" "べ") ("bo" "ぼ")
    ("bya" "びゃ") ("byu" "びゅ") ("byo" "びょ")
    ("pa" "ぱ") ("pi" "ぴ") ("pu" "ぷ") ("pe" "ぺ") ("po" "ぽ")
    ("pya" "ぴゃ") ("pyu" "ぴゅ") ("pyo" "ぴょ")
    ("vu" "ゔ")
    ("xa" "ぁ") ("xi" "ぃ") ("xu" "ぅ") ("xe" "ぇ") ("xo" "ぉ")
    ("xtu" "っ") ("xya" "ゃ") ("xyu" "ゅ") ("xyo" "ょ")
    ("-" "ー"))
  "Alist of romaji and hiragana.")

(define howm-hiragana
  "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔ"
  "Hiragana letters, in the same order as `howm-katakana'.")
(define howm-katakana
  "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴ"
  "Katakana letters, in the same order as `howm-hiragana'.")

(define (howm-roma-lookup roma i)
  "Return the romaji spelling at index I of ROMA, or #f.
The result is (ROMAJI KANA); longer spellings are tried first."
  (let loop ((n 3))
    (and (> n 0)
         (or (and (<= (+ i n) (string-length roma))
                  (assoc (substring roma i (+ i n)) howm-roma-table))
             (loop (- n 1))))))

(define (howm-roma-to-hiragana roma)
  "Convert ROMA to hiragana.
Letters that cannot be converted are kept."
  (let loop ((i 0) (acc '()))
    (if (>= i (string-length roma))
        (apply string-append (reverse acc))
        (let ((c (substring roma i (+ i 1)))
              (next (if (< (+ i 1) (string-length roma)) (substring roma (+ i 1) (+ i 2)) "")))
          (if (and (string=? c next) (regexp-match "^[b-df-hj-mp-tv-z]$" c))
              ;; 同じ子音が続けば, っ
              (loop (+ i 1) (cons "っ" acc))
              (let ((found (howm-roma-lookup roma i)))
                (cond (found (loop (+ i (string-length (car found))) (cons (cadr found) acc)))
                      ((string=? c "n") (loop (+ i 1) (cons "ん" acc)))
                      (else (loop (+ i 1) (cons c acc))))))))))

(define (howm-hiragana-to-katakana text)
  "Convert the hiragana in TEXT to katakana."
  (apply string-append
         (map (lambda (i)
                (let* ((c (substring text i (+ i 1)))
                       (found (string-search c howm-hiragana)))
                  (if found (substring howm-katakana found (+ found 1)) c)))
              (howm-range (string-length text)))))

(define (howm-migemo-regexp roma)
  "Return a regexp matching ROMA as romaji, hiragana or katakana."
  (let ((hiragana (howm-roma-to-hiragana (string-downcase roma))))
    (string-append "(?i)(?:"
                   (string-join (map regexp-quote
                                     (howm-unique
                                      (list roma hiragana (howm-hiragana-to-katakana hiragana))))
                                "|")
                   ")")))

(define-command (howm-search-migemo)
  "Read romaji and search the memos for it.
Hiragana and katakana are also searched for."
  (let ((roma (read-string "Search (roma): ")))
    (unless (string=? roma "")
      (howm-history-add roma)
      (howm-search (howm-migemo-regexp roma) roma))))

;;; 履歴

(define (howm-history-path)
  "Return the path of the history file."
  (or howm-history-file (string-append (howm-dir) "/.howm-history")))

(define (howm-history-load)
  "Return the search history, newest first.
The history is read from the file the first time."
  (unless howm-history-list
    (set! howm-history-list
          (if (file-exists? (howm-history-path))
              (filter (lambda (k) (not (string=? k "")))
                      (split-string (read-file-string (howm-history-path)) "\n"))
              '())))
  howm-history-list)

(define (howm-history-add keyword)
  "Add KEYWORD to the search history and write the history file."
  (set! howm-history-list
        (howm-take (cons keyword (filter (lambda (k) (not (string=? k keyword))) (howm-history-load)))
                   howm-history-limit))
  (write-file-string (howm-history-path) (string-join howm-history-list "\n")))

(define-command (howm-history)
  "List the search history.
RET on a line searches for its keyword."
  (howm-show-summary (map (lambda (k) (list #f 0 k)) (howm-history-load)) "No history"))

;;; 一覧

(define-command (howm-list-all)
  "List all memos, newest first."
  (howm-show-summary (howm-file-items (reverse (howm-files))) "No memo"))

(define-command (howm-list-recent)
  "List the recent memos."
  (howm-show-summary (howm-file-items (howm-take (howm-recent-files) howm-menu-recent-num))
                     "No memo"))

(define-command (howm-list-around)
  "List all memos with point on the current memo."
  (let ((file (buffer-file-name))
        (files (reverse (howm-files))))
    (howm-show-summary (howm-file-items files) "No memo")
    (let ((index (howm-index-of file files)))
      (when index
        (goto-char (howm-line-start (buffer-string) (+ index 1)))
        (howm-summary-contents)))))

(define (howm-buffer-links)
  "Return the words to search for in the current buffer.
These are the links and the keywords."
  (howm-unique
   (apply append
          (map (lambda (line)
                 (map (lambda (action) (cadr (list-ref action 2)))
                      (filter (lambda (action)
                                (member (car (list-ref action 2)) '("search" "wiki")))
                              (howm-line-actions line #f))))
               (split-string (buffer-string) "\n")))))

(define-command (howm-random-walk)
  "Take one step of a random walk.
In the summary, open a random line.  In a memo, search for a random link,
or open a random memo if there are no links.  Call it again to continue."
  (if (string=? (buffer-name) howm-summary-name)
      (begin
        (goto-char (howm-line-start (buffer-string) (+ (random (length howm-summary-items)) 1)))
        (howm-summary-open))
      (let ((links (howm-buffer-links)))
        (if (null? links)
            (let ((files (howm-files)))
              (if (null? files)
                  (message "No memo")
                  (howm-open (howm-pick files) 1)))
            (howm-search-keyword (howm-pick links)))))
  (message "Random walk: C-c , w で次へ"))

;;; 予定と todo

(define howm-reminder-regexp
  "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\]([-+@!.~])([0-9]*)(.*)$"
  "Regexp of a reminder, such as [2026-10-25]+7.
The groups are the date, the type, the laziness and the rest.")
(define howm-reminder-action-regexp
  "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\](([-+@!.~])([0-9]*))"
  "Regexp of the parts of a reminder that RET acts on.
The groups are the date, the type and laziness, the type and the laziness.")

(define howm-reminder-laziness '(("-" 1) ("+" 7) ("~" 30) ("!" 7) ("@" 1) ("." 1))
  "Alist of reminder types and their default laziness in days.")

(define howm-huge 77777 "Priority of the bottom and the top, as howm-huge of howm.")
(define howm-huge+ 88888 "Priority above `howm-huge'.")
(define howm-huge++ 99999 "Priority above `howm-huge+'.")

(define (howm-priority type late laziness)
  "Return the priority of a reminder of TYPE.
LATE is the number of days since its date; LAZINESS is in days.
This is howm-todo-priority-* of howm."
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

(define (howm-reminders)
  "Return the reminders in the memos.
Each is a list of (PRIORITY DATE TYPE FILE LINE TEXT)."
  (let ((now (current-time)))
    (map (lambda (found)
           (let* ((parts (regexp-match howm-reminder-regexp (list-ref found 2)))
                  (date (list-ref parts 1))
                  (type (list-ref parts 2))
                  (laziness (if (string=? (list-ref parts 3) "")
                                (cadr (assoc type howm-reminder-laziness))
                                (max 1 (string->number (list-ref parts 3)))))
                  (late (/ (- now (date-to-time date)) howm-day)))
             (list (howm-priority type late laziness) date type
                   (car found) (cadr found) (car parts))))
         (grep howm-reminder-regexp (howm-files)))))

(define (howm-reminder-item reminder)
  "Return REMINDER as a summary item, (FILE LINE TEXT)."
  (list (list-ref reminder 3) (list-ref reminder 4) (list-ref reminder 5)))

(define (howm-schedule days)
  "Return the schedule (@), sorted by date.
If DAYS is a number, only the next DAYS days from today."
  (let ((today (howm-today-date))
        (last (howm-date-shift (howm-today-date) (or days 0))))
    (sort (filter (lambda (r)
                    (and (string=? (list-ref r 2) "@")
                         (or (not days)
                             (and (not (string<? (list-ref r 1) today))
                                  (not (string<? last (list-ref r 1)))))))
                  (howm-reminders))
          (lambda (a b) (string<? (list-ref a 1) (list-ref b 1))))))

(define (howm-todo)
  "Return the todo items, sorted by priority.
Schedules, done items and items sunk to the bottom are excluded."
  (sort (filter (lambda (r)
                  (and (not (member (list-ref r 2) '("@" ".")))
                       (> (car r) (- howm-huge+))))
                (howm-reminders))
        (lambda (a b) (> (car a) (car b)))))

(define-command (howm-list-schedule)
  "List the schedule."
  (howm-show-summary (map howm-reminder-item (howm-schedule #f)) "No schedule"))

(define-command (howm-list-todo)
  "List the todo items."
  (howm-show-summary (map howm-reminder-item (howm-todo)) "No todo"))

(define (howm-reminder-action from)
  "Act on the type of the reminder in the current line.
RET marks it done, x cancels it, a type letter or a number changes the
type or the laziness.  FROM is the column of the date."
  (let* ((line (car (howm-current-line-and-column)))
         (found (regexp-match-positions howm-reminder-action-regexp line from))
         (date (howm-group line (list (car (car found)) (car (list-ref found 2)))))
         (type (howm-group line (list-ref found 3)))
         (lazy (howm-group line (list-ref found 4)))
         (desc (substring line (cadr (car found))))
         (c (read-string "RET (done), x (cancel), symbol (type), num (laziness): "))
         (modify (regexp-match "^([-+@!.~]?)([0-9]*)$" c))
         (done (string-append (format-time-string howm-date-format) ". "))
         (new (cond ((string=? c "") (string-append done date ":" type lazy desc))
                    ((string=? c "x") (string-append done "cancel " date ":" type lazy desc))
                    (modify (string-append date
                                           (if (string=? (cadr modify) "") type (cadr modify))
                                           (if (string=? (caddr modify) "") lazy (caddr modify))
                                           desc))
                    (else #f))))
    (if new
        (howm-replace-in-line from (string-length line) new)
        (message "Can't understand %s." c))))

(define (howm-date-action date from to new?)
  "Act on the date from FROM to TO in the current line.
RET lists it (unless NEW?), +3 or -1 shifts it, yymmdd changes it, . sets
today, ~yymmdd repeats it, and [ or ] shifts it by one day."
  (let* ((replace (lambda (new) (howm-replace-in-line from to (string-append "[" new "]"))))
         (c (read-string
             (format "%s [%s] %s+num(shift), yymmdd(set), ~yymmdd(repeat), .(today), [(-1), ](+1): "
                     date (howm-day-of-week date) (if new? "RET(ok), " "RET(list), ")))))
    (cond ((string=? c "")
           (unless new? (howm-search-keyword (string-append "[" date))))
          ((regexp-match "^[-+][0-9]+$" c) (replace (howm-date-shift date (string->number c))))
          ((regexp-match "^[0-9]+$" c) (replace (howm-date-expand c date)))
          ((regexp-match "^~[0-9]+$" c)
           (howm-date-repeat (howm-date-expand (substring c 1) date) date from to))
          ((string=? c ".") (replace (howm-today-date)))
          ((string=? c "[") (howm-date-quickshift -1 date replace))
          ((string=? c "]") (howm-date-quickshift 1 date replace))
          (else (message "Can't understand %s." c)))))

(define (howm-date-quickshift days date replace)
  "Shift DATE by DAYS days and call REPLACE with the new date.
Then read [ or ] to shift it by one more day, or . for today; anything
else ends."
  (let ((new (howm-date-shift date days)))
    (replace new)
    (let ((c (read-string (format "%s [%s] RET(ok), [(-1), ](+1), .(today): "
                                  new (howm-day-of-week new)))))
      (cond ((string=? c "[") (howm-date-quickshift -1 new replace))
            ((string=? c "]") (howm-date-quickshift 1 new replace))
            ((string=? c ".") (howm-date-quickshift 0 (howm-today-date) replace))
            (else #t)))))

(define (howm-date-repeat until date from to)
  "Repeat the current line below it with the date changed until UNTIL.
The date is DATE, from FROM to TO in the line."
  (let* ((every (read-string "Every? [RET(all), num(days), w(week), m(month), y(year)] "))
         (next (cond ((string=? every "y") (lambda (d) (howm-date-add-months d 12)))
                     ((string=? every "m") (lambda (d) (howm-date-add-months d 1)))
                     ((string=? every "w") (lambda (d) (howm-date-shift d 7)))
                     (else (let ((days (max 1 (or (string->number every) 1))))
                             (lambda (d) (howm-date-shift d days))))))
         (line (car (howm-current-line-and-column))))
    (let loop ((d (next date)) (dates '()))
      (if (or (string<? until d) (>= (length dates) howm-date-repeat-max))
          (let ((start (howm-current-line-start)))
            (goto-char (+ start (string-length line)))
            (insert (apply string-append
                           (map (lambda (d)
                                  (string-append "\n" (substring line 0 from) "[" d "]" (substring line to)))
                                (reverse dates))))
            (goto-char (+ start from))
            (message "%d lines" (length dates)))
          (loop (next d) (cons d dates))))))

;;; メモを作る

(define (howm-create-memo title)
  "Create a new memo with TITLE and open it.
If TITLE is empty, put point where the title goes."
  (let ((file (howm-new-file)))
    (make-directory (howm-file-directory file))
    (find-file file)
    (insert howm-view-title-header " " title "\n" (format-time-string howm-dtime-format) "\n\n")
    (when (string=? title "")
      (goto-char (+ (string-length howm-view-title-header) 2)))
    (howm-mode)))

(define-command (howm-create)
  "Create a new memo."
  (howm-create-memo ""))

(define (howm-wiki keyword)
  "Act on [[KEYWORD]].
Create a memo of \"<<< KEYWORD\" if there is none, or search for it.
If KEYWORD is %name%, open that menu."
  (cond ((and (howm-menu-keyword? keyword) (howm-menu-find-section keyword))
         (howm-menu-open-section keyword))
        ((null? (grep (string-append "<<<\\s*" (regexp-quote keyword)) (howm-files)))
         (howm-create-memo (string-append "<<< " keyword)))
        (else (howm-search-keyword-with-history keyword))))

(define-command (howm-remember)
  "Write a quick note in a buffer of its own.
\\[howm-remember-submit] saves it as a new memo whose title is its first line.
\\[howm-remember-discard] discards it."
  (unless (string=? (buffer-name) howm-remember-name)
    (set! howm-remember-previous-buffer (buffer-name)))
  (switch-to-buffer howm-remember-name)
  (local-set-key "C-c C-c" 'howm-remember-submit)
  (local-set-key "C-c C-k" 'howm-remember-discard)
  (message "C-c C-c: save, C-c C-k: discard"))

(define (howm-remember-close)
  "Kill the quick note buffer and return to the previous buffer."
  (let ((buffer (buffer-name)))
    (set-buffer-modified-p #f)
    (howm-switch-back howm-remember-previous-buffer)
    (kill-buffer buffer)))

(define-command (howm-remember-submit)
  "Save the quick note as a new memo."
  (let ((text (string-trim (buffer-string))))
    (if (string=? text "")
        (message "Empty memo")
        (let* ((lines (split-string text "\n"))
               (file (howm-write-memo (car lines) (cons "" (cdr lines)))))
          (howm-remember-close)
          (message "Saved: %s" file)))))

(define-command (howm-remember-discard)
  "Discard the quick note."
  (howm-remember-close)
  (message "Discarded"))

(define-command (howm-dup)
  "Copy the memo at point to a new memo and open it."
  (let ((file (howm-new-file)))
    (write-file-string file
                       (string-append (string-join (howm-trim-trailing-lines (howm-current-section)) "\n")
                                      "\n"))
    (howm-open file 1)))

;;; コピーと挿入

(define-command (howm-title-to-kill-ring)
  "Put the title of the memo at point in the kill ring.
If it has no title, use the file name."
  (let ((text (or (howm-current-title)
                  (let ((file (buffer-file-name)))
                    (if file (howm-file-label file) (buffer-name))))))
    (kill-new text)
    (message "%s" text)))

(define-command (howm-name-to-kill-ring)
  "Put the file name of the current buffer in the kill ring."
  (let ((text (or (buffer-file-name) (buffer-name))))
    (kill-new text)
    (message "%s" text)))

(define-command (howm-insert-keyword)
  "Read a keyword with completion and insert it."
  (let ((keyword (completing-read "Keyword: " (howm-keywords))))
    (unless (string=? keyword "")
      (insert keyword))))

(define-command (howm-insert-date)
  "Insert the date of today and act on it as RET on a date does."
  (let ((column (cadr (howm-current-line-and-column)))
        (date (format-time-string howm-date-format)))
    (insert date)
    (howm-date-action (howm-today-date) column (+ column (string-length date)) #t)))

(define-command (howm-insert-dtime)
  "Insert the current date and time."
  (insert (format-time-string howm-dtime-format)))

;;; バッファ

(define (howm-buffer? name)
  "Return #t if NAME is a howm buffer.
Memos, the menu, the summary and quick notes are howm buffers."
  (or (string-prefix? "*howm" name)
      (howm-memo-file? (buffer-file-name name))))

(define-command (howm-kill-all)
  "Kill all howm buffers except modified ones."
  (let loop ((names (buffer-list)) (n 0))
    (cond ((null? names) (message "%d buffers killed" n))
          ((and (howm-buffer? (car names)) (not (buffer-modified-p (car names))))
           (kill-buffer (car names))
           (loop (cdr names) (+ n 1)))
          (else (loop (cdr names) n)))))

(define-command (howm-list-buffers)
  "List the buffers."
  (call-interactively 'list-buffers))

(define-command (howm-refresh)
  "Underline the current buffer again, or refresh it if it is the menu."
  (if (howm-menu-buffer?)
      (howm-menu-refresh)
      (begin (howm-mode) (message "Refreshed"))))

(define-command (howm-settings)
  "Open the init file."
  (if user-init-file
      (find-file user-init-file)
      (message "No init file")))

;;; RET (ボタン, リンク, 日付, 予定と todo, 一覧とメニューの項目)

(define (howm-link-action keyword)
  "Act on the word of \">>> KEYWORD\".
If KEYWORD is %name%, open that menu; otherwise search for it."
  (if (howm-menu-keyword? keyword) (list "menu" keyword) (list "search" keyword)))

(define (howm-action-rules menu?)
  "Return the rules of the parts that RET acts on.
Each is (REGEXP GROUP MAKE-ACTION); earlier rules win.
If MENU? is true, the rules of the menu are included."
  (append
   (if menu?
       (list (list "%eval%(.*)$" 0 (lambda (line found) (list "eval" (howm-group line (cadr found)))))
             (list "%call%(.*)$" 0 (lambda (line found) (list "call" (howm-group line (cadr found)))))
             (list "\\[([^\\[\\]]+)\\]" 0
                   (lambda (line found)
                     (let ((label (howm-group line (cadr found))))
                       (and (howm-button label) (list "button" label))))))
       '())
   (list (list howm-reminder-action-regexp 2
               (lambda (line found) (list "reminder" (car (car found)))))
         (list "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\]" 0
               (lambda (line found)
                 (list "date" (howm-group line (cadr found)) (car (car found)) (cadr (car found)))))
         (list ">>>\\s*(.*\\S)" 0 (lambda (line found) (howm-link-action (howm-group line (cadr found)))))
         (list howm-come-from-regexp 0 (lambda (line found) (list "search" (howm-group line (cadr found)))))
         (list "\\[\\[([^\\]\\n]+)\\]\\]" 0 (lambda (line found) (list "wiki" (howm-group line (cadr found)))))
         (list "file://(\\S+)" 0 (lambda (line found) (list "file" (howm-group line (cadr found))))))
   (let ((keywords (howm-keywords)))
     (if (null? keywords)
         '()
         (list (list (howm-keyword-regexp keywords) 0
                     (lambda (line found) (list "search" (howm-group line (car found))))))))))

(define (howm-line-actions line menu?)
  "Return the parts of LINE that RET acts on, a list of (START END ACTION).
If MENU? is true, the rules of the menu are used too."
  (apply append
         (map (lambda (rule)
                (filter (lambda (x) x)
                        (map (lambda (found)
                               (let ((action ((caddr rule) line found))
                                     (span (list-ref found (cadr rule))))
                                 (and action (list (car span) (cadr span) action))))
                             (howm-matches (car rule) line))))
              (howm-action-rules menu?))))

(define (howm-action-at line column menu?)
  "Return the action at COLUMN of LINE, or #f.
If MENU? is true, the rules of the menu are used too."
  (let loop ((actions (howm-line-actions line menu?)))
    (cond ((null? actions) #f)
          ((and (<= (car (car actions)) column) (< column (cadr (car actions))))
           (list-ref (car actions) 2))
          (else (loop (cdr actions))))))

(define (howm-run-action action)
  "Run ACTION."
  (let ((kind (car action))
        (arg (cadr action)))
    (cond ((string=? kind "search") (howm-search-keyword-with-history arg))
          ((string=? kind "wiki") (howm-wiki arg))
          ((string=? kind "menu") (howm-menu-open-section arg))
          ((string=? kind "file") (find-file arg))
          ((string=? kind "date") (howm-date-action arg (list-ref action 2) (list-ref action 3) #f))
          ((string=? kind "reminder") (howm-reminder-action arg))
          ((string=? kind "eval") (howm-menu-in-previous (lambda () (eval (read arg)))))
          ((string=? kind "call") (howm-menu-in-previous (lambda () (call-interactively (read arg)))))
          ((string=? kind "button") (howm-run-button arg))
          ((string=? kind "move")
           (let ((found (string-search arg (buffer-string))))
             (when found (goto-char (+ found 1))))))))

(define (howm-action-at-point)
  "Run the action at point.  Return #t if there was one."
  (let* ((here (howm-current-line-and-column))
         (action (howm-action-at (car here) (cadr here) (howm-menu-buffer?))))
    (and action (begin (howm-run-action action) #t))))

(define (howm-menu-open-item items)
  "Open the menu item of the current line among ITEMS.
On a reminder type or a date, act on it in the memo.  Return #t if opened."
  (let ((item (assoc (howm-current-line) items)))
    (and item
         (let* ((here (howm-current-line-and-column))
                (action (howm-action-at (car here) (cadr here) #f)))
           (howm-open (cadr item) (list-ref item 2))
           (when (and action (member (car action) '("reminder" "date")))
             (let ((offset (string-search (car here) (car (howm-current-line-and-column)))))
               (when offset
                 (goto-char (+ (howm-current-line-start) offset (cadr here)))
                 (howm-action-at-point))))
           #t))))

(define-command (howm-action)
  "Act on the thing at point.
In the summary, open the memo of the line.  In the menu, open the item.
On a button or a link, run it."
  (cond ((string=? (buffer-name) howm-summary-name) (howm-summary-open))
        ((and (howm-menu-buffer?) (howm-menu-open-item (howm-menu-items))) #t)
        ((howm-action-at-point) #t)
        (else (message "No action here"))))

(define-command (howm-action-or-newline)
  "Act on the thing at point, or insert a newline if there is none.
This is RET in memo buffers."
  (unless (howm-action-at-point)
    (call-interactively 'newline)))

;;; メニュー

(define (howm-menu-buffer?)
  "Return #t if the current buffer is a menu."
  (string-prefix? "*howmM:" (buffer-name)))

(define (howm-menu-keyword? keyword)
  "Return #t if KEYWORD has the form %name%."
  (regexp-match "^%.*%$" keyword))

(define (howm-button label)
  "Return the button of LABEL, (LABEL COMMAND [menu]), or #f."
  (assoc label howm-menu-buttons))

(define (howm-menu-in-previous thunk)
  "Call THUNK in the buffer shown before the menu."
  (when (howm-menu-buffer?)
    (howm-switch-back howm-menu-previous-buffer))
  (thunk))

(define (howm-run-button label)
  "Press the button of LABEL."
  (let ((button (howm-button label)))
    (if (= (length button) 3)
        (call-interactively (cadr button))
        (howm-menu-in-previous (lambda () (call-interactively (cadr button)))))))

(define (howm-menu-path)
  "Return the path of the menu file.
Write the default menu if the file does not exist."
  (let ((file (or howm-menu-file (string-append (howm-dir) "/0000-00-00-000000.txt"))))
    (unless (file-exists? file)
      (write-file-string file howm-default-menu))
    (expand-file-name file)))

(define (howm-section-of text keyword)
  "Return the memo of \"<<< KEYWORD\" in TEXT, or #f.
It runs from the title line to the line before the next title line."
  (let ((lines (split-string text "\n"))
        (regexp (string-append "<<<\\s*" (regexp-quote keyword) "\\s*$")))
    (let loop ((rest lines) (n 1))
      (cond ((null? rest) #f)
            ((and (howm-title-line? (car rest)) (regexp-match regexp (car rest)))
             (string-join (howm-lines-in lines (howm-section-range lines n)) "\n"))
            (else (loop (cdr rest) (+ n 1)))))))

(define (howm-menu-find-section keyword)
  "Find the menu of \"<<< KEYWORD\".
The menu file is searched first, then the memos.
Return (FILE TEXT), or #f."
  (let loop ((files (cons (howm-menu-path) (howm-files))))
    (if (null? files)
        #f
        (let ((text (howm-section-of (read-file-string (car files)) keyword)))
          (if text (list (car files) text) (loop (cdr files)))))))

(define (howm-menu-current-data)
  "Return the entry of `howm-menu-data' for the current buffer, or #f."
  (assoc (buffer-name) howm-menu-data))

(define (howm-menu-items)
  "Return the items of the current menu buffer."
  (let ((data (howm-menu-current-data)))
    (if data (list-ref data 3) '())))

(define-command (howm-menu)
  "Show the howm menu."
  (let* ((file (howm-menu-path))
         (text (read-file-string file)))
    (howm-menu-show "%menu%" file (or (howm-section-of text "%menu%") text)
                    (lambda () (howm-menu)))))

(define-command (howm)
  "Show the howm menu."
  (howm-menu))

(define (howm-menu-open-section keyword)
  "Show the menu of \"<<< KEYWORD\" (>>> %name%)."
  (let ((found (howm-menu-find-section keyword)))
    (if found
        (howm-menu-show keyword (car found) (cadr found)
                        (lambda () (howm-menu-open-section keyword)))
        (message "No menu: %s" keyword))))

(define (howm-menu-open file)
  "Show FILE, relative to `howm-directory', as a menu.
For example, %eval%(howm-menu-open \"file.txt\")."
  (let ((path (expand-file-name file (howm-dir))))
    (howm-menu-show (howm-file-label path) path (read-file-string path)
                    (lambda () (howm-menu-open file)))))

(define-command (howm-menu-refresh)
  "Refresh the menu."
  (let ((data (howm-menu-current-data)))
    (if data ((list-ref data 1)) (howm-menu))))

(define-command (howm-menu-edit)
  "Open the menu file."
  (let ((data (howm-menu-current-data)))
    (find-file (if data (list-ref data 4) (howm-menu-path)))
    (howm-mode)))

(define howm-menu-key-regexp "%\"(([^\\n%\"])[^\\n%\"]*(%+[^\\n%\"]+)*)(%)?\""
  "Regexp of a shortcut in the menu, such as %\"c\"[New] or %\"fuga%\".
The groups are the word, the key, the rest of the word and the action-only %.")

(define (howm-menu-line-shortcuts line)
  "Return the shortcuts of LINE of the menu, a list of (KEY ACTION).
A shortcut runs the first action after its \"."
  (let ((actions (if (string-search "%\"" line)
                     (sort (howm-line-actions line #t) (lambda (a b) (< (car a) (car b))))
                     '())))
    (filter cadr
            (map (lambda (found)
                   (list (howm-group line (list-ref found 2))
                         (if (list-ref found 4)
                             (list "move" (howm-group line (list-ref found 1)))
                             (let loop ((rest actions))
                               (cond ((null? rest) #f)
                                     ((>= (car (car rest)) (cadr (car found))) (list-ref (car rest) 2))
                                     (else (loop (cdr rest))))))))
                 (howm-matches howm-menu-key-regexp line)))))

(define (howm-menu-visible line)
  "Return (TEXT HIDDEN) for LINE of the menu.
TEXT is the part made visible by %|; HIDDEN is #t if the end of the line
is hidden, and then the newline is removed too."
  (let loop ((parts (split-string line "%|")) (visible #t) (acc '()))
    (let ((acc (if visible (cons (car parts) acc) acc)))
      (if (null? (cdr parts))
          (list (apply string-append (reverse acc)) (not visible))
          (loop (cdr parts) (not visible) acc)))))

(define howm-menu-display-rules
  '(("%sdays" "%here%howm-menu-schedule-days")
    ("%tnum" "%here%howm-menu-todo-num")
    ("%schedule" "%here%(howm-menu-schedule)")
    ("%todo" "%here%(howm-menu-todo)")
    ("%reminder" "%here%(howm-menu-reminder)")
    ("%recent" "%here%(howm-menu-recent)")
    ("%random" "%here%(howm-menu-random)")
    ("%last-update" "%here%(howm-menu-last-update)"))
  "Rules that turn %sdays and others into the form of %here%.")

(define howm-menu-here-regexp "%here%(\\([^()\\n]*\\)|[^\\s()\",;']+)" "Regexp of %here%.")

(define (howm-menu-here text)
  "Return the value of the %here% expression TEXT.
It is the value of a variable, or of a procedure in `howm-menu-allow'.
The arguments are not evaluated."
  (let ((expr (read text)))
    (cond ((symbol? expr) (eval expr))
          ((and (pair? expr) (member (car expr) howm-menu-allow))
           (apply (eval (car expr)) (cdr expr)))
          (else (format "Not allowed: %s" text)))))

(define (howm-menu-value-lines value)
  "Return VALUE as menu lines.
Each line is a string or (STRING FILE LINE)."
  (cond ((string? value) (split-string value "\n"))
        ((list? value)
         (map (lambda (line)
                (if (or (string? line) (and (pair? line) (string? (car line))))
                    line
                    (format "%s" line)))
              value))
        (else (list (format "%s" value)))))

(define (howm-menu-value-text value)
  "Return VALUE as a string."
  (string-join (map (lambda (line) (if (string? line) line (car line)))
                    (howm-menu-value-lines value))
               "\n"))

(define (howm-menu-expand line)
  "Replace the %here% in the visible LINE with its value.
If LINE is only %here%, it is replaced with the lines of the value."
  (let ((whole (regexp-match (string-append "^\\s*" howm-menu-here-regexp "\\s*$") line)))
    (if whole
        (howm-menu-value-lines (howm-menu-here (cadr whole)))
        (split-string (howm-replace-matches howm-menu-here-regexp line
                                            (lambda (line found)
                                              (howm-menu-value-text
                                               (howm-menu-here (howm-group line (cadr found))))))
                      "\n"))))

(define (howm-menu-strip-keys line)
  "Show the shortcuts of LINE as they look (%\"c\" to c, %\"fuga%\" to fuga)."
  (howm-replace-matches howm-menu-key-regexp line
                        (lambda (line found) (howm-group line (cadr found)))))

(define (howm-menu-render template)
  "Render the menu TEMPLATE into lines and shortcuts.
Each line is a string or (STRING FILE LINE)."
  (let ((raw (split-string
              (let loop ((text template) (rules howm-menu-display-rules))
                (if (null? rules)
                    text
                    (loop (howm-replace-all text (car (car rules)) (cadr (car rules))) (cdr rules))))
              "\n")))
    (let loop ((raw raw) (pending "") (lines '()) (keys '()))
      (if (null? raw)
          (list (reverse (if (string=? pending "")
                             lines
                             (append (reverse (howm-menu-expand pending)) lines)))
                (reverse keys))
          (let* ((visible (howm-menu-visible (car raw)))
                 (text (string-append pending (howm-menu-strip-keys (car visible))))
                 (keys (append (reverse (howm-menu-line-shortcuts (car raw))) keys)))
            (if (cadr visible)
                (loop (cdr raw) text lines keys)
                (loop (cdr raw) "" (append (reverse (howm-menu-expand text)) lines) keys)))))))

(define (howm-menu-item-lines lines)
  "Return the item lines of LINES, a list of (LINE FILE FILE-LINE)."
  (let loop ((lines lines) (n 1) (acc '()))
    (cond ((null? lines) (reverse acc))
          ((pair? (car lines))
           (loop (cdr lines) (+ n 1) (cons (list n (cadr (car lines)) (caddr (car lines))) acc)))
          (else (loop (cdr lines) (+ n 1) acc)))))

(define (howm-menu-key-command key)
  "Return the command of the shortcut KEY.
It runs the action of KEY in the current menu."
  (let ((command (string->symbol (string-append "howm-menu-key-" key))))
    (eval (list 'define-command (list command) (list 'howm-menu-run-key key)))
    command))

(define (howm-menu-run-key key)
  "Run the action of the shortcut KEY in the current menu."
  (let* ((data (howm-menu-current-data))
         (found (and data (assoc key (list-ref data 2)))))
    (when found
      (howm-run-action (cadr found)))))

(define (howm-menu-show name file template reload)
  "Show the menu NAME, such as %menu%, rendered from TEMPLATE.
FILE is the file of the template; RELOAD refreshes the menu."
  (unless (howm-menu-buffer?)
    (set! howm-menu-previous-buffer (buffer-name)))
  (let* ((rendered (howm-menu-render template))
         (lines (car rendered))
         (keys (filter (lambda (key) (regexp-match "^\\S$" (car key))) (cadr rendered)))
         (buffer (string-append "*howmM:" name "*")))
    (switch-to-buffer buffer)
    (howm-set-buffer-text
     (string-join (map (lambda (line) (if (string? line) line (car line))) lines) "\n"))
    (set! howm-menu-data
          (cons (list buffer reload keys (howm-menu-item-lines lines) file)
                (filter (lambda (data) (not (string=? (car data) buffer))) howm-menu-data)))
    (for-each (lambda (key) (local-set-key (car key) (howm-menu-key-command (car key)))) keys)
    (local-set-key "RET" 'howm-action)
    (howm-highlight
     (list (string-append "\\[(?:"
                          (string-join (map (lambda (b) (regexp-quote (car b))) howm-menu-buttons) "|")
                          ")\\]")
           "%(?:eval|call)%.*"))))

(define (howm-menu-legend)
  "Return the legend of the reminder types, for %here%."
  (format "[予定・Todo] … 予定@, 〆切! を %d 日先まで表示.\n〆切!, todo+, 覚書-, 保留~ を上位 %d 個まで表示."
          howm-menu-schedule-days howm-menu-todo-num))

(define (howm-reminder-lines reminders)
  "Return REMINDERS as menu lines."
  (map (lambda (r) (list (list-ref r 5) (list-ref r 3) (list-ref r 4))) reminders))

(define (howm-menu-schedule)
  "Return the schedule for the menu, for %here%."
  (howm-reminder-lines (howm-schedule howm-menu-schedule-days)))

(define (howm-menu-todo)
  "Return the todo items for the menu, for %here%."
  (howm-reminder-lines (howm-take (howm-todo) howm-menu-todo-num)))

(define (howm-menu-reminder)
  "Return the schedule and the todo items for the menu, for %here%."
  (append (howm-menu-schedule) (howm-menu-todo)))

(define (howm-menu-file-lines files)
  "Return FILES as menu lines."
  (map (lambda (file)
         (list (format "%s | %s" (howm-file-label file) (howm-title-or-label file)) file 1))
       files))

(define (howm-menu-recent)
  "Return the recent memos for the menu, for %here%."
  (howm-menu-file-lines (howm-take (howm-recent-files) howm-menu-recent-num)))

(define (howm-menu-random)
  "Return random memos for the menu, for %here%."
  (howm-menu-file-lines (howm-random-files howm-menu-random-num)))

(define (howm-menu-search keyword)
  "Return the memos matching KEYWORD for the menu, for %here%."
  (map (lambda (found)
         (list (format "%s | %s" (howm-file-label (car found)) (list-ref found 2))
               (car found) (cadr found)))
       (grep (howm-case-fold (regexp-quote keyword) keyword) (howm-files))))

(define (howm-menu-last-update)
  "Return the current date and time for the menu, for %here%."
  (format-time-string howm-dtime-format))

;;; フック (エディタが, ファイルを開いたときと, 保存したときに実行する)

(add-hook 'find-file-hook 'howm-find-file-hook)
(add-hook 'after-save-hook 'howm-after-save)

;;; キー (howm と同じく C-c , に続けて押す)

(global-set-key "C-c , ," 'howm-menu)
(global-set-key "C-c , c" 'howm-create)
(global-set-key "C-c , e" 'howm-remember)
(global-set-key "C-c , D" 'howm-dup)
(global-set-key "C-c , s" 'howm-search-fixed)
(global-set-key "C-c , g" 'howm-search-regexp)
(global-set-key "C-c , m" 'howm-search-migemo)
(global-set-key "C-c , o" 'howm-occur)
(global-set-key "C-c , h" 'howm-history)
(global-set-key "C-c , a" 'howm-list-all)
(global-set-key "C-c , l" 'howm-list-recent)
(global-set-key "C-c , A" 'howm-list-around)
(global-set-key "C-c , w" 'howm-random-walk)
(global-set-key "C-c , y" 'howm-list-schedule)
(global-set-key "C-c , t" 'howm-list-todo)
(global-set-key "C-c , ." 'howm-today)
(global-set-key "C-c , :" 'howm-yesterday)
(global-set-key "C-c , b" 'howm-list-buffers)
(global-set-key "C-c , r" 'howm-refresh)
(global-set-key "C-c , Q" 'howm-kill-all)
(global-set-key "C-c , K" 'howm-title-to-kill-ring)
(global-set-key "C-c , d" 'howm-insert-date)
(global-set-key "C-c , T" 'howm-insert-dtime)
(global-set-key "C-c , i" 'howm-insert-keyword)
(global-set-key "C-c , RET" 'howm-action)

(provide 'howm)

;;; howm.scm ends here
