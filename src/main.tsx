/**
 * フロントエンドの入口. ログを設定して, アプリの画面 ({@link App}) を描きます.
 */

import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import { setupLogging } from './lib/logForwarder'

// バックエンドのログファイルにも残すため, 転送を設定して, レベルを決める (開発版はデバッグ, リリース版は情報).
setupLogging()

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
