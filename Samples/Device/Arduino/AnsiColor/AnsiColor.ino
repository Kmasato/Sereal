#include <Arduino.h>

constexpr auto ColorCountManx = 8;
constexpr auto ForeColorBase = 30;
constexpr auto ForeLightColorBase = 90;
constexpr auto BackColorBase = 40;
constexpr auto BackLightColorBase = 100;

void setup()
{
    Serial.begin(115200);
}

void loop()
{
    for (auto i = -1; i < 2 * ColorCountManx; i++)
    {

        String backColorStr = "";
        if (0 <= i)
        {
            const auto backBase = i / ColorCountManx == 0
                                      ? BackColorBase
                                      : BackLightColorBase;
            const auto backColor = backBase + i % ColorCountManx;
            backColorStr = ";" + String(backColor);
        }

        for (auto j = 0; j < 2 * ColorCountManx; j++)
        {
            const auto foreBase = j / ColorCountManx == 0
                                      ? ForeColorBase
                                      : ForeLightColorBase;
            const auto foreColor = foreBase + j % ColorCountManx;

            Serial.print("\x1b[" + String(foreColor) + backColorStr + "m");
            Serial.print("\\x1b[" + String(foreColor) + backColorStr + "m");
            Serial.print("\x1b[0m");
            if (j == ColorCountManx - 1)
            {
                Serial.println("");
            }

            delay(100);
        }
        Serial.println();
    }
}