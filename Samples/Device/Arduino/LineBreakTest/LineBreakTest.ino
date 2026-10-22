
#include <Arduino.h>

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    Serial.println("=== Line Ending Test Start ===");
    delay(1000);

    // 1. CRLF (\r\n) - 一般的な Windows / Serial.println() の標準
    Serial.print("[Test 1] CRLF (\\r\\n): Hello Sereal");
    Serial.print("\r\n");
    delay(1000);

    // 2. LF のみ (\n) - UNIX / Linux スタイル
    Serial.print("[Test 2] LF only (\\n): Hello Sereal");
    Serial.print("\n");
    delay(1000);

    // 3. CR のみ (\r) - 古い Mac や進捗表示形式（行頭復帰のみ）
    // ※ Sereal では無視する
    Serial.print("[Test 3] CR only (\\r): Hello Sereal");
    Serial.print("\r");
    delay(1000);

    // 4. 改行なし (No newline) - バッファフラッシュや即時描画の確認
    Serial.print("[Test 4] No newline: [Chunk 1] ");
    delay(500);
    Serial.print("[Chunk 2] ");
    delay(500);
    Serial.print("[Chunk 3]");
    Serial.print("\r\n");
    delay(1000);

    // 5. 連続出力
    for (int i = 0; i < 3; i++)
    {
        Serial.print("[Test 5] Fast loop counter: ");
        Serial.println(i);
        delay(100);
    }

    Serial.println("=== Line Ending Test End (Wait 3s) ===\r\n");
    delay(3000);
}
