#include <Arduino.h>

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    static auto count = 0;

    Serial.println(count);
    delay(100);
    count++;

    if (count % 100 == 0)
    {
        count = 0;
    }
}
