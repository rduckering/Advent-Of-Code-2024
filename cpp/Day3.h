#pragma once

#include <array>
#include <string>
#include <vector>

class Day3
{
public:
    Day3();
    ~Day3() = default;

    void runPart1();
    void runPart2();

private:
    // mul(111,222)

    std::vector<std::string> validInstructions;

    static void processInput (const std::vector<std::string>& contentLines, std::vector<std::string>& instructions);
};
