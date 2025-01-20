#pragma once

#include <filesystem>
#include <vector>

class Day1
{
public:
    Day1();
    ~Day1() = default;

    void runPart1();
    void runPart2();

private:
    std::vector<int> leftNumbers;
    std::vector<int> rightNumbers;
};
