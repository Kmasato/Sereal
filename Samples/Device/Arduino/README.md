
# Arduino Samples

This directory contains sample Arduino sketches that can be used as a counterpart for Sereal.

## How to Compile and Upload using arduino-cli

This guide explains how to compile and upload sketches using `arduino-cli`.

### 1. Install arduino-cli

If you haven't installed `arduino-cli`, please refer to the official documentation to install it.

- [arduino-cli official documentation](https://arduino.github.io/arduino-cli/latest/installation/)

### 2. Update Core Index

Update the list of available platforms.

```bash
arduino-cli core update-index
```

### 3. Install the necessary Core

Install the core for your board. The example below is for an Arduino Uno (AVR core). Please replace `arduino:avr` with the appropriate core for your board.

```bash
arduino-cli core install arduino:avr
```

To find the right core for your board, you can search for it. For example, to find a core for the "ESP32", you would run:
```bash
arduino-cli core search esp32
```

### 4. Compile the Sketch

Compile the sketch. You need to specify the board identifier for your board. This is specified with the `--fqbn` (or `-b`) flag. You can find a list of available board identifiers by running `arduino-cli board listall`. The example below uses `arduino:avr:uno` for an Arduino Uno.

```bash
arduino-cli compile -b arduino:avr:uno Samples/Device/Arduino/SampleUartLog
```

### 5. Upload the Sketch

First, connect your Arduino board to your PC. Then, find the port it is connected to.

```bash
arduino-cli board list
```

Upload the sketch using the board identifier and the port you found. Please replace `/dev/cu.usbmodem14101` with your actual port.

```bash
arduino-cli upload -b arduino:avr:uno -p /dev/cu.usbmodem14101 Samples/Device/Arduino/SampleUartLog
```

---

# Arduino サンプル

このディレクトリには、Serealの対向として動作させるArduinoのサンプルスケッチが含まれています。

## arduino-cli を利用したコンパイルと書き込みの方法

`arduino-cli` を使ってスケッチをコンパイルし、書き込む方法を説明します。

### 1. arduino-cli のインストール

`arduino-cli` をインストールしていない場合は、公式ドキュメントを参照してインストールしてください。

- [arduino-cli 公式ドキュメント](https://arduino.github.io/arduino-cli/latest/installation/)

### 2. コアのインデックスを更新

利用可能なプラットフォームのリストを更新します。

```bash
arduino-cli core update-index
```

### 3. 必要なコアをインストール

お使いのボードに対応するコアをインストールします。以下の例は Arduino Uno (AVRコア) のためのものです。お使いのボードに合わせて `arduino:avr` の部分を適宜変更してください。

```bash
arduino-cli core install arduino:avr
```

ボードに適したコアを探すには、検索機能が利用できます。例えば "ESP32" のコアを探す場合は、次のように実行します。
```bash
arduino-cli core search esp32
```

### 4. スケッチをコンパイル

スケッチをコンパイルします。コンパイルには、お使いのボードに対応するボード識別子を `--fqbn` (または `-b`) フラグで指定する必要があります。`arduino-cli board listall` を実行すると、利用可能なボード識別子のリストを確認できます。以下の例では、Arduino Uno を示す `arduino:avr:uno` を使用しています。

```bash
arduino-cli compile -b arduino:avr:uno Samples/Device/Arduino/SampleUartLog
```

### 5. スケッチを書き込む

まず、ArduinoボードをPCに接続します。次に、接続されているポートを調べます。

```bash
arduino-cli board list
```

ボード識別子とポートを指定して、スケッチを書き込みます。`/dev/cu.usbmodem14101` の部分は、実際に表示されたポート名に置き換えてください。

```bash
arduino-cli upload -b arduino:avr:uno -p /dev/cu.usbmodem14101 Samples/Device/Arduino/SampleUartLog
```
