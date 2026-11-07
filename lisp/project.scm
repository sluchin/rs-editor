;;; project.scm --- operations on the current project -*- scheme -*-

;; Emacs の project.el に当たるもの (C-x p の下のコマンド). 起動のときに読む (core/src/scheme/loadup.rs).
;; プロジェクトは, バージョン管理 (.git か .hg) のあるディレクトリ. ファイルの一覧は, Git なら
;; `git ls-files' で (.gitignore のファイルは除く), そうでなければ, ディレクトリの下の全てのファイル.

(define project-vc-markers '(".git" ".hg")
  "The names whose presence in a directory makes the directory the root of a project.")

(define project-switch-commands
  '((project-find-file "Find file" "f")
    (project-find-regexp "Find regexp" "g")
    (project-find-dir "Find directory" "d")
    (project-dired "Dired" "D")
    (project-switch-to-buffer "Switch to buffer" "b")
    (project-compile "Compile" "c"))
  "The commands `project-switch-project' offers: each is (COMMAND LABEL KEY).")

(define project--override #f
  "The root `project-switch-project' runs its command in, or #f.")

(define project--list '()
  "The roots of the projects used in this session, the newest first.")

(define (project--directory-name dir)
  "Return DIR expanded, with a slash at the end."
  (let ((dir (expand-file-name dir)))
    (if (string-suffix? "/" dir) dir (string-append dir "/"))))

(define (project--parent dir)
  "Return the parent of the directory DIR (with a slash at the end), or #f for the root."
  (let ((trimmed (substring dir 0 (- (string-length dir) 1))))
    (if (equal? trimmed "")
        #f
        (let ((parent (file-name-directory trimmed)))
          (and parent (project--directory-name parent))))))

(define (project--find-root dir)
  "Return the nearest directory at or above DIR that has one of `project-vc-markers', or #f."
  (let loop ((dir (project--directory-name dir)))
    (cond ((not dir) #f)
          ((pair? (filter (lambda (marker) (file-exists? (string-append dir marker))) project-vc-markers))
           dir)
          (else (loop (project--parent dir))))))

(define (project--default-directory)
  "Return the directory of the current buffer (of the Dired buffer, of the visited file, or the home)."
  (compile--default-directory))

(define (project--remember root)
  (set! project--list (cons root (filter (lambda (r) (not (equal? r root))) project--list))))

(define (project--read-directory)
  "Read the root of a project: one of `project--list' or another directory."
  (let* ((choose "... (choose a dir)")
         (choice (if (null? project--list)
                     choose
                     (completing-read "Select project: " (append project--list (list choose)) #f #t #f #f
                                      (car project--list)))))
    (if (equal? choice choose)
        (project--directory-name (read-file-name "Select directory: " (project--default-directory)))
        choice)))

(define (project-current . args)
  "Return the project of DIRECTORY (default: that of the current buffer), as (vc BACKEND ROOT).
If there is none and MAYBE-PROMPT is true, read a directory and use it as the root; otherwise return #f.
The arguments are MAYBE-PROMPT and DIRECTORY."
  (let* ((maybe-prompt (and (pair? args) (car args)))
         (directory (if (and (pair? args) (pair? (cdr args)) (cadr args)) (cadr args) (project--default-directory)))
         (root (or project--override (project--find-root directory))))
    (cond (root
           (project--remember root)
           (list 'vc (if (file-exists? (string-append root ".git")) 'Git 'Hg) root))
          (maybe-prompt
           (let ((dir (project--read-directory)))
             (project--remember dir)
             (list 'transient #f dir)))
          (else #f))))

(define (project-root project)
  "Return the root directory of PROJECT."
  (caddr project))

(define (project-name project)
  "Return the name of PROJECT: the name of its root directory."
  (let ((root (project-root project)))
    (file-name-nondirectory (substring root 0 (- (string-length root) 1)))))

(define (project-files project)
  "Return the files of PROJECT (absolute names).
For Git, the files `git ls-files' lists (tracked, and untracked but not ignored); otherwise all the files
below the root."
  (let* ((root (project-root project))
         (result (and (eq? (cadr project) 'Git)
                      (call-shell-command "git -c core.quotepath=off ls-files -co --exclude-standard" #f root))))
    (if (and result (equal? (car result) 0))
        (map (lambda (file) (string-append root file))
             (filter (lambda (file) (not (equal? file ""))) (split-string (cadr result) "\n")))
        (filter (lambda (file) (not (regexp-match "/(\\.git|\\.hg)/" file)))
                (directory-files-recursively root "")))))

(define (project--relative root file)
  (substring file (string-length root) (string-length file)))

(define (project--current-or-ask)
  (project-current #t))

(define-command (project-find-file . args)
  "Visit a file (with completion) in the current project.
The completion candidates are the files of the project, relative to its root."
  (let* ((project (project--current-or-ask))
         (root (project-root project))
         (files (map (lambda (file) (project--relative root file)) (project-files project)))
         (file (completing-read (format "Find file in %s: " (grep--home-name root)) files #f #f)))
    (unless (equal? file "")
      (find-file (expand-file-name file root)))
    #f))

(define-command (project-or-external-find-file . args)
  "Visit a file (with completion) in the current project (this editor has no external roots)."
  (project-find-file))

(define (project--directories project)
  "Return the directories of the files of PROJECT, and the root, relative to the root."
  (let ((root (project-root project)))
    (let loop ((files (project-files project)) (dirs (list "./")))
      (if (null? files)
          (sort dirs string<?)
          (let* ((dir (file-name-directory (car files)))
                 (relative (if (equal? dir root) "./" (project--relative root dir))))
            (loop (cdr files) (if (member relative dirs) dirs (cons relative dirs))))))))

(define-command (project-find-dir . args)
  "Start Dired in a directory inside the current project."
  (let* ((project (project--current-or-ask))
         (root (project-root project))
         (dir (completing-read "Dired: " (project--directories project) #f #t)))
    (dired (expand-file-name dir root))
    #f))

(define-command (project-dired . args)
  "Start Dired in the current project's root."
  (dired (project-root (project--current-or-ask)))
  #f)

(define (project--buffer-directory name)
  "Return the directory of the buffer NAME: of its file, or of its Dired listing; #f otherwise."
  (let ((file (buffer-file-name name))
        (dired (assoc name dired--directories)))
    (cond (file (file-name-directory file))
          (dired (project--directory-name (cadr dired)))
          (else #f))))

(define (project--buffers project)
  "Return the names of the buffers of PROJECT: those that visit a file or a directory under its root."
  (let ((root (project-root project)))
    (filter (lambda (name)
              (let ((dir (project--buffer-directory name)))
                (and dir (string-prefix? root (project--directory-name dir)))))
            (buffer-list))))

(define-command (project-switch-to-buffer . args)
  "Display buffer BUFFER-OR-NAME in the selected window.
Interactively, read the name among the buffers of the current project; the default is
another buffer of the project."
  (let* ((project (project--current-or-ask))
         (buffers (project--buffers project))
         (others (filter (lambda (name) (not (equal? name (buffer-name)))) buffers))
         (default (and (pair? others) (car others)))
         (name (if (pair? args)
                   (car args)
                   (completing-read (if default (format "Switch to buffer (default %s): " default) "Switch to buffer: ")
                                    buffers #f #f #f #f default))))
    (unless (equal? name "")
      (switch-to-buffer name))
    #f))

(define-command (project-kill-buffers . args)
  "Kill the buffers that belong to the current project (those that visit a file or a directory under it).
Ask first, unless NO-CONFIRM is true."
  (let* ((project (project--current-or-ask))
         (buffers (project--buffers project))
         (no-confirm (and (pair? args) (car args))))
    (cond ((null? buffers) (message "No buffers to kill"))
          ((or no-confirm
               (yes-or-no-p (format "Kill %d buffers in %s? " (length buffers) (project-name project))))
           (for-each kill-buffer buffers)))
    #f))

(define (project--call-in-root project thunk)
  "Call THUNK with the shell commands and `compile' running in the root of PROJECT."
  (let ((shell simple--shell-directory)
        (compile compile--directory))
    (set! simple--shell-directory (project-root project))
    (set! compile--directory (project-root project))
    (unwind-protect (thunk)
      (set! simple--shell-directory shell)
      (set! compile--directory compile))))

(define-command (project-shell-command . args)
  "Run `shell-command' in the current project's root directory."
  (let ((project (project--current-or-ask)))
    (project--call-in-root project (lambda () (call-interactively 'shell-command)))
    #f))

(define-command (project-async-shell-command . args)
  "Run `async-shell-command' in the current project's root directory."
  (let ((project (project--current-or-ask)))
    (project--call-in-root project (lambda () (call-interactively 'async-shell-command)))
    #f))

(define-command (project-compile . args)
  "Run `compile' in the project root."
  (let ((project (project--current-or-ask)))
    (project--call-in-root project (lambda () (call-interactively 'compile)))
    #f))

(define-command (project-find-regexp . args)
  "Find all matches for REGEXP in the current project's files.
Interactively, read the regexp (the default is the symbol at point).  The matches are listed
in the *xref* buffer as FILE:LINE:TEXT; \\[next-error] visits them."
  (let* ((project (project--current-or-ask))
         (root (project-root project))
         (regexp (if (pair? args)
                     (car args)
                     (let ((default (xref--identifier-at-point)))
                       (read-string (if default (format "Find regexp (default %s): " default) "Find regexp: ")
                                    #f #f default))))
         (found (rust-grep regexp (project-files project))))
    (if (null? found)
        (user-error (format "No matches for: %s" regexp)))
    (xref--push-marker-stack)
    (xref--show (format "Matches of %s in %s:" regexp (grep--home-name root))
                (map (lambda (hit) (format "%s:%d:%s" (project--relative root (car hit)) (cadr hit) (caddr hit))) found)
                #t)
    (compile--set-source "*xref*" root next-error--start next-error--end)
    #f))

(define-command (project-or-external-find-regexp . args)
  "Find all matches for REGEXP in the current project's files (this editor has no external roots)."
  (apply project-find-regexp args))

(define (project--menu-prompt)
  (string-join (map (lambda (row) (format "%s %s" (caddr row) (cadr row))) project-switch-commands) "  "))

(define-command (project-switch-project . args)
  "\"Switch\" to another project by running an Emacs command.
Read the root of the project, then a key of `project-switch-commands', and run its command
in that project."
  (let* ((dir (if (pair? args) (project--directory-name (car args)) (project--read-directory)))
         (command (let loop ((prompt (string-append "Choose: " (project--menu-prompt))))
                    (let* ((key (read-key prompt))
                           (row (filter (lambda (row) (equal? (caddr row) key)) project-switch-commands)))
                      (cond ((pair? row) (car (car row)))
                            ((equal? key "C-g") (keyboard-quit))
                            (else (loop (format "Choose: %s Unrecognized input: %s" (project--menu-prompt) key))))))))
    (project--remember dir)
    (let ((saved project--override))
      (set! project--override dir)
      (unwind-protect (call-interactively command)
        (set! project--override saved)))
    #f))

(define-command (project-forget-project . args)
  "Remove the root of a project (read in the minibuffer) from the list of known projects."
  (let ((dir (if (pair? args) (car args) (completing-read "Forget project: " project--list #f #t))))
    (set! project--list (filter (lambda (r) (not (equal? r dir))) project--list))
    (message (format "Project `%s' removed from known projects" dir))
    #f))

(provide 'project)

;;; project.scm ends here
