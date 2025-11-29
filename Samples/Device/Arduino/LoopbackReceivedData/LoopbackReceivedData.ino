#include <Arduino.h>

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    char receivedBuffer[128];
    if (Serial.available())
    {
        auto receivedSize =
            Serial.readBytes(receivedBuffer, sizeof(receivedBuffer) / sizeof(char));
        Serial.print("Received:");
        Serial.write(receivedBuffer, receivedSize);
    }
}