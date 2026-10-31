# Sereal

Sereal is a cross-platform serial monitor application implemented in Rust.  
It allows you to manage multiple ports simultaneously using a tabbed interface.  
Confirmed to work on Windows and macOS.

<p align="center">
<img src="./Docs/Images/sereal_sample_view.png" width="600">
</p>

[Go to Japanese README / 日本語はこちら](#japanese)

# Features

- Serial communication (transmission and reception)
- Tab splitting
- Log search

# Usage

Sereal is currently a work-in-progress project.  
Please download and use the latest version from [Releases](https://github.com/Kmasato/Sereal/releases).  
As Sereal is a personal hobby project, the software is not digitally signed.  
Please follow these steps when running the app for the first time:

## macOS

- When you run Sereal.app for the first time, it cannot be opened because the app cannot be verified.
- Allow the application to run from **System Settings > Privacy & Security > Security**.
- Open Sereal.app again.

## Windows

- When you open Sereal.exe for the first time, you may see "Windows protected your PC".
- Select **"More info"** and choose **"Run anyway"**.

# Development

## Prerequisites

- Node.js
- Rust

## Build and Run

To build and run the application locally, use the following commands:

```bash
cd Sereal
npm install
npm run tauri dev
```

To build the application:

```bash
cd Sereal
npm run tauri build
```

## Project Structure

- `Sereal/`: Source code of the application.
- `Samples/`: Sample firmware for various devices.
  - `Samples/Device/Arduino/`: Arduino sketches used to verify each function of Sereal.

# License

This project is licensed under the MIT License.  
See the [LICENSE](LICENSE) file for details.

<br>
<br>

---

<br>
<br>

<a id="japanese"></a>

# Sereal (Japanese)

Sereal は、Rust で実装されたクロスプラットフォーム対応のシリアルモニタアプリケーションです。  
Sereal ではタブ機能を用いて複数のポートを同時に扱うことができます。  
Windows, macOS で動作確認をしています。

<p align="center">
<img src="./Docs/Images/sereal_sample_view.png" width="600">
</p>

# 主な機能

- シリアル通信の受信・送信機能
- タブ分割機能
- ログ検索機能

# 利用方法

Sereal は試作中のプロジェクトです。  
最新版は [Releases](https://github.com/Kmasato/Sereal/releases) からダウンロードして利用してください。  
Sereal は個人の趣味プロジェクトのため、ソフトウェアの署名が行われていません。  
初回は以下の手順で開いてください。

## macOS

- 初めて Sereal.app を実行するとアプリの検証ができないため、開くことができません。
- システム設定 > プライバシーとセキュリティ > セキュリティ からアプリケーションの実行を許可してください。
- 再度、Sereal.app を開いてください。

## Windows

- 初めて Sereal.exe を開いた場合、「Windows によって PC が保護されました」と表示されます。
- 「詳細情報」を選択し、実行してください。

# 開発手順

## 前提条件

- Node.js
- Rust

## ビルドと実行

ユーザーの手元でビルド、実行する場合には以下のコマンドを使用してください：

```bash
cd Sereal
npm install
npm run tauri dev
```

ビルドを行う場合：

```bash
cd Sereal
npm run tauri build
```

## プロジェクト構成

- `Sereal/`: アプリケーション本体のソースコード。
- `Samples/`: 各種デバイス用のサンプルファームウェア。
  - `Samples/Device/Arduino/`: Sereal の各機能の動作確認に利用する Arduino スケッチ。

# ライセンス

このプロジェクトはMITライセンスの下で公開されています。  
詳細は [LICENSE](LICENSE) ファイルを参照してください。
