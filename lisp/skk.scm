;;; skk.scm --- SKK, a Japanese input method -*- scheme -*-

;; ddskk の, 基本の部分. 起動のときに読む (core/src/scheme/loadup.rs).
;; `C-\` (`M-x skk-mode`) で入れる. ひらがな, カタカナ, 英数 (latin) の 3 つの状態を持つ.
;; 打った文字は, 打ったあとに (post-self-insert-hook) ローマ字からかなに直す.
;; 大文字で始めると, ▽ から読みを入れる. SPC で辞書を引いて, ▼ に候補を出す (SPC で次の候補, x で前の候補).
;; ほかの文字で確定する. 辞書は, `skk-jisyo` (利用者の辞書. 登録した語を書く) と, `skk-large-jisyo` (SKK-JISYO.L).
;; 移していないもの: 送り仮名 (読みの途中の大文字), 抽象的な文字種 (全角英数, 略語), 補完, 数値変換, 辞書の学習 (候補の並べ替え).

(define skk-jisyo "~/.skk-jisyo"
  "File of the dictionary of the user.  A word that is registered is added to it.")

(define skk-large-jisyo #f
  "File of the large dictionary (SKK-JISYO.L), or #f to use the first one that exists of
/usr/share/skk/SKK-JISYO.L, ~/.skk/SKK-JISYO.L and ~/SKK-JISYO.L.")

(define skk--mode 'hiragana
  "The state of the input: hiragana, katakana or latin.")

(define skk--pending 0
  "The number of letters before point that are typed romaji not yet turned into kana.")

(define skk--phase 'none
  "none, preedit (after ▽: the reading is being typed) or convert (after ▼: a candidate is shown).")

(define skk--start #f
  "The position after ▽ or ▼, where the reading or the candidate starts.")

(define skk--reading "" "The reading that is being converted.")
(define skk--candidates '() "The candidates of the reading.")
(define skk--index 0 "The position in `skk--candidates' of the candidate that is shown.")
(define skk--dictionary #f "The text of the dictionaries, or #f before it is read.")

(define skk--rows
  '(("" "あ" "い" "う" "え" "お") ("k" "か" "き" "く" "け" "こ") ("s" "さ" "し" "す" "せ" "そ")
    ("t" "た" "ち" "つ" "て" "と") ("n" "な" "に" "ぬ" "ね" "の") ("h" "は" "ひ" "ふ" "へ" "ほ")
    ("m" "ま" "み" "む" "め" "も") ("y" "や" #f "ゆ" #f "よ") ("r" "ら" "り" "る" "れ" "ろ")
    ("w" "わ" "うぃ" "う" "うぇ" "を") ("g" "が" "ぎ" "ぐ" "げ" "ご") ("z" "ざ" "じ" "ず" "ぜ" "ぞ")
    ("d" "だ" "ぢ" "づ" "で" "ど") ("b" "ば" "び" "ぶ" "べ" "ぼ") ("p" "ぱ" "ぴ" "ぷ" "ぺ" "ぽ")
    ("f" "ふぁ" "ふぃ" "ふ" "ふぇ" "ふぉ") ("j" "じゃ" "じ" "じゅ" "じぇ" "じょ")
    ("v" "ゔぁ" "ゔぃ" "ゔ" "ゔぇ" "ゔぉ") ("x" "ぁ" "ぃ" "ぅ" "ぇ" "ぉ"))
  "Rows of the romaji table: (CONSONANT A I U E O).")

(define skk--youon
  '(("ky" "き") ("gy" "ぎ") ("sy" "し") ("sh" "し") ("zy" "じ") ("jy" "じ") ("ty" "ち") ("ch" "ち")
    ("cy" "ち") ("dy" "ぢ") ("ny" "に") ("hy" "ひ") ("by" "び") ("py" "ぴ") ("my" "み") ("ry" "り")
    ("fy" "ふ") ("xy" "") ("th" "て") ("dh" "で"))
  "Romaji that make a small ya, yu, yo and e with a vowel: (PREFIX KANA).")

(define skk--table
  (let ((table '()))
    ;; the rows with the vowels
    (for-each
     (lambda (row)
       (let loop ((vowels '("a" "i" "u" "e" "o")) (kana (cdr row)))
         (unless (null? vowels)
           (if (car kana)
               (set! table (cons (list (string-append (car row) (car vowels)) (car kana)) table)))
           (loop (cdr vowels) (cdr kana)))))
     skk--rows)
    ;; the small ya, yu, yo and e
    (for-each
     (lambda (youon)
       (for-each
        (lambda (suffix)
          (set! table (cons (list (string-append (car youon) (car suffix))
                                  (string-append (cadr youon) (cadr suffix)))
                            table)))
        '(("a" "ゃ") ("u" "ゅ") ("o" "ょ") ("e" "ぇ"))))
     skk--youon)
    (append table
            '(("shi" "し") ("chi" "ち") ("tsu" "つ") ("si" "し") ("ti" "ち") ("tu" "つ") ("hu" "ふ")
              ("zi" "じ") ("ji" "じ") ("nn" "ん") ("n'" "ん") ("xtu" "っ") ("xtsu" "っ") ("xwa" "ゎ")
              ("xka" "ゕ") ("xke" "ゖ") ("-" "ー") ("," "、") ("." "。") ("[" "「") ("]" "」")
              ("z." "…") ("z-" "〜") ("z," "‥") ("zh" "←") ("zj" "↓") ("zk" "↑") ("zl" "→")
              ("tha" "てゃ") ("thi" "てぃ") ("thu" "てゅ") ("tho" "てょ")
              ("dha" "でゃ") ("dhi" "でぃ") ("dhu" "でゅ") ("dho" "でょ"))))
  "The romaji table: a list of (ROMAJI KANA).")

(define skk--hiragana-chars "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔ")
(define skk--katakana-chars "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾタダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボポマミムメモャヤュユョヨラリルレロヮワヰヱヲンヴ")

(define (skk--katakana text)
  "Return TEXT with the hiragana turned into katakana."
  (let loop ((i 0) (result ""))
    (if (>= i (string-length text))
        result
        (let* ((c (substring text i (+ i 1)))
               (found (string-search c skk--hiragana-chars)))
          (loop (+ i 1)
                (string-append result (if found (substring skk--katakana-chars found (+ found 1)) c)))))))

(define (skk--on) (buffer-minor-mode-p 'skk))

(define (skk--romaji-prefix? key)
  "Return true if some romaji of the table starts with the string KEY."
  (let loop ((entries skk--table))
    (cond ((null? entries) #f)
          ((string-prefix? key (car (car entries))) #t)
          (else (loop (cdr entries))))))

(define (skk--output kana)
  "Return KANA as it is typed in the present state: katakana in the katakana state,
except in a reading after ▽, which is always hiragana (the dictionaries hold hiragana readings)."
  (if (and (eq? skk--mode 'katakana) (eq? skk--phase 'none)) (skk--katakana kana) kana))

(define (skk--replace from to text)
  "Replace the text from FROM to TO with TEXT, leaving point after it."
  (delete-region from to)
  (goto-char from)
  (insert text))

(define (skk--convert ch)
  "Turn the romaji before point into kana.  CH is the letter just typed; it is before point,
and the `skk--pending' letters before it are typed romaji."
  (let* ((end (point))
         (typed (- end 1))
         (start (- typed skk--pending))
         (pending (buffer-substring start typed))
         (key (string-append pending ch))
         (found (assoc key skk--table)))
    (cond
     (found
      (skk--replace start end (skk--output (cadr found)))
      (set! skk--pending 0))
     ;; n before a consonant makes ん
     ((and (equal? pending "n") (not (member ch '("a" "i" "u" "e" "o" "n" "y"))) (not (equal? ch "'")))
      (skk--replace start end (string-append (skk--output "ん")))
      (insert ch)
      (set! skk--pending 0)
      (skk--convert ch))
     ;; a doubled consonant makes っ
     ((and (= skk--pending 1) (equal? pending ch) (not (member ch '("a" "i" "u" "e" "o" "n"))))
      (skk--replace start (+ start 1) (skk--output "っ"))
      (goto-char end))
     ((skk--romaji-prefix? key)
      (set! skk--pending (string-length pending))
      (set! skk--pending (+ skk--pending 1)))
     ((> skk--pending 0)
      ;; the romaji so far is no kana: drop it, and try the letter again
      (delete-region start typed)
      (set! skk--pending 0)
      (skk--convert ch))
     (else (set! skk--pending 0)))))

(define (skk--read-dictionary)
  "Read the dictionaries into `skk--dictionary' (one string, with a newline before each line)."
  (let* ((large (or skk-large-jisyo
                    (let loop ((files '("/usr/share/skk/SKK-JISYO.L" "~/.skk/SKK-JISYO.L" "~/SKK-JISYO.L")))
                      (cond ((null? files) #f)
                            ((file-exists? (expand-file-name (car files))) (car files))
                            (else (loop (cdr files)))))))
         (user (expand-file-name skk-jisyo)))
    (set! skk--dictionary
          (string-append "\n"
                         (if (file-exists? user) (read-file-string user) "")
                         "\n"
                         (if (and large (file-exists? (expand-file-name large)))
                             (read-file-string (expand-file-name large))
                             "")))))

(define (skk--candidates-of reading)
  "Return the candidates of READING in the dictionaries, in order."
  (unless skk--dictionary (skk--read-dictionary))
  (let ((found (string-search (string-append "\n" reading " /") skk--dictionary)))
    (if (not found)
        '()
        (let* ((from (+ found (string-length reading) 3))
               (to (string-search "\n" skk--dictionary from))
               (line (substring skk--dictionary from (or to (string-length skk--dictionary)))))
          (let loop ((words (split-string line "/")) (candidates '()))
            (if (null? words)
                (reverse candidates)
                (let* ((word (car words))
                       (semi (string-search ";" word))
                       (text (if semi (substring word 0 semi) word)))
                  (loop (cdr words)
                        (if (or (equal? text "") (string-prefix? "(" text) (member text candidates))
                            candidates
                            (cons text candidates))))))))))

(define (skk--register reading word)
  "Add WORD as a candidate of READING to the dictionary of the user."
  (let ((user (expand-file-name skk-jisyo)))
    (write-file-string user
                       (string-append (if (file-exists? user) (read-file-string user) "")
                                      reading " /" word "/\n"))
    (set! skk--dictionary #f)))

(define (skk--valid-p)
  "Return true if the state of the conversion still agrees with the buffer."
  (and skk--start
       (> skk--start 1)
       (<= skk--start (point))
       (equal? (buffer-substring (- skk--start 1) skk--start) (if (eq? skk--phase 'convert) "▼" "▽"))))

(define (skk--reset)
  (set! skk--phase 'none)
  (set! skk--start #f)
  (set! skk--pending 0))

(define (skk--commit)
  "Accept the text of the conversion: remove the mark ▽ or ▼."
  (when (skk--valid-p)
    (delete-region (- skk--start 1) skk--start))
  (skk--reset))

(define (skk--show-candidate)
  "Show the candidate `skk--index' after ▼."
  (skk--replace skk--start (point) (list-ref skk--candidates skk--index))
  (message (format "%s (%s/%s)" skk--reading (+ skk--index 1) (length skk--candidates))))

(define (skk--start-conversion)
  "Look up the reading between ▽ and point, and show the first candidate after ▼."
  (when (= skk--pending 1)
    (when (equal? (buffer-substring (- (point) 1) (point)) "n")
      (skk--replace (- (point) 1) (point) (skk--output "ん")))
    (set! skk--pending 0))
  (set! skk--reading (buffer-substring skk--start (point)))
  (set! skk--candidates (skk--candidates-of skk--reading))
  (if (null? skk--candidates)
      (let ((word (read-string (format "%s " skk--reading))))
        (when (not (equal? word ""))
          (skk--register skk--reading word)
          (skk--replace skk--start (point) word))
        (skk--commit))
      (begin
        (let ((end (point)))
          (skk--replace (- skk--start 1) skk--start "▼")
          (goto-char end))
        (set! skk--phase 'convert)
        (set! skk--index 0)
        (skk--show-candidate))))

(define (skk--set-mode mode label)
  (set! skk--mode mode)
  (message label))

(define (skk--after-insert)
  "Turn the typed letter into kana, and handle ▽ and ▼, for `skk-mode'."
  (when (and (skk--on) (= (string-length self-insert-text) 1))
    (let ((ch self-insert-text))
      (when (and (not (eq? skk--phase 'none)) (not (skk--valid-p)))
        (skk--reset))
      (cond
       ((eq? skk--mode 'latin) #f)
       ;; ▼: SPC and x choose a candidate, any other character accepts the candidate
       ((eq? skk--phase 'convert)
        (cond ((member ch '(" " "x"))
               (delete-char -1)
               (set! skk--index (modulo (+ skk--index (if (equal? ch " ") 1 -1)) (length skk--candidates)))
               (skk--show-candidate))
              (else
               (skk--commit)
               (skk--after-insert))))
       ;; ▽: SPC converts, q makes katakana
       ((and (eq? skk--phase 'preedit) (equal? ch " "))
        (delete-char -1)
        (skk--start-conversion))
       ((and (eq? skk--phase 'preedit) (equal? ch "q") (= skk--pending 0))
        (delete-char -1)
        (let ((reading (buffer-substring skk--start (point))))
          (skk--replace skk--start (point) (skk--katakana reading))
          (skk--commit)))
       ;; a capital letter starts the reading
       ((and (eq? skk--phase 'none) (regexp-match "^[A-Z]$" ch))
        (delete-char -1)
        (set! skk--pending 0)
        (insert "▽")
        (set! skk--phase 'preedit)
        (set! skk--start (point))
        (insert (string-downcase ch))
        (skk--convert (string-downcase ch)))
       ((and (eq? skk--phase 'none) (= skk--pending 0) (equal? ch "l"))
        (delete-char -1)
        (skk--set-mode 'latin "SKK: latin"))
       ((and (eq? skk--phase 'none) (= skk--pending 0) (equal? ch "q"))
        (delete-char -1)
        (if (eq? skk--mode 'katakana)
            (skk--set-mode 'hiragana "SKK: hiragana")
            (skk--set-mode 'katakana "SKK: katakana")))
       ((regexp-match "^[a-zA-Z.,'\\[\\]-]$" ch)
        ;; a capital letter in the middle of a reading (okurigana) is taken as a small one
        (when (regexp-match "^[A-Z]$" ch)
          (delete-char -1)
          (insert (string-downcase ch)))
        (skk--convert (string-downcase ch)))
       (else #f)))))

(add-hook 'post-self-insert-hook 'skk--after-insert)

(define-command (skk-kakutei . args)
  "Accept the conversion that is going on; in the latin state, go to the hiragana state;
otherwise insert a newline."
  (cond ((not (skk--on)) (newline))
        ((and (not (eq? skk--phase 'none)) (skk--valid-p)) (skk--commit))
        ((eq? skk--mode 'latin) (skk--set-mode 'hiragana "SKK: hiragana"))
        (else (newline)))
  #f)

(define-command (skk-return . args)
  "Accept the conversion that is going on, and insert a newline."
  (when (and (skk--on) (not (eq? skk--phase 'none)) (skk--valid-p))
    (skk--commit))
  (newline)
  #f)

(define-command (skk-mode . args)
  "Toggle SKK, a Japanese input method, in the current buffer.
Type romaji for hiragana (a capital letter starts a reading, SPC converts it, x goes back to the previous candidate),
l for the latin state, q to switch between hiragana and katakana, and C-j to accept a conversion
(in the latin state, to go to the hiragana state).  The dictionaries are `skk-jisyo' and `skk-large-jisyo'.
With a positive argument, turn it on; with zero or a negative one, turn it off."
  (let ((on (modes--command args 'skk #f "SKK mode")))
    (skk--reset)
    (set! skk--mode 'hiragana)
    (if on
        (begin (local-set-key "C-j" 'skk-kakutei) (local-set-key "RET" 'skk-return)))
    on))

(global-set-key "C-\\" 'skk-mode)
