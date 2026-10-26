;;; howm.scm --- howm (Hitori Otegaru Wiki Modoki) の Scheme 版 -*- scheme -*-

;; Emacs の howm (howm.el) を, rs-editor の Scheme で書き直したもの.
;;
;; 使い方: このファイルを ~/.rseditor.d/lisp/howm.scm に置き, 初期設定ファイル
;; (~/.rseditor.d/init.scm) に (require 'howm) と書く. M-x howm か C-c , , でメニューを出す.
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

;; メモを置くディレクトリ
(define howm-directory "~/howm")
;; メモのファイルの名前 (format-time-string の書式. howm-directory からのパス)
(define howm-file-name-format "%Y/%m/%Y-%m-%d-%H%M%S.txt")
;; メモのファイルの名前 (正規表現. これに合うファイルを, メモとして扱う)
(define howm-file-name-regexp "\\.txt$")
;; メモとして扱わないファイル (パスの正規表現. メニューのファイル, 履歴, バックアップなど)
(define howm-excluded-file-regexp
  "(^|/)([.]|(menu(_edit)?|0000-00-00-0+))|[~#]$|\\.bak$|/CVS/")
;; メニューのファイル (無ければ, 既定のメニューを書く)
(define howm-menu-file #f)
;; メニューに出す, 予定の日数 (今日から), todo の数, 最近のメモの数, ランダムなメモの数
(define howm-menu-schedule-days 7)
(define howm-menu-todo-num 50)
(define howm-menu-recent-num 20)
(define howm-menu-random-num 10)
;; メニューの %here% で呼べる手続き
(define howm-menu-allow
  '(howm-menu-legend howm-menu-schedule howm-menu-todo howm-menu-reminder
    howm-menu-recent howm-menu-random howm-menu-search howm-menu-last-update))
;; メモの題の行の始め
(define howm-view-title-header "=")
;; 日付と日時の書式 (メモを作るとき, 日付を入れるときに使う)
(define howm-date-format "[%Y-%m-%d]")
(define howm-dtime-format "[%Y-%m-%d %H:%M]")
;; 一覧で動くと, その行のメモを, もう 1 つの窓に出す (#f なら出さない)
(define howm-view-contents #t)
;; 検索の履歴のファイル (#f なら, howm-directory の .howm-history) と, 覚える数
(define howm-history-file #f)
(define howm-history-limit 50)
;; 日付を繰り返すとき (~yymmdd) に, 足す行の数の上限
(define howm-date-repeat-max 200)
;; メニュー, 一覧, 速記のバッファの名前
(define howm-menu-name "*howmM:%menu%*")
(define howm-summary-name "*howmS*")
(define howm-remember-name "*howm-remember*")

;; 既定のメニュー (howm の ja/0000-00-00-000000.txt から, この版で使えるものを残した)
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
")

;; ボタンの名前と, コマンド. menu と書いたものはメニューで, ほかはメニューを開く前のバッファで動かす
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
    ("menu 編集" howm-menu-edit menu)))

;; 今の一覧の項目 ((ファイル 行の番号 文字) のリスト. ファイルが #f なら, 文字を探す)
(define howm-summary-items '())
;; メニューのバッファごとの, ((バッファの名前 出し直す手続き ショートカット 項目 ファイル) ...)
(define howm-menu-data '())
;; メニューと速記を開く前のバッファの名前
(define howm-menu-previous-buffer #f)
(define howm-remember-previous-buffer #f)
;; 検索の履歴 (新しい順. 読む前は #f) と, キーワード ("<<< 言葉" の言葉. 集める前は #f)
(define howm-history-list #f)
(define howm-keyword-cache #f)

;;; 文字列とリスト

;; 文字列の中の from を, 全て to にする
(define (howm-replace-all text from to)
  (let loop ((start 0) (acc '()))
    (let ((found (string-search from text start)))
      (if found
          (loop (+ found (string-length from))
                (cons to (cons (substring text start found) acc)))
          (apply string-append (reverse (cons (substring text start) acc)))))))

;; リストの始めの n 個と, 始めの n 個を除いたもの
(define (howm-take items n)
  (if (or (null? items) (<= n 0))
      '()
      (cons (car items) (howm-take (cdr items) (- n 1)))))

(define (howm-drop items n)
  (if (or (null? items) (<= n 0))
      items
      (howm-drop (cdr items) (- n 1))))

;; 同じものを除く (始めのものを残す)
(define (howm-unique items)
  (let loop ((items items) (acc '()))
    (cond ((null? items) (reverse acc))
          ((member (car items) acc) (loop (cdr items) acc))
          (else (loop (cdr items) (cons (car items) acc))))))

;; 0 から n - 1 までの数
(define (howm-range n)
  (let loop ((i (- n 1)) (acc '()))
    (if (< i 0) acc (loop (- i 1) (cons i acc)))))

;; リストの中の位置 (0 から. 無ければ #f)
(define (howm-index-of item items)
  (let loop ((items items) (i 0))
    (cond ((null? items) #f)
          ((equal? (car items) item) i)
          (else (loop (cdr items) (+ i 1))))))

;; ランダムに 1 つ
(define (howm-pick items)
  (list-ref items (random (length items))))

;; 数の切り捨て
(define (howm-floor x)
  (- x (modulo x 1)))

;; 大文字を含まなければ, 大文字と小文字を区別しない正規表現にする (howm と同じ)
(define (howm-case-fold regexp keyword)
  (if (string=? keyword (string-downcase keyword))
      (string-append "(?i)" regexp)
      regexp))

;; 行の中の, 正規表現に合う所の位置のリスト. それぞれ, 合った所と括弧ごとの (始め 終わり) (文字の数. 合わなかった括弧は #f)
(define (howm-matches regexp line)
  (let loop ((start 0) (acc '()))
    (let ((found (and (<= start (string-length line))
                      (regexp-match-positions regexp line start))))
      (if found
          (loop (max (cadr (car found)) (+ (car (car found)) 1)) (cons found acc))
          (reverse acc)))))

;; 位置 ((始め 終わり)) の文字
(define (howm-group line position)
  (substring line (car position) (cadr position)))

;; 正規表現に合う所を, 全て (proc 行 位置) の文字にする
(define (howm-replace-matches regexp line proc)
  (let loop ((matches (howm-matches regexp line)) (pos 0) (acc '()))
    (if (null? matches)
        (apply string-append (reverse (cons (substring line pos) acc)))
        (let ((span (car (car matches))))
          (loop (cdr matches)
                (cadr span)
                (cons (proc line (car matches)) (cons (substring line pos (car span)) acc)))))))

;; 終わりの空の行を除く
(define (howm-trim-trailing-lines lines)
  (let loop ((rest (reverse lines)))
    (if (and (pair? rest) (string=? (car rest) ""))
        (loop (cdr rest))
        (reverse rest))))

;; 2 桁の数 (1 → "01")
(define (howm-pad2 n)
  (if (< n 10) (string-append "0" (number->string n)) (number->string n)))

;;; 日付 (2026-10-25 の形)

(define howm-day (* 24 60 60))
(define howm-day-of-week-names '("日" "月" "火" "水" "木" "金" "土"))

;; 日付の時刻 (その日の昼. 夏時間でずれないように)
(define (howm-date-time date)
  (date-to-time (string-append date " 12:00")))

;; 今日の日付
(define (howm-today-date)
  (format-time-string "%Y-%m-%d"))

;; 日付の n 日後
(define (howm-date-shift date days)
  (format-time-string "%Y-%m-%d" (+ (howm-date-time date) (* days howm-day))))

;; 曜日 (日, 月, ...)
(define (howm-day-of-week date)
  (list-ref howm-day-of-week-names
            (string->number (format-time-string "%w" (howm-date-time date)))))

;; 年, 月, 日の数と, 日付
(define (howm-date-parts date)
  (map string->number (split-string date "-")))

(define (howm-make-date y m d)
  (string-append (number->string y) "-" (howm-pad2 m) "-" (howm-pad2 d)))

;; 月の日数
(define (howm-days-in-month y m)
  (cond ((= m 2)
         (if (and (= (modulo y 4) 0) (or (not (= (modulo y 100) 0)) (= (modulo y 400) 0)))
             29
             28))
        ((member m '(4 6 9 11)) 30)
        (else 31)))

;; 日付の n か月後 (日は, その月の終わりまでにする)
(define (howm-date-add-months date months)
  (let* ((parts (howm-date-parts date))
         (index (+ (* (car parts) 12) (- (cadr parts) 1) months))
         (y (quotient index 12))
         (m (+ (modulo index 12) 1)))
    (howm-make-date y m (min (caddr parts) (howm-days-in-month y m)))))

;; 数字 (yyyymmdd, yymmdd, mmdd, dd) の日付. 省いた所 (と 0 の所) は, base の年, 月, 日にする
(define (howm-date-expand digits base)
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

;; メモのディレクトリ (絶対パス)
(define (howm-dir)
  (expand-file-name howm-directory))

;; メモのファイル (名前の順). 除くファイルは, howm-directory からのパスで決める
(define (howm-files)
  (let ((skip (+ (string-length (howm-dir)) 1)))
    (filter (lambda (file)
              (not (regexp-match howm-excluded-file-regexp (substring file skip))))
            (directory-files-recursively (howm-dir) howm-file-name-regexp))))

;; howm-directory の下のファイルなら #t
(define (howm-memo-file? file)
  (and (string? file) (string-prefix? (string-append (howm-dir) "/") file)))

;; ファイルの名前 (ディレクトリと .txt を除く. 一覧に出す)
(define (howm-file-label file)
  (cadr (regexp-match "([^/]*?)(\\.txt)?$" file)))

;; ファイルのディレクトリ
(define (howm-file-directory file)
  (cadr (regexp-match "^(.*)/[^/]*$" file)))

;; 題の行なら #t ("= " で始まる行)
(define (howm-title-line? line)
  (or (string=? line howm-view-title-header)
      (string-prefix? (string-append howm-view-title-header " ") line)))

;; 題の行の, 題
(define (howm-title-of line)
  (string-trim (substring line (string-length howm-view-title-header))))

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
        (let ((picked (howm-pick files)))
          (loop (filter (lambda (f) (not (equal? f picked))) files)
                (- n 1)
                (cons picked acc))))))

;; 新しいメモのファイルのパス (同じ名前のファイルがあれば, 1 秒ずつ後の時刻にする)
(define (howm-new-file)
  (let loop ((time (current-time)))
    (let ((file (string-append (howm-dir) "/" (format-time-string howm-file-name-format time))))
      (if (file-exists? file) (loop (+ time 1)) file))))

;; 新しいメモを書く (題, 日時, 本文の行)
(define (howm-write-memo title lines)
  (let ((file (howm-new-file)))
    (write-file-string file
                       (string-append howm-view-title-header " " title "\n"
                                      (format-time-string howm-dtime-format) "\n"
                                      (string-join lines "\n") "\n"))
    (set! howm-keyword-cache #f)
    file))

;; キーワード (メモの "<<< 言葉" の言葉. 集めたものを使い回す)
(define howm-come-from-regexp "<<<\\s*(.*\\S)")

(define (howm-keywords)
  (unless howm-keyword-cache
    (set! howm-keyword-cache
          (howm-unique
           (map (lambda (found) (cadr (regexp-match howm-come-from-regexp (list-ref found 2))))
                (grep howm-come-from-regexp (howm-files))))))
  howm-keyword-cache)

;; キーワードのどれかに合う正規表現 (長いものを先に)
(define (howm-keyword-regexp keywords)
  (string-join (map regexp-quote
                    (sort keywords (lambda (a b) (> (string-length a) (string-length b)))))
               "|"))

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

;; カーソルの行の始めの位置
(define (howm-current-line-start)
  (howm-line-start (buffer-string) (howm-current-line)))

;; カーソルの行の文字と, 行の中の位置 (0 から)
(define (howm-current-line-and-column)
  (let* ((text (buffer-string))
         (start (howm-line-start text (howm-current-line)))
         (rest (substring text (- start 1)))
         (end (string-search "\n" rest)))
    (list (if end (substring rest 0 end) rest) (- (point) start))))

;; カーソルの行の, from から to まで (行の中の位置) を, text にする. カーソルは動かさない
(define (howm-replace-in-line from to text)
  (let ((start (howm-current-line-start))
        (here (point)))
    (delete-region (+ start from) (+ start to))
    (goto-char (+ start from))
    (insert text)
    (goto-char here)))

;; 行の中の, line 行目 (1 から) を含むメモ (題の行から, 次の題の行の前まで) の, 始めと終わりの行 (0 から. 終わりは含まない)
(define (howm-section-range lines line)
  (let loop ((rest lines) (i 0) (start 0))
    (cond ((null? rest) (list start i))
          ((and (howm-title-line? (car rest)) (> i start))
           (if (< i line) (loop (cdr rest) (+ i 1) i) (list start i)))
          (else (loop (cdr rest) (+ i 1) start)))))

;; 行の中の, 範囲の行
(define (howm-lines-in lines range)
  (howm-take (howm-drop lines (car range)) (- (cadr range) (car range))))

;; カーソルのメモ (題の行から, 次の題の行の前まで) の行
(define (howm-current-section)
  (let ((lines (split-string (buffer-string) "\n")))
    (howm-lines-in lines (howm-section-range lines (howm-current-line)))))

;; カーソルのメモの題 (無ければ #f)
(define (howm-current-title)
  (let ((titles (filter howm-title-line? (howm-current-section))))
    (and (pair? titles)
         (not (string=? (howm-title-of (car titles)) ""))
         (howm-title-of (car titles)))))

;; バッファがあれば, そこへ移る
(define (howm-switch-back name)
  (when (and name (member name (buffer-list)))
    (switch-to-buffer name)))

;; ファイルを開いて, 行へ動く
(define (howm-open file line)
  (find-file file)
  (goto-char (howm-line-start (buffer-string) line))
  (howm-mode))

;; メモのバッファにする (RET で動かし, 下線を引く. キーワードは集め直す)
(define (howm-mode)
  (set! howm-keyword-cache #f)
  (local-set-key "RET" 'howm-action-or-newline)
  (howm-highlight '()))

;; 下線を引く正規表現 (画面のエディタの, JavaScript の正規表現で書く)
(define howm-highlight-regexps
  '(">>>[ \\t]*\\S.*"
    "<<<[ \\t]*\\S.*"
    "\\[[0-9]{4}-[0-9]{2}-[0-9]{2}\\]([-+@!.~][0-9]*)?"
    "\\[\\[[^\\]\\n]+\\]\\]"
    "file://\\S+"))

;; 下線を引き直す (リンク, 日付, キーワードと, extra の正規表現)
(define (howm-highlight extra)
  (let ((keywords (howm-keywords)))
    (unhighlight-regexp #t)
    (for-each highlight-regexp (append howm-highlight-regexps extra))
    (unless (null? keywords)
      (highlight-regexp (howm-keyword-regexp keywords)))))

;;; 一覧 (*howmS*)

;; 一覧の 1 行
(define (howm-summary-line item)
  (if (car item)
      (format "%s | %s" (howm-file-label (car item)) (list-ref item 2))
      (list-ref item 2)))

;; 項目 ((ファイル 行の番号 文字) のリスト) を, 一覧のバッファに出す
(define (howm-show-summary items empty-message)
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

;; カーソルの行の項目 (無ければ #f)
(define (howm-summary-item)
  (let ((n (howm-current-line)))
    (and (<= n (length howm-summary-items))
         (list-ref howm-summary-items (- n 1)))))

;; カーソルの行のメモを, もう 1 つの窓に出す
(define (howm-summary-contents)
  (let ((item (howm-summary-item)))
    (when (and howm-view-contents item (car item))
      (display-buffer (find-file-noselect (car item)) (cadr item)))))

;; カーソルの行の項目を開く (ファイルが無ければ, 文字を探す)
(define (howm-summary-open)
  (let ((item (howm-summary-item)))
    (cond ((not item) #f)
          ((car item) (howm-open (car item) (cadr item)))
          (else (howm-search-keyword-with-history (list-ref item 2))))))

;; 次と前の行へ動いて, その行のメモを出す (一覧の n と p)
(define-command (howm-summary-next)
  (call-interactively 'next-line)
  (howm-summary-contents))

(define-command (howm-summary-previous)
  (call-interactively 'previous-line)
  (howm-summary-contents))

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

;; 履歴に残して, 言葉で探す
(define (howm-search-keyword-with-history keyword)
  (howm-history-add keyword)
  (howm-search-keyword keyword))

;; 言葉を (キーワードを補完して) 聞いて探す (C-c , s. メニューの s)
(define-command (howm-search-fixed)
  (let ((keyword (completing-read "Search: " (howm-keywords))))
    (unless (string=? keyword "")
      (howm-search-keyword-with-history keyword))))

;; 正規表現を聞いて探す (C-c , g. メニューの g)
(define-command (howm-search-regexp)
  (let ((regexp (read-string "Search (regexp): ")))
    (unless (string=? regexp "")
      (howm-history-add regexp)
      (howm-search (howm-case-fold regexp regexp) regexp))))

;; 今日から n 日後の日付の, メモ
(define (howm-search-day days)
  (howm-search-keyword (string-append "[" (howm-date-shift (howm-today-date) days))))

;; 今日と昨日の日付のメモ (メニューの . と :)
(define-command (howm-today) (howm-search-day 0))
(define-command (howm-yesterday) (howm-search-day -1))

;; 今のバッファの中を, 正規表現で探す (C-c , o. メニューの o)
(define-command (howm-occur)
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
    ("-" "ー")))

(define howm-hiragana
  "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔ")
(define howm-katakana
  "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴ")

;; ローマ字の, i 文字目からの綴り ((ローマ字 かな). 長いものを先に. 無ければ #f)
(define (howm-roma-lookup roma i)
  (let loop ((n 3))
    (and (> n 0)
         (or (and (<= (+ i n) (string-length roma))
                  (assoc (substring roma i (+ i n)) howm-roma-table))
             (loop (- n 1))))))

;; ローマ字を, ひらがなにする (かなにできない文字は, そのまま)
(define (howm-roma-to-hiragana roma)
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

;; ひらがなを, カタカナにする
(define (howm-hiragana-to-katakana text)
  (apply string-append
         (map (lambda (i)
                (let* ((c (substring text i (+ i 1)))
                       (found (string-search c howm-hiragana)))
                  (if found (substring howm-katakana found (+ found 1)) c)))
              (howm-range (string-length text)))))

;; ローマ字, ひらがな, カタカナのどれかに合う正規表現
(define (howm-migemo-regexp roma)
  (let ((hiragana (howm-roma-to-hiragana (string-downcase roma))))
    (string-append "(?i)(?:"
                   (string-join (map regexp-quote
                                     (howm-unique
                                      (list roma hiragana (howm-hiragana-to-katakana hiragana))))
                                "|")
                   ")")))

;; ローマ字を聞いて探す (C-c , m. メニューの m)
(define-command (howm-search-migemo)
  (let ((roma (read-string "Search (roma): ")))
    (unless (string=? roma "")
      (howm-history-add roma)
      (howm-search (howm-migemo-regexp roma) roma))))

;;; 履歴

;; 履歴のファイル
(define (howm-history-path)
  (or howm-history-file (string-append (howm-dir) "/.howm-history")))

;; 履歴 (新しい順. 始めに, ファイルから読む)
(define (howm-history-load)
  (unless howm-history-list
    (set! howm-history-list
          (if (file-exists? (howm-history-path))
              (filter (lambda (k) (not (string=? k "")))
                      (split-string (read-file-string (howm-history-path)) "\n"))
              '())))
  howm-history-list)

;; 履歴に足して, ファイルに書く
(define (howm-history-add keyword)
  (set! howm-history-list
        (howm-take (cons keyword (filter (lambda (k) (not (string=? k keyword))) (howm-history-load)))
                   howm-history-limit))
  (write-file-string (howm-history-path) (string-join howm-history-list "\n")))

;; 履歴の一覧 (C-c , h. RET で, その言葉を探す)
(define-command (howm-history)
  (howm-show-summary (map (lambda (k) (list #f 0 k)) (howm-history-load)) "No history"))

;;; 一覧

;; 全てのメモの一覧 (新しい順. C-c , a)
(define-command (howm-list-all)
  (howm-show-summary (howm-file-items (reverse (howm-files))) "No memo"))

;; 最近のメモの一覧 (C-c , l)
(define-command (howm-list-recent)
  (howm-show-summary (howm-file-items (howm-take (howm-recent-files) howm-menu-recent-num))
                     "No memo"))

;; 前後のメモ (全てのメモの一覧で, 今のメモの行にカーソルを置く. C-c , A)
(define-command (howm-list-around)
  (let ((file (buffer-file-name))
        (files (reverse (howm-files))))
    (howm-show-summary (howm-file-items files) "No memo")
    (let ((index (howm-index-of file files)))
      (when index
        (goto-char (howm-line-start (buffer-string) (+ index 1)))
        (howm-summary-contents)))))

;; 今のバッファの, 探せる言葉 (リンクとキーワード)
(define (howm-buffer-links)
  (howm-unique
   (apply append
          (map (lambda (line)
                 (map (lambda (action) (cadr (list-ref action 2)))
                      (filter (lambda (action)
                                (member (car (list-ref action 2)) '("search" "wiki")))
                              (howm-line-actions line #f))))
               (split-string (buffer-string) "\n")))))

;; 酔歩 (C-c , w): 一覧なら, ランダムな行を開く. メモなら, ランダムなリンクを探す.
;; リンクが無ければ, ランダムなメモを開く. 1 回に 1 歩なので, 続けるときは, もう一度呼ぶ
(define-command (howm-random-walk)
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

;; 予定と todo の正規表現 ([2026-10-25]+7 の, 日付, 種類, 猶予, 残り)
(define howm-reminder-regexp
  "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\]([-+@!.~])([0-9]*)(.*)$")
;; RET で動かす所 (日付, 種類と猶予, 種類, 猶予)
(define howm-reminder-action-regexp
  "\\[([0-9]{4}-[0-9]{2}-[0-9]{2})\\](([-+@!.~])([0-9]*))")

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
                  (late (/ (- now (date-to-time date)) howm-day)))
             (list (howm-priority type late laziness) date type
                   (car found) (cadr found) (car parts))))
         (grep howm-reminder-regexp (howm-files)))))

;; 項目を, 一覧の項目 ((ファイル 行の番号 文字)) にする
(define (howm-reminder-item reminder)
  (list (list-ref reminder 3) (list-ref reminder 4) (list-ref reminder 5)))

;; 予定 (@) を日付の順に. days が数なら, 今日からその日数先まで
(define (howm-schedule days)
  (let ((today (howm-today-date))
        (last (howm-date-shift (howm-today-date) (or days 0))))
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

;; 予定と todo の一覧 (C-c , y と C-c , t)
(define-command (howm-list-schedule)
  (howm-show-summary (map howm-reminder-item (howm-schedule #f)) "No schedule"))

(define-command (howm-list-todo)
  (howm-show-summary (map howm-reminder-item (howm-todo)) "No todo"))

;; 予定と todo の記号の上の RET: 済み (RET), 取り消し (x), 種類 (記号) と猶予 (数) を変える.
;; from は, 行の中の, 日付の位置
(define (howm-reminder-action from)
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

;; 日付の上の RET: 一覧 (RET. new? なら何もしない), ずらす (+3, -1), 変える (yymmdd), 今日 (.),
;; 繰り返す (~yymmdd), 1 日ずつずらす ([ と ]). 日付は, カーソルの行の from から to まで
(define (howm-date-action date from to new?)
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

;; 1 日ずつずらす (続けて [ か ] を入れる. . は今日. ほかは終わり)
(define (howm-date-quickshift days date replace)
  (let ((new (howm-date-shift date days)))
    (replace new)
    (let ((c (read-string (format "%s [%s] RET(ok), [(-1), ](+1), .(today): "
                                  new (howm-day-of-week new)))))
      (cond ((string=? c "[") (howm-date-quickshift -1 new replace))
            ((string=? c "]") (howm-date-quickshift 1 new replace))
            ((string=? c ".") (howm-date-quickshift 0 (howm-today-date) replace))
            (else #t)))))

;; カーソルの行を, until の日付まで, 日付を変えて繰り返す (下に足す)
(define (howm-date-repeat until date from to)
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

;; 新しいメモを作って開く (題の行と日時を入れる. 題が空なら, 題の所にカーソルを置く)
(define (howm-create-memo title)
  (let ((file (howm-new-file)))
    (make-directory (howm-file-directory file))
    (find-file file)
    (insert howm-view-title-header " " title "\n" (format-time-string howm-dtime-format) "\n\n")
    (when (string=? title "")
      (goto-char (+ (string-length howm-view-title-header) 2)))
    (howm-mode)))

;; 新しいメモ (C-c , c)
(define-command (howm-create)
  (howm-create-memo ""))

;; [[言葉]] の上の RET: "<<< 言葉" のメモが無ければ作り, あれば探す. %名前% なら, そのメニューを開く
(define (howm-wiki keyword)
  (cond ((and (howm-menu-keyword? keyword) (howm-menu-find-section keyword))
         (howm-menu-open-section keyword))
        ((null? (grep (string-append "<<<\\s*" (regexp-quote keyword)) (howm-files)))
         (howm-create-memo (string-append "<<< " keyword)))
        (else (howm-search-keyword-with-history keyword))))

;; 速記 (C-c , e): *howm-remember* に書いて, C-c C-c で新しいメモにする (1 行目が題). C-c C-k で取り消す
(define-command (howm-remember)
  (unless (string=? (buffer-name) howm-remember-name)
    (set! howm-remember-previous-buffer (buffer-name)))
  (switch-to-buffer howm-remember-name)
  (local-set-key "C-c C-c" 'howm-remember-submit)
  (local-set-key "C-c C-k" 'howm-remember-discard)
  (message "C-c C-c: save, C-c C-k: discard"))

;; 速記のバッファを消して, 前のバッファに戻る
(define (howm-remember-close)
  (let ((buffer (buffer-name)))
    (set-buffer-modified-p #f)
    (howm-switch-back howm-remember-previous-buffer)
    (kill-buffer buffer)))

(define-command (howm-remember-submit)
  (let ((text (string-trim (buffer-string))))
    (if (string=? text "")
        (message "Empty memo")
        (let* ((lines (split-string text "\n"))
               (file (howm-write-memo (car lines) (cons "" (cdr lines)))))
          (howm-remember-close)
          (message "Saved: %s" file)))))

(define-command (howm-remember-discard)
  (howm-remember-close)
  (message "Discarded"))

;; 複製 (C-c , D): カーソルのメモ (題の行から, 次の題の行の前まで) を, 新しいメモにして開く
(define-command (howm-dup)
  (let ((file (howm-new-file)))
    (write-file-string file
                       (string-append (string-join (howm-trim-trailing-lines (howm-current-section)) "\n")
                                      "\n"))
    (howm-open file 1)))

;;; コピーと挿入

;; カーソルのメモの題を, kill ring に入れる (無ければ, ファイルの名前. C-c , K)
(define-command (howm-title-to-kill-ring)
  (let ((text (or (howm-current-title)
                  (let ((file (buffer-file-name)))
                    (if file (howm-file-label file) (buffer-name))))))
    (kill-new text)
    (message "%s" text)))

;; ファイルの名前を, kill ring に入れる
(define-command (howm-name-to-kill-ring)
  (let ((text (or (buffer-file-name) (buffer-name))))
    (kill-new text)
    (message "%s" text)))

;; キーワードを補完して入れる (C-c , i)
(define-command (howm-insert-keyword)
  (let ((keyword (completing-read "Keyword: " (howm-keywords))))
    (unless (string=? keyword "")
      (insert keyword))))

;; 今日の日付を入れて, 日付の RET と同じく変えられるようにする (C-c , d)
(define-command (howm-insert-date)
  (let ((column (cadr (howm-current-line-and-column)))
        (date (format-time-string howm-date-format)))
    (insert date)
    (howm-date-action (howm-today-date) column (+ column (string-length date)) #t)))

;; 今の日時を入れる (C-c , T)
(define-command (howm-insert-dtime)
  (insert (format-time-string howm-dtime-format)))

;;; バッファ

;; howm のバッファ (メモ, メニュー, 一覧, 速記) なら #t
(define (howm-buffer? name)
  (or (string-prefix? "*howm" name)
      (howm-memo-file? (buffer-file-name name))))

;; howm のバッファを全て消す (書き換えたものは残す. C-c , Q)
(define-command (howm-kill-all)
  (let loop ((names (buffer-list)) (n 0))
    (cond ((null? names) (message "%d buffers killed" n))
          ((and (howm-buffer? (car names)) (not (buffer-modified-p (car names))))
           (kill-buffer (car names))
           (loop (cdr names) (+ n 1)))
          (else (loop (cdr names) n)))))

;; バッファの一覧 (C-c , b)
(define-command (howm-list-buffers)
  (call-interactively 'list-buffers))

;; 下線を引き直す (メニューなら, 出し直す. C-c , r)
(define-command (howm-refresh)
  (if (howm-menu-buffer?)
      (howm-menu-refresh)
      (begin (howm-mode) (message "Refreshed"))))

;; 初期設定ファイルを開く ([設定])
(define-command (howm-settings)
  (if user-init-file
      (find-file user-init-file)
      (message "No init file")))

;;; RET (ボタン, リンク, 日付, 予定と todo, 一覧とメニューの項目)

;; >>> の言葉 (%名前% なら, そのメニューを開く)
(define (howm-link-action keyword)
  (if (howm-menu-keyword? keyword) (list "menu" keyword) (list "search" keyword)))

;; RET で動く所の規則 ((正規表現 位置の括弧 動きを作る手続き) のリスト). 先のものを優先する
(define (howm-action-rules menu?)
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

;; 行の中の, RET で動く所 ((始め 終わり 動き) のリスト. 規則の順)
(define (howm-line-actions line menu?)
  (apply append
         (map (lambda (rule)
                (filter (lambda (x) x)
                        (map (lambda (found)
                               (let ((action ((caddr rule) line found))
                                     (span (list-ref found (cadr rule))))
                                 (and action (list (car span) (cadr span) action))))
                             (howm-matches (car rule) line))))
              (howm-action-rules menu?))))

;; 行の中の, 位置 (0 から) で動くもの (無ければ #f)
(define (howm-action-at line column menu?)
  (let loop ((actions (howm-line-actions line menu?)))
    (cond ((null? actions) #f)
          ((and (<= (car (car actions)) column) (< column (cadr (car actions))))
           (list-ref (car actions) 2))
          (else (loop (cdr actions))))))

;; 動かす
(define (howm-run-action action)
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

;; カーソルの所のものを動かす. 動かしたら #t
(define (howm-action-at-point)
  (let* ((here (howm-current-line-and-column))
         (action (howm-action-at (car here) (cadr here) (howm-menu-buffer?))))
    (and action (begin (howm-run-action action) #t))))

;; メニューの項目を開く. 予定と todo の記号か日付の上なら, メモのその所で RET を押す. 開いたら #t
(define (howm-menu-open-item items)
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

;; RET: 一覧なら, その行のメモを開く. メニューの項目なら開く. ボタンやリンクなら動かす
(define-command (howm-action)
  (cond ((string=? (buffer-name) howm-summary-name) (howm-summary-open))
        ((and (howm-menu-buffer?) (howm-menu-open-item (howm-menu-items))) #t)
        ((howm-action-at-point) #t)
        (else (message "No action here"))))

;; 動くものがあれば動かし, 無ければ改行する (メモのバッファの RET)
(define-command (howm-action-or-newline)
  (unless (howm-action-at-point)
    (call-interactively 'newline)))

;;; メニュー

;; メニューのバッファなら #t
(define (howm-menu-buffer?)
  (string-prefix? "*howmM:" (buffer-name)))

;; %名前% の形なら #t
(define (howm-menu-keyword? keyword)
  (regexp-match "^%.*%$" keyword))

;; ボタン ((名前 コマンド [menu])). 無ければ #f
(define (howm-button label)
  (assoc label howm-menu-buttons))

;; メニューを開く前のバッファに戻って, 動かす
(define (howm-menu-in-previous thunk)
  (when (howm-menu-buffer?)
    (howm-switch-back howm-menu-previous-buffer))
  (thunk))

;; ボタンを押す
(define (howm-run-button label)
  (let ((button (howm-button label)))
    (if (= (length button) 3)
        (call-interactively (cadr button))
        (howm-menu-in-previous (lambda () (call-interactively (cadr button)))))))

;; メニューのファイル (無ければ, 既定のメニューを書く)
(define (howm-menu-path)
  (let ((file (or howm-menu-file (string-append (howm-dir) "/0000-00-00-000000.txt"))))
    (unless (file-exists? file)
      (write-file-string file howm-default-menu))
    (expand-file-name file)))

;; 文字の中の, "<<< 言葉" の題の行から, 次の題の行の前まで (無ければ #f)
(define (howm-section-of text keyword)
  (let ((lines (split-string text "\n"))
        (regexp (string-append "<<<\\s*" (regexp-quote keyword) "\\s*$")))
    (let loop ((rest lines) (n 1))
      (cond ((null? rest) #f)
            ((and (howm-title-line? (car rest)) (regexp-match regexp (car rest)))
             (string-join (howm-lines-in lines (howm-section-range lines n)) "\n"))
            (else (loop (cdr rest) (+ n 1)))))))

;; "<<< %名前%" のメニュー ((ファイル 文字). メニューのファイル, メモの順に探す. 無ければ #f)
(define (howm-menu-find-section keyword)
  (let loop ((files (cons (howm-menu-path) (howm-files))))
    (if (null? files)
        #f
        (let ((text (howm-section-of (read-file-string (car files)) keyword)))
          (if text (list (car files) text) (loop (cdr files)))))))

;; 今のメニューのバッファの, データ (無ければ #f) と項目
(define (howm-menu-current-data)
  (assoc (buffer-name) howm-menu-data))

(define (howm-menu-items)
  (let ((data (howm-menu-current-data)))
    (if data (list-ref data 3) '())))

;; メニューを出す (M-x howm, C-c , ,)
(define-command (howm-menu)
  (let* ((file (howm-menu-path))
         (text (read-file-string file)))
    (howm-menu-show "%menu%" file (or (howm-section-of text "%menu%") text)
                    (lambda () (howm-menu)))))

(define-command (howm) (howm-menu))

;; "<<< %名前%" のメニューを出す (>>> %名前%)
(define (howm-menu-open-section keyword)
  (let ((found (howm-menu-find-section keyword)))
    (if found
        (howm-menu-show keyword (car found) (cadr found)
                        (lambda () (howm-menu-open-section keyword)))
        (message "No menu: %s" keyword))))

;; ファイルを, メニューとして出す (howm-directory からのパス. %eval%(howm-menu-open "file.txt"))
(define (howm-menu-open file)
  (let ((path (expand-file-name file (howm-dir))))
    (howm-menu-show (howm-file-label path) path (read-file-string path)
                    (lambda () (howm-menu-open file)))))

;; メニューを出し直す ([menu 更新])
(define-command (howm-menu-refresh)
  (let ((data (howm-menu-current-data)))
    (if data ((list-ref data 1)) (howm-menu))))

;; メニューのファイルを開く ([menu 編集])
(define-command (howm-menu-edit)
  (let ((data (howm-menu-current-data)))
    (find-file (if data (list-ref data 4) (howm-menu-path)))
    (howm-mode)))

;; メニューのショートカット (%"c"[新規] と %"fuga%") の正規表現 (言葉, キー, 言葉の続き, 動くだけの %)
(define howm-menu-key-regexp "%\"(([^\\n%\"])[^\\n%\"]*(%+[^\\n%\"]+)*)(%)?\"")

;; メニューの元の 1 行の, ショートカット ((キー 動き) のリスト). " の後の, 最初の動くものを動かす
(define (howm-menu-line-shortcuts line)
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

;; %| で切り替える, 見せる所だけの文字と, 行の終わりを見せないなら #t (改行も消す)
(define (howm-menu-visible line)
  (let loop ((parts (split-string line "%|")) (visible #t) (acc '()))
    (let ((acc (if visible (cons (car parts) acc) acc)))
      (if (null? (cdr parts))
          (list (apply string-append (reverse acc)) (not visible))
          (loop (cdr parts) (not visible) acc)))))

;; %sdays などを, %here% の形にする
(define howm-menu-display-rules
  '(("%sdays" "%here%howm-menu-schedule-days")
    ("%tnum" "%here%howm-menu-todo-num")
    ("%schedule" "%here%(howm-menu-schedule)")
    ("%todo" "%here%(howm-menu-todo)")
    ("%reminder" "%here%(howm-menu-reminder)")
    ("%recent" "%here%(howm-menu-recent)")
    ("%random" "%here%(howm-menu-random)")
    ("%last-update" "%here%(howm-menu-last-update)")))

(define howm-menu-here-regexp "%here%(\\([^()\\n]*\\)|[^\\s()\",;']+)")

;; %here% の式の値 (変数の値か, howm-menu-allow の手続きを呼んだ値. 引数は評価しない)
(define (howm-menu-here text)
  (let ((expr (read text)))
    (cond ((symbol? expr) (eval expr))
          ((and (pair? expr) (member (car expr) howm-menu-allow))
           (apply (eval (car expr)) (cdr expr)))
          (else (format "Not allowed: %s" text)))))

;; 値を, メニューの行 (文字か (文字 ファイル 行の番号)) のリストにする
(define (howm-menu-value-lines value)
  (cond ((string? value) (split-string value "\n"))
        ((list? value)
         (map (lambda (line)
                (if (or (string? line) (and (pair? line) (string? (car line))))
                    line
                    (format "%s" line)))
              value))
        (else (list (format "%s" value)))))

;; 値を, 文字にする
(define (howm-menu-value-text value)
  (string-join (map (lambda (line) (if (string? line) line (car line)))
                    (howm-menu-value-lines value))
               "\n"))

;; 見せる 1 行の %here% を, 値にする (行が %here% だけなら, 値の行に置き換える)
(define (howm-menu-expand line)
  (let ((whole (regexp-match (string-append "^\\s*" howm-menu-here-regexp "\\s*$") line)))
    (if whole
        (howm-menu-value-lines (howm-menu-here (cadr whole)))
        (split-string (howm-replace-matches howm-menu-here-regexp line
                                            (lambda (line found)
                                              (howm-menu-value-text
                                               (howm-menu-here (howm-group line (cadr found))))))
                      "\n"))))

;; ショートカットを, 見せる形にする (%"c" → c. %"fuga%" → fuga)
(define (howm-menu-strip-keys line)
  (howm-replace-matches howm-menu-key-regexp line
                        (lambda (line found) (howm-group line (cadr found)))))

;; メニューの元を, 行 (文字か (文字 ファイル 行の番号)) とショートカットにする
(define (howm-menu-render template)
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

;; 行の中の, 項目の行 ((行の番号 ファイル 行の番号))
(define (howm-menu-item-lines lines)
  (let loop ((lines lines) (n 1) (acc '()))
    (cond ((null? lines) (reverse acc))
          ((pair? (car lines))
           (loop (cdr lines) (+ n 1) (cons (list n (cadr (car lines)) (caddr (car lines))) acc)))
          (else (loop (cdr lines) (+ n 1) acc)))))

;; ショートカットのキーのコマンド (キーごとに作る. 今のメニューの, そのキーの動きを動かす)
(define (howm-menu-key-command key)
  (let ((command (string->symbol (string-append "howm-menu-key-" key))))
    (eval (list 'define-command (list command) (list 'howm-menu-run-key key)))
    command))

(define (howm-menu-run-key key)
  (let* ((data (howm-menu-current-data))
         (found (and data (assoc key (list-ref data 2)))))
    (when found
      (howm-run-action (cadr found)))))

;; メニューを出す. name は %menu% など, file は元のファイル, reload は出し直す手続き
(define (howm-menu-show name file template reload)
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

;; %here% で呼べる手続き (予定と todo の説明, 予定, todo, 最近, ランダム, 検索, 出した時刻)
(define (howm-menu-legend)
  (format "[予定・Todo] … 予定@, 〆切! を %d 日先まで表示.\n〆切!, todo+, 覚書-, 保留~ を上位 %d 個まで表示."
          howm-menu-schedule-days howm-menu-todo-num))

(define (howm-reminder-lines reminders)
  (map (lambda (r) (list (list-ref r 5) (list-ref r 3) (list-ref r 4))) reminders))

(define (howm-menu-schedule)
  (howm-reminder-lines (howm-schedule howm-menu-schedule-days)))

(define (howm-menu-todo)
  (howm-reminder-lines (howm-take (howm-todo) howm-menu-todo-num)))

(define (howm-menu-reminder)
  (append (howm-menu-schedule) (howm-menu-todo)))

(define (howm-menu-file-lines files)
  (map (lambda (file)
         (list (format "%s | %s" (howm-file-label file) (howm-title-or-label file)) file 1))
       files))

(define (howm-menu-recent)
  (howm-menu-file-lines (howm-take (howm-recent-files) howm-menu-recent-num)))

(define (howm-menu-random)
  (howm-menu-file-lines (howm-random-files howm-menu-random-num)))

(define (howm-menu-search keyword)
  (map (lambda (found)
         (list (format "%s | %s" (howm-file-label (car found)) (list-ref found 2))
               (car found) (cadr found)))
       (grep (howm-case-fold (regexp-quote keyword) keyword) (howm-files))))

(define (howm-menu-last-update)
  (format-time-string howm-dtime-format))

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
