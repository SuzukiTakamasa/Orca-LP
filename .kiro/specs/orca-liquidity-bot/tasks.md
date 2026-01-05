# Implementation Plan: Orca Liquidity Bot

## Overview

Rustを使用してOrcaプールでの流動性供給ポジションを自動管理するbotシステムを実装する。Google Cloud Runでホストされ、Cloud Schedulerによって定期実行される。段階的に各コンポーネントを実装し、プロパティベーステストで正確性を検証する。

## Tasks

- [ ] 1. プロジェクト構造とコア型定義の設定
  - Cargoプロジェクトの初期化とディレクトリ構造の作成
  - 依存関係の設定（tokio, serde, reqwest, thiserror, proptest等）
  - コアデータ型とエラー型の定義
  - _Requirements: 全般_

- [ ]* 1.1 プロジェクト構造のプロパティテスト
  - **Property 15: Configuration validation**
  - **Validates: Requirements 6.3**

- [ ] 2. Price Monitorコンポーネントの実装
  - [ ] 2.1 PriceMonitor構造体と基本メソッドの実装
    - 価格データ取得のためのHTTPクライアント設定
    - 現在価格と履歴価格の取得メソッド実装
    - _Requirements: 4.1, 4.2_

  - [ ]* 2.2 価格データ取得のプロパティテスト
    - **Property 9: Price data fetching reliability**
    - **Validates: Requirements 4.1, 4.2**

  - [ ] 2.3 価格レンジチェック機能の実装
    - レンジ内外判定ロジックの実装
    - _Requirements: 1.1_

  - [ ]* 2.4 レンジ監視精度のプロパティテスト
    - **Property 1: Range monitoring accuracy**
    - **Validates: Requirements 1.1**

  - [ ] 2.5 価格データエラーハンドリングの実装
    - リトライ機能とエラー処理の実装
    - _Requirements: 4.3, 4.4_

  - [ ]* 2.6 価格データエラーハンドリングのプロパティテスト
    - **Property 10: Price data error handling**
    - **Property 11: Price data validation**
    - **Validates: Requirements 4.3, 4.4**

- [ ] 3. Position Managerコンポーネントの実装
  - [ ] 3.1 PositionManager構造体とOrca SDK統合
    - Orca Whirlpool SDKの統合
    - ウォレット接続とポジション取得機能
    - _Requirements: 5.1, 5.2_

  - [ ] 3.2 最適レンジ計算アルゴリズムの実装
    - 市場条件に基づくレンジ計算ロジック
    - _Requirements: 1.3, 5.1_

  - [ ]* 3.3 最適レンジ計算のプロパティテスト
    - **Property 3: Optimal range calculation**
    - **Validates: Requirements 1.3, 5.1**

  - [ ] 3.4 Yield回収とポジション操作の実装
    - Yield回収機能の実装
    - ポジションのクローズと作成機能
    - _Requirements: 2.1, 2.2, 5.2, 5.3_

  - [ ]* 3.5 ポジション操作のプロパティテスト
    - **Property 12: Position operation atomicity**
    - **Property 5: Yield inclusion in repositioning**
    - **Validates: Requirements 2.2, 5.3**

  - [ ] 3.6 ポジション操作エラーハンドリング
    - エラー処理とリトライ機能の実装
    - _Requirements: 5.4_

  - [ ]* 3.7 ポジション操作エラーハンドリングのプロパティテスト
    - **Property 13: Position operation error handling**
    - **Validates: Requirements 5.4**

- [ ] 4. LINE Notifierコンポーネントの実装
  - [ ] 4.1 LineNotifier構造体とLINE SDK統合
    - LINE Messaging API SDKの統合
    - 基本的な通知送信機能
    - _Requirements: 3.1, 3.2_

  - [ ] 4.2 通知メッセージフォーマット機能の実装
    - 日本語メッセージフォーマット機能
    - リポジション通知と日次Yield通知のテンプレート
    - _Requirements: 3.3_

  - [ ]* 4.3 通知内容とフォーマットのプロパティテスト
    - **Property 6: Notification content completeness**
    - **Property 7: Japanese message formatting**
    - **Validates: Requirements 3.1, 3.2, 3.3**

  - [ ] 4.4 通知リトライ機能の実装
    - 送信失敗時のリトライロジック
    - _Requirements: 3.4_

  - [ ]* 4.5 通知リトライ機能のプロパティテスト
    - **Property 8: Notification retry mechanism**
    - **Validates: Requirements 3.4**

- [ ] 5. State Managerコンポーネントの実装
  - [ ] 5.1 StateManager構造体と状態永続化機能
    - ボット状態の保存と読み込み機能
    - JSON形式での状態管理
    - _Requirements: 全般（状態管理）_

  - [ ]* 5.2 状態管理のユニットテスト
    - 状態の保存と読み込みのテスト
    - エラーケースのテスト

- [ ] 6. Scheduler Handlerとメインロジックの実装
  - [ ] 6.1 SchedulerHandler構造体の実装
    - 時間別チェックと日次Yield回収のハンドラー
    - 各コンポーネントの統合
    - _Requirements: 1.2, 2.1_

  - [ ]* 6.2 リポジション条件のプロパティテスト
    - **Property 2: Repositioning trigger conditions**
    - **Property 4: Daily yield collection timing**
    - **Validates: Requirements 1.2, 2.1**

  - [ ] 6.3 システムエラーロギングの実装
    - 詳細なエラーログ機能
    - _Requirements: 6.2_

  - [ ]* 6.4 システムエラーロギングのプロパティテスト
    - **Property 14: System error logging**
    - **Validates: Requirements 6.2**

- [ ] 7. Cloud Run統合とHTTPサーバーの実装
  - [ ] 7.1 HTTPサーバーとエンドポイントの実装
    - Cloud Schedulerからのリクエストを処理するHTTPサーバー
    - ヘルスチェックエンドポイント
    - _Requirements: 6.1_

  - [ ] 7.2 環境変数と設定管理の実装
    - 必要な環境変数の読み込みと検証
    - 設定の初期化処理
    - _Requirements: 6.3_

- [ ] 8. 統合テストとデプロイメント準備
  - [ ] 8.1 ユニット統合テストの実装
    - モックを使用したコンポーネント間の統合テスト
    - エラーケースとエッジケースのテスト
    - _Requirements: 全般_

  - [ ]* 8.2 Solana devnet統合テストの実装
    - devnet環境でのOrcaプール操作テスト
    - 実際のブロックチェーン環境でのポジション作成・管理テスト
    - ネットワーク障害とトランザクション確認のテスト
    - _Requirements: 5.1, 5.2, 5.3_

  - [ ]* 8.3 devnetテスト環境の設定
    - テスト用ウォレットとdevnet SOL/USDCの設定
    - Orca devnetプールとの接続設定
    - テストデータのクリーンアップ機能
    - _Requirements: 6.3_

  - [ ] 8.4 Dockerfileとデプロイメント設定
    - Cloud Run用のDockerfile作成
    - 必要な環境変数とシークレットの設定
    - _Requirements: 6.1_

  - [ ] 8.5 Cloud Scheduler設定ドキュメント
    - 時間別チェック（毎時）と日次Yield回収（毎日0時）のスケジュール設定
    - デプロイメント手順の文書化
    - _Requirements: 6.1_

- [ ] 9. 最終チェックポイント
  - すべてのテストが通ることを確認し、ユーザーに質問があれば確認する

## Notes

- `*`マークのタスクはオプションで、より迅速なMVPのためにスキップ可能
- 各タスクは特定の要件への追跡可能性のために要件を参照
- チェックポイントで段階的な検証を確保
- プロパティテストで普遍的な正確性プロパティを検証
- ユニットテストで特定の例とエッジケースを検証
- devnet統合テストで実際のブロックチェーン環境での動作を検証
- devnetテストは実資金を使わずに安全にテスト可能