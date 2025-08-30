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
    for (auto i = 0; i < 2 * ColorCountManx; i++)
    {
        const auto backBase = i / ColorCountManx == 0
                                  ? BackColorBase
                                  : BackLightColorBase;
        const auto backColor = backBase + i % ColorCountManx;

        for (auto j = 0; j < 2 * ColorCountManx; j++)
        {
            const auto foreBase = j / ColorCountManx == 0
                                      ? ForeColorBase
                                      : ForeLightColorBase;
            const auto foreColor = foreBase + j % ColorCountManx;

            Serial.print("\x1b[" + String(foreColor) + ";" + String(backColor) + "m");
            Serial.print("\\x1b[" + String(foreColor) + ";" + String(backColor) + "m");
            Serial.println("\x1b[0m");

            delay(100);
        }
    }
}