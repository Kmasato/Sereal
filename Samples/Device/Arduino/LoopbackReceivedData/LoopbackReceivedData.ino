#include <Arduino.h>

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    char receivedBuffer[128];
    String receivedHexText;

    if (Serial.available())
    {
        auto receivedSize =
            Serial.readBytes(receivedBuffer, sizeof(receivedBuffer));

        receivedHexText = "";
        receivedHexText.reserve(receivedSize * 5); // "0xXX," = 最大5文字

        for (auto i = 0; i < receivedSize; i++)
        {
            char hex[8];
            snprintf(hex, sizeof(hex), "0x%02x,", (unsigned char)receivedBuffer[i]);
            receivedHexText += hex;
        }

        Serial.print("Received:");
        Serial.write(receivedBuffer, receivedSize);
        Serial.println(("(" + receivedHexText + ")").c_str());
    }
}