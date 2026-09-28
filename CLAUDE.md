# Scheme Editor Project Guide

## Overview
A Tauri + React + Rust Scheme language editor with a built-in interpreter and REPL.

## Tech Stack
- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 2
- **Build**: Cargo + npm
- **Development**: Hot reload for both frontend and Rust backend

## Key Files

### Frontend
- `src/App.tsx` - Main app component, state management, minibuffer/file-op logic
- `src/components/CodeEditor.tsx` - Plain textarea buffer + C-x prefix key handling
- `src/components/MenuBar.tsx` - Emacs-style menu bar (static)
- `src/components/ModeLine.tsx` - Emacs-style mode line
- `src/components/Minibuffer.tsx` - Message display / interactive prompt input
- `src/components/REPL.tsx` - Output display and interactive REPL (currently unused in App.tsx)
- `src/lib/fileOps.ts` - Wrappers around the Tauri file-system commands
- `src/lib/prefixKeymap.ts` - C-x C-f / C-x C-s prefix key detection
- `src/lib/schemeTokenizer.ts`, `src/lib/bracketMatch.ts`, `src/lib/emacsKeymap.ts`,
  `src/lib/bracketAutoClose.ts` - Syntax highlighting / bracket matching / Emacs
  keybinding logic, implemented but not yet wired into `CodeEditor` (deferred feature)
- `vite.config.ts` - Vite configuration (also holds the Vitest `test` config)

### Backend (Scheme Interpreter + file I/O)
- `src-tauri/src/main.rs` - Tauri app setup, `eval_scheme` and file I/O commands
  (`read_file_content`, `write_file_content`, `path_exists`, `home_dir`)
- `src-tauri/src/scheme/parser.rs` - Scheme expression parser (+ unit tests)
- `src-tauri/src/scheme/evaluator.rs` - Expression evaluator (+ unit tests)
- `src-tauri/src/scheme/value.rs` - Value type definitions (+ unit tests)
- `src-tauri/tauri.conf.json` - Tauri app configuration

## Development Workflow

1. **Start dev server**: `npm run tauri:dev`
2. **Edit React files** in `src/` - changes hot-reload automatically
3. **Edit Rust files** in `src-tauri/src/` - app reloads on save
4. **Run tests**: `npm run test` (Vitest for `src/`, `cargo test` for `src-tauri/`)
5. **Lint**: `npm run lint` (ESLint + `cargo clippy -D warnings`)
6. **Format**: `npm run format` (Prettier + `cargo fmt`), or `npm run format:check` to verify only
7. **Run everything CI checks**: `npm run check` (format:check + lint + test)

CI (`.github/workflows/ci.yml`) runs `npm run check` and `npm run build` on every push/PR to `main`.

## Adding Features

### Adding Scheme Built-in Functions
Edit `src-tauri/src/scheme/evaluator.rs`:
1. Add new match arm in `eval_builtin()` function
2. Implement the logic using pattern matching on `Value` types
3. Return `Value` result or error string

### Updating the UI
Edit components in `src/`:
1. Components are in `src/components/`
2. Styles are in `src/styles/`
3. Update `App.tsx` for state/prop changes

### Tauri Commands
To add new backend functions callable from React:
1. Add `#[tauri::command]` function in `src-tauri/src/main.rs`
2. Add to `.invoke_handler(tauri::generate_handler![...])` list
3. Call from React: `await invoke('function_name', { arg: value })`

## Current Implementation Status

### Implemented
- ✅ Parser: Lists, symbols, numbers, strings, booleans
- ✅ Basic arithmetic: +, -, *, /
- ✅ List operations: quote, list
- ✅ REPL with history (last 10 items)
- ✅ Error handling and display
- ✅ Dark theme UI

### TODO
- [ ] More operators: >, <, =, etc.
- [ ] Control flow: if, cond, case
- [ ] Variable binding: define, let, let*, letrec
- [ ] Lambda: lambda, define-syntax
- [ ] List operations: car, cdr, cons, append, length, map
- [ ] Type checking: integer?, string?, etc.
- [ ] Comments in editor (already parsed)
- [ ] Syntax highlighting (consider Monaco Editor)
- [ ] Better error messages with line numbers

## Building and Distribution

```bash
# Development
npm run tauri:dev

# Build for current platform
npm run tauri:build

# Built app location
src-tauri/target/release/rseditor  # or .exe on Windows, .app on macOS
```

## Tips

- Frontend hot-reload works via Vite dev server
- Rust code changes require app reload (happens automatically)
- Check console for errors: Ctrl+Shift+I in Tauri app
- Rust compilation can be slow on first build
