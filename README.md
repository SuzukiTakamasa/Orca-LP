# Orcaのプールに流動性供給したSOl-USDCのポジションの収益性の確保を自動化するbot

## 1.仕様

### Positionのレンジ監視
→1時間おきにセットしたPositionがレンジの範囲内かどうかを監視し、現在価格と1時間前の価格がいずれもレンジから逸脱している場合はYieldの回収とPositionの引き直しを自動で行う
→引き直し後、

1. 新しいPositionのレンジ
2. Positionの総残高
3. SOL/USDCの比率
4. 回収したYieldの量
5. SOLのUSDC建ての価格

をLINE botに通知する

### Yieldの回収
→1日に一回0時にYieldを回収し、Positionを引き直す
→上記のタイミングで

1. Positionの総残高
2. SOL/USDCの比率
3. 回収したYieldの量
4. SOLのUSDC建ての価格

をLINE botに通知する

## 2.技術選定

### 言語
Rust

### デプロイ先
Google Cloud(Cloud Run)