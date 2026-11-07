;;; replace.scm --- search and replace commands -*- scheme -*-

;; Emacs の replace.el に当たるもの (`replace-string`, `replace-regexp`, `query-replace`, `query-replace-regexp`, `occur`).
;; `perform-replace` は, Emacs 30 の同じ名前の手続きと同じ順で探し, 置き換える.

(define case-replace #t
  "Non-nil means `query-replace' should preserve case in replacements.")

(define query-replace-from-history '()
  "History list for the search strings of `query-replace'.")

(define (replace--no-upper-p string regexp)
  "Return #t if STRING has no uppercase letter (a letter after a backslash does not count in a regexp)."
  (let loop ((i 0) (quoted #f))
    (cond ((>= i (string-length string)) #t)
          ((and regexp (not quoted) (equal? (substring string i (+ i 1)) "\\")) (loop (+ i 1) #t))
          (quoted (loop (+ i 1) #f))
          ((not (equal? (substring string i (+ i 1)) (string-downcase (substring string i (+ i 1))))) #f)
          (else (loop (+ i 1) #f)))))

(define (replace--search string limit regexp delimited backward)
  "Search for STRING from point up to LIMIT.  Return #t on success, leaving point after (before if BACKWARD) the match."
  (cond ((or regexp delimited)
         (let ((pattern (if delimited
                            (string-append "\\b" (if regexp string (regexp-quote string)) "\\b")
                            string)))
           (if backward
               (re-search-backward pattern limit #t)
               (re-search-forward pattern limit #t))))
        (backward (search-backward string limit #t))
        (else (search-forward string limit #t))))

(define (replace--prompt from to regexp delimited backward)
  (format "Query replacing %s%s%s%s with %s: (? for help) "
          (if backward "backward " "") (if delimited "word " "") (if regexp "regexp " "") from to))

(define (replace--shift data delta)
  "Shift every position of the match data DATA by DELTA."
  (map (lambda (p) (and p (+ p delta))) data))

(define (perform-replace from to query regexp delimited start end backward)
  "Replace some occurrences of FROM with TO, and return #f.
QUERY means ask about each match.  REGEXP means FROM is a regexp, and TO may use \\& and \\N.
DELIMITED means match only whole words.  START and END (or #f) bound the replacement; BACKWARD searches backward."
  (let* ((saved-fold case-fold-search)
         (fold (and case-fold-search (replace--no-upper-p from regexp)))
         (nocasify (not (and case-replace fold)))
         (literal (not regexp))
         (limit (and end (save-excursion
                           (goto-char (if backward (min start end) (max start end)))
                           (point-marker))))
         (count 0)
         (match-again #t)
         (keep-going #t)
         (query-flag query))
    (define (limit-pos) (and limit (marker-position limit)))
    (define (can-continue)
      (if backward
          (not (or (bobp) (and limit (<= (point) (limit-pos)))))
          (not (or (eobp) (and limit (>= (point) (limit-pos)))))))
    (define (next-match)
      "Return the match data of the next match to replace, or #f."
      (cond ((pair? match-again)
             (goto-char (if backward (car match-again) (cadr match-again)))
             (set-match-data match-again)
             match-again)
            (match-again
             (and (replace--search from (limit-pos) regexp delimited backward) (match-data)))
            ((and (if backward (> (- (point) 1) (point-min)) (< (+ (point) 1) (point-max)))
                  (or (not limit) (if backward (> (- (point) 1) (limit-pos)) (< (+ (point) 1) (limit-pos)))))
             (let ((opoint (point)))
               (if backward (backward-char 1) (forward-char 1))
               (if (replace--search from (limit-pos) regexp delimited backward)
                   (match-data)
                   (begin (goto-char opoint) #f))))
            (else #f)))
    (define (replace-here data)
      "Replace the match DATA.  Return the change in length."
      (let ((old-end (cadr data)))
        (set-match-data data)
        (replace-match to nocasify literal)
        (if backward (goto-char (car data)))
        (let ((delta (- (cadr (match-data)) old-end)))
          (set! count (+ count 1))
          (if (pair? match-again) (set! match-again (replace--shift match-again delta)))
          delta)))
    (set! case-fold-search fold)
    (unwind-protect
        (begin
          (if start (goto-char (if backward (max start end) (min start end))))
          (push-mark (point))
          (let loop ()
            (let ((data (and keep-going (can-continue) (next-match))))
              (when data
                (let ((nonempty (not (= (car data) (cadr data)))))
                  (set! match-again
                        (and nonempty
                             (or (not regexp)
                                 (and (not backward)
                                      (looking-at from)
                                      (let ((m (match-data)))
                                        (and (not (= (car m) (cadr m))) m))))))
                  (if (not query-flag)
                      (replace-here data)
                      (let ask ((replaced #f))
                        (set-match-data data)
                        (let ((key (read-key (replace--prompt from (match-substitute-replacement to nocasify literal) regexp delimited backward))))
                          (cond ((member key '(" " "y"))
                                 (if (not replaced) (replace-here data)))
                                ((member key '("Backspace" "Delete" "n")) #f)
                                ((member key '("Enter" "q")) (set! keep-going #f))
                                ((equal? key ".")
                                 (if (not replaced) (replace-here data))
                                 (set! keep-going #f))
                                ((equal? key ",")
                                 (if (not replaced) (replace-here data))
                                 (ask #t))
                                ((equal? key "!")
                                 (if (not replaced) (replace-here data))
                                 (set! query-flag #f))
                                (else (ask replaced))))))
                  (loop))))))
      (set! case-fold-search saved-fold)
      (if limit (marker-release limit)))
    (message (format "Replaced %d occurrence%s" count (if (= count 1) "" "s")))
    #f))

(define (replace--read-args regexp)
  "Read the string to replace and its replacement from the minibuffer.  Return (FROM TO)."
  (let* ((from (read-string (if regexp "Replace regexp: " "Replace string: ")))
         (to (read-string (format "Replace %s with: " from))))
    (list from to)))

(define (replace--region-bounds)
  "Return (START END) of the active region, or (#f #f)."
  (if (use-region-p)
      (list (region-beginning) (region-end))
      (list #f #f)))

(define (replace--command query regexp args)
  "Run a replace command: ARGS are (FROM TO DELIMITED START END BACKWARD), read interactively if empty."
  (let* ((interactive (null? args))
         (strings (if interactive (replace--read-args regexp) (list (car args) (cadr args))))
         (rest (if interactive '() (cddr args)))
         (prefix (if interactive (current-prefix-arg) #f))
         (delimited (if interactive (and prefix (not (eq? prefix '-))) (and (pair? rest) (car rest))))
         (backward (if interactive (or (eq? prefix '-) (and (number? prefix) (< prefix 0))) (and (> (length rest) 3) (list-ref rest 3))))
         (bounds (if interactive (replace--region-bounds) (list (and (> (length rest) 1) (list-ref rest 1)) (and (> (length rest) 2) (list-ref rest 2))))))
    (perform-replace (car strings) (cadr strings) query regexp delimited (car bounds) (cadr bounds) backward)
    (if (and interactive (car bounds)) (set-deactivate-mark #t))
    #f))

(define-command (replace-string . args)
  "Replace occurrences of FROM-STRING with TO-STRING, from point to the end of the buffer (or in the region).
Arguments: FROM-STRING TO-STRING [DELIMITED START END BACKWARD]."
  (replace--command #f #f args))

(define-command (replace-regexp . args)
  "Replace things after point matching REGEXP with TO-STRING.
In TO-STRING, \\& stands for the whole match, and \\N for the Nth parenthesized group."
  (replace--command #f #t args))

(define-command (query-replace . args)
  "Replace some occurrences of FROM-STRING with TO-STRING, asking about each.
Keys: SPC or y replaces, DEL or n skips, RET or q exits, . replaces and exits, , replaces and asks again, ! replaces all the rest."
  (replace--command #t #f args))

(define-command (query-replace-regexp . args)
  "Replace some things after point matching REGEXP with TO-STRING, asking about each.
See `query-replace' for the keys."
  (replace--command #t #t args))


;;; occur

(define occur--targets '()
  "Alist of the occur buffers and their targets: (BUFFER-NAME (OCCUR-LINE SOURCE-BUFFER POSITION) ...).")

(define (occur--count-newlines text)
  (let loop ((i 0) (n 0))
    (cond ((>= i (string-length text)) n)
          ((equal? (substring text i (+ i 1)) "\n") (loop (+ i 1) (+ n 1)))
          (else (loop (+ i 1) n)))))

(define (occur--collect regexp)
  "Return the matching lines of the current buffer as a list of (LINE TEXT POSITION MATCHES LAST-LINE SPANS), and the number of matches.
SPANS are the matches in TEXT, as lists (START END) of offsets."
  (save-excursion
    (goto-char (point-min))
    (let ((line 1) (line-pos (point-min)) (total 0))
      (let loop ((acc '()))
        (if (and (not (eobp)) (re-search-forward regexp #f #t))
            (let* ((mb (match-beginning 0))
                   (me (match-end 0))
                   (begpt (save-excursion (goto-char mb) (line-beginning-position)))
                   (endpt (save-excursion (goto-char me) (line-end-position)))
                   (matches 1)
                   (spans (list (list (- mb begpt) (- me begpt)))))
              (if (= mb me) (if (< (point) endpt) (forward-char 1)))
              ;; 同じ行の, 続きの一致を数える
              (let more ()
                (when (and (< (point) endpt) (re-search-forward regexp endpt #t))
                  (set! matches (+ matches 1))
                  (set! spans (cons (list (- (match-beginning 0) begpt) (- (match-end 0) begpt)) spans))
                  (if (= (match-beginning 0) (match-end 0)) (if (< (point) endpt) (forward-char 1)))
                  (more)))
              (set! line (+ line (occur--count-newlines (buffer-substring line-pos begpt))))
              (set! line-pos begpt)
              (set! total (+ total matches))
              (goto-char (max (point) endpt))
              (if (< (point) (point-max)) (forward-char 1))
              (loop (cons (let ((text (buffer-substring begpt endpt)))
                            (list line text mb matches (+ line (occur--count-newlines text)) (reverse spans)))
                          acc)))
            (list (reverse acc) total))))))

(define (occur--indent-continuation text)
  "Prefix each line of TEXT after the first (a match across lines) with the context marker."
  (string-join (split-string text "\n") "\n       :"))

(define (occur--pad number)
  (let ((text (number->string number)))
    (string-append (make-string-of-spaces (- 7 (string-length text))) text ":")))

(define (make-string-of-spaces n)
  (if (<= n 0) "" (string-append " " (make-string-of-spaces (- n 1)))))

(define (occur--lines-of text-lines total-lines nlines)
  "Return the line numbers to show: each matching line (in TEXT-LINES, with the lines it spans) and NLINES lines around it."
  (let loop ((rest text-lines) (shown '()))
    (if (null? rest)
        (reverse shown)
        (let* ((n (car (car rest)))
               (last (list-ref (car rest) 4))
               (from (max 1 (- n nlines)))
               (to (max last (min total-lines (+ last nlines)))))
          (let fill ((i (if (and (pair? shown) (>= (car shown) from)) (+ (car shown) 1) from)) (shown shown))
            (if (> i to)
                (loop (cdr rest) shown)
                (fill (+ i 1) (cons i shown))))))))

;; 結果は, バッファごとのまとまり (chunk) を作って並べる. Emacs と同じく, 後のバッファが先に来る.

(define (occur--plural n word)
  (if (= n 1) word (string-append word (if (equal? word "match") "es" "s"))))

(define (occur--counts matches lines)
  "Return `N matches' with ` in M lines' when the numbers differ."
  (string-append (format "%d %s" matches (occur--plural matches "match"))
                 (if (= matches lines) "" (format " in %d %s" lines (occur--plural lines "line")))))

(define (occur--span-offset text offset)
  "Return where the OFFSET of TEXT lies after the lines of TEXT are prefixed (8 characters each)."
  (+ 8 offset (* 8 (occur--count-newlines (substring text 0 offset)))))

(define (occur--chunk source regexp nlines multi)
  "Return the occur output for the buffer SOURCE as (TEXT TARGETS SPANS MATCHES LINES), or #f if nothing matches.
TARGETS are (LINE SOURCE POSITION) with LINE counted from 1 in TEXT, SPANS are (START END) offsets in TEXT."
  (with-current-buffer source
    (let* ((fold (and case-fold-search (replace--no-upper-p regexp #t)))
           (saved case-fold-search)
           (found (begin (set! case-fold-search fold)
                         (unwind-protect (occur--collect regexp) (set! case-fold-search saved))))
           (matches (car found))
           (total (cadr found)))
      (if (null? matches)
          #f
          (let* ((lines (split-string (buffer-string) "\n"))
                 (all-lines (if (equal? (car (reverse lines)) "") (- (length lines) 1) (length lines)))
                 (context (and nlines (> nlines 0) nlines))
                 (out (format "%s%s in buffer: %s\n"
                              (occur--counts total (length matches))
                              (if multi "" (format " for \"%s\"" regexp))
                              source))
                 (targets '())
                 (spans '())
                 (out-line 2))
            (define (add-hit! hit)
              (set! targets (cons (list out-line source (caddr hit)) targets))
              (let ((base (string-length out)))
                (for-each (lambda (span)
                            (set! spans (cons (list (+ base (occur--span-offset (cadr hit) (car span)))
                                                    (+ base (occur--span-offset (cadr hit) (cadr span))))
                                              spans)))
                          (list-ref hit 5))))
            (if context
                ;; 前後の行も出す. 離れた組の間は, 線で分ける
                (let loop ((shown (occur--lines-of matches all-lines context)) (previous #f))
                  (when (pair? shown)
                    (let* ((n (car shown))
                           (hit (assoc n matches)))
                      (if (and previous (<= n previous))
                          (loop (cdr shown) previous)
                          (begin
                            (when (and previous (> n (+ previous 1)))
                              (set! out (string-append out "-------\n"))
                              (set! out-line (+ out-line 1)))
                            (if hit (add-hit! hit))
                            (set! out (string-append out
                                                     (if hit (occur--pad n) "       :")
                                                     (occur--indent-continuation (if hit (cadr hit) (list-ref lines (- n 1))))
                                                     "\n"))
                            (set! out-line (+ out-line 1 (if hit (- (list-ref hit 4) n) 0)))
                            (loop (cdr shown) (if hit (list-ref hit 4) n)))))))
                (for-each (lambda (m)
                            (add-hit! m)
                            (set! out (string-append out (occur--pad (car m)) (occur--indent-continuation (cadr m)) "\n"))
                            (set! out-line (+ out-line 1 (- (list-ref m 4) (car m)))))
                          matches))
            (list out (reverse targets) (reverse spans) total (length matches)))))))

(define occur--spans '()
  "Alist of the occur buffers and the positions of the matches in them: (BUFFER-NAME (START END) ...).")

(define (occur--run regexp nlines . buffers)
  "Show the lines of BUFFERS (default: the current buffer) that match REGEXP in *Occur*."
  (if (equal? regexp "")
      (error "Occur doesn't work with the empty regexp"))
  (let* ((origin (buffer-name))
         (names (filter get-buffer (if (pair? buffers) (car buffers) (list origin))))
         (multi (> (length names) 1))
         (chunks (filter (lambda (c) c) (map (lambda (name) (occur--chunk name regexp nlines multi)) names)))
         (total (apply + (map (lambda (c) (list-ref c 3)) chunks)))
         (name "*Occur*"))
    (message (format "Searched %d %s; %s %s for \"%s\""
                     (length names) (occur--plural (length names) "buffer")
                     (if (= total 0) "no" (number->string total)) (occur--plural total "match") regexp))
    (if (= total 0)
        (if (get-buffer name) (kill-buffer name))
        (let* ((header (if multi
                           (format "%s total for \"%s\":\n"
                                   (occur--counts total (apply + (map (lambda (c) (list-ref c 4)) chunks)))
                                   regexp)
                           ""))
               (out header)
               (targets '())
               (spans '()))
          ;; 後のバッファを先に置く
          (for-each (lambda (chunk)
                      (let ((line (+ 1 (occur--count-newlines out)))
                            (offset (string-length out)))
                        (set! targets (append targets
                                              (map (lambda (t) (cons (+ (car t) line -1) (cdr t))) (cadr chunk))))
                        (set! spans (append spans
                                            (map (lambda (sp) (list (+ (car sp) offset 1) (+ (cadr sp) offset 1)))
                                                 (caddr chunk))))
                        (set! out (string-append out (car chunk)))))
                    (reverse chunks))
          (switch-to-buffer name)
          (set-buffer-read-only #f)
          (delete-region (point-min) (point-max))
          (insert out)
          (goto-char (point-min))
          (set-buffer-modified-p #f)
          (set! occur--targets (cons (cons name targets)
                                     (filter (lambda (e) (not (equal? (car e) name))) occur--targets)))
          (set! occur--spans (cons (cons name spans)
                                   (filter (lambda (e) (not (equal? (car e) name))) occur--spans)))
          (local-set-key "RET" 'occur-mode-goto-occurrence)
          (local-set-key "n" 'occur-next)
          (local-set-key "p" 'occur-prev)
          (switch-to-buffer origin)
          (display-buffer name 1)
          ;; next-error は, この結果をたどる (compile.scm を読んでいれば)
          (if (fboundp 'compile--set-source)
              (compile--set-source name "" 1 1 'occur))))))

(define-command (occur . args)
  "Show all lines in the current buffer containing a match for REGEXP, in the *Occur* buffer.
With a numeric prefix argument NLINES, also show NLINES lines of context around each match.
In the *Occur* buffer, RET goes to the match on the current line, and \\[next-error] visits the matches."
  (let ((regexp (if (null? args) (read-string "List lines matching regexp: ") (car args)))
        (nlines (if (or (null? args) (null? (cdr args)))
                    (let ((p (if (null? args) (current-prefix-arg) #f))) (and p (prefix-numeric-value p)))
                    (cadr args))))
    (occur--run regexp nlines)))

(define-command (multi-occur . args)
  "Show all lines in the buffers BUFS containing a match for REGEXP, like `occur'.
Arguments: BUFS REGEXP [NLINES].  Interactively, read the buffer names one by one (an empty name ends)."
  (if (pair? args)
      (occur--run (cadr args) (if (pair? (cddr args)) (caddr args) #f) (map occur--buffer-name (car args)))
      (let* ((first (completing-read "First buffer to search: " (buffer-list) #f #t #f #f (buffer-name)))
             (bufs (let loop ((bufs (list first)))
                     (let ((next (completing-read "Next buffer to search (RET to end): " (buffer-list) #f #t)))
                       (if (equal? next "")
                           (reverse bufs)
                           (loop (if (member next bufs) bufs (cons next bufs)))))))
             (regexp (read-string "List lines matching regexp: "))
             (p (current-prefix-arg)))
        (occur--run regexp (and p (prefix-numeric-value p)) bufs))))

(define-command (multi-occur-in-matching-buffers . args)
  "Show all lines matching REGEXP in the buffers whose file names match BUFREGEXP.
Arguments: BUFREGEXP REGEXP [ALLBUFS].  With ALLBUFS (interactively, a prefix argument), match the
buffer names instead, including the buffers that do not visit files."
  (let* ((allbufs (if (pair? args) (and (pair? (cddr args)) (caddr args)) (current-prefix-arg)))
         (bufregexp (if (pair? args)
                        (car args)
                        (read-string (if allbufs
                                         "List lines in buffers whose names match regexp: "
                                         "List lines in buffers whose filenames match regexp: "))))
         (regexp (if (pair? args) (cadr args) (read-string "List lines matching regexp: "))))
    (occur--run regexp #f
                (filter (lambda (name)
                          (let ((file (with-current-buffer name (buffer-file-name))))
                            (if allbufs
                                (string-match bufregexp name)
                                (and file (string-match bufregexp file)))))
                        (buffer-list)))))

(define (occur--buffer-name buffer)
  (if (string? buffer) buffer (buffer-name buffer)))

;;; next-error in the *Occur* buffer (`occur-next-error')

(define (occur--next-change position forward)
  "Return where the matches of the *Occur* buffer begin or end next after (or before) POSITION, or #f.
This is `next-single-property-change' (or `previous-...') of the property `occur-match'."
  (let ((spans (let ((entry (assoc (buffer-name) occur--spans))) (if entry (cdr entry) '()))))
    (if forward
        (let loop ((rest spans))
          (cond ((null? rest) #f)
                ((> (cadr (car rest)) position) (if (<= (caar rest) position) (cadr (car rest)) (caar rest)))
                (else (loop (cdr rest)))))
        (let loop ((rest (reverse spans)))
          (cond ((null? rest) #f)
                ((< (caar rest) position) (if (<= position (cadr (car rest))) (caar rest) (cadr (car rest))))
                (else (loop (cdr rest))))))))

(define (occur--in-match? position)
  (let ((entry (assoc (buffer-name) occur--spans)))
    (and entry (pair? (filter (lambda (span) (and (<= (car span) position) (< position (cadr span)))) (cdr entry))))))

(define (occur--find-match n forward)
  "Move point over N matches of the *Occur* buffer, forward or backward (`occur-find-match')."
  (let loop ((n n))
    (when (> n 0)
      (let* ((r (occur--next-change (point) forward))
             (r (if (and r (occur--in-match? r)) (occur--next-change r forward) r)))
        (if r (goto-char r) (user-error "No more matches"))
        (loop (- n 1))))))

(define (occur-next-error n)
  "Move to the Nth (backward if N is negative) next match of the current *Occur* buffer, and return its target (SOURCE POSITION)."
  (goto-char (if (< n 0) (line-beginning-position) (line-end-position)))
  (occur--find-match (abs n) (> n 0))
  (or (occur--current-target) (error "No occurrence on this line")))

(define (occur--current-target)
  "Return (SOURCE POSITION) of the occurrence on the current line of the *Occur* buffer, or #f."
  (let* ((line (+ 1 (occur--count-newlines (buffer-substring (point-min) (line-beginning-position)))))
         (entry (assoc (buffer-name) occur--targets))
         (target (and entry (assoc line (cdr entry)))))
    (and target (cdr target))))

(define-command (occur-mode-goto-occurrence . args)
  "Go to the occurrence on the current line of the *Occur* buffer."
  (let ((target (occur--current-target)))
    (if (not target)
        (error "No occurrence on this line")
        (begin
          (switch-to-buffer (car target))
          (goto-char (cadr target))))))

(define-command (occur-next . args)
  "Move to the next occurrence in the *Occur* buffer."
  (let loop ((n (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args))))
    (when (> n 0)
      (forward-line 1)
      (if (not (or (eobp) (occur--current-target))) (loop n) (loop (- n 1))))))

(define-command (occur-prev . args)
  "Move to the previous occurrence in the *Occur* buffer."
  (let loop ((n (if (null? args) (prefix-numeric-value (current-prefix-arg)) (car args))))
    (when (> n 0)
      (forward-line -1)
      (if (not (or (bobp) (occur--current-target))) (loop n) (loop (- n 1))))))


;;; hi-lock: highlight-regexp and the commands that use it

(define hi-lock-face-defaults
  '("hi-yellow" "hi-pink" "hi-green" "hi-blue" "hi-salmon" "hi-aquamarine"
    "hi-black-b" "hi-blue-b" "hi-red-b" "hi-green-b" "hi-black-hb")
  "Default face names for the hi-lock commands.")

(define hi-lock--face-index 0)

(define (hi-lock--next-face)
  "Return the next face of `hi-lock-face-defaults', cycling through them."
  (let ((face (list-ref hi-lock-face-defaults (modulo hi-lock--face-index (length hi-lock-face-defaults)))))
    (set! hi-lock--face-index (+ hi-lock--face-index 1))
    face))

(define hi-lock--highlight highlight-regexp)
(define hi-lock--unhighlight unhighlight-regexp)

(define-command (highlight-regexp . args)
  "Highlight each match of REGEXP in the current buffer with FACE (SUBEXP selects a group).
Interactively, read REGEXP and use the next face of `hi-lock-face-defaults'."
  (if (null? args)
      (let ((regexp (read-string "Regexp to highlight: ")))
        (hi-lock--highlight regexp (hi-lock--next-face))
        regexp)
      (apply hi-lock--highlight args)))

(define-command (unhighlight-regexp . args)
  "Remove the highlighting of REGEXP set by `highlight-regexp'.
With #t, or with a prefix argument interactively, remove all of it."
  (cond ((pair? args) (hi-lock--unhighlight (car args)))
        ((current-prefix-arg) (hi-lock--unhighlight #t))
        (else (let ((regexps (highlighted-regexps)))
                (if (null? regexps)
                    (message "No highlighted regexps")
                    (hi-lock--unhighlight (completing-read "Regexp to unhighlight: " regexps #f)))))))

(define-command (highlight-phrase . args)
  "Highlight each match of PHRASE, in which any run of white space matches any other."
  (let* ((phrase (if (null? args) (read-string "Phrase to highlight: ") (car args)))
         (regexp (string-join (map regexp-quote (split-string phrase "[ \t\n]+")) "[ \t\n]+")))
    (hi-lock--highlight regexp (if (and (pair? args) (pair? (cdr args))) (cadr args) (hi-lock--next-face)))
    regexp))

(define (hi-lock--symbol-at-point)
  "Return the symbol at or next to point, or #f."
  (save-excursion
    (skip-syntax-backward "w_")
    (let ((start (point)))
      (skip-syntax-forward "w_")
      (if (= start (point)) #f (buffer-substring start (point))))))

(define-command (highlight-symbol-at-point . args)
  "Highlight each occurrence of the symbol at point."
  (let ((symbol (hi-lock--symbol-at-point)))
    (if (not symbol)
        (error "No symbol at point")
        (let ((regexp (string-append "\\_<" (regexp-quote symbol) "\\_>")))
          (hi-lock--highlight regexp (hi-lock--next-face))
          regexp))))

(define-command (highlight-lines-matching-regexp . args)
  "Highlight the lines that match REGEXP."
  (let* ((regexp (if (null? args) (read-string "Regexp to highlight line: ") (car args)))
         (face (if (and (pair? args) (pair? (cdr args))) (cadr args) (hi-lock--next-face))))
    (hi-lock--highlight (string-append "^.*\\(?:" regexp "\\).*$") face)
    regexp))

;;; Deleting and counting the lines that match

(define (replace--line-bounds start end)
  "Return (START END) with default values: from point to the end of the buffer."
  (list (or start (point)) (or end (point-max))))

(define-command (flush-lines . args)
  "Delete lines containing matches for REGEXP.
When called from Lisp, delete lines between START and END (default: from point to the end of the
buffer).  If a match is split across lines, all the lines it lies in are deleted.
Return the number of deleted matching lines."
  (let* ((interactive (null? args))
         (regexp (if interactive (read-string "Flush lines containing match for regexp: ") (car args)))
         (bounds (replace--line-bounds (and (pair? (cdr args)) (cadr args))
                                       (and (pair? (cdr args)) (pair? (cddr args)) (caddr args))))
         (tail (- (point-max) (cadr bounds))))
    (let ((count
           (save-excursion
             (goto-char (car bounds))
             (let loop ((count 0))
               (if (and (< (point) (- (point-max) tail))
                        (re-search-forward regexp (- (point-max) tail) #t))
                   (begin
                     (delete-region (save-excursion (goto-char (match-beginning 0)) (line-beginning-position))
                                    (begin (forward-line 1) (point)))
                     (loop (+ count 1)))
                   count)))))
      (if interactive
          (message (format "Deleted %d matching line%s" count (if (= count 1) "" "s"))))
      count)))

(define-command (keep-lines . args)
  "Delete all lines except those containing matches for REGEXP.
When called from Lisp, operate between START and END (default: from point to the end of the
buffer).  Start at the next line when point is not at the beginning of a line."
  (let* ((interactive (null? args))
         (regexp (if interactive (read-string "Keep lines containing match for regexp: ") (car args)))
         (bounds (replace--line-bounds (and (pair? (cdr args)) (cadr args))
                                       (and (pair? (cdr args)) (pair? (cddr args)) (caddr args))))
         (tail (- (point-max) (cadr bounds))))
    (save-excursion
      (goto-char (car bounds))
      (unless (bolp) (forward-line 1))
      (let loop ((start (point)))
        (when (< (point) (- (point-max) tail))
          (if (not (re-search-forward regexp (- (point-max) tail) #t))
              (delete-region start (- (point-max) tail))
              (let ((end (save-excursion (goto-char (match-beginning 0)) (line-beginning-position))))
                (when (< start end)
                  (delete-region start end))
                (let ((next (save-excursion (forward-line 1) (point))))
                  (if (and (< (point) (- (point-max) tail)) (= (match-beginning 0) (match-end 0)))
                      (forward-char 1))
                  (loop next)))))))
    #f))

(define-command (how-many . args)
  "Print and return number of matches for REGEXP following point.
When called from Lisp, count between START and END (default: from point to the end of the buffer)."
  (let* ((interactive (null? args))
         (regexp (if interactive (read-string "How many matches for regexp: ") (car args)))
         (bounds (replace--line-bounds (and (pair? (cdr args)) (cadr args))
                                       (and (pair? (cdr args)) (pair? (cddr args)) (caddr args))))
         (count
          (save-excursion
            (goto-char (car bounds))
            (let loop ((count 0))
              (let ((before (point)))
                (if (and (< (point) (cadr bounds)) (re-search-forward regexp (cadr bounds) #t))
                    (if (= before (point))
                        (begin (forward-char 1) (loop count))
                        (loop (+ count 1)))
                    count))))))
    (if interactive
        (message (format "%d occurrence%s" count (if (= count 1) "" "s"))))
    count))

(provide 'replace)

;;; replace.scm ends here
