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
- `src/App.tsx` - Main app component, state management
- `src/components/Editor.tsx` - Code editor with keyboard shortcuts
- `src/components/REPL.tsx` - Output display and interactive REPL
- `vite.config.ts` - Vite configuration

### Backend (Scheme Interpreter)
- `src-tauri/src/main.rs` - Tauri app setup and `eval_scheme` command
- `src-tauri/src/scheme/parser.rs` - Scheme expression parser
- `src-tauri/src/scheme/evaluator.rs` - Expression evaluator
- `src-tauri/src/scheme/value.rs` - Value type definitions
- `src-tauri/tauri.conf.json` - Tauri app configuration

## Development Workflow

1. **Start dev server**: `npm run tauri:dev`
2. **Edit React files** in `src/` - changes hot-reload automatically
3. **Edit Rust files** in `src-tauri/src/` - app reloads on save
4. **Test in the UI** - manually test features

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
